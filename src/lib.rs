
use std::collections::HashMap;
use std::any::Any;

use reqwest::Client as ReqwestClient;

pub mod types;
pub mod scopes;
pub mod methods;

use types::*;

pub struct SlackClient {
    token: String,
    req_client: ReqwestClient,
}

impl SlackClient {
    const URL_BASE: &'static str = "https://slack.com/api/";

    pub fn new(token: &str) -> SlackClient {
        SlackClient {
            token: token.to_string(),
            req_client: ReqwestClient::new(),
        }
    }

    async fn make_request<T: methods::Method>(&self, inputs: HashMap<String, &dyn Any>) -> Result<SlackResponse<T::Return>, SlackError> {
        let url = Self::URL_BASE.to_string() + T::api_str();

        let request = self.req_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token));

        let request = T::write_out(request, inputs);

        // TODO: What if it's gzipped or not JSON?
        let raw_response: RawResponse = request
            .send()
            .await?
            .json()
            .await
            .unwrap();

        if raw_response.ok {
            let response = SlackResponse {
                data: T::parse_data(raw_response.other),
                warnings: raw_response.warnings.map(Warning::from_str)
            };
            Ok(response)
        } else {
            let response = SlackError::ApiError(Error::from_str(raw_response.error.unwrap()));
            Err(response)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basics() {
        let client = SlackClient::new(
            env!("SLACK_TOKEN")
        );

        println!(
            "{:#?}",
            client.make_request::<Method!["conversations.list"]>(HashMap::new()).await,
        );

        println!(
            "{:#?}",
            client.make_request::<Method!["conversations.list"]>(
                HashMap::from([
                    (String::from("exclude_archived"), &true as &dyn std::any::Any),
                ])
            ).await,
        )
    }
}
