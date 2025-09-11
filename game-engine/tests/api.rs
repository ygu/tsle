use axum::{http::Request, body::Body};
use game_engine::build_app;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn create_game_works() {
    let app = build_app().await;

    let req = Request::builder()
        .method("POST")
        .uri("/create-game")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), 200);
}
