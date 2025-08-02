use async_trait::async_trait;
use serde_json::Value;

use crate::util::error::Error;

use super::model::CreateDataRequest;

#[async_trait]
pub trait DataRepository: Send + Sync {
    async fn create(&self, data: &CreateDataRequest) -> Result<Value, Error>;
}
