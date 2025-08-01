use anyhow::Result;
use serde_json::json;
use sevria_api::domain::auth::model::LoginResponse;

use crate::common::TestCase;

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
async fn login_validation() -> Result<()> {
    let server = common::setup().await?;

    let test_cases = vec![
        TestCase {
            data: json!({"email": "", "password": "Sevria123"}),
            expected_error: String::from("Email must be between 3 and 50 characters"),
        },
        TestCase {
            data: json!({"email": "not-a-valid-email", "password": "Sevria123"}),
            expected_error: String::from("Invalid email"),
        },
        TestCase {
            data: json!({"email": "very-very-long-email-more-than-50-characters@mailbox.com", "password": "Sevria123"}),
            expected_error: String::from("Email must be between 3 and 50 characters"),
        },
        TestCase {
            data: json!({"email": "user@example.com", "password": ""}),
            expected_error: String::from("Password must be between 8 and 50 characters"),
        },
        TestCase {
            data: json!({"email": "user@example.com", "password": "this-is-a-very-long-password-more-than-50-characters"}),
            expected_error: String::from("Password must be between 8 and 50 characters"),
        },
    ];

    for test_case in test_cases {
        let res = server.post("/auth/login").json(&test_case.data).await;
        res.assert_status_bad_request();
        res.assert_json_contains(&json!({"message": test_case.expected_error}));
    }

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

#[tokio::test]
async fn refresh_validation() -> Result<()> {
    let server = common::setup().await?;

    let test_cases = vec![
        TestCase {
            data: json!({"token": "", "user_id": "7XSAgkxMcDSrb2LYHtzEi"}),
            expected_error: String::from("Token must be 36 characters"),
        },
        TestCase {
            data: json!({"token": "very-short", "user_id": "7XSAgkxMcDSrb2LYHtzEi"}),
            expected_error: String::from("Token must be 36 characters"),
        },
        TestCase {
            data: json!({"token": "this-takes-more-than-36-characters-long", "user_id": "7XSAgkxMcDSrb2LYHtzEi"}),
            expected_error: String::from("Token must be 36 characters"),
        },
        TestCase {
            data: json!({"token": "qtlf3L9_FQtwuJaTZgs4L2rlUFRnKSZuAyS3", "user_id": ""}),
            expected_error: String::from("User ID must be 21 characters"),
        },
        TestCase {
            data: json!({"token": "qtlf3L9_FQtwuJaTZgs4L2rlUFRnKSZuAyS3", "user_id": "very-short"}),
            expected_error: String::from("User ID must be 21 characters"),
        },
        TestCase {
            data: json!({"token": "qtlf3L9_FQtwuJaTZgs4L2rlUFRnKSZuAyS3", "user_id": "more-than-21-characters-long"}),
            expected_error: String::from("User ID must be 21 characters"),
        },
    ];

    for test_case in test_cases {
        let res = server.post("/auth/refresh").json(&test_case.data).await;
        res.assert_status_bad_request();
        res.assert_json_contains(&json!({"message": test_case.expected_error}));
    }

    Ok(())
}
