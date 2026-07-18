use crate::client::invoice::InvoicesEndpoint;
use reqwest::Url;
use serde_json::json;

// Represents the client for the Fakturownia API.
pub struct Client {
    pub api_base: Url,
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

    pub fn build_url_for_post_request(&self, path: &str) -> Url {
        self.build_url_helper(path, false)
    }

    pub fn build_url(&self, path: &str) -> Url {
        self.build_url_helper(path, true)
    }

    fn build_url_helper(&self, path: &str, include_api_token: bool) -> Url {
        let mut url = self.api_base.join(path).unwrap();
        if include_api_token {
            url.query_pairs_mut()
                .append_pair("api_token", &self.api_key);
        }

        // Add .json to the path if it's not there
        if !url.path().ends_with(".json") {
            url.set_path(format!("{}.json", url.path()).as_str());
        }
        url
    }

    /// Makes a POST request to the specified endpoint with JSON body.
    /// Automatically adds the API token to the request body.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (without domain)
    /// * `body` - JSON body to send with the request
    ///
    /// # Returns
    /// The HTTP response from the server
    ///
    /// # Errors
    /// Returns an error if the request fails or the network is unreachable
    pub async fn post(
        &self,
        path: &str,
        mut body: serde_json::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.build_url_for_post_request(path);

        // Add api_token to the body
        body["api_token"] = json!(self.api_key);
        let response = self.client.post(url).json(&body).send().await?;
        Ok(response)
    }

    pub fn invoices(&self) -> InvoicesEndpoint<'_> {
        InvoicesEndpoint(self)
    }
}
