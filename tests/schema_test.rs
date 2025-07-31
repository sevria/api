use anyhow::Result;
use fake::{Fake, Faker};
use serde_json::json;
use sevria_api::domain::schema::model::CreateSchemaRequest;

mod common;

#[tokio::test]
async fn create_schema_success() -> Result<()> {
    let server = common::setup().await?;
    let req = Faker.fake::<CreateSchemaRequest>();
    let res = server.post("/schemas").json(&req).await;

    res.assert_status_ok();
    res.assert_json_contains(&req);

    Ok(())
}

#[tokio::test]
async fn get_schemas_success() -> Result<()> {
    let server = common::setup().await?;
    let res = server.get("/schemas").await;

    res.assert_status_ok();
    res.assert_json_contains(&json!({
        "data": [],
        "page": 1,
    }));

    Ok(())
}

#[tokio::test]
async fn get_schema_success() -> Result<()> {
    let server = common::setup().await?;
    let res = server.get("/schemas/Q5OhKpzaanvf0rdYVaOrg").await;

    res.assert_status_ok();
    res.assert_json_contains(&json!({
        "name": "posts",
    }));

    Ok(())
}

#[tokio::test]
async fn update_schema_success() -> Result<()> {
    let server = common::setup().await?;
    let req = json!({
        "name": "post_comments",
    });
    let res = server
        .patch("/schemas/PmEQ33frGUZEKvUH6x12q")
        .json(&req)
        .await;

    res.assert_status_ok();
    res.assert_json_contains(&json!({
        "name": "post_comments",
    }));

    Ok(())
}
