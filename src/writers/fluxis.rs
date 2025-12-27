use crate::models::common::*;
use crate::models::fluxis::{self, FscFile, Colors};
use crate::models::generic::{ GenericManiaChart, HitSoundType};

fn get_hitsound_type(hitsound_type: HitSoundType) -> String {
    match hitsound_type {
        HitSoundType::Normal => ":normal".to_string(),
        HitSoundType::Clap => ":clap".to_string(),
        HitSoundType::Whistle => ":whistle".to_string(),
        HitSoundType::Finish => ":finish".to_string(),
    }
}

pub(crate) fn to_fsc(
    chart: &GenericManiaChart,
) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = fluxis::Metadata {
        title: chart.metadata.title.clone(),
        title_rm: Some(chart.metadata.alt_title.clone()),
        artist: chart.metadata.artist.clone(),
        artist_rm: Some(chart.metadata.alt_artist.clone()),
        mapper: chart.metadata.creator.clone(),
        difficulty: chart.chartinfo.difficulty_name.clone(),
        source: Some(chart.metadata.source.clone()),
        tags: chart.metadata.tags.join(" "),
        previewtime: chart.chartinfo.preview_time as i32,
        ..fluxis::Metadata::default()
    };

    let timing_points = chart
        .timing_points
        .bpm_changes()
        .map(|tp| fluxis::TimingPoint {
            time: tp.time as f32,
            bpm: tp.change.value,
            ..fluxis::TimingPoint::default()
        })
        .collect();

    let scroll_velocities = chart
        .timing_points
        .sv_changes()
        .map(|sv| fluxis::ScrollVelocity {
            time: sv.time as f32,
            multiplier: sv.change.value,
            ..fluxis::ScrollVelocity::default()
        })
        .collect();

    let mut fsc_hitobjects = Vec::new();

    for hitobject in chart.hitobjects.iter() {
        let time = hitobject.time as f32;
        let lane = hitobject.lane as isize;

        match hitobject.key.key_type {
            KeyType::Normal => {
                fsc_hitobjects.push(fluxis::HitObject {
                    time,
                    lane,
                    hitsound: get_hitsound_type(HitSoundType::Normal),
                    ..fluxis::HitObject::default()
                });
            }
            KeyType::SliderStart => {
                let slider_end_time = if let Some(time) = hitobject.key.slider_end_time() {
                    time
                } else {
                    0
                };

                fsc_hitobjects.push(fluxis::HitObject {
                    time,
                    lane,
                    holdtime: (slider_end_time - hitobject.time) as f32,
                    hitsound: get_hitsound_type(hitobject.keysound.hitsound_type),
                    ..fluxis::HitObject::default()
                });
            }
            _ => continue,
        }
    }

    let fsc_chart = FscFile {
        audio_file: chart.chartinfo.song_path.clone(),
        background_file: chart.chartinfo.bg_path.clone(),
        video_file: chart.chartinfo.video_path.clone(),
        metadata,
        colors: Colors::default(),
        hit_objects: fsc_hitobjects,
        timing_points,
        scroll_velocities,
        accuracy_difficulty: Some(chart.chartinfo.od),
        health_difficulty: Some(chart.chartinfo.hp),
        ..FscFile::default()
    };

    let json_string = FscFile::to_json(&fsc_chart)?;
    Ok(json_string)
}