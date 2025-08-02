use serde_json::Value;
use validator::ValidationError;

pub fn data(data: &Value) -> Result<(), ValidationError> {
    if data.is_null() {
        return Err(ValidationError::new("Data is required"));
    }

    if !data.is_object() {
        return Err(ValidationError::new("Data must be an object"));
    }

    if data.as_object().unwrap().is_empty() {
        return Err(ValidationError::new("Data must not be empty"));
    }

    Ok(())
}
