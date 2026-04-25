use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::quaver::timing_points::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrollGroupData {
    #[serde(rename = "InitialScrollVelocity")]
    pub initial_scroll_velocity: f64,

    #[serde(rename = "ScrollVelocities")]
    pub scroll_velocities: Vec<SliderVelocity>,
}


#[derive(Debug)]
pub enum TimingGroup {
    ScrollGroup(ScrollGroupData),
}

impl<'de> Deserialize<'de> for TimingGroup {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_yaml_ng::Value::deserialize(deserializer)?;

        if let serde_yaml_ng::Value::Tagged(tagged) = value {
            match tagged.tag.to_string().trim_start_matches('!') {
                "ScrollGroup" => {
                    let data: ScrollGroupData = serde_yaml_ng::from_value(tagged.value)
                        .map_err(serde::de::Error::custom)?;
                    return Ok(TimingGroup::ScrollGroup(data));
                }
                other => return Err(serde::de::Error::custom(
                    format!("unknown timing group tag: {}", other)
                )),
            }
        }

        Err(serde::de::Error::custom("expected a tagged YAML value"))
    }
}

impl Serialize for TimingGroup {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TimingGroup::ScrollGroup(data) => {
                let value = serde_yaml_ng::to_value(data).map_err(serde::ser::Error::custom)?;
                let tagged = serde_yaml_ng::Value::Tagged(Box::new(serde_yaml_ng::value::TaggedValue {
                    tag: serde_yaml_ng::value::Tag::new("ScrollGroup"),
                    value,
                }));
                tagged.serialize(serializer)
            }
        }
    }
}