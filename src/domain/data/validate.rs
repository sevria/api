use serde_json::Value;
use validator::ValidationError;

pub fn data(data: &Value) -> Result<(), ValidationError> {
    if data.is_null() {
        return Err(ValidationError::new("").with_message("Data is required".into()));
    }

    if !data.is_object() {
        return Err(ValidationError::new("").with_message("Data must be an object".into()));
    }

    if data.as_object().unwrap().is_empty() {
        return Err(ValidationError::new("").with_message("Data must not be empty".into()));
    }

    Ok(())
}
