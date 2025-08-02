use anyhow::Result;
use serde_json::json;

mod common;

#[tokio::test]
async fn create_data_success() -> Result<()> {
    let server = common::setup().await?;
    let req = json!({
        "foo": "bar",
    });
    let res = server.post("/schemas/posts/data").json(&req).await;

    res.assert_status_ok();

    Ok(())
}
