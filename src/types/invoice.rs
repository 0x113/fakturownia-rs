use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
}

#[derive(Debug, Deserialize)]
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
    pub company: Option<bool>,
    #[serde(rename = "buyer_mobile_phone")]
    pub mobile_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
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
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    pub id: u64,
    pub number: String,
    #[serde(flatten)]
    pub seller: Seller,
    #[serde(flatten)]
    pub buyer: Buyer,
    pub positions: Option<Vec<Position>>,
}
