mod common;

use common::{API_TOKEN, client_with_mock_server, invoice_fixture};
use fakturownia_rs::client::invoice::{
    Income, InvoiceOrder, InvoiceOrderField, InvoiceRequestOptions, ListInvoicesOptions, Period,
    SearchDateType,
};
use fakturownia_rs::{FlexibleValue, InvoicePositionWrite, InvoiceWrite, Kind};
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
            ..Default::default()
        }))
        .await
        .unwrap();

    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].id, 123);
    assert_eq!(invoices[0].number, "FV/1/2026");
}

#[tokio::test]
async fn lists_invoices_with_all_documented_filters() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("GET"))
        .and(path("/invoices.json"))
        .and(query_param("api_token", API_TOKEN))
        .and(query_param("period", "more"))
        .and(query_param("date_from", "2026-01-01"))
        .and(query_param("date_to", "2026-01-31"))
        .and(query_param("income", "no"))
        .and(query_param("invoice_ids", "1,2"))
        .and(query_param("number", "FV/1"))
        .and(query_param("kind", "vat"))
        .and(query_param("kinds[]", "vat"))
        .and(query_param("kinds[]", "proforma"))
        .and(query_param("client_id", "7"))
        .and(query_param("search_date_type", "transaction_date"))
        .and(query_param("order", "updated_at.desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    client
        .invoices()
        .list_invoices(Some(ListInvoicesOptions {
            period: Some(Period::More),
            date_from: Some("2026-01-01".into()),
            date_to: Some("2026-01-31".into()),
            income: Some(Income::Expense),
            invoice_ids: Some(vec![1, 2]),
            number: Some("FV/1".into()),
            kind: Some(Kind::Vat),
            kinds: vec![Kind::Vat, Kind::Proforma],
            client_id: Some(7),
            search_date_type: Some(SearchDateType::TransactionDate),
            order: Some(InvoiceOrder::descending(InvoiceOrderField::UpdatedAt)),
            ..Default::default()
        }))
        .await
        .unwrap();
}

#[tokio::test]
async fn creates_an_invoice_with_envelope_options() {
    let (server, client) = client_with_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/invoices.json"))
        .and(body_json(json!({
            "api_token": API_TOKEN,
            "identify_oss": "1",
            "fill_default_descriptions": true,
            "invoice": {"kind": "vat", "buyer_name": "Example Buyer"}
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(invoice_fixture()))
        .mount(&server)
        .await;

    let invoice = client
        .invoices()
        .create_invoice(
            &InvoiceWrite {
                kind: Some(Kind::Vat),
                buyer_name: Some("Example Buyer".into()),
                ..Default::default()
            },
            Some(&InvoiceRequestOptions {
                identify_oss: Some("1".into()),
                fill_default_descriptions: Some(true),
            }),
        )
        .await
        .unwrap();
    assert_eq!(invoice.id, 123);
}

#[tokio::test]
async fn updates_positions_and_propagates_errors() {
    let (server, client) = client_with_mock_server().await;
    let payload = InvoiceWrite {
        positions: Some(vec![InvoicePositionWrite {
            id: Some(42),
            destroy: Some(FlexibleValue::from(1_u64)),
            ..Default::default()
        }]),
        ..Default::default()
    };
    Mock::given(method("PUT"))
        .and(path("/invoices/123.json"))
        .and(body_json(json!({
            "api_token": API_TOKEN,
            "invoice": {"positions": [{"id": 42, "_destroy": 1}]}
        })))
        .respond_with(ResponseTemplate::new(422))
        .mount(&server)
        .await;

    let error = client
        .invoices()
        .update_invoice(123, &payload, None)
        .await
        .unwrap_err();
    assert_eq!(
        error.status(),
        Some(reqwest::StatusCode::UNPROCESSABLE_ENTITY)
    );
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
