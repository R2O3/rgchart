use crate::models::common::*;
use crate::models::fluxis::{self, FscFile};
use crate::models::generic::{
    GenericManiaChart, ChartInfo, HitObject, HitObjects, 
    KeySound, Metadata, TimingPoints
};
use crate::models::timeline::{TimelineOps, TimelineTimingPoint, TimingPointTimeline};
use crate::utils::rhythm::calculate_beat_from_time;

fn process_timing_points(
    timing_points: Vec<fluxis::TimingPoint>,
    chartinfo: &mut ChartInfo,
    timeline: &mut TimingPointTimeline,
) -> Result<(), Box<dyn std::error::Error>> {
    for timing_point in timing_points {
        timeline.add_sorted(TimelineTimingPoint {
            time: timing_point.time as i32,
            value: timing_point.bpm,
            group: String::new(),
            change_type: TimingChangeType::Bpm,
        });
    }

    let start_time = if timeline.is_empty() {
        0
    } else {
        timeline[0].time
    };

    chartinfo.audio_offset = start_time;
    Ok(())
}

fn add_sv(timeline: &mut TimingPointTimeline, sv: &fluxis::ScrollVelocity, group: &str) {
    timeline.add_sorted(TimelineTimingPoint {
        time: sv.time as i32,
        value: sv.multiplier,
        group: group.to_string(),
        change_type: TimingChangeType::Sv,
    });
}

fn process_sv(
    slider_velocities: Vec<fluxis::ScrollVelocity>,
    timeline: &mut TimingPointTimeline,
    process_single_column_scroll: bool
) -> Result<(), Box<dyn std::error::Error>> {
    for sv in slider_velocities {
        let groups = sv.groups.as_ref();

        match groups {
            None => {
                add_sv(timeline, &sv, "");
            }

            Some(groups) if groups.is_empty() => {
                add_sv(timeline, &sv, "");
            }

            Some(groups) => {
                if process_single_column_scroll {
                    let mut has_dollar = false;

                    for g in groups {
                        if g.starts_with('$') {
                            has_dollar = true;
                        }

                        add_sv(timeline, &sv, g);

                        // this is necessary to make it so barlines are affected by SVs
                        if has_dollar {
                            add_sv(timeline, &sv, "");
                        }
                    }
                }
                else { // in this case any sv with a group starting with '$' will generate a groupless TimelineTimingPoint
                    let mut has_dollar = false;
                    let mut normal_groups = Vec::new();

                    for g in groups {
                        if g.starts_with('$') {
                            has_dollar = true;
                        } else {
                            normal_groups.push(g);
                        }
                    }

                    if has_dollar {
                        add_sv(timeline, &sv, "");
                    }

                    for g in normal_groups {
                        add_sv(timeline, &sv, g);
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_notes(
    fluxis_hitobjects: Vec<fluxis::HitObject>,
    hitobjects: &mut HitObjects,
    chartinfo: &mut ChartInfo,
    offset: i32,
    bpms_times: &Vec<i32>,
    bpms: &Vec<f32>,
    use_column_as_group: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let key_count = chartinfo.key_count as usize;

    for hitobject in fluxis_hitobjects {
        let beat = calculate_beat_from_time(hitobject.time as i32, offset, (bpms_times, bpms));

        let group = if let Some(group) = &hitobject.group {
            if group.is_empty() {
                if use_column_as_group {
                    Some(format!("${}", hitobject.lane))
                }
                else {
                    None
                }
            }
            else {
                Some(group.clone())
            }
        }
        else {
            if use_column_as_group {
                Some(format!("${}", hitobject.lane))
            }
            else {
                None
            }
        };

        if hitobject.is_ln() {
            let slider = HitObject {
                time: hitobject.time as i32,
                beat,
                lane: hitobject.lane as u8,
                key: Key::slider_start(Some(hitobject.end_time() as i32)),
                keysound: KeySound::default(),
                group: group.clone(),
            };

            let slider_end = HitObject {
                time: hitobject.end_time() as i32,
                beat,
                lane: hitobject.lane as u8,
                key: Key::slider_end(),
                keysound: KeySound::default(),
                group: group.clone(),
            };

            hitobjects.add_hitobject_sorted(slider);
            hitobjects.add_hitobject_sorted(slider_end);
        } else {
            if hitobject.is_tick() {
                continue; // skipping ticks until I think of a solution for them
            }
            hitobjects.add_hitobject_sorted(HitObject {
                time: hitobject.time as i32,
                beat,
                lane: hitobject.lane as u8,
                key: Key::normal(),
                keysound: KeySound::default(),
                group: group.clone(),
            });
        }
    }

    chartinfo.key_count = key_count as u8;

    Ok(())
}

pub(crate) fn from_fsc_generic(
    raw_chart: &str,
) -> Result<GenericManiaChart, Box<dyn std::error::Error>> {
    // idk where this should be configurable, but we might want to have this false sometimes to make editing the sv on quaver easier
    let process_single_column_scroll= true;

    let fsc_file = FscFile::from_str(&raw_chart)?;

    let key_count = fsc_file.key_count();

    let metadata = Metadata {
        title: fsc_file.metadata.title.clone(),
        alt_title: fsc_file.metadata.display_title().to_string(),
        artist: fsc_file.metadata.artist.clone(),
        alt_artist: fsc_file.metadata.display_artist().to_string(),
        source: fsc_file
            .metadata
            .source
            .unwrap_or(ChartDefaults::SOURCE.to_string()),
        tags: fsc_file
            .metadata
            .tags
            .split(",")
            .map(|s| s.trim().to_string())
            .collect(),
        creator: fsc_file.metadata.mapper,
        ..Metadata::empty()
    };

    let mut chartinfo = ChartInfo {
        song_path: fsc_file.audio_file,
        preview_time: fsc_file.metadata.previewtime as i32,
        bg_path: fsc_file.background_file,
        video_path: fsc_file.video_file,
        key_count: key_count as u8,
        difficulty_name: fsc_file.metadata.difficulty,
        od: fsc_file.accuracy_difficulty.unwrap_or(8.0),
        hp: fsc_file.health_difficulty.unwrap_or(8.0),
        bpm_affects_sv: false,
        ..ChartInfo::empty()
    };

    let mut timing_points = TimingPoints::with_capacity(64);
    let mut hitobjects = HitObjects::with_capacity(2048);
    let mut timeline = TimingPointTimeline::with_capacity(64);

    let offset = fsc_file.timing_points[0].time as i32;

    process_timing_points(fsc_file.timing_points, &mut chartinfo, &mut timeline)?;
    process_sv(fsc_file.scroll_velocities, &mut timeline, process_single_column_scroll)?;
    timeline.to_timing_points(&mut timing_points, chartinfo.audio_offset);
    process_notes(
        fsc_file.hit_objects,
        &mut hitobjects,
        &mut chartinfo,
        offset,
        &timing_points.bpms_times(),
        &timing_points.bpms(),
        process_single_column_scroll
    )?;

    Ok(GenericManiaChart::new(
        metadata,
        chartinfo,
        timing_points,
        hitobjects,
        None,
    ))
}
