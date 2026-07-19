#![recursion_limit = "256"]

pub mod client;
pub mod types;

// Re-export main types for convenience
pub use client::client::Client;
pub use client::invoice::{
    Income, InvoiceOrder, InvoiceOrderField, InvoiceRequestOptions, ListInvoicesOptions, Period,
    SearchDateType,
};
pub use types::invoice::{FlexibleValue, Invoice, InvoicePositionWrite, InvoiceWrite, Kind};
