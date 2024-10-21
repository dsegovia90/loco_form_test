use axum::http::response;
use form_tester::app::App;
use insta::assert_debug_snapshot;
use loco_rs::testing;
use serial_test::serial;

// TODO: see how to dedup / extract this to app-local test utils
// not to framework, because that would require a runtime dep on insta
macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("home_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_get_home() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let notes = request.get("/api").await;

        assert_debug_snapshot!((notes.status_code(), notes.text()));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_post_home_json() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let response = request
            .post("/api")
            .json(&serde_json::json!({
                "name": "Daniel",
                "age": 99
            }))
            .await;

        assert_debug_snapshot!((response.status_code(), response.text()));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_post_home_form() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let response = request
            .post("/api")
            .form(&serde_json::json!({
                "name": "Daniel",
                "age": 99
            }))
            .await;

        assert_debug_snapshot!((response.status_code(), response.text()));
    })
    .await;
}
