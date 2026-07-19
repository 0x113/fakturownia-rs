use serde::{Deserialize, Serialize};

/// A lossless API scalar used where Fakturownia accepts strings, numbers, or booleans.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FlexibleValue {
    String(String),
    Number(serde_json::Number),
    Bool(bool),
}

impl From<&str> for FlexibleValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for FlexibleValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u64> for FlexibleValue {
    fn from(value: u64) -> Self {
        Self::Number(value.into())
    }
}

impl From<i64> for FlexibleValue {
    fn from(value: i64) -> Self {
        Self::Number(value.into())
    }
}

impl From<bool> for FlexibleValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Seller {
    #[serde(rename = "seller_name")]
    pub name: String,
    #[serde(rename = "seller_tax_no")]
    pub tax_no: String,
    #[serde(rename = "seller_street")]
    pub street: String,
    #[serde(rename = "seller_post_code")]
    pub post_code: String,
    #[serde(rename = "seller_city")]
    pub city: String,
    #[serde(rename = "seller_country")]
    pub country: String,
    #[serde(rename = "seller_email")]
    pub email: String,
    #[serde(rename = "seller_phone")]
    pub phone: String,
    #[serde(rename = "seller_fax")]
    pub fax: String,
    #[serde(rename = "seller_www")]
    pub www: String,
    #[serde(rename = "seller_person")]
    pub person: String,
    #[serde(rename = "seller_bank")]
    pub bank: String,
    #[serde(rename = "seller_bank_account")]
    pub bank_account: Option<String>,
    #[serde(rename = "seller_tax_no_kind")]
    pub tax_no_kind: Option<String>,
    #[serde(rename = "seller_bdo_no")]
    pub bdo_no: Option<String>,
    #[serde(rename = "seller_jst")]
    pub jst: Option<FlexibleValue>,
    #[serde(rename = "seller_gv")]
    pub gv: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Buyer {
    #[serde(rename = "buyer_name")]
    pub name: String,
    #[serde(rename = "buyer_tax_no")]
    pub tax_no: Option<String>,
    #[serde(rename = "buyer_street")]
    pub street: Option<String>,
    #[serde(rename = "buyer_post_code")]
    pub post_code: Option<String>,
    #[serde(rename = "buyer_city")]
    pub city: Option<String>,
    #[serde(rename = "buyer_first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "buyer_country")]
    pub country: Option<String>,
    #[serde(rename = "buyer_email")]
    pub email: Option<String>,
    #[serde(rename = "buyer_www")]
    pub www: Option<String>,
    #[serde(rename = "buyer_fax")]
    pub fax: Option<String>,
    #[serde(rename = "buyer_phone")]
    pub phone: Option<String>,
    #[serde(rename = "buyer_last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "buyer_person")]
    pub person: Option<String>,
    #[serde(rename = "buyer_bank_account")]
    pub bank_account: Option<String>,
    #[serde(rename = "buyer_bank")]
    pub bank: Option<String>,
    #[serde(rename = "buyer_mass_payment_code")]
    pub mass_payment_code: Option<String>,
    #[serde(rename = "buyer_company")]
    pub company: Option<FlexibleValue>,
    #[serde(rename = "buyer_mobile_phone")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "buyer_tax_no_kind")]
    pub tax_no_kind: Option<String>,
    #[serde(rename = "disable_tax_no_validation")]
    pub disable_tax_no_validation: Option<FlexibleValue>,
    #[serde(rename = "buyer_note")]
    pub note: Option<String>,
    #[serde(rename = "buyer_jst")]
    pub jst: Option<FlexibleValue>,
    #[serde(rename = "buyer_gv")]
    pub gv: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CalculatingStrategy {
    pub position: String,
    pub sum: String,
    pub invoice_form_price_kind: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CorrectionPositionAttributes {
    pub name: Option<String>,
    pub quantity: Option<FlexibleValue>,
    pub total_price_gross: Option<FlexibleValue>,
    pub tax: Option<FlexibleValue>,
    pub kind: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Position {
    pub id: u64,
    pub invoice_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub price_net: String,
    pub quantity: String,
    pub total_price_gross: String,
    pub total_price_net: String,
    pub account_id: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
    pub additional_info: Option<String>,
    pub quantity_unit: String,
    pub tax: String,
    pub price_gross: String,
    pub price_tax: String,
    pub total_price_tax: String,
    pub kind: Option<String>,
    pub invoice_position_id: Option<u64>,
    pub product_id: Option<u64>,
    pub deleted: bool,
    pub discount: Option<String>,
    pub discount_percent: Option<String>,
    pub tax2: String,
    pub exchange_rate: String,
    pub accounting_tax_kind: Option<String>,
    pub code: Option<String>,
    pub discount_net: Option<String>,
    pub lump_sum_tax: Option<String>,
    pub corrected_pos_kind: Option<String>,
    pub gtu_code: Option<String>,
    pub correction_before_attributes: Option<CorrectionPositionAttributes>,
    pub correction_after_attributes: Option<CorrectionPositionAttributes>,
    pub total_purchase_price: Option<FlexibleValue>,
    pub vat_margin_tax: Option<FlexibleValue>,
    pub vat_margin_total_price_net: Option<FlexibleValue>,
    pub vat_margin_total_price_tax: Option<FlexibleValue>,
    pub vat_margin_total_price_gross: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InvoiceParty {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tax_no: Option<String>,
    pub company: Option<FlexibleValue>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub post_code: Option<String>,
    pub street: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub note: Option<String>,
    pub role: Option<String>,
    pub participation: Option<FlexibleValue>,
    pub role_description: Option<String>,
    #[serde(rename = "_destroy")]
    pub destroy: Option<FlexibleValue>,
}

pub type Recipient = InvoiceParty;
pub type Issuer = InvoiceParty;

#[derive(Clone, Debug, Deserialize)]
pub struct InvoiceDescription {
    pub id: Option<u64>,
    pub kind: Option<String>,
    pub content: Option<String>,
    pub position_index: Option<i64>,
    pub row_number: Option<u64>,
    #[serde(rename = "_destroy")]
    pub destroy: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SettlementPosition {
    pub id: Option<u64>,
    pub kind: Option<String>,
    pub amount: Option<FlexibleValue>,
    pub reason: Option<String>,
    #[serde(rename = "_destroy")]
    pub destroy: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Vat,
    Proforma,
    Bill,
    Receipt,
    Advance,
    Final,
    Correction,
    VatMp,
    VatRr,
    #[serde(rename = "invoice_other")]
    Other,
    VatMargin,
    Kp,
    Kw,
    Dw,
    Wnt,
    Wdt,
    Estimate,
    CorrectionNote,
    AccountingNote,
    ClientOrder,
    ImportService,
    ImportServiceEu,
    ImportProducts,
    ExportProducts,
}

/// Complete invoice response. Expense invoices retain the API's `seller_*` and
/// `buyer_*` names even though their displayed business roles are reversed.
#[derive(Clone, Debug, Deserialize)]
pub struct Invoice {
    pub id: u64,
    pub number: String,
    #[serde(flatten)]
    pub seller: Seller,
    #[serde(flatten)]
    pub buyer: Buyer,
    pub positions: Option<Vec<Position>>,
    pub kind: Option<Kind>,
    pub token: String,
    pub income: Option<FlexibleValue>,
    pub issue_date: Option<String>,
    /// The transaction date shown on the invoice; list filtering calls it `transaction_date`.
    pub sell_date: Option<String>,
    pub place: Option<String>,
    pub category_id: Option<FlexibleValue>,
    pub department_id: Option<FlexibleValue>,
    pub client_id: Option<FlexibleValue>,
    pub invoice_template_id: Option<FlexibleValue>,
    pub invoice_id: Option<FlexibleValue>,
    pub from_invoice_id: Option<FlexibleValue>,
    pub delivery_date: Option<String>,
    pub oid: Option<String>,
    pub oid_unique: Option<FlexibleValue>,
    pub warehouse_id: Option<FlexibleValue>,
    pub status: Option<String>,
    pub accounting_kind: Option<String>,
    pub accounting_vat_tax_date: Option<String>,
    pub accounting_income_tax_date: Option<String>,
    pub accounting_note_kind: Option<String>,
    pub use_invoice_issuer: Option<FlexibleValue>,
    pub invoice_issuer: Option<String>,
    pub recipient_id: Option<FlexibleValue>,
    pub recipient_name: Option<String>,
    pub recipient_street: Option<String>,
    pub recipient_post_code: Option<String>,
    pub recipient_city: Option<String>,
    pub recipient_country: Option<String>,
    pub recipient_email: Option<String>,
    pub recipient_phone: Option<String>,
    pub recipient_note: Option<String>,
    pub payment_type: Option<String>,
    pub payment_to_kind: Option<FlexibleValue>,
    pub payment_to: Option<String>,
    pub paid: Option<FlexibleValue>,
    pub paid_date: Option<String>,
    pub split_payment: Option<FlexibleValue>,
    pub skonto_active: Option<FlexibleValue>,
    pub skonto_discount_date: Option<String>,
    pub skonto_discount_value: Option<FlexibleValue>,
    pub currency: Option<String>,
    pub lang: Option<String>,
    pub exchange_currency: Option<String>,
    pub exchange_kind: Option<String>,
    pub exchange_currency_rate: Option<FlexibleValue>,
    pub exchange_note: Option<String>,
    pub description: Option<String>,
    pub description_footer: Option<String>,
    pub description_long: Option<String>,
    pub additional_invoice_field: Option<String>,
    pub internal_note: Option<String>,
    pub additional_info: Option<FlexibleValue>,
    pub additional_info_desc: Option<String>,
    pub show_discount: Option<FlexibleValue>,
    pub discount_kind: Option<String>,
    pub calculating_strategy: Option<CalculatingStrategy>,
    #[serde(alias = "use_moss")]
    pub use_oss: Option<FlexibleValue>,
    pub procedure_designations: Option<Vec<String>>,
    pub exempt_tax_kind: Option<String>,
    pub np_tax_kind: Option<String>,
    pub reverse_charge: Option<FlexibleValue>,
    pub procedure_vat_margin: Option<String>,
    pub corrected_content_before: Option<String>,
    pub corrected_content_after: Option<String>,
    pub exclude_from_stock_level: Option<FlexibleValue>,
    pub show_attachments: Option<FlexibleValue>,
    pub recipients: Option<Vec<Recipient>>,
    pub issuers: Option<Vec<Issuer>>,
    pub descriptions: Option<Vec<InvoiceDescription>>,
    pub settlement_positions: Option<Vec<SettlementPosition>>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoicePartyWrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_description: Option<String>,
    #[serde(rename = "_destroy", skip_serializing_if = "Option::is_none")]
    pub destroy: Option<FlexibleValue>,
}

pub type RecipientWrite = InvoicePartyWrite;
pub type IssuerWrite = InvoicePartyWrite;

#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoiceDescriptionWrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_index: Option<i64>,
    #[serde(rename = "_destroy", skip_serializing_if = "Option::is_none")]
    pub destroy: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SettlementPositionWrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "_destroy", skip_serializing_if = "Option::is_none")]
    pub destroy: Option<FlexibleValue>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoicePositionWrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(rename = "_destroy", skip_serializing_if = "Option::is_none")]
    pub destroy: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtu_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_net: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_gross: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price_net: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price_gross: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_tax: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_before_attributes: Option<CorrectionPositionAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_after_attributes: Option<CorrectionPositionAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_purchase_price: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_margin_tax: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_margin_total_price_net: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_margin_total_price_tax: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_margin_total_price_gross: Option<FlexibleValue>,
}

/// Shared partial payload accepted by both create and update endpoints.
#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoiceWrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid_unique: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_to_kind: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_template_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_invoice_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_tax_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_tax_no_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_www: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_person: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_bank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_bank_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_bdo_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_jst: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_gv: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_tax_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_tax_no_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_tax_no_validation: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_person: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_company: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_jst: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_gv: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_invoice_issuer: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_override: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_invoice_from: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_creation_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_value: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_prices_from_price_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_params: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_invoice_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_payment: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skonto_active: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skonto_discount_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skonto_discount_value: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_currency_rate: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_discount: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculating_strategy: Option<CalculatingStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_oss: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_vat_margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_designations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_tax_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub np_tax_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_charge: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrected_content_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrected_content_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_stock_level: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_attachments: Option<FlexibleValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_vat_tax_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_income_tax_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_note_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<InvoicePositionWrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<RecipientWrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<IssuerWrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<InvoiceDescriptionWrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_positions: Option<Vec<SettlementPositionWrite>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_invoice_with_flattened_parties() {
        let invoice: Invoice =
            serde_json::from_str(include_str!("../../tests/fixtures/invoice.json")).unwrap();
        assert_eq!(invoice.id, 123);
        assert_eq!(invoice.seller.name, "Example Seller");
        assert_eq!(invoice.buyer.name, "Example Buyer");
        assert!(matches!(invoice.kind, Some(Kind::Vat)));
    }

