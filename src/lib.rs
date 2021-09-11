
use reqwest::Client as ReqwestClient;

pub mod types;

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

    async fn make_request<T: methods::Method>(&self, req: T::Input, opt: T::OptInput) -> Result<SlackResponse<T::Return>, SlackError> {
        let url = Self::URL_BASE.to_string() + T::api_str();

        let request = self.req_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token));

        let request = T::write_out(request, req, opt);

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

    async fn make_request_req<T: methods::Method>(&self, req: T::Input) -> Result<SlackResponse<T::Return>, SlackError> {
        self.make_request::<T>(req, T::opt_empty()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basics() {
        let client = SlackClient::new(
            "xoxb-2463664257206-2483035347057-kXleXcVXzYGmBc1Kt1B5xdcw"
        );

        println!(
            "{:#?}",
            client.make_request_req::<methods::ConversationList>(()).await,
        );

        println!(
            "{:#?}",
            client.make_request::<methods::ConversationList>((), (None, Some(true), None, None, None)).await,
        )
    }
}
