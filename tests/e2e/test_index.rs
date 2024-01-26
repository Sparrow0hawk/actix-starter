use crate::fixtures;
use thirtyfour::prelude::*;

#[tokio::test]
async fn test_index_shows_hello_world() {
    let app = fixtures::spawn_app().await;

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .expect("Web driver failed to start");

    driver
        .goto(&app.address)
        .await
        .expect("Failed to load home page");

    let header = driver.find(By::Tag("h1")).await.unwrap();

    assert!(header.text().await.unwrap() == "Hello world!");

    driver.quit().await.unwrap();
}
