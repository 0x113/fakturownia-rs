use super::client::Client;
use crate::types::invoice::{Invoice, InvoiceWrite, Kind};
use reqwest::{Error, Method};
use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug, Default)]
pub struct ListInvoicesOptions {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub include_positions: bool,
    pub period: Option<Period>,
    /// Used with `Period::More`; values are sent unchanged rather than discarded.
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub income: Option<Income>,
    pub invoice_ids: Option<Vec<u64>>,
    pub number: Option<String>,
    pub kind: Option<Kind>,
    pub kinds: Vec<Kind>,
    pub client_id: Option<u64>,
    pub search_date_type: Option<SearchDateType>,
    pub order: Option<InvoiceOrder>,
}

#[derive(Clone, Debug)]
pub enum Period {
    Last12Months,
    LastYear,
    LastMonth,
    Last30Days,
    ThisMonth,
    ThisYear,
    All,
    More,
}

impl Period {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Last12Months => "last_12_months",
            Self::LastYear => "last_year",
            Self::LastMonth => "last_month",
            Self::Last30Days => "last_30_days",
            Self::ThisMonth => "this_month",
            Self::ThisYear => "this_year",
            Self::All => "all",
            Self::More => "more",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Income {
    Income,
    Expense,
}

impl Income {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Income => "yes",
            Self::Expense => "no",
        }
    }
}

#[derive(Clone, Debug)]
pub enum SearchDateType {
    IssueDate,
    PaidDate,
    /// Selects the invoice's `sell_date`.
    TransactionDate,
}

impl SearchDateType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::IssueDate => "issue_date",
            Self::PaidDate => "paid_date",
            Self::TransactionDate => "transaction_date",
        }
    }
}

#[derive(Clone, Debug)]
pub enum InvoiceOrderField {
    Number,
    UpdatedAt,
    PriceNet,
    PriceGross,
    PriceTax,
    IssueDate,
    PaymentTo,
    PaidDate,
    TransactionDate,
    BuyerName,
    BuyerTaxNo,
    SellerName,
    Oid,
}

impl InvoiceOrderField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Number => "number",
            Self::UpdatedAt => "updated_at",
            Self::PriceNet => "price_net",
            Self::PriceGross => "price_gross",
            Self::PriceTax => "price_tax",
            Self::IssueDate => "issue_date",
            Self::PaymentTo => "payment_to",
            Self::PaidDate => "paid_date",
            Self::TransactionDate => "transaction_date",
            Self::BuyerName => "buyer_name",
            Self::BuyerTaxNo => "buyer_tax_no",
            Self::SellerName => "seller_name",
            Self::Oid => "oid",
        }
    }
}

#[derive(Clone, Debug)]
pub struct InvoiceOrder {
    pub field: InvoiceOrderField,
    pub descending: bool,
}

impl InvoiceOrder {
    pub fn ascending(field: InvoiceOrderField) -> Self {
        Self {
            field,
            descending: false,
        }
    }

    pub fn descending(field: InvoiceOrderField) -> Self {
        Self {
            field,
            descending: true,
        }
    }

    fn to_query_value(&self) -> String {
        let suffix = if self.descending { ".desc" } else { "" };
        format!("{}{suffix}", self.field.as_str())
    }
}

/// Controls placed beside `invoice` in create and update request bodies.
#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoiceRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_oss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_default_descriptions: Option<bool>,
}

#[derive(Serialize)]
struct InvoiceEnvelope<'a> {
    invoice: &'a InvoiceWrite,
    #[serde(flatten)]
    options: &'a InvoiceRequestOptions,
}

pub struct InvoicesEndpoint<'c>(pub &'c Client);

