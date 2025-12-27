use crate::models::common::*;
use crate::models::generic::{GenericManiaChart, HitSoundType, SoundBank};
use crate::models::osu::{self, *};

#[allow(unused)]
use crate::errors;

#[inline(always)]
fn bpm_to_beatlength(bpm: &f32) -> f32 {
    60000.0 / bpm
}

#[inline(always)]
fn multiplier_to_beatlength(multiplier: &f32) -> f32 {
    if *multiplier == 0.0 {
        return -10000.0;
    }
    -100.0 / multiplier.abs()
}

#[inline(always)]
fn column_to_coords(lane: u8, key_count: usize) -> u16 {
    ((lane - 1) as f32 * 512.0 / key_count as f32).ceil() as u16
}

pub(crate) fn to_osu(
    chart: &GenericManiaChart,
) -> Result<String, Box<dyn std::error::Error>> {
    let key_count = chart.chartinfo.key_count;

    let general = osu::General {
        audio_filename: chart.chartinfo.song_path.clone(),
        audio_lead_in: 0,
        preview_time: chart.chartinfo.preview_time,
        countdown: 0,
        sample_set: osu::SampleSet::Soft,
        stack_leniency: 0.7,
        mode: 3,
        letterbox_in_breaks: false,
        special_style: false,
        widescreen_storyboard: true,
        ..Default::default()
    };

    let editor = Some(osu::Editor {
        distance_spacing: Some(1.0),
        beat_divisor: Some(4),
        grid_size: Some(4),
        timeline_zoom: Some(1.0),
        ..Default::default()
    });

    let metadata = osu::Metadata {
        title: chart.metadata.title.replace("\n", ""),
        title_unicode: chart.metadata.alt_title.clone(),
        artist: chart.metadata.artist.clone(),
        artist_unicode: chart.metadata.alt_artist.clone(),
        creator: chart.metadata.creator.clone(),
        version: chart.chartinfo.difficulty_name.clone(),
        source: chart.metadata.source.clone(),
        tags: chart.metadata.tags.clone(),
        beatmap_id: -1,
        beatmap_set_id: -1,
    };

    let difficulty = osu::Difficulty {
        hp_drain_rate: 8.5,
        circle_size: key_count as f32,
        overall_difficulty: 8.0,
        approach_rate: 5.0,
        slider_multiplier: 1.4,
        slider_tick_rate: 1.0,
    };

    let mut events = osu::Events {
        background: if !chart.chartinfo.bg_path.trim().is_empty() {
            Some(osu::Background {
                filename: chart.chartinfo.bg_path.clone(),
                x_offset: 0,
                y_offset: 0,
            })
        } else {
            None
        },
        video: if !chart.chartinfo.video_path.trim().is_empty() {
            Some(osu::Video {
                start_time: 0,
                filename: chart.chartinfo.video_path.clone(),
                x_offset: 0,
                y_offset: 0,
            })
        } else {
            None
        },
        ..Default::default()
    };

    if let Some(ref soundbank) = chart.soundbank {
        for sound_effect in &soundbank.sound_effects {
            let sample_path = soundbank
                .get_sound_sample(sound_effect.sample)
                .unwrap_or_default();
            events.add_sample(
                sound_effect.time,
                0,
                sample_path,
                sound_effect.volume,
            );
        }
    }

    let mut timing_points = osu::TimingPoints::new();

    for timing_point in chart.timing_points.iter() {
        match timing_point.change.change_type {
            TimingChangeType::Bpm => {
                timing_points.add_timing_point(osu::TimingPoint {
                    time: timing_point.time as f32,
                    beat_length: bpm_to_beatlength(&timing_point.change.value),
                    meter: 4,
                    sample_set: 1,
                    sample_index: 0,
                    volume: 100,
                    uninherited: true,
                    effects: 0,
                });
            }
            TimingChangeType::Sv => {
                timing_points.add_timing_point(osu::TimingPoint {
                    time: timing_point.time as f32,
                    beat_length: multiplier_to_beatlength(&timing_point.change.value),
                    meter: 4,
                    sample_set: 1,
                    sample_index: 0,
                    volume: 100,
                    uninherited: false,
                    effects: 0,
                });
            }
            _ => {}
        }
    }

    let mut hitobjects = osu::HitObjects::new();
    let mut soundbank = chart.soundbank.clone().unwrap_or(SoundBank::new());

    for hitobject in chart.hitobjects.iter() {
        let time = hitobject.time as f32;
        let lane = hitobject.lane;
        let coords = column_to_coords(lane, key_count as usize);

        let keysound = hitobject.keysound;
        let hitsound = keysound.hitsound_type;
        let hitsound_value = match hitsound {
            HitSoundType::Normal => 0,
            HitSoundType::Whistle => 2,
            HitSoundType::Finish => 4,
            HitSoundType::Clap => 8,
        };

        let custom_sample = if keysound.has_custom {
            soundbank
                .get_sound_sample(keysound.sample.unwrap_or(0))
                .unwrap_or_default()
        } else {
            String::new()
        };

        let volume = if keysound.volume >= 100 {
            0
        } else {
            keysound.volume
        };

        let hit_sample = osu::HitSample {
            normal_set: 0,
            addition_set: 0,
            index: 0,
            volume,
            filename: custom_sample,
        };

        match hitobject.key.key_type {
            KeyType::Normal => {
                let hit_object = osu::HitObject {
                    x: coords as i32,
                    y: 192,
                    time,
                    object_type: 1,
                    hit_sound: hitsound_value,
                    object_params: Vec::new(),
                    hit_sample,
                };
                hitobjects.add_hit_object(hit_object);
            }
            KeyType::SliderStart => {
                let slider_end_time = if let Some(end_time) = hitobject.key.slider_end_time() {
                    end_time
                } else {
                    0
                };

                let hit_object = osu::HitObject {
                    x: coords as i32,
                    y: 192,
                    time,
                    object_type: 128,
                    hit_sound: hitsound_value,
                    object_params: vec![slider_end_time.to_string()],
                    hit_sample,
                };
                hitobjects.add_hit_object(hit_object);
            }
            _ => continue,
        }
    }

    let osu_file = OsuFile {
        general,
        editor,
        metadata,
        difficulty,
        events,
        timing_points,
        hitobjects,
    };

    Ok(osu_file.to_osu_format_mania(&mut soundbank))
}