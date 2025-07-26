use anyhow::Result;
use fake::{Fake, Faker};
use serde_json::json;
use sevria_api::domain::field::model::CreateFieldRequest;

mod common;

#[tokio::test]
async fn create_field_success() -> Result<()> {
    let server = common::setup().await?;
    let req = Faker.fake::<CreateFieldRequest>();
    let res = server.post("/schemas/1/fields").json(&req).await;

    res.assert_status_ok();
    res.assert_json_contains(&req);

    Ok(())
}

#[tokio::test]
async fn list_fields_success() -> Result<()> {
    let server = common::setup().await?;
    let res = server.get("/schemas/1/fields").await;

    res.assert_status_ok();
    res.assert_json_contains(&json!([
        {
            "name": "content",
            "value_type": "string",
        }
    ]));

    Ok(())
}
