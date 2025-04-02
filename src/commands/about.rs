use std::time::{SystemTime, UNIX_EPOCH};
use twilight_http::Client as HttpClient;
use twilight_model::id::marker::InteractionMarker;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::InteractionContextType,
    },
    channel::message::{embed::EmbedField, Embed},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
    oauth::ApplicationIntegrationType,
    util::Timestamp,
};

pub async fn handle(
    http: &HttpClient,
    interaction_id: Id<InteractionMarker>,
    interaction_token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = Timestamp::from_secs(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards(?)")
            .as_secs() as i64,
    )?;
    let embed = Embed {
        title: Some("about me!".to_string()),
        description: Some("i'm a discord bot named **R. Yui**. i was developed by **reinacchi** for no absolute reason.".to_string()),
        color: Some(0xc8f4f4),
        fields: vec![EmbedField {
            name: "github repository".to_string(),
            value: "i happily am an open-source project. you can find my **github** [here](https://github.com/reinacchi/R.Yui). i'm written in [rust](https://rust-lang.org) using the [twilight](https://github.com/twilight-rs/twilight) discord library.".to_string(),
            inline: false,
        }],
        author: None,
        footer: None,
        image: None,
        thumbnail: None,
        video: None,
        provider: None,
        timestamp: Some(timestamp),
        url: None,
        kind: "rich".to_string(),
    };

    http.interaction(interaction_id.cast())
        .create_response(
            interaction_id,
            interaction_token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    embeds: Some(vec![embed]),
                    flags: None,
                    ..Default::default()
                }),
            },
        )
        .await?;

    Ok(())
}

pub fn register(application_id: Id<ApplicationMarker>) -> Command {
    Command {
        application_id: Some(application_id),
        name: "about".to_string(),
        description: "get information about me.".to_string(),
        options: Vec::new(),
        default_member_permissions: None,
        guild_id: None,
        id: Id::new_checked(0),
        kind: CommandType::ChatInput,
        name_localizations: None,
        dm_permission: Some(true),
        description_localizations: None,
        contexts: Some(vec![
            InteractionContextType::Guild,
            InteractionContextType::BotDm,
            InteractionContextType::PrivateChannel,
        ]),
        integration_types: Some(vec![
            ApplicationIntegrationType::GuildInstall,
            ApplicationIntegrationType::UserInstall,
        ]),
        nsfw: Some(false),
        version: Id::new(1),
    }
}
