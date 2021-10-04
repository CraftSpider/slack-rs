use std::collections::HashMap;
use std::any::Any;

use reqwest::RequestBuilder;
use serde::de::{Deserialize, DeserializeOwned};

use crate::scopes::Scope;
use crate::types::*;

#[macro_export]
macro_rules! Method {
    ("admin.analytics.getFile") => { $crate::methods::AdminAnalyticsGetFile };
    ("admin.apps.approve") => { $crate::methods::AdminAppsApprove };
    ("conversations.list") => { $crate::methods::ConversationsList };
}

macro_rules! method_def {
    ($name:ident =>
        path: $api:literal,
        $(scopes: [$($scopes:literal),*],)?
        $(ratelimit: $limit:ident,)?
        $(inputs: [$($in_names:literal $(if $in_cond:expr)? => $in_tys:ty),* $(,)?],)?
        $(outputs: [$($ret_names:literal => $ret_tys:ty),* $(,)?],)?
    ) => {
        pub enum $name {}

        #[allow(unused_parens)]
        impl Method for $name {
            type Input = ( $( $( $in_tys ),* )? );

            type Return = ($( $( $ret_tys ),* )?);

            fn api_str() -> &'static str {
                $api
            }

            $(
            fn required_scopes() -> Vec<Scope> {
                vec![$( Scope::from_name($scopes) ),*]
            }
            )?

            $(
            fn rate_limit() -> Option<RateLimit> {
                Some(RateLimit::$limit)
            }
            )?

            #[allow(unused_variables, unused_mut)]
            fn write_out(mut request: RequestBuilder, mut inputs: HashMap<String, &dyn Any>) -> RequestBuilder {
                let mut json_inputs: HashMap<&'static str, serde_json::Value> = HashMap::new();

                $(
                $(

                let raw_input = inputs
                    .get($in_names);

                if let Some(input_ref) = raw_input {
                    json_inputs.insert(
                        $in_names,
                        serde_json::to_value(input_ref.downcast_ref::<$in_tys>().unwrap())
                            .expect(concat!("Couldn't serialize ", $in_names)),
                    );
                }
                // If user supplies a condition, we panic if it doesn't hold
                $( else if $in_cond && raw_input.is_none() {
                    panic!(concat!("Expected parameter ", $in_names));
                } )?

                )*
                )?

                // TODO: Handle unknown parameters
                // assert!(inputs.is_empty(), "Unhandled inputs to `write_out`: {:?}", inputs.keys());

                request.form(&json_inputs)
            }

            #[allow(unused_variables, unused_mut)]
            fn parse_data(mut map: HashMap<String, serde_json::Value>) -> Self::Return {
                let out = ();

                $(
                let out = ($(
                    <$ret_tys>::deserialize(map.remove($ret_names).unwrap()).unwrap()
                ),*);
                )?

                debug_assert!(map.is_empty(), "Expected extra data to be empty after parsing, instead got {:?}", map);
                out
            }
        }
    };
}

pub trait Method {
    type Input;

    type Return: DeserializeOwned;

    fn api_str() -> &'static str;

    /// The scopes necessary to use this endpoint. Defaults to none
    fn required_scopes() -> Vec<Scope> {
        vec![]
    }

    /// The rate limits on this API. Defaults to none
    fn rate_limit() -> Option<RateLimit> {
        None
    }

    fn write_out(request: RequestBuilder, inputs: HashMap<String, &dyn Any>) -> RequestBuilder;

    fn parse_data(map: HashMap<String, serde_json::Value>) -> Self::Return;
}

method_def! {
    AdminAnalyticsGetFile =>
        path: "admin.analytics.getFile",
        scopes: ["admin.analytics:read"],
        ratelimit: Tier2,
        inputs: ["type" if true => String, "date" => String, "metadata_only" => bool],
}

method_def! {
    AdminAppsApprove =>
        path: "admin.apps.approve",
        scopes: ["admin.apps:write"],
        ratelimit: Tier2,
        inputs: [
            "app_id" => AppId,
            "enterprise_id" => EnterpriseId,
            "request_id" => String,
            "team_id" => TeamId,
        ],
        outputs: [],
}

// TODO
// method_def! {
//     ChatPostMessage =>
//         path: "chat.postMessage",
//         scopes: ["chat:write", "chat:write:user", "chat:write:bot"],
//         ratelimit: Tier4,
//         req_inputs: [channel | "channel" => ChannelId, text | "text" => String, user | "user" => UserId],
//         inputs: []
// }

method_def! {
    ConversationsList =>
        path: "conversations.list",
        scopes: ["channels:read", "groups:read", "im:read", "mpim:read"],
        ratelimit: Tier2,
        inputs: [
            "cursor" => String,
            "exclude_archived" => bool,
            "limit "=> u64,
            "team_id" => TeamId,
            "types" => String,
        ],
        outputs: ["channels" => Vec<Conversation>, "response_metadata" => ResponseMeta],
}
