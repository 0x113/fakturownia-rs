use crate::client::invoice::InvoicesEndpoint;
use reqwest::{Method, Url};
use serde::Serialize;
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

        Ok(Self::with_base_url(api_base, api_key)?)
    }

    pub fn with_base_url(api_base: Url, api_key: String) -> Result<Client, reqwest::Error> {
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
        body["api_token"] = json!(self.api_key);
        self.json_request(Method::POST, path, &body).await
    }

    pub(crate) async fn authenticated_json_request<T: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut body = serde_json::to_value(body).expect("request payload must serialize");
        body["api_token"] = json!(self.api_key);
        self.json_request(method, path, &body).await
    }

    async fn json_request<T: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .request(method, self.build_url_for_post_request(path))
            .json(body)
            .send()
            .await?
            .error_for_status()
    }

    pub fn invoices(&self) -> InvoicesEndpoint<'_> {
        InvoicesEndpoint(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> Client {
        Client::with_base_url(
            Url::parse("http://localhost:3000").unwrap(),
            "test-token".to_owned(),
        )
        .unwrap()
    }

    #[test]
    fn build_url_adds_json_suffix_and_api_token() {
        let url = client().build_url("invoices/123");

        assert_eq!(url.path(), "/invoices/123.json");
        assert_eq!(url.query(), Some("api_token=test-token"));
    }

    #[test]
    fn build_url_does_not_duplicate_json_suffix() {
        let url = client().build_url("invoices.json");

        assert_eq!(url.path(), "/invoices.json");
    }

    #[test]
    fn post_url_omits_api_token_from_query() {
        let url = client().build_url_for_post_request("invoices/cancel");

        assert_eq!(url.as_str(), "http://localhost:3000/invoices/cancel.json");
    }
}