impl<'c> InvoicesEndpoint<'c> {
    pub async fn list_invoices(
        &self,
        options: Option<ListInvoicesOptions>,
    ) -> Result<Vec<Invoice>, Error> {
        let mut url = self.0.build_url("invoices");
        if let Some(options) = options {
            let mut query = url.query_pairs_mut();
            if let Some(page) = options.page {
                query.append_pair("page", &page.to_string());
            }
            if let Some(per_page) = options.per_page {
                query.append_pair("per_page", &per_page.to_string());
            }
            if options.include_positions {
                query.append_pair("include_positions", "true");
            }
            if let Some(period) = options.period {
                query.append_pair("period", period.as_str());
            }
            if let Some(value) = options.date_from {
                query.append_pair("date_from", &value);
            }
            if let Some(value) = options.date_to {
                query.append_pair("date_to", &value);
            }
            if let Some(income) = options.income {
                query.append_pair("income", income.as_str());
            }
            if let Some(ids) = options.invoice_ids {
                query.append_pair(
                    "invoice_ids",
                    &ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
                );
            }
            if let Some(number) = options.number {
                query.append_pair("number", &number);
            }
            if let Some(kind) = options.kind {
                query.append_pair("kind", kind.as_str());
            }
            for kind in options.kinds {
                query.append_pair("kinds[]", kind.as_str());
            }
            if let Some(client_id) = options.client_id {
                query.append_pair("client_id", &client_id.to_string());
            }
            if let Some(search_date_type) = options.search_date_type {
                query.append_pair("search_date_type", search_date_type.as_str());
            }
            if let Some(order) = options.order {
                query.append_pair("order", &order.to_query_value());
            }
        }
        self.0
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    pub async fn get_invoice(&self, id: u64) -> Result<Invoice, Error> {
        self.0
            .client
            .get(self.0.build_url(&format!("invoices/{id}")))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Creates an invoice using a partial write payload.
    pub async fn create_invoice(
        &self,
        invoice: &InvoiceWrite,
        options: Option<&InvoiceRequestOptions>,
    ) -> Result<Invoice, Error> {
        self.write_invoice(Method::POST, "invoices", invoice, options)
            .await
    }

    /// Updates only the fields present in a partial write payload.
    pub async fn update_invoice(
        &self,
        id: u64,
        invoice: &InvoiceWrite,
        options: Option<&InvoiceRequestOptions>,
    ) -> Result<Invoice, Error> {
        self.write_invoice(Method::PUT, &format!("invoices/{id}"), invoice, options)
            .await
    }

    async fn write_invoice(
        &self,
        method: Method,
        path: &str,
        invoice: &InvoiceWrite,
        options: Option<&InvoiceRequestOptions>,
    ) -> Result<Invoice, Error> {
        let default_options = InvoiceRequestOptions::default();
        let envelope = InvoiceEnvelope {
            invoice,
            options: options.unwrap_or(&default_options),
        };
        self.0
            .authenticated_json_request(method, path, &envelope)
            .await?
            .json()
            .await
    }

    pub async fn invoice_preview_url(&self, id: u64) -> Result<String, Error> {
        let invoice = self.get_invoice(id).await?;
        Ok(format!("{}invoice/{}", self.0.api_base, invoice.token))
    }

    pub async fn invoice_pdf_url(&self, id: u64) -> Result<String, Error> {
        let invoice = self.get_invoice(id).await?;
        Ok(format!("{}invoice/{}.pdf", self.0.api_base, invoice.token))
    }

    pub async fn cancel_invoice(&self, id: u64, reason: Option<&str>) -> Result<(), Error> {
        let mut body = json!({ "cancel_invoice_id": id });
        if let Some(reason) = reason {
            body["cancel_reason"] = json!(reason);
        }
        self.0.post("invoices/cancel", body).await?;
        Ok(())
    }
}

impl Kind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Vat => "vat",
            Self::Proforma => "proforma",
            Self::Bill => "bill",
            Self::Receipt => "receipt",
            Self::Advance => "advance",
            Self::Final => "final",
            Self::Correction => "correction",
            Self::VatMp => "vat_mp",
            Self::VatRr => "vat_rr",
            Self::Other => "invoice_other",
            Self::VatMargin => "vat_margin",
            Self::Kp => "kp",
            Self::Kw => "kw",
            Self::Dw => "dw",
            Self::Wnt => "wnt",
            Self::Wdt => "wdt",
            Self::Estimate => "estimate",
            Self::CorrectionNote => "correction_note",
            Self::AccountingNote => "accounting_note",
            Self::ClientOrder => "client_order",
            Self::ImportService => "import_service",
            Self::ImportServiceEu => "import_service_eu",
            Self::ImportProducts => "import_products",
            Self::ExportProducts => "export_products",
        }
    }
}
