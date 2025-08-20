use crate::models::generic;
use crate::models::generic::sound::{KeySound, KeySoundRow, HitSoundType};
use crate::models::common::{GameMode, KeyType, Row};
use crate::models::quaver::{
    chart::QuaFile,
    // editor,
    sound,
    timing_points,
    hitobjects,
};
use crate::utils::time::find_sliderend_time;
use crate::errors;

fn get_mode_string(key_count: u8) -> Result<String, Box<dyn std::error::Error>> {
    match key_count {
        4 | 7 => Ok(format!("Keys{}", key_count)),
        5 => Ok("Keys4".to_string()),
        8 => Ok("Keys7".to_string()),
        _ => Err(Box::new(errors::WriteError::<GameMode>::InvalidKeyCount(
            key_count,
            "Quaver".to_string(),
            "4k, 4k+1, 7k and 7k+1".to_string(),
        ))),
    }
}

fn get_hitsound_type(hitsound_type: HitSoundType) -> Option<String> {
    match hitsound_type {
        HitSoundType::Normal => None,
        HitSoundType::Clap => Some("Clap".to_string()),
        HitSoundType::Whistle => Some("Whistle".to_string()),
        HitSoundType::Finish => Some("Finish".to_string()),
    }
}

fn create_keysounds(keysound: KeySound) -> Vec<sound::KeySound> {
    if keysound.has_custom && keysound.sample.is_some() {
        vec![sound::KeySound {
            sample: keysound.sample.unwrap() + 1,
            volume: keysound.volume,
        }]
    } else {
        vec![]
    }
}

pub(crate) fn to_qua(chart: &generic::chart::Chart) -> Result<String, Box<dyn std::error::Error>> {
    let key_count = chart.chartinfo.key_count;
    let mode = get_mode_string(key_count)?;

    let custom_audio_samples = match &chart.soundbank {
        Some(soundbank) => soundbank
            .get_sample_paths()
            .iter()
            .map(|path| sound::AudioSample {
                path: path.clone(),
            })
            .collect(),
        None => vec![],
    };

    let sound_effects = match &chart.soundbank {
        Some(soundbank) => soundbank
            .sound_effects
            .iter()
            .map(|effect| sound::SoundEffect {
                start_time: effect.time as f32,
                sample: effect.sample + 1,
                volume: effect.volume,
            })
            .collect(),
        None => vec![],
    };

    let timing_points = chart
        .timing_points
        .bpm_changes_zipped()
        .map(|(time, _, change)| timing_points::TimingPoint {
            start_time: *time as f32,
            bpm: change.value,
        })
        .collect();

    let slider_velocities = chart
        .timing_points
        .sv_changes_zipped()
        .map(|(time, _, change)| timing_points::SliderVelocity {
            start_time: *time as f32,
            multiplier: Some(change.value),
        })
        .collect();

    let hitobjects: Vec<(&i32, &f32, &KeySoundRow, &Row)> = chart.hitobjects.iter_zipped().collect();
    let mut qua_hitobjects = Vec::new();

    for (row_idx, (time, _, keysounds, row)) in hitobjects.iter().enumerate() {
        for (i, key) in row.iter().enumerate() {
            let keysound = if keysounds.is_empty {
                KeySound::normal(100)
            } else {
                keysounds[i]
            };

            match key.key_type {
                KeyType::Normal => {
                    qua_hitobjects.push(hitobjects::HitObject {
                        start_time: **time as f32,
                        lane: i + 1,
                        hit_sound: get_hitsound_type(keysound.hitsound_type),
                        key_sounds: create_keysounds(keysound),
                        ..Default::default()
                    });
                }
                KeyType::SliderStart => {
                    let slider_end_time = if let Some(time) = key.slider_end_time() {
                        time
                    } else {
                        find_sliderend_time(row_idx, i, &hitobjects)
                    };

                    qua_hitobjects.push(hitobjects::HitObject {
                        start_time: **time as f32,
                        lane: i + 1,
                        endtime: Some(slider_end_time as f32),
                        hit_sound: get_hitsound_type(keysound.hitsound_type),
                        key_sounds: create_keysounds(keysound),
                        ..Default::default()
                    });
                }
                _ => continue,
            }
        }
    }

    let qua_chart = QuaFile {
        audio_file: chart.chartinfo.song_path.clone(),
        song_preview_time: chart.chartinfo.preview_time,
        background_file: chart.chartinfo.bg_path.clone(),
        map_id: -1,
        mapset_id: -1,
        mode,
        title: chart.metadata.title.replace('\n', ""),
        artist: chart.metadata.artist.clone(),
        source: chart.metadata.source.clone(),
        tags: chart.metadata.tags.join(","),
        creator: chart.metadata.creator.clone(),
        difficulty_name: chart.chartinfo.difficulty_name.clone(),
        bpm_does_not_affect_scroll_velocity: true,
        initial_scroll_velocity: 1.0,
        has_scratch_key: (key_count == 8 || key_count == 5),
        editor_layers: vec![],
        custom_audio_samples,
        sound_effects,
        timing_points,
        slider_velocities,
        hitobjects: qua_hitobjects,
    };

    let yaml_string = QuaFile::to_yaml(&qua_chart)?;
    Ok(yaml_string)
}