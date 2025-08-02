use std::sync::Arc;

use serde_json::Value;

use crate::util::{error::Error, validator::validate};

use super::{model::CreateDataRequest, repository::DataRepository};

pub struct DataService {
    data_repository: Arc<dyn DataRepository>,
}

impl DataService {
    pub fn new(data_repository: Arc<dyn DataRepository>) -> DataService {
        DataService { data_repository }
    }
}

impl DataService {
    pub async fn create(&self, data: &CreateDataRequest) -> Result<Value, Error> {
        validate(data)?;
        self.data_repository.create(data).await
    }
}
