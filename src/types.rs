
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::Error as SerdeError;
use std::collections::HashMap;

fn bool_false<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>
{
    if bool::deserialize(de)? {
        Err(D::Error::custom("Expected boolean false value"))
    } else {
        Ok(true)
    }
}

pub mod methods {
    use reqwest::RequestBuilder;
    use serde::de::DeserializeOwned;

    use super::*;

    macro_rules! method {
        ($name:ident =>
            path: $api:literal,
            $(scopes: [$($scopes:ident),*],)?
            $(ratelimit: $limit:ident,)?
            $(req_inputs: [$($in_names:ident => $in_tys:ty),*],)?
            $(inputs: [$($in_opt_names:ident => $in_opt_tys:ty),*],)?
            $(outputs: [$($ret_names:ident => $ret_tys:ty),*],)?
        ) => {
            pub enum $name {}

            #[allow(unused_parens)]
            impl Method for $name {
                type Input = ( $( $( $in_tys ),* )? );
                type OptInput = ( $( $( Option<$in_opt_tys> ),* )? );

                type Return = ($( $( $ret_tys ),* )?);

                fn api_str() -> &'static str {
                    $api
                }

                $(
                fn required_scopes() -> Vec<Scope> {
                    vec![$( Scope::$scopes ),*]
                }
                )?

                $(
                fn rate_limit() -> Option<RateLimit> {
                    Some(RateLimit::$limit)
                }
                )?

                fn opt_empty() -> Self::OptInput {
                    ( $( $( None::<$in_opt_tys> ),* )? )
                }

                #[allow(unused_variables, unused_mut)]
                fn write_out(mut request: RequestBuilder, req: Self::Input, opt: Self::OptInput) -> RequestBuilder {
                    let mut output: HashMap<_, serde_json::Value> = HashMap::new();

                    $(
                    let ( $( $in_names ),* ) = req;

                    $(
                    output.insert(
                        stringify!($in_names),
                        serde_json::to_value($in_names).unwrap()
                    );
                    )*

                    )?

                    $(
                    let ( $( $in_opt_names ),* ) = opt;

                    $(
                    $in_opt_names
                        .map(|val| output.insert(
                            stringify!($in_opt_names),
                            serde_json::to_value(val).unwrap()
                        ));
                    )*

                    )?

                    request.form(&output)
                }

                #[allow(unused_variables, unused_mut)]
                fn parse_data(mut map: HashMap<String, serde_json::Value>) -> Self::Return {
                    let out = ();

                    $(
                    let out = ($(
                        <$ret_tys>::deserialize(map.remove(stringify!($ret_names)).unwrap()).unwrap()
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
        type OptInput;

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

        fn opt_empty() -> Self::OptInput;

        fn write_out(request: RequestBuilder, req: Self::Input, opt: Self::OptInput) -> RequestBuilder;

        fn parse_data(map: HashMap<String, serde_json::Value>) -> Self::Return;
    }

    method! {
        ConversationList =>
            path: "conversations.list",
            scopes: [ChannelsRead, GroupsRead, ImRead, MpimRead],
            ratelimit: Tier2,
            inputs: [cursor => String, exclude_archived => bool, limit => u64, team_id => TeamId, types => String],
            outputs: [channels => Vec<Conversation>, response_metadata => ResponseMeta],
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    NotAuthed,
    NotAllowedTokenType,
}

impl Error {
    pub(crate) fn from_str(str: String) -> Vec<Error> {
        str.split(",")
            .map(|item| {
                match item {
                    "not_authed" => Error::NotAuthed,
                    "not_allowed_token_type" => Error::NotAllowedTokenType,
                    _ => panic!("Unknown Error: {}", item)
                }
            })
            .collect()

    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Warning {}

impl Warning {
    pub(crate) fn from_str(str: String) -> Vec<Warning> {
        str.split(",")
            .map(|item| {
                match item {
                    _ => panic!("Unknown Warning: {}", item)
                }
            })
            .collect()
    }
}

pub enum Scope {
    ChannelsRead,
    GroupsRead,
    ImRead,
    MpimRead,
}

pub enum RateLimit {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
    TierSpecial,
    PostMessage,
    IncomingWebhooks,
    Events,
}

#[derive(Debug)]
pub struct AppToken(String);

impl AppToken {
    fn new(str: &str) -> Option<AppToken> {
        if str.starts_with("xapp") {
            Some(AppToken(str.to_string()))
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct BotId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct EnterpriseId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceId(String);

#[derive(Debug)]
pub enum SlackError {
    ReqwestError(reqwest::Error),
    ApiError(Vec<Error>),
}

impl From<reqwest::Error> for SlackError {
    fn from(err: reqwest::Error) -> SlackError {
        SlackError::ReqwestError(err)
    }
}

#[derive(Debug)]
pub struct SlackResponse<T> {
    pub(crate) data: T,
    pub(crate) warnings: Option<Vec<Warning>>,
}

#[derive(Serialize, Deserialize)]
pub struct RawResponse {
    pub(crate) ok: bool,
    pub(crate) warnings: Option<String>,
    pub(crate) error: Option<String>,
    #[serde(flatten)]
    pub(crate) other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ResponseMeta {
    PagingWarning {
        next_cursor: String,
        messages: Vec<String>,
        warnings: Vec<Warning>,
    },
    Paging {
        next_cursor: String
    },
    Warning {
        messages: Vec<String>,
        warnings: Vec<Warning>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    value: String,
    creator: String,
    last_set: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Purpose {
    value: String,
    creator: String,
    last_set: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timestamp(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayCounts {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Share {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotProfile {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileShort {
    // TODO
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    attachments: Option<Vec<Attachment>>,
    blocks: Option<Block>,
    bot_id: Option<BotId>,
    bot_profile: Option<BotProfile>,
    client_msg_id: Option<String>,
    comment: Option<Comment>,
    display_as_bot: Option<bool>,
    file: Option<File>,
    files: Option<Vec<File>>,
    icons: Option<Icon>,
    inviter: Option<UserId>,

    is_delayed_message: Option<bool>,
    is_intro: Option<bool>,
    is_starred: Option<bool>,

    last_read: Option<Timestamp>,
    latest_reply: Option<Timestamp>,
    name: Option<String>,
    old_name: Option<String>,
    parent_user_id: Option<UserId>,
    permalink: Option<String>, // TODO: Actually URI
    pinned_to: Option<Vec<ChannelId>>,
    purpose: Option<String>,
    reactions: Option<Vec<Reaction>>,
    reply_count: Option<u64>,
    reply_users: Option<Vec<UserId>>,
    reply_users_count: Option<u64>,
    source_team: Option<WorkspaceId>,
    subscribed: Option<bool>,
    subtype: Option<String>,
    team: Option<WorkspaceId>,
    text: String,
    thread_ts: Option<Timestamp>,
    topic: Option<String>,
    ts: Timestamp,
    #[serde(rename = "type")]
    ty: String,
    unread_count: Option<u64>,
    upload: Option<bool>,
    user: Option<UserId>,
    user_profile: Option<UserProfileShort>,
    user_team: Option<WorkspaceId>,
    username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Channel {
    accepted_user: UserId,
    created: u64,
    creator: UserId,
    id: ChannelId,

    is_archived: bool,
    is_channel: bool,
    is_frozen: bool,
    is_general: bool,
    is_member: bool,
    is_moved: bool,
    is_mpim: bool,
    is_non_threadable: bool,
    is_org_shared: bool,
    is_pending_ext_shared: bool,
    is_private: bool,
    is_read_only: bool,
    is_shared: bool,
    is_thread_only: bool,

    last_read: Timestamp,
    latest: Option<Message>,
    members: Vec<UserId>,
    name: String,
    name_normalized: String,
    num_members: u64,
    pending_shared: Vec<TeamId>,
    previous_names: Vec<String>,
    priority: f64,
    purpose: Purpose,
    topic: Topic,
    unlinked: u64,
    unread_count: u64,
    unread_count_display: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Conversation {
    Base(ConversationBase),
    Mpim(ConversationMpim),
    Im(ConversationIm),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationBase {
    accepted_user: Option<UserId>,
    connected_team_ids: Option<Vec<WorkspaceId>>,
    conversation_host_id: Option<WorkspaceId>,
    created: u64,
    creator: UserId,
    display_counts: Option<DisplayCounts>,
    enterprise_id: Option<EnterpriseId>,
    has_pins: Option<bool>,
    id: ChannelId,
    internal_team_ids: Option<Vec<TeamId>>,

    is_archived: bool,
    is_channel: bool,
    is_ext_shared: Option<bool>,
    is_frozen: Option<bool>,
    is_general: bool,
    is_global_shared: Option<bool>,
    is_group: bool,
    is_im: bool,
    is_member: Option<bool>,
    is_moved: Option<u64>,
    #[serde(deserialize_with = "bool_false")]
    is_mpim: bool,
    is_non_threadable: Option<bool>,
    is_open: Option<bool>,
    is_org_default: Option<bool>,
    is_org_mandatory: Option<bool>,
    is_org_shared: bool,
    is_pending_ext_shared: Option<bool>,
    is_private: bool,
    is_read_only: Option<bool>,
    is_shared: bool,
    is_starred: Option<bool>,
    is_thread_only: Option<bool>,

    last_read: Option<Timestamp>,
    latest: Option<Message>,
    members: Option<Vec<UserId>>,
    name: String,
    name_normalized: String,
    num_members: Option<u64>,
    parent_conversation: Option<ChannelId>,
    pending_connected_team_ids: Option<Vec<TeamId>>,
    pending_shared: Option<Vec<TeamId>>,
    pin_count: Option<u64>,
    previous_names: Option<Vec<String>>,
    priority: Option<f64>,
    purpose: Purpose,
    shared_team_ids: Option<Vec<TeamId>>,
    shares: Option<Vec<Share>>,
    timezone_count: Option<u64>,
    topic: Topic,
    unlinked: Option<u64>,
    unread_count: Option<u64>,
    unread_count_display: Option<u64>,
    use_case: Option<String>,
    user: Option<UserId>,
    version: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationMpim {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationIm {

}
