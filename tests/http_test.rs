use ygo_destiny::logic::utils::http;

#[test]
fn update_successful() {
    assert!(matches!(http::update(), http::UpdateStatus::Complete));
}
