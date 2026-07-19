mod common;

use common::{API_TOKEN, client_with_mock_server, invoice_fixture};
use fakturownia_rs::client::invoice::{ListInvoicesOptions, Period};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn lists_invoices_with_query_options() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("GET"))
        .and(path("/invoices.json"))
        .and(query_param("api_token", API_TOKEN))
        .and(query_param("page", "2"))
        .and(query_param("per_page", "50"))
        .and(query_param("include_positions", "true"))
        .and(query_param("period", "last_month"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([invoice_fixture()])))
        .expect(1)
        .mount(&server)
        .await;

    let invoices = client
        .invoices()
        .list_invoices(Some(ListInvoicesOptions {
            page: Some(2),
            per_page: Some(50),
            include_positions: true,
            period: Some(Period::LastMonth),
        }))
        .await
        .unwrap();

    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].id, 123);
    assert_eq!(invoices[0].number, "FV/1/2026");
}

#[tokio::test]
async fn gets_an_invoice_by_id() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("GET"))
        .and(path("/invoices/123.json"))
        .and(query_param("api_token", API_TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(invoice_fixture()))
        .expect(1)
        .mount(&server)
        .await;

    let invoice = client.invoices().get_invoice(123).await.unwrap();

    assert_eq!(invoice.id, 123);
    assert_eq!(invoice.token, "invoice-preview-token");
}

#[tokio::test]
async fn cancels_an_invoice_with_a_reason() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/invoices/cancel.json"))
        .and(body_json(json!({
            "api_token": API_TOKEN,
            "cancel_invoice_id": 123,
            "cancel_reason": "Duplicate invoice"
        })))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client
        .invoices()
        .cancel_invoice(123, Some("Duplicate invoice"))
        .await
        .unwrap();
}

#[tokio::test]
async fn cancels_an_invoice_without_a_reason() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/invoices/cancel.json"))
        .and(body_json(json!({
            "api_token": API_TOKEN,
            "cancel_invoice_id": 123
        })))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client.invoices().cancel_invoice(123, None).await.unwrap();
}

#[tokio::test]
async fn returns_an_error_for_unsuccessful_statuses() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("GET"))
        .and(path("/invoices/999.json"))
        .and(query_param("api_token", API_TOKEN))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({ "error": "not found" })))
        .expect(1)
        .mount(&server)
        .await;

    let error = client.invoices().get_invoice(999).await.unwrap_err();

    assert_eq!(error.status(), Some(reqwest::StatusCode::NOT_FOUND));
}

#[tokio::test]
async fn builds_preview_and_pdf_urls_from_the_invoice_token() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("GET"))
        .and(path("/invoices/123.json"))
        .and(query_param("api_token", API_TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(invoice_fixture()))
        .expect(2)
        .mount(&server)
        .await;

    let preview_url = client.invoices().invoice_preview_url(123).await.unwrap();
    let pdf_url = client.invoices().invoice_pdf_url(123).await.unwrap();

    assert_eq!(
        preview_url,
        format!("{}/invoice/invoice-preview-token", server.uri())
    );
    assert_eq!(
        pdf_url,
        format!("{}/invoice/invoice-preview-token.pdf", server.uri())
    );
}
