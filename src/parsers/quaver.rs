use std::collections::HashMap;

use crate::errors;
use crate::models::common::*;
use crate::models::generic::{
    self,
    GenericManiaChart,
    ChartInfo,
    HitObjects,
    KeySound,
    Metadata,
    SoundBank,
    SoundEffect,
    TimingPoints
};
use crate::models::quaver::{self, QuaFile};
use crate::models::timeline::{TimelineOps, TimelineTimingPoint, TimingPointTimeline};
use crate::quaver::TimingGroup;
use crate::utils::quaver::get_keycount_from_str;
use crate::utils::rhythm::{calculate_beat_from_time, get_ms_per_beat_at};

fn process_timing_points(
    timing_points: Vec<quaver::TimingPoint>,
    chartinfo: &mut ChartInfo,
    timeline: &mut TimingPointTimeline,
) -> Result<(), Box<dyn std::error::Error>> {
    for timing_point in timing_points {
        timeline.add_sorted(TimelineTimingPoint {
            time: timing_point.start_time as i32,
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

fn process_sv(
    slider_velocities: Vec<quaver::SliderVelocity>,
    timeline: &mut TimingPointTimeline,
) -> Result<(), Box<dyn std::error::Error>> {
    for sv in slider_velocities {
        timeline.add_sorted(TimelineTimingPoint {
            time: sv.start_time as i32,
            value: sv.multiplier.unwrap_or(1.0),
            group: String::new(),
            change_type: TimingChangeType::Sv,
        });
    }
    Ok(())
}

fn process_timing_groups(
    timing_groups: HashMap<String, TimingGroup>,
    timeline: &mut TimingPointTimeline,
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, timing_group) in &timing_groups {
        let quaver::TimingGroup::ScrollGroup(group) = timing_group;
        // TODO: handle initial velocity?

        for sv in &group.scroll_velocities {
            timeline.add_sorted(TimelineTimingPoint {
                time: sv.start_time as i32,
                value: sv.multiplier.unwrap_or(1.0),
                group: name.clone(),
                change_type: TimingChangeType::Sv,
             });
        }
    }
    Ok(())
}

fn process_soundeffects(
    sound_effects: Vec<quaver::SoundEffect>,
    soundbank: &mut SoundBank,
) -> Result<(), Box<dyn std::error::Error>> {
    for sound_effect in sound_effects {
        soundbank.add_sound_effect(SoundEffect {
            time: sound_effect.start_time as i32,
            volume: sound_effect.volume,
            sample: sound_effect.sample,
        });
    }
    Ok(())
}

fn process_samples(
    samples: Vec<quaver::AudioSample>,
    soundbank: &mut SoundBank,
) -> Result<(), Box<dyn std::error::Error>> {
    for sample in samples {
        soundbank.add_sound_sample(sample.path);
    }
    Ok(())
}

fn process_notes(
    quaver_hitobjects: Vec<quaver::HitObject>,
    hitobjects: &mut HitObjects,
    chartinfo: &mut ChartInfo,
    offset: i32,
    bpm_times: &Vec<i32>,
    bpms: &Vec<f32>,
    has_scratch: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut key_count = chartinfo.key_count as usize;

    if has_scratch {
        key_count += 1;
    }

    for hitobject in quaver_hitobjects {
        let lane = hitobject.lane();
        let key_sound = hitobject.get_generic_keysound();
        let time = hitobject.start_time() as i32;

        let beat = calculate_beat_from_time(time, offset, (bpm_times, bpms));

        match hitobject.hit_type {
            quaver::QuaverHitType::NormalOrHold => {
                if hitobject.is_ln() {
                    let slider = generic::HitObject {
                        time,
                        beat,
                        lane,
                        key: Key::slider_start(Some(hitobject.end_time().unwrap_or(0))),
                        keysound: key_sound,
                        group: hitobject.timing_group().map(|s| s.to_string()),
                    };

                    let slider_end_time = hitobject.end_time().unwrap_or(0);
                    let end_time_beat = calculate_beat_from_time(slider_end_time, offset, (bpm_times, bpms));

                    let slider_end = generic::HitObject {
                        time: slider_end_time,
                        beat: end_time_beat,
                        lane,
                        key: Key::slider_end(),
                        keysound: KeySound::default(),
                        group: hitobject.timing_group().map(|s| s.to_string()),
                    };

                    hitobjects.add_hitobject_sorted(slider);
                    hitobjects.add_hitobject_sorted(slider_end);
                } else {
                    hitobjects.add_hitobject_sorted(generic::HitObject {
                        time,
                        beat,
                        lane,
                        key: Key::normal(),
                        keysound: key_sound,
                        group: hitobject.timing_group().map(|s| s.to_string()),
                    });
                }
            },
            quaver::QuaverHitType::Mine => {
                if hitobject.is_ln() {
                    // most games doesn't support LN mines, so let's just put a bunch of mines every 1/4 beat
                    // we do have support for LN mines but, we're expanding the LNs in the quaver parser instead of doing it in every writer, hence Key::mine(None)
                    let end_time = hitobject.end_time().unwrap_or(0) as f32;
                    let mut t = time as f32;
                    let mut last_placed = t;

                    while t < end_time {
                        // in case the bpm changes during the LN mine (idk might happen on some weird maps)
                        let ms_per_beat = get_ms_per_beat_at(t as i32, bpm_times, bpms);
                        let beat = calculate_beat_from_time(t as i32, offset, (bpm_times, bpms));

                        hitobjects.add_hitobject_sorted(generic::HitObject {
                            time: t as i32,
                            beat,
                            lane,
                            key: Key::mine(None),
                            keysound: KeySound::default(),
                            group: hitobject.timing_group().map(|s| s.to_string()),
                        });

                        last_placed = t;
                        t += ms_per_beat / 4.0;
                    }

                    // if the endtime is too far from the last mine we placed then put a mine directly at the endtime
                    let ms_per_beat_at_end = get_ms_per_beat_at(end_time as i32, bpm_times, bpms);
                    if (last_placed - end_time).abs() > ms_per_beat_at_end / 16.0 {
                        let end_beat = calculate_beat_from_time(end_time as i32, offset, (bpm_times, bpms));
                        hitobjects.add_hitobject_sorted(generic::HitObject {
                            time: end_time as i32,
                            beat: end_beat,
                            lane,
                            key: Key::mine(None),
                            keysound: KeySound::default(),
                            group: hitobject.timing_group().map(|s| s.to_string()),
                        });
                    }
                } else {
                    hitobjects.add_hitobject_sorted(generic::HitObject {
                        time,
                        beat,
                        lane,
                        key: Key::mine(None),
                        keysound: key_sound,
                        group: hitobject.timing_group().map(|s| s.to_string()),
                    });
                }
            },
        }
    }

    chartinfo.key_count = key_count as u8;

    Ok(())
}

pub(crate) fn from_qua_generic(
    raw_chart: &str,
) -> Result<GenericManiaChart, Box<dyn std::error::Error>> {
    let quaver_file = QuaFile::from_str(&raw_chart)?;

    let key_count = get_keycount_from_str(quaver_file.mode.as_str())
    .ok_or_else(|| Box::new(errors::ParseError::<GameMode>::InvalidChart(
        "Quaver only supports Keys1 up to Keys10 for Mode".to_string(),
    )))?;

    let metadata = Metadata {
        title: quaver_file.title,
        artist: quaver_file.artist,
        source: quaver_file.source,
        tags: quaver_file
            .tags
            .split(" ")
            .map(|s| s.to_string())
            .collect(),
        creator: quaver_file.creator,
        ..Metadata::empty()
    };

    let mut chartinfo = ChartInfo {
        song_path: quaver_file.audio_file,
        preview_time: quaver_file.song_preview_time as i32,
        bg_path: quaver_file.background_file,
        key_count,
        difficulty_name: quaver_file.difficulty_name,
        bpm_affects_sv: !quaver_file.bpm_does_not_affect_scroll_velocity.unwrap_or(true),
        ..ChartInfo::empty()
    };

    let mut timing_points = TimingPoints::with_capacity(64);
    let mut hitobjects = HitObjects::with_capacity(2048);

    let mut soundbank = SoundBank::new();
    let mut timeline = TimingPointTimeline::with_capacity(64);

    process_samples(quaver_file.custom_audio_samples, &mut soundbank)?;
    process_soundeffects(quaver_file.sound_effects, &mut soundbank)?;
    process_timing_points(quaver_file.timing_points, &mut chartinfo, &mut timeline)?;
    process_sv(quaver_file.slider_velocities, &mut timeline)?;
    process_timing_groups(quaver_file.timing_groups, &mut timeline)?;

    let offset = chartinfo.audio_offset;
    timeline.to_timing_points(&mut timing_points, chartinfo.audio_offset);
    process_notes(
        quaver_file.hitobjects,
        &mut hitobjects,
        &mut chartinfo,
        offset,
        &timing_points.bpms_times(),
        &timing_points.bpms(),
        quaver_file.has_scratch_key,
    )?;

    Ok(GenericManiaChart::new(
        metadata,
        chartinfo,
        timing_points,
        hitobjects,
        Some(soundbank),
    ))
}
