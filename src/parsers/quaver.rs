use crate::models;
use crate::models::generic;
use crate::models::common::{
    GameMode,
    Key,
    TimingChangeType
};
use crate::models::generic::sound::{SoundBank, SoundEffect};
use crate::models::quaver;
use crate::errors;

fn process_timing_points(timing_points: Vec<quaver::timing_points::TimingPoint>,
    chartinfo: &mut generic::chartinfo::ChartInfo,
    timeline: &mut models::timeline::TimingPointTimeline) -> Result<(), Box<dyn std::error::Error>> {
    use models::timeline::TimelineTimingPoint;
    
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

fn process_sv(slider_velocities: Vec<quaver::timing_points::SliderVelocity>,
    timeline: &mut models::timeline::TimingPointTimeline) -> Result<(), Box<dyn std::error::Error>> {
    use models::timeline::TimelineTimingPoint;
    
    for sv in slider_velocities {
        timeline.add_sorted(TimelineTimingPoint {
            time: sv.start_time as i32,
            value: sv.multiplier.unwrap_or(1.0),
            change_type: TimingChangeType::Sv,
        });
    }
    Ok(())
}

fn process_soundeffects(sound_effects: Vec<quaver::sound::SoundEffect>, soundbank: &mut SoundBank) -> Result<(), Box<dyn std::error::Error>> {
    for sound_effect in sound_effects {
        soundbank.add_sound_effect(SoundEffect {
            time: sound_effect.start_time as i32,
            volume: sound_effect.volume,
            sample: sound_effect.sample
        });
    }
    Ok(())
}

fn process_samples(samples: Vec<quaver::sound::AudioSample>, soundbank: &mut SoundBank) -> Result<(), Box<dyn std::error::Error>> {
    for sample in samples {
        soundbank.add_sound_sample(sample.path);
    }
    Ok(())
}

fn process_notes(quaver_hitobjects: Vec<quaver::hitobjects::HitObject>,
    hitobjects: &mut generic::hitobjects::HitObjects,
    chartinfo: &mut generic::chartinfo::ChartInfo,
    bpms_times: &Vec<i32>,
    bpms: &Vec<f32>,
    has_scratch: bool
    ) -> Result<(), Box<dyn std::error::Error>> {
        use models::timeline::{HitObjectTimeline, TimelineHitObject};
        let mut key_count = chartinfo.key_count as usize;
        
        let mut timeline: HitObjectTimeline = HitObjectTimeline::with_capacity((quaver_hitobjects.len() / 3) as usize);

        if has_scratch {
            key_count += 1;
        }

        for hitobject in quaver_hitobjects {
            let lane = hitobject.lane() - 1;
            
            if hitobject.is_ln() {
                let slider = TimelineHitObject {
                    time: hitobject.start_time() as i32,
                    column: lane,
                    key: Key::slider_start(Some(hitobject.end_time().unwrap_or(0.0) as i32)),
                    keysound: Some(hitobject.get_generic_keysound())
                };

                let slider_end = TimelineHitObject {
                    time: hitobject.end_time().unwrap_or(0f32) as i32,
                    column: lane,
                    key: Key::slider_end(),
                    keysound: None
                };
            
                timeline.add_sorted(slider);
                timeline.add_sorted(slider_end);
            } else {
                timeline.add_sorted(
                TimelineHitObject {
                        time: hitobject.start_time() as i32,
                        column: lane,
                        key: Key::normal(),
                        keysound: Some(hitobject.get_generic_keysound())
                    }
                );
            }
        }

        chartinfo.key_count = key_count as u8;
        timeline.to_hitobjects(hitobjects, chartinfo.audio_offset, key_count, bpms_times, bpms);
        
        Ok(())
}

pub(crate) fn from_qua(raw_chart: &str) -> Result<generic::chart::Chart, Box<dyn std::error::Error>>  {
    use generic::{
        metadata::Metadata,
        chartinfo::ChartInfo,
        timing_points::TimingPoints,
        hitobjects::HitObjects,
        chart::Chart,
        sound::SoundBank
    };
    use models::timeline::TimingPointTimeline;
    use quaver::chart::QuaFile;

    let quaver_file: QuaFile = QuaFile::from_yaml(&raw_chart)?;

    let key_count = match quaver_file.mode.as_str() {
        "Keys4" => 4,
        "Keys7" => 7,
        _ => return Err(Box::new(errors::ParseError::<GameMode>::InvalidChart(
            "Quaver only supports Keys4 and Keys7 for Mode".to_string()
        ))),
    };

    let metadata = Metadata {
        title: quaver_file.title,
        artist: quaver_file.artist,
        source: quaver_file.source,
        tags: quaver_file.tags.split(" ").map(|s| s.to_string()).collect(),
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
    let mut timeline: TimingPointTimeline = TimingPointTimeline::with_capacity(64);

    process_samples(quaver_file.custom_audio_samples, &mut soundbank)?;
    process_soundeffects(quaver_file.sound_effects, &mut soundbank)?;
    process_timing_points(quaver_file.timing_points, &mut chartinfo, &mut timeline)?;
    process_sv(quaver_file.slider_velocities, &mut timeline)?;
    timeline.to_timing_points(&mut timing_points, chartinfo.audio_offset);
    process_notes(
        quaver_file.hitobjects,
        &mut hitobjects, 
        &mut chartinfo,
        &timing_points.times, 
        &timing_points.bpms(),
    quaver_file.has_scratch_key)?;

    Ok(Chart::new(metadata, chartinfo, timing_points, hitobjects, Some(soundbank)))
}
