use serde::Serializer;

pub fn trim_float<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.fract() == 0.0 {
        serializer.serialize_i32(*value as i32)
    } else {
        serializer.serialize_f32(*value)
    }
}