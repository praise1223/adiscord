use serde_repr::{Deserialize_repr, Serialize_repr};

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MessageType {
    DEFAULT,
    RECIPIENT_ADD,
    RECIPIENT_REMOVE,
    CALL,
    CHANNEL_NAME_CHANGE,
    CHANNEL_ICON_CHANGE,
    CHANNEL_PINNED_MESSAGE,
    USER_JOIN,
    GUILD_BOOST,
    GUILD_BOOST_TIER_1,
    GUILD_BOOST_TIER_2,
    GUILD_BOOST_TIER_3,
    CHANNEL_FOLLOW_ADD,
    GUILD_DISCOVERY_DISQUALIFIED = 14,
    GUILD_DISCOVERY_REQUALIFIED,
    GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING,
    GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING,
    THREAD_CREATED,
    REPLY,
    CHAT_INPUT_COMMAND,
    THREAD_STARTER_MESSAGE,
    GUILD_INVITE_REMINDER,
    CONTEXT_MENU_COMMAND,
    AUTO_MODERATION_ACTION,
    ROLE_SUBSCRIPTION_PURCHASE,
    INTERACTION_PREMIUM_UPSELL,
    STAGE_START,
    STAGE_END,
    STAGE_SPEAKER,
    STAGE_TOPIC,
    GUILD_APPLICATION_PREMIUM_SUBSCRIPTION,
}
