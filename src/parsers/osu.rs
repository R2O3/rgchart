use crate::models::generic::sound::KeySound;
use crate::models::osu::*;
use crate::models::generic;
use crate::models::common::{
    GameMode,
    Key,
    TimingChangeType,
};
use crate::utils::serde::process_bracket_sections;
use crate::utils::rhythm::{
    calculate_beat_from_time,
};
use crate::errors;
use std::str::FromStr;

fn validate_mode_mania(mode: GameMode) -> Result<bool, Box<dyn std::error::Error>> {
    if mode != GameMode::Mania {
        return Err(Box::new(errors::ParseError::InvalidMode(mode.to_string(), GameMode::Mania)));
    }
    Ok(true)
}

pub(crate) fn from_osu_raw(raw_chart: &str) -> Result<chart::OsuFile, Box<dyn std::error::Error>> {
    let mut general: general::General = Default::default();
    let mut editor: editor::Editor = Default::default();
    let mut metadata: metadata::Metadata = Default::default();
    let mut difficulty: difficulty::Difficulty = Default::default();
    let mut events: events::Events = Default::default();
    let mut timing_points: timing_points::TimingPoints = Default::default();
    let mut hitobjects: hitobjects::HitObjects = Default::default();

    process_bracket_sections(raw_chart, |section, content| {
        match section {
            "General" => general = general::General::from_str(content)?,
            
            "Editor" => editor = editor::Editor::from_str(content)?,
    
            "Metadata" => metadata = metadata::Metadata::from_str(content)?,

            "Difficulty" => difficulty = difficulty::Difficulty::from_str(content)?,

            "Events" => events = events::Events::from_str(content)?,

            "TimingPoints" => timing_points = timing_points::TimingPoints::from_str(content)?,
            
            "HitObjects" => hitobjects = hitobjects::HitObjects::from_str_with_mode(content, &hitobjects::OsuMode::Mania)?,
            
            _ => { },
        }
        Ok(())
    })?;

    let osu_file = chart::OsuFile { 
        general, 
        editor: Some(editor), 
        metadata, 
        difficulty, 
        events, 
        timing_points, 
        hitobjects 
    };
    
    Ok(osu_file)
}

pub(crate) fn from_osu(raw_chart: &str) -> Result<generic::chart::Chart, Box<dyn std::error::Error>> {
    use generic::{
        metadata::Metadata,
        chartinfo::ChartInfo,
        timing_points::TimingPoints,
        hitobjects::{HitObjects, HitObject},
        chart::Chart,
        sound::SoundBank
    };

    let osu_file = from_osu_raw(raw_chart)?;

    validate_mode_mania(osu_file.general.get_mode())?;

    let metadata = Metadata {
        title: osu_file.metadata.title.clone(),
        alt_title: osu_file.metadata.display_title().to_string(),
        artist: osu_file.metadata.artist.clone(),
        alt_artist: osu_file.metadata.display_artist().to_string(),
        creator: osu_file.metadata.creator,
        tags: osu_file.metadata.tags,
        source: osu_file.metadata.source,
        ..Metadata::empty()
    };

    let mut chartinfo = ChartInfo {
        song_path: osu_file.general.audio_filename,
        preview_time: osu_file.general.preview_time,
        difficulty_name: osu_file.metadata.version,
        od: osu_file.difficulty.overall_difficulty,
        hp: osu_file.difficulty.hp_drain_rate,
        key_count: osu_file.difficulty.circle_size as u8,
        ..ChartInfo::empty()
    };
    
    if let Some(ref bg) = osu_file.events.background {
        chartinfo.bg_path = bg.filename.clone();
    }

    if let Some(ref video) = osu_file.events.video {
        chartinfo.video_path = video.filename.clone();
    }

    let mut timing_points = TimingPoints::with_capacity(osu_file.timing_points.count());
    
    for tp in &osu_file.timing_points.timing_points {
        if tp.is_uninherited() {
            let bpm = tp.bpm().unwrap_or(120.0);
            timing_points.add(tp.time as i32, 0.0, generic::timing_points::TimingChange {
                change_type: TimingChangeType::Bpm,
                value: bpm,
            });
        } else {
            let sv = tp.slider_velocity_multiplier().unwrap_or(1.0);
            timing_points.add(tp.time as i32, 0.0, generic::timing_points::TimingChange {
                change_type: TimingChangeType::Sv,
                value: sv,
            });
        }
    }

    let start_time = *timing_points.bpms_times().first().unwrap_or(&0);
    chartinfo.audio_offset = start_time;

    let bpm_times: Vec<i32> = timing_points.bpms_times();
    let bpms: Vec<f32> = timing_points.bpms();

    timing_points.iter_mut().for_each(|b| {
        b.beat = calculate_beat_from_time(b.time, start_time, (&bpm_times, &bpms));
    });

    let mut hitobjects = HitObjects::with_capacity(osu_file.hitobjects.count());
    let mut soundbank = SoundBank::new();
    soundbank.audio_tracks.push(chartinfo.song_path.clone());

    let key_count = chartinfo.key_count;

    for hit_object in osu_file.hitobjects.iter() {
        let object_time = hit_object.time as i32;
        let object_column = hit_object.mania_column(key_count);

        let beat = calculate_beat_from_time(object_time, start_time, (&bpm_times, &bpms));
        
        let key_sound = hit_object.get_generic_keysound(&mut soundbank);

        if hit_object.is_hold() {
            let end_time = hit_object.end_time().unwrap_or(object_time);
            
            let slider_start = HitObject {
                time: object_time,
                beat: beat,
                lane: object_column,
                key: Key::slider_start(Some(end_time)),
                keysound: key_sound,
            };

            let end_time_beat = calculate_beat_from_time(end_time, start_time, (&bpm_times, &bpms));

            let slider_end = HitObject {
                time: end_time,
                beat: end_time_beat,
                lane: object_column,
                key: Key::slider_end(),
                keysound: KeySound::default(),
            };

            hitobjects.add_hitobject_sorted(slider_start);
            hitobjects.add_hitobject_sorted(slider_end);
        } else if hit_object.is_normal() {
            hitobjects.add_hitobject_sorted(HitObject {
                time: object_time,
                beat: beat,
                lane: object_column,
                key: Key::normal(),
                keysound: key_sound,
            });
        }
    }

    Ok(Chart::new(metadata, chartinfo, timing_points, hitobjects, Some(soundbank)))
}