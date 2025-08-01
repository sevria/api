use anyhow::Result;
use serde_json::json;
use sevria_api::domain::auth::model::LoginResponse;

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
            "id": "spCe6NvSJ1W2uyybeRIZv",
            "email": "admin@example.com",
        }
    }));

    Ok(())
}

#[tokio::test]
async fn refresh_success() -> Result<()> {
    let server = common::setup().await?;
    let login_req = json!({
        "email": "admin@example.com",
        "password": "Sevria123",
    });
    let login_res = server.post("/auth/login").json(&login_req).await;

    login_res.assert_status_ok();

    let login_res_data = login_res.json::<LoginResponse>();

    let refresh_req = json!({
        "token": login_res_data.refresh.token,
        "user_id": login_res_data.user.id,
    });
    let refresh_res = server.post("/auth/refresh").json(&refresh_req).await;

    refresh_res.assert_status_ok();
    refresh_res.assert_json_contains(&json!({
        "user": {
            "id": "spCe6NvSJ1W2uyybeRIZv",
            "email": "admin@example.com",
        }
    }));

    Ok(())
}
