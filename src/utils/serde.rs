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

pub fn is_default_f32(value: &f32) -> bool {
    *value == 0.0
}

pub fn process_bracket_sections<F>(string: &str, mut lambda: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&str, &str) -> Result<(), Box<dyn std::error::Error>>,
{
    let mut current_content = String::with_capacity(string.len());
    let mut current_section = "";
    
    for line in string.lines().map(str::trim) {
        if line.starts_with('[') && line.ends_with(']') {
            if !current_content.is_empty() && !current_section.is_empty() {
                lambda(current_section, &current_content)?;
                current_content.clear();
            }
            current_section = &line[1..line.len()-1];
        } else {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }
    
    if !current_content.is_empty() && !current_section.is_empty() {
        lambda(current_section, &current_content)?;
    }
    
    Ok(())
}