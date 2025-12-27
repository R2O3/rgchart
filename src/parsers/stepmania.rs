use crate::models;
use crate::models::common::{
    ChartDefaults, GameMode, HitObjectRow, Key, TimingChangeType
};
use crate::models::timeline::{TimelineOps, TimelineTimingPoint};
use crate::utils::string::{
    remove_comments,
    StrDefaultExtension,
    StrNumericDefaultExtension,
    trim_split_iter,
};
use crate::utils::time::{
    to_millis,
    merge_bpm_and_stops,
};
use crate::utils::rhythm::{
    calculate_time_from_beat,
};
use crate::errors;

type BpmsAndStops = (Vec<f32>, Vec<f32>, Vec<TimingChangeType>);

pub fn parse_bpms(raw: &str) -> (Vec<f32>, Vec<f32>) {
    match raw {
        ChartDefaults::RAW_BPMS => return (vec![0.0], vec![*ChartDefaults::BPM]),
        _ => {}
    }

    let mut beats = Vec::new();
    let mut bpms = Vec::new();

    raw.split(',')
        .filter_map(|beat_bpm_str| {
            let mut beat_bpm = beat_bpm_str.trim().split('=');

            if let (Some(beat_str), Some(bpm_str)) = (beat_bpm.next(), beat_bpm.next()) {
                if let (Ok(beat), Ok(bpm)) = (beat_str.parse::<f32>(), bpm_str.parse::<f32>()) {
                    debug_assert_ne!(bpm, 0.0);
                    return Some((beat, bpm));
                }
            }
            None
        })
        .for_each(|(beat, bpm)| {
            beats.push(beat);
            bpms.push(bpm);
        });

    (beats, bpms)
}

pub fn parse_stops(raw: &str) -> (Vec<f32>, Vec<f32>) {
    match raw {
        ChartDefaults::RAW_STOPS => return (vec![], vec![]),
        _ => {}
    }

    let mut beats = Vec::new();
    let mut durations = Vec::new();

    raw.split(',')
        .filter_map(|beat_bpm_str| {
            let mut beat_bpm = beat_bpm_str.trim().split('=');

            if let (Some(beat_str), Some(duration_str)) = (beat_bpm.next(), beat_bpm.next()) {
                if let (Ok(beat), Ok(duration)) = (beat_str.parse::<f32>(), duration_str.parse::<f32>()) {
                    return Some((beat, to_millis(duration)));
                }
            }
            None
        })
        .for_each(|(beat, bpm)| {
            beats.push(beat);
            durations.push(bpm);
        });

    (beats, durations)
}


pub fn parse_keys_in_row(row: &str) -> Vec<Key> {

    let mut result: Vec<Key> = Vec::with_capacity(row.len());

    for c in row.chars() {
        result.push(get_sm_note_type(c));
    }

    result
}

#[inline]
pub(crate) fn get_sm_note_type(note: char) -> Key {
    match note {
        '0' => Key::empty(),
        '1' => Key::normal(),
        '2' => Key::slider_start(None),
        '3' => Key::slider_end(),
        '4' => Key::slider_start(None),
        'M' => Key::mine(),
        'F' => Key::fake(),
        _ => Key::unknown(),
    }
}

fn process_sections<F>(raw_chart: &str, mut lambda: F)
where
    F: FnMut(&str, &str),
{
    for pair in raw_chart.split(';') {
        if let Some(colon_index) = pair.find(":") {
            let header = pair[..colon_index].trim();
            let content = pair[colon_index + 1..].trim();

            if let Some(second_colon_index) = content.find(":"){
              let first_content = content[..second_colon_index].trim();
              let second_content = content[second_colon_index + 1..].trim();
              let full_content = format!("{}:{}",first_content,second_content);
              lambda(header, full_content.as_str());
            } else {
              lambda(header, content);
            }
        }
    }
}

fn process_timing_points(
    bpms_and_stops: &BpmsAndStops, 
    start_time: i32
) -> models::generic::timing_points::TimingPoints {
    use models::generic::timing_points::TimingPoints;
    use crate::models::timeline::TimingPointTimeline;
    
    let mut timeline = TimingPointTimeline::new();
    let (beats, bpms_and_durations, change_types) = bpms_and_stops;

    let stops: Vec<_> = beats.iter()
        .zip(bpms_and_durations.iter())
        .zip(change_types.iter())
        .enumerate()
        .filter_map(|(_i, ((beat, value), change_type))| {
            let insert_time = calculate_time_from_beat(
                *beat,
                start_time,
                (beats, bpms_and_durations, change_types)
            );
            
            match change_type {
                TimingChangeType::Bpm => {
                    timeline.add(TimelineTimingPoint {
                        time: insert_time,
                        value: *value,
                        change_type: TimingChangeType::Bpm,
                    });
                    None
                },
                TimingChangeType::Stop => {
                    Some((insert_time, *value))
                },
                _ => None
            }
        })
        .collect();

    for (stop_time, stop_duration) in stops {
        timeline.add(TimelineTimingPoint {
            time: stop_time,
            value: 0.0,
            change_type: TimingChangeType::Sv,
        });
        
        let stop_end_time = stop_time + stop_duration as i32;
        
        timeline.add(TimelineTimingPoint {
            time: stop_end_time,
            value: 1.0,
            change_type: TimingChangeType::Sv,
        });
    }

    let mut timing_points = TimingPoints::with_capacity(timeline.len());
    timeline.to_timing_points(&mut timing_points, start_time);
    
    timing_points
}

fn process_notes(raw_note_data: &str, chartinfo: &mut models::generic::chartinfo::ChartInfo, bpms_and_stops: &BpmsAndStops)
    -> models::generic::hitobjects::HitObjects {
    use crate::models::generic::hitobjects::HitObjects;
    use crate::models::timeline::HitObjectTimeline;

    if raw_note_data.contains("No Note Data") { return HitObjects::with_capacity(2048) }

    let (beats, bpms_and_durations, change_types) = bpms_and_stops;
    
    let start_time = chartinfo.audio_offset;
    let separated_note_data: Vec<&str> = trim_split_iter(raw_note_data.split(":"), false);

    let difficulty_name = separated_note_data[2];
    chartinfo.difficulty_name = difficulty_name.or_default_empty(ChartDefaults::DIFFICULTY_NAME);
    
    // TODO: make error for stepmania if converting from keys other than 4
    let key_count = 4; // TODO: change this later if gonna make this function generic to support Beatmania

    let raw_notes = separated_note_data.last().unwrap_or(&"Failed to get raw notes in notes section");
    let measures: Vec<&str> = raw_notes.split(",").collect();

    let mut measure_beat_count: f32 = 0.0;
    let mut rows = Vec::new();

    for measure in measures {
        let trimmed_measure = measure.trim();
        let measure_rows: Vec<_> = trimmed_measure.split('\n').collect();
        let row_count = measure_rows.len();
        let beat_time_per_row = 4.0 / row_count as f32;
    
        for (row_index, row) in measure_rows.into_iter().enumerate() {
            let row_beat = measure_beat_count + row_index as f32 * beat_time_per_row;
            
            let row_time = calculate_time_from_beat(
                row_beat,
                start_time,
                (beats, bpms_and_durations, change_types)
            );
    
            let keys = parse_keys_in_row(row);
            
            rows.push(HitObjectRow {
                time: row_time,
                beat: row_beat,
                keys,
            });
        }
    
        measure_beat_count += 4.0;
    }

    let flattened = HitObjectTimeline::flatten_rows(&rows, key_count);

    HitObjects::new(flattened)
}

pub(crate) fn from_sm(raw_chart: &str) -> Result<models::generic::chart::Chart, Box<dyn std::error::Error>>  {
    use models::generic::{metadata::Metadata, chartinfo::ChartInfo, chart::Chart};

    let uncommented_chart = remove_comments(raw_chart, "//");

    if uncommented_chart.trim().is_empty() {
        return Err(Box::new(errors::ParseError::<GameMode>::EmptyChartData));
    }

    let mut metadata = Metadata::empty();
    let mut chartinfo = ChartInfo::empty();

    let mut bpms: (std::vec::Vec<f32>, std::vec::Vec<f32>) = (vec![0.0], vec![0.0]);
    let mut raw_bpms = ChartDefaults::RAW_BPMS.to_string();
    let mut stops = (vec![], vec![]);
    let mut raw_stops = ChartDefaults::RAW_STOPS.to_string();
    let mut raw_notes = ChartDefaults::RAW_NOTES.to_string();

    process_sections(&uncommented_chart, |header, content| {
        match header {
            "#TITLE" => metadata.title = content.or_default_empty(ChartDefaults::TITLE),
            "#ARTIST" => metadata.artist = content.or_default_empty(ChartDefaults::ARTIST),
            "#SUBTITLE" => metadata.source = content.or_default_empty(ChartDefaults::SOURCE),
            "#TITLETRANSLIT" => metadata.alt_title = content.or_default_empty(ChartDefaults::ALT_TITLE),
            "#ARTISTTRANSLIT" => metadata.alt_artist = content.or_default_empty(ChartDefaults::ALT_ARTIST),
            "#SUBTITLETRANSLIT" => {},
            "#GENRE" => metadata.genre = content.or_default_empty(ChartDefaults::GENRE),
            "#CREDIT" => metadata.creator = content.or_default_empty(ChartDefaults::CREATOR),
            "#BACKGROUND"=> chartinfo.bg_path = content.or_default_empty(ChartDefaults::BG_PATH),
            "#MUSIC" => chartinfo.song_path = content.or_default_empty(ChartDefaults::SONG_PATH),
            "#OFFSET" => chartinfo.audio_offset = -to_millis(content.or_default_empty_as(*ChartDefaults::AUDIO_OFFSET as f32)) as i32,
            "#SAMPLESTART" => chartinfo.preview_time = to_millis(content.or_default_empty_as(*ChartDefaults::PREVIEW_TIME as f32)) as i32,
            "#BPMS" => {
                raw_bpms = content.or_default_empty(ChartDefaults::RAW_BPMS);
                bpms = parse_bpms(&raw_bpms);
            },
            "#STOPS" => {
                raw_stops = content.or_default_empty(ChartDefaults::RAW_STOPS);
                stops = parse_stops(&raw_stops);
            },
            "#NOTES" => {
                raw_notes = content.or_default_empty(ChartDefaults::RAW_NOTES)
            },
            _ => {},
        }
    });
    
    let bpms_and_stops = merge_bpm_and_stops(bpms.0, bpms.1, stops.0, stops.1);

    let timing_points = process_timing_points(&bpms_and_stops, chartinfo.audio_offset);

    let hitobjects = process_notes(&raw_notes, &mut chartinfo, &bpms_and_stops);

    Ok(Chart::new(metadata, chartinfo, timing_points, hitobjects, None))
}

#[allow(unused)]
pub(crate) fn from_sma(raw_chart: &str) -> Result<models::generic::chart::Chart, Box<dyn std::error::Error>>  {
    unimplemented!();
}

#[allow(unused)]
pub(crate) fn from_ssc(raw_chart: &str) -> Result<models::generic::chart::Chart, Box<dyn std::error::Error>>  {
    unimplemented!();
}