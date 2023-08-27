use crate::helpers::{spawn_app, RESPONSE_ERR};

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect(RESPONSE_ERR);

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
