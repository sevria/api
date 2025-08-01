use validator::Validate;

use super::error::Error;

pub fn validate<T: Validate>(value: &T) -> Result<(), Error> {
    if let Err(err) = value.validate() {
        if let Some((_, errors)) = err.field_errors().iter().next() {
            let message = errors.first().map(|e| e.to_string()).unwrap_or_default();
            return Err(Error::InvalidArgument(message));
        }
    }
    Ok(())
}
