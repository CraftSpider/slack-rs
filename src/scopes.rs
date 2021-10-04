use TokenType::{App, Bot, User};

pub enum TokenType {
    App,
    Bot,
    User,
}

pub struct Scope {
    name: &'static str,
    valid_tokens: &'static [TokenType]
}

#[doc(hidden)]
impl Scope {
    const ADMIN: Scope = Scope::new("admin", &[User]);
    const ADMIN_ANALYTICS_READ: Scope = Scope::new("admin.analytics:read", &[User]);
    const ADMIN_APPS_READ: Scope = Scope::new("admin.apps:read", &[User]);
    const ADMIN_APPS_WRITE: Scope = Scope::new("admin.apps:write", &[User]);
    const ADMIN_BARRIERS_READ: Scope = Scope::new("admin.barriers:read", &[User]);
    const ADMIN_BARRIERS_WRITE: Scope = Scope::new("admin.barriers:write", &[User]);
    const ADMIN_CONVERSATIONS_READ: Scope = Scope::new("admin.conversations:read", &[User]);
    const ADMIN_CONVERSATIONS_WRITE: Scope = Scope::new("admin.conversations:write", &[User]);
    const ADMIN_INVITES_READ: Scope = Scope::new("admin.invites:read", &[User]);
    const ADMIN_INVITES_WRITE: Scope = Scope::new("admin.invites:write", &[User]);
    const ADMIN_TEAMS_READ: Scope = Scope::new("admin.teams:read", &[User]);
    const ADMIN_TEAMS_WRITE: Scope = Scope::new("admin.teams:write", &[User]);
    const ADMIN_USERGROUPS_READ: Scope = Scope::new("admin.usergroups:read", &[User]);
    const ADMIN_USERGROUPS_WRITE: Scope = Scope::new("admin.uesrgroups:write", &[User]);
    const ADMIN_USERS_READ: Scope = Scope::new("admin.users:read", &[User]);
    const ADMIN_USERS_WRITE: Scope = Scope::new("admin.users:write", &[User]);
    const APP_MENTIONS_READ: Scope = Scope::new("app_mentions:read", &[Bot]);
    const AUDITLOGS_READ: Scope = Scope::new("auditlogs:read", &[User]);
    const AUTHORIZATIONS_READ: Scope = Scope::new("authorizations:read", &[App]);
    const CALLS_READ: Scope = Scope::new("calls:read", &[Bot, User]);
    const CALLS_WRITE: Scope = Scope::new("calls:write", &[Bot, User]);
    const CHANNELS_HISTORY: Scope = Scope::new("channels:history", &[Bot, User]);
    const CHANNELS_JOIN: Scope = Scope::new("channels:join", &[Bot]);
    const CHANNELS_MANAGE: Scope = Scope::new("channels:manage", &[Bot]);
    const CHANNELS_READ: Scope = Scope::new("channels:read", &[Bot, User]);
    const CHANNELS_WRITE: Scope = Scope::new("channels:write", &[User]);
    const CHAT_WRITE: Scope = Scope::new("chat:write", &[Bot, User]);
    const CHAT_WRITE_CUSTOMIZE: Scope = Scope::new("chat:write.customize", &[Bot]);
    const CHAT_WRITE_PUBLIC: Scope = Scope::new("chat:write.public", &[Bot]);
    const CHAT_WRITE_BOT: Scope = Scope::new("chat:write:bot", &[User]);
    const CHAT_WRITE_USER: Scope = Scope::new("chat:write:user", &[User]);
    const COMMANDS: Scope = Scope::new("commands", &[Bot, User]);
    const CONNECTIONS_WRITE: Scope = Scope::new("connections:write", &[App]);
    const CONVERSATIONS_CONNECT_MANNAGE: Scope = Scope::new("conversations.connect:manage", &[Bot]);
    const CONVERSATIONS_CONNECT_READ: Scope = Scope::new("conversations.connect:read", &[Bot]);
    const CONVERSATIONS_CONNECT_WRITE: Scope = Scope::new("conversations.connect:write", &[Bot]);
    const GROUPS_READ: Scope = Scope::new("groups:read", &[Bot, User]);
    const IM_READ: Scope = Scope::new("im:read", &[Bot, User]);
    const MPIM_READ: Scope = Scope::new("mpim:read", &[Bot, User]);

    const fn new(name: &'static str, valid_tokens: &'static [TokenType]) -> Scope {
        Scope { name, valid_tokens }
    }

    pub fn from_name(name: &'static str) -> Scope {
        match name {
            "admin" => Self::ADMIN,
            "admin.analytics:read" => Self::ADMIN_ANALYTICS_READ,
            "admin.apps:read" => Self::ADMIN_APPS_READ,
            "admin.apps:write" => Self::ADMIN_APPS_WRITE,
            "channels:read" => Self::CHANNELS_READ,
            "groups:read" => Self::GROUPS_READ,
            "im:read" => Self::IM_READ,
            "mpim:read" => Self::MPIM_READ,
            _ => panic!("Unrecognized scope name"),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn token_types(&self) -> &'static [TokenType] {
        self.valid_tokens
    }
}

pub enum OldScope {
    Admin,
    AdminAnalyticsRead,
    ChannelsRead,
    GroupsRead,
    ImRead,
    MpimRead,
}
