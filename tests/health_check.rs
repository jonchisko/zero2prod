use std::net::TcpListener;

const RESPONSE_ERR: &'static str = "Failed to execute request.";

const EXPECTED_HEALTH_CHECK_LOCATION: &'static str = "http://127.0.0.1:8000/health_check";

#[tokio::test]
async fn health_check_test() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(EXPECTED_HEALTH_CHECK_LOCATION)
        .send()
        .await
        .expect(RESPONSE_ERR);

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to create tcp listener.");
    let server = zero2prod::run(listener).expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}
