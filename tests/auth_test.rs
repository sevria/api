use anyhow::Result;
use serde_json::json;

mod common;

#[tokio::test]
async fn login_success() -> Result<()> {
    let server = common::setup().await?;
    let req = json!({
        "email": "admin@example.com",
        "password": "Sevria123",
    });
    let res = server.post("/auth/login").json(&req).await;

    res.assert_status_ok();
    res.assert_json_contains(&json!({
        "user": {
            "id": 1,
            "email": "admin@example.com",
        }
    }));

    Ok(())
}