    #[test]
    fn use_moss_is_an_alias_and_scalars_are_lossless() {
        let mut value: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/invoice.json")).unwrap();
        value["use_moss"] = json!("1");
        value["paid"] = serde_json::Value::Number("1234567890.123456789".parse().unwrap());
        let invoice: Invoice = serde_json::from_value(value).unwrap();
        assert_eq!(invoice.use_oss, Some(FlexibleValue::from("1")));
        assert!(matches!(invoice.paid, Some(FlexibleValue::Number(_))));
    }

    #[test]
    fn partial_write_omits_unset_fields_and_renames_destroy() {
        let payload = InvoiceWrite {
            buyer_name: Some("New buyer".to_owned()),
            positions: Some(vec![InvoicePositionWrite {
                id: Some(42),
                destroy: Some(true.into()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(payload).unwrap(),
            json!({"buyer_name":"New buyer","positions":[{"id":42,"_destroy":true}]})
        );
    }

    #[test]
    fn deserializes_nested_and_extended_invoice_records() {
        let mut value: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/invoice.json")).unwrap();
        value["seller_tax_no_kind"] = json!("NIP");
        value["seller_bdo_no"] = json!("BDO-1");
        value["buyer_note"] = json!("note");
        value["recipient_id"] = json!(9);
        value["recipient_name"] = json!("Legacy recipient");
        value["currency"] = json!("PLN");
        value["accounting_kind"] = json!("expenses");
        value["discount_kind"] = json!("amount");
        value["calculating_strategy"] = json!({
            "position": "keep_gross", "sum": "keep_net", "invoice_form_price_kind": "gross"
        });
        value["recipients"] = json!([{
            "id": 1, "name": "Recipient", "company": "true", "role": "Odbiorca",
            "participation": "10.00"
        }]);
        value["issuers"] = json!([{"id": 2, "name": "Issuer", "company": true}]);
        value["descriptions"] = json!([{
            "id": 3, "kind": "Uwaga", "content": "Content", "position_index": null,
            "row_number": 1
        }]);
        value["settlement_positions"] = json!([{
            "id": 4, "kind": "charge", "amount": "12.30", "reason": "Shipping"
        }]);
        value["positions"] = json!([{
            "id": 5, "invoice_id": 123, "name": "Margin service", "description": null,
            "price_net": "80.00", "quantity": "1", "total_price_gross": "100.00",
            "total_price_net": "80.00", "account_id": null, "created_at": "2026-01-01",
            "updated_at": "2026-01-01", "additional_info": null, "quantity_unit": "szt",
            "tax": "disabled", "price_gross": "100.00", "price_tax": "20.00",
            "total_price_tax": "20.00", "kind": "correction", "invoice_position_id": null,
            "product_id": null, "deleted": false, "discount": null, "discount_percent": null,
            "tax2": "0", "exchange_rate": "1", "accounting_tax_kind": null, "code": null,
            "discount_net": null, "lump_sum_tax": null, "corrected_pos_kind": null,
            "gtu_code": "GTU_12", "total_purchase_price": "70.00", "vat_margin_tax": 23,
            "vat_margin_total_price_net": "24.39", "vat_margin_total_price_tax": "5.61",
            "vat_margin_total_price_gross": "30.00",
            "correction_before_attributes": {"name":"Old", "quantity":"2", "kind":"correction_before"},
            "correction_after_attributes": {"name":"New", "quantity":1, "kind":"correction_after"}
        }]);

        let invoice: Invoice = serde_json::from_value(value).unwrap();
        assert_eq!(invoice.seller.bdo_no.as_deref(), Some("BDO-1"));
        assert_eq!(invoice.recipient_name.as_deref(), Some("Legacy recipient"));
        assert_eq!(
            invoice.recipients.unwrap()[0].name.as_deref(),
            Some("Recipient")
        );
        assert_eq!(invoice.descriptions.unwrap()[0].row_number, Some(1));
        assert_eq!(
            invoice.settlement_positions.unwrap()[0].amount,
            Some(FlexibleValue::from("12.30"))
        );
        assert!(
            invoice.positions.unwrap()[0]
                .correction_after_attributes
                .is_some()
        );
    }

    #[test]
    fn serializes_nested_write_records_and_exact_decimal_text() {
        let payload = InvoiceWrite {
            positions: Some(vec![InvoicePositionWrite {
                name: Some("Service".into()),
                total_price_gross: Some("10.2300".into()),
                correction_before_attributes: Some(CorrectionPositionAttributes {
                    quantity: Some(2_u64.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            recipients: Some(vec![InvoicePartyWrite {
                name: Some("Recipient".into()),
                ..Default::default()
            }]),
            descriptions: Some(vec![InvoiceDescriptionWrite {
                content: Some("Note".into()),
                ..Default::default()
            }]),
            settlement_positions: Some(vec![SettlementPositionWrite {
                amount: Some("12.30".into()),
                reason: Some("Shipping".into()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let value = serde_json::to_value(payload).unwrap();
        assert_eq!(value["positions"][0]["total_price_gross"], "10.2300");
        assert_eq!(value["settlement_positions"][0]["amount"], "12.30");
    }

    #[test]
    fn deserializes_every_supported_invoice_kind() {
        let values = [
            "vat",
            "proforma",
            "bill",
            "receipt",
            "advance",
            "final",
            "correction",
            "vat_mp",
            "vat_rr",
            "invoice_other",
            "vat_margin",
            "kp",
            "kw",
            "dw",
            "wnt",
            "wdt",
            "estimate",
            "correction_note",
            "accounting_note",
            "client_order",
            "import_service",
            "import_service_eu",
            "import_products",
            "export_products",
        ];
        for value in values {
            serde_json::from_str::<Kind>(&format!("\"{value}\"")).unwrap();
        }
        assert!(serde_json::from_str::<Kind>("\"future_kind\"").is_err());
    }
}
