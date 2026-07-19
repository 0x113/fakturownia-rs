use fakturownia_rs::Client;
use reqwest::Url;
use serde_json::Value;
use wiremock::MockServer;

pub const API_TOKEN: &str = "test-api-token";

pub async fn client_with_mock_server() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client =
        Client::with_base_url(Url::parse(&server.uri()).unwrap(), API_TOKEN.to_owned()).unwrap();

    (server, client)
}

pub fn invoice_fixture() -> Value {
    serde_json::from_str(include_str!("../fixtures/invoice.json")).unwrap()
}
