use crate::models::fluxis::metadata::{self, Colors};
use crate::models::generic;
use crate::models::generic::sound::HitSoundType;
use crate::models::common::KeyType;
use crate::models::fluxis::{
    chart::FscFile,
    timing_points,
    hitobjects,
};

fn get_hitsound_type(hitsound_type: HitSoundType) -> String {
    match hitsound_type {
        HitSoundType::Normal => ":normal".to_string(),
        HitSoundType::Clap => ":clap".to_string(),
        HitSoundType::Whistle => ":whistle".to_string(),
        HitSoundType::Finish => ":finish".to_string(),
    }
}

pub(crate) fn to_fsc(chart: &generic::chart::Chart) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = metadata::Metadata {
      title: chart.metadata.title.clone(),
      title_rm: Some(chart.metadata.alt_title.clone()),
      artist: chart.metadata.artist.clone(),
      artist_rm: Some(chart.metadata.alt_artist.clone()),
      mapper: chart.metadata.creator.clone(),
      difficulty: chart.chartinfo.difficulty_name.clone(),
      source: Some(chart.metadata.source.clone()),
      tags: chart.metadata.tags.join(" "),
      previewtime: chart.chartinfo.preview_time as i32,
      ..metadata::Metadata::default()
    };

    let timing_points = chart
        .timing_points
        .bpm_changes()
        .map(|tp| timing_points::TimingPoint {
            time: tp.time as f32,
            bpm: tp.change.value,
            ..timing_points::TimingPoint::default()
        })
        .collect();

    let scroll_velocities = chart
        .timing_points
        .sv_changes()
        .map(|sv| timing_points::ScrollVelocity {
            time: sv.time as f32,
            multiplier: sv.change.value,
            ..timing_points::ScrollVelocity::default()
        })
        .collect();

    let mut fsc_hitobjects = Vec::new();

    for hitobject in chart.hitobjects.iter() {
        let time = hitobject.time as f32;
        let lane = hitobject.lane as isize;

        match hitobject.key.key_type {
            KeyType::Normal => {
                fsc_hitobjects.push(hitobjects::HitObject {
                    time: time,
                    lane: lane,
                    hitsound: get_hitsound_type(HitSoundType::Normal),
                    ..hitobjects::HitObject::default()
                });
            }
            KeyType::SliderStart => {
                let slider_end_time = if let Some(time) = hitobject.key.slider_end_time() {
                    time
                } else {
                    0
                };

                fsc_hitobjects.push(hitobjects::HitObject {
                    time: time,
                    lane: lane,
                    holdtime:   ((slider_end_time - hitobject.time) as f32),
                    hitsound: get_hitsound_type(hitobject.keysound.hitsound_type),
                    ..hitobjects::HitObject::default()
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