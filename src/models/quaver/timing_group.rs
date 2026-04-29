use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::quaver::timing_points::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrollGroup {
    #[serde(rename = "InitialScrollVelocity")]
    pub initial_scroll_velocity: f32,

    #[serde(rename = "ScrollVelocities")]
    pub scroll_velocities: Vec<SliderVelocity>,
}


#[derive(Debug)]
pub enum TimingGroup {
    ScrollGroup(ScrollGroup),
}

impl<'de> Deserialize<'de> for TimingGroup {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let serde_yaml_ng::Value::Tagged(tagged) = serde_yaml_ng::Value::deserialize(deserializer)?
        else {
            return Err(serde::de::Error::custom("expected a tagged YAML value"));
        };

        match tagged.tag.to_string().trim_start_matches('!') {
            "ScrollGroup" => {
                let data: ScrollGroup =
                    serde_yaml_ng::from_value(tagged.value).map_err(serde::de::Error::custom)?;
                Ok(TimingGroup::ScrollGroup(data))
            }
            other => Err(serde::de::Error::custom(
                format!("unknown timing group tag: {}", other),
            )),
        }
    }
}

impl Serialize for TimingGroup {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (tag, value) = match self {
            TimingGroup::ScrollGroup(data) => (
                "ScrollGroup",
                serde_yaml_ng::to_value(data).map_err(serde::ser::Error::custom)?,
            ),
        };

        serde_yaml_ng::Value::Tagged(Box::new(serde_yaml_ng::value::TaggedValue {
            tag: serde_yaml_ng::value::Tag::new(tag),
            value,
        }))
        .serialize(serializer)
    }
}