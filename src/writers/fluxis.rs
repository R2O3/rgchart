use crate::models::fluxis::metadata::{self, Colors};
use crate::models::generic;
use crate::models::generic::sound::{KeySound, KeySoundRow, HitSoundType};
use crate::models::common::{KeyType, Row};
use crate::models::fluxis::{
    chart::FscFile,
    timing_points,
    hitobjects,
};
use crate::utils::time::find_sliderend_time;

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
        .bpm_changes_zipped()
        .map(|(time, _, change)| timing_points::TimingPoint {
            time: *time as f32,
            bpm: change.value,
            ..timing_points::TimingPoint::default()
        })
        .collect();

    let scroll_velocities = chart
        .timing_points
        .sv_changes_zipped()
        .map(|(time, _, change)| timing_points::ScrollVelocity {
            time: *time as f32,
            multiplier: change.value,
            ..timing_points::ScrollVelocity::default()
        })
        .collect();

    let hitobjects: Vec<(&i32, &f32, &KeySoundRow, &Row)> = chart.hitobjects.iter_zipped().collect();
    let mut fsc_hitobjects = Vec::new();

    for (row_idx, (time, _, keysounds, row)) in hitobjects.iter().enumerate() {
        for (i, key) in row.iter().enumerate() {
            let keysound = if keysounds.is_empty {
                KeySound::normal(100)
            } else {
                keysounds[i]
            };

            match key.key_type {
                KeyType::Normal => {
                    fsc_hitobjects.push(hitobjects::HitObject {
                        time: **time as f32,
                        lane: (i + 1) as isize,
                        hitsound: get_hitsound_type(keysound.hitsound_type),
                        ..hitobjects::HitObject::default()
                    });
                }
                KeyType::SliderStart => {
                    let slider_end_time = if let Some(time) = key.slider_end_time() {
                        time
                    } else {
                        find_sliderend_time(row_idx, i, &hitobjects)
                    };
                    
                    

                    fsc_hitobjects.push(hitobjects::HitObject {
                        time: **time as f32,
                        lane: (i + 1) as isize,
                        holdtime:   ((slider_end_time - **time) as f32),
                        hitsound: get_hitsound_type(keysound.hitsound_type),
                        ..hitobjects::HitObject::default()
                    });
                }
                _ => continue,
            }
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