use anyhow::Result;
use fake::{Fake, Faker};
use sevria_api::domain::schema::model::CreateSchemaRequest;

mod common;

#[tokio::test]
async fn create_schema_success() -> Result<()> {
    let state = common::setup().await?;
    let req: CreateSchemaRequest = Faker.fake();
    let res = state.server.post("/schemas").json(&req).await;

    res.assert_status_ok();

    Ok(())
}
