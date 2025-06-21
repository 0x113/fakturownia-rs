use super::client::Client;
use crate::types::Invoice;
use reqwest::Error;

pub struct ListInvoicesOptions {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub include_positions: bool,
    pub period: Option<Period>,
}

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
    fn as_str(&self) -> &str {
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

pub struct InvoicesEndpoint<'c>(pub &'c Client);

impl<'c> InvoicesEndpoint<'c> {
    pub async fn list_invoices(
        &self,
        options: Option<ListInvoicesOptions>,
    ) -> Result<Vec<Invoice>, Error> {
        let mut url = self.0.build_url("invoices");
        if let Some(options) = options {
            if let Some(page) = options.page {
                url.query_pairs_mut().append_pair("page", &page.to_string());
            }
            if let Some(per_page) = options.per_page {
                url.query_pairs_mut()
                    .append_pair("per_page", &per_page.to_string());
            }
            if options.include_positions {
                url.query_pairs_mut()
                    .append_pair("include_positions", "true");
            }
            if let Some(period) = options.period {
                url.query_pairs_mut().append_pair("period", period.as_str());
            }
        }
        let response = self.0.client.get(url).send().await?;
        let invoices: Vec<Invoice> = response.json().await?;
        Ok(invoices)
    }

    pub async fn get_invoice(&self, id: u64) -> Result<Invoice, Error> {
        let url = self.0.build_url(&format!("invoices/{}", id));
        let response = self.0.client.get(url).send().await?;
        let invoice: Invoice = response.json().await?;
        Ok(invoice)
    }
}
