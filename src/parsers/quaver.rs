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
use crate::utils::rhythm::calculate_beat_from_time;

fn process_timing_points(
    timing_points: Vec<quaver::TimingPoint>,
    chartinfo: &mut ChartInfo,
    timeline: &mut TimingPointTimeline,
) -> Result<(), Box<dyn std::error::Error>> {
    for timing_point in timing_points {
        timeline.add_sorted(TimelineTimingPoint {
            time: timing_point.start_time as i32,
            value: timing_point.bpm,
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
            change_type: TimingChangeType::Sv,
        });
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

        if hitobject.is_ln() {
            let slider = generic::HitObject {
                time,
                beat,
                lane,
                key: Key::slider_start(Some(hitobject.end_time().unwrap_or(0.0) as i32)),
                keysound: key_sound,
            };

            let slider_end_time = hitobject.end_time().unwrap_or(0.0) as i32;
            let end_time_beat = calculate_beat_from_time(slider_end_time, offset, (bpm_times, bpms));

            let slider_end = generic::HitObject {
                time: slider_end_time,
                beat: end_time_beat,
                lane,
                key: Key::slider_end(),
                keysound: KeySound::default(),
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
            });
        }
    }

    chartinfo.key_count = key_count as u8;

    Ok(())
}

pub(crate) fn from_qua_generic(
    raw_chart: &str,
) -> Result<GenericManiaChart, Box<dyn std::error::Error>> {
    let quaver_file = QuaFile::from_yaml(&raw_chart)?;

    let key_count = match quaver_file.mode.as_str() {
        "Keys4" => 4,
        "Keys7" => 7,
        _ => {
            return Err(Box::new(errors::ParseError::<GameMode>::InvalidChart(
                "Quaver only supports Keys4 and Keys7 for Mode".to_string(),
            )))
        }
    };

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
