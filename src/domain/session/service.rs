use std::sync::Arc;

use crate::{
    domain::session::{
        model::{CreateSessionRequest, Session},
        repository::SessionRepository,
    },
    util::error::Error,
};

pub struct SessionService {
    session_repository: Arc<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> SessionService {
        SessionService { session_repository }
    }
}

impl SessionService {
    pub async fn create(&self, req: &CreateSessionRequest) -> Result<Session, Error> {
        let data = Session::new(&req.user_id, req.expires_at);
        self.session_repository.create(&data).await
    }

    pub async fn get(&self, token: &str, user_id: &str) -> Result<Session, Error> {
        self.session_repository.get(token, user_id).await
    }
}
