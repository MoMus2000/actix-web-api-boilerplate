mod utils;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = utils::spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
    // Use the returned application address
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}