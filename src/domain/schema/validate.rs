use regex::Regex;
use validator::ValidationError;

pub fn name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::new("Schema name is required"));
    }

    if name.len() > 50 {
        return Err(ValidationError::new(
            "Schema name must not exceed 50 characters",
        ));
    }

    let regex = Regex::new(r"^[a-z][a-z0-9_]*$").unwrap();
    if !regex.is_match(name) {
        return Err(ValidationError::new(
            "Schema name must start with a lowercase letter and contain only lowercase letters, numbers, and underscores",
        ));
    }

    Ok(())
}
