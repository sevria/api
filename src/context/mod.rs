use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    config::Config,
    domain::{
        auth::service::AuthService,
        data::{
            repository::DataRepository, repository_impl::DataRepositoryImpl, service::DataService,
        },
        field::{
            repository::FieldRepository, repository_impl::FieldRepositoryImpl,
            service::FieldService,
        },
        schema::{
            repository::SchemaRepository, repository_impl::SchemaRepositoryImpl,
            service::SchemaService,
        },
        session::{repository::SessionRepository, repository_impl::SessionRepositoryImpl},
        user::{repository::UserRepository, repository_impl::UserRepositoryImpl},
    },
};

pub struct Context {
    pub data_repository: Arc<dyn DataRepository>,
    pub field_repository: Arc<dyn FieldRepository>,
    pub schema_repository: Arc<dyn SchemaRepository>,
    pub session_repository: Arc<dyn SessionRepository>,
    pub user_repository: Arc<dyn UserRepository>,

    pub auth_service: Arc<AuthService>,
    pub data_service: Arc<DataService>,
    pub field_service: Arc<FieldService>,
    pub schema_service: Arc<SchemaService>,
}

impl Context {
    pub fn new(config: Arc<Config>, db: Arc<Pool<Postgres>>) -> Self {
        let data_repository = Arc::new(DataRepositoryImpl::new(db.clone()));
        let field_repository = Arc::new(FieldRepositoryImpl::new(db.clone()));
        let schema_repository = Arc::new(SchemaRepositoryImpl::new(db.clone()));
        let session_repository = Arc::new(SessionRepositoryImpl::new(db.clone()));
        let user_repository = Arc::new(UserRepositoryImpl::new(db.clone()));

        let auth_service = Arc::new(AuthService::new(
            config,
            session_repository.clone(),
            user_repository.clone(),
        ));
        let data_service = Arc::new(DataService::new(data_repository.clone()));
        let field_service = Arc::new(FieldService::new(field_repository.clone()));
        let schema_service = Arc::new(SchemaService::new(schema_repository.clone()));

        Context {
            data_repository,
            field_repository,
            schema_repository,
            session_repository,
            user_repository,

            auth_service,
            data_service,
            field_service,
            schema_service,
        }
    }
}
