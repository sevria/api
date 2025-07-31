use anyhow::Result;
use serde_json::json;

mod common;

#[tokio::test]
async fn create_field_success() -> Result<()> {
    let server = common::setup().await?;
    let req = json!({
        "name": "author",
        "value_type": "string",
        "required": true,
        "default_value": "unknown",
    });
    let res = server
        .post("/schemas/Q5OhKpzaanvf0rdYVaOrg/fields")
        .json(&req)
        .await;

    res.assert_status_ok();
    res.assert_json_contains(&req);

    Ok(())
}

#[tokio::test]
async fn list_fields_success() -> Result<()> {
    let server = common::setup().await?;
    let res = server.get("/schemas/Q5OhKpzaanvf0rdYVaOrg/fields").await;

    res.assert_status_ok();

    Ok(())
}
