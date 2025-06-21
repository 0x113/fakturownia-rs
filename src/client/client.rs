use crate::client::invoice::InvoicesEndpoint;
use reqwest::Url;

pub struct Client {
    api_base: Url,
    api_key: String,
    pub client: reqwest::Client,
}

impl Client {
    pub fn new(domain: String, api_key: String) -> Result<Client, Box<dyn std::error::Error>> {
        let api_base = Url::parse(format!("https://{}.fakturownia.pl", domain).as_str())?;
        let client = reqwest::Client::builder().build()?;

        Ok(Client {
            api_base,
            api_key,
            client,
        })
    }

    pub fn build_url(&self, path: &str) -> Url {
        let mut url = self.api_base.join(path).unwrap();
        url.query_pairs_mut()
            .append_pair("api_token", &self.api_key);

        // Add .json to the path if it's not there
        if !url.path().ends_with(".json") {
            url.set_path(format!("{}.json", url.path()).as_str());
        }
        url
    }

    pub fn invoices(&self) -> InvoicesEndpoint<'_> {
        InvoicesEndpoint(self)
    }
}
