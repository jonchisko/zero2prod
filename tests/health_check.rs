const SPAWN_APP_ERR: &'static str = "Failed to spawn our app.";
const RESPONSE_ERR: &'static str = "Failed to execute request.";

const EXPECTED_HEALTH_CHECK_LOCATION: &'static str = "http://127.0.0.1:8000/health_check";

#[tokio::test]
async fn health_check_expect_status_200_test() {
    // Arrange 
    spawn_app().await.expect(SPAWN_APP_ERR);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(EXPECTED_HEALTH_CHECK_LOCATION)
        .send()
        .await
        .expect(RESPONSE_ERR);

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn health_check_expect_content_length_0_test() {
    // Arrange
    spawn_app().await.expect(SPAWN_APP_ERR);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(EXPECTED_HEALTH_CHECK_LOCATION)
        .send()
        .await
        .expect(RESPONSE_ERR);

    // Assert
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<(), std::io::Error> {
    todo!()
}