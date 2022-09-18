use apid_telegram_bot::types::{
    Chat, ChatKind, ChatUser, Message, MessageContent, MessageEntity, MessageEntityKind, User,
};
use pretty_assertions::assert_eq;

#[test]
fn deserialize_message() {
    let src = r#"
        {
            "message_id": 1,
            "from": {
                "id": 229584557,
                "is_bot": false,
                "first_name": "RanolP`22",
                "username": "FunctionalInterface",
                "language_code": "en"
            },
            "chat": {
                "id": 229584557,
                "first_name": "RanolP`22",
                "username": "FunctionalInterface",
                "type": "private"
            },
            "date": 1663491550,
            "text": "/start",
            "entities": [
                {
                    "offset": 0,
                    "length": 6,
                    "type": "bot_command"
                }
            ]
        }
    "#;
    let update: Message = serde_json::from_str(src).unwrap();
    assert_eq!(
        Message {
            message_id: 1,
            from: Some(User {
                id: 229584557,
                is_bot: false,
                first_name: "RanolP`22".to_string(),
                last_name: None,
                username: Some("FunctionalInterface".to_string()),
                language_code: Some("en".to_string()),
                is_premium: false,
                added_to_attachment_menu: false,
                can_join_groups: false,
                can_read_all_group_messages: false,
                supports_inline_queries: false,
            }),
            sender_chat: None,
            date: 1663491550,
            chat: Chat {
                id: 229584557,
                kind: ChatKind::Private,
                title: None,
                username: Some("FunctionalInterface".to_string()),
                chat_user: Some(ChatUser {
                    first_name: "RanolP`22".to_string(),
                    last_name: None,
                    bio: None
                }),
                photo: None,
                has_private_forwards: false,
                has_restricted_voice_and_video_messages: false,
                join_to_send_messages: false,
                join_by_request: false,
                description: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                slow_mode_delay: None,
                message_auto_delete_time: None,
                has_protected_content: false,
                sticker_set_name: None,
                can_set_sticker_set: false,
                linked_chat_id: None,
                location: None
            },
            forward_from: None,
            forward_from_chat: None,
            forward_from_message_id: None,
            forward_signature: None,
            forward_sender_name: None,
            forward_date: None,
            is_automatic_forward: false,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: false,
            media_group_id: None,
            author_signature: None,
            content: MessageContent::Text {
                text: "/start".to_string(),
                entities: vec![MessageEntity {
                    kind: MessageEntityKind::BotCommand,
                    offset: 0,
                    length: 6
                }]
            },
            web_app_data: None,
            reply_markup: None,
        },
        update,
    )
}
