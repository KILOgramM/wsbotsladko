//strukture 4 discord for bot
//use websocket::{OwnedMessage};
use crate::dstruct::DoubleChanel;
use serde_json::Value;
use crate::dstruct::{DMessage,DUser};

pub enum LocalLink{
	None
}
impl Default for LocalLink{
	fn default() -> LocalLink{LocalLink::None}
}

pub enum OutLink{
	None,
	Event(Event),
}
impl Default for OutLink{
	fn default() -> OutLink{OutLink::None}
}

pub enum UniChanel{
	Responce(Value),
	Close,
	None,
}
impl Default for UniChanel{
	fn default() -> UniChanel{UniChanel::None}
}

pub enum GlobE{
	GetChanel(DoubleChanel<OutLink>),
	Drop,
	None,
}
impl Default for GlobE{
	fn default() -> GlobE{GlobE::None}
}
#[derive(Clone, Debug)]
pub enum Event{
//	Hello(Value),
	Ready(Value),
//	Resumed(Value),
//	InvalidSession(Value),
	ChannelCreate(Value),
	ChannelUpdate(Value),
	ChannelDelete(Value),
	ChannelPinsUpdate(Value),
	GuildCreate(Value),
	GuildUpdate(Value),
	GuildDelete(Value),
	GuildBanAdd(Value),
	GuildBanRemove(Value),
	GuildEmojisUpdate(Value),
	GuildIntegrationsUpdate(Value),
	GuildMemberAdd(Value),
	GuildMemberRemove(Value),
	GuildMemberUpdate(Value),
	GuildMembersChunk(Value),
	GuildRoleCreate(Value),
	GuildRoleUpdate(Value),
	GuildRoleDelete(Value),
	MessageCreate(DMessage),
	MessageUpdate(Value),
	MessageDelete(Value),
	MessageDeleteBulk(Value),
	MessageReactionAdd(Value),
	MessageReactionRemove(Value),
	MessageReactionRemoveAll(Value),
//	PresenceUpdate(Value),
	TypingStart(Value),
	UserUpdate(Value),
//	VoiceStateUpdate(Value),
//	VoiceServerUpdate(Value),
//	WebhooksUpdate(Value)
}
impl Event{
	pub fn frome_json(value: Value) -> Option<Event>{

		let json:Value = value["d"].clone();
		if let Some(t) = value["t"].as_str(){
			match t {
//				"HELLO" => {return Some(Event::Hello(json) ); }
				"READY" => {return Some(Event::Ready(json) ); }
//				"RESUMED" => {return Some(Event::Resumed(json) ); }
//				"INVALID_SESSION" => {return Some(Event::InvalidSession(json) ); }
				"CHANNEL_CREATE" => {return Some(Event::ChannelCreate(json) ); }
				"CHANNEL_UPDATE" => {return Some(Event::ChannelUpdate(json) ); }
				"CHANNEL_DELETE" => {return Some(Event::ChannelDelete(json) ); }
				"CHANNEL_PINS_UPDATE" => {return Some(Event::ChannelPinsUpdate(json) ); }
				"GUILD_CREATE" => {return Some(Event::GuildCreate(json) ); }
				"GUILD_UPDATE" => {return Some(Event::GuildUpdate(json) ); }
				"GUILD_DELETE" => {return Some(Event::GuildDelete(json) ); }
				"GUILD_BAN_ADD" => {return Some(Event::GuildBanAdd(json) ); }
				"GUILD_BAN_REMOVE" => {return Some(Event::GuildBanRemove(json) ); }
				"GUILD_EMOJIS_UPDATE" => {return Some(Event::GuildEmojisUpdate(json) ); }
				"GUILD_INTEGRATIONS_UPDATE" => {return Some(Event::GuildIntegrationsUpdate(json) ); }
				"GUILD_MEMBER_ADD" => {return Some(Event::GuildMemberAdd(json) ); }
				"GUILD_MEMBER_REMOVE" => {return Some(Event::GuildMemberRemove(json) ); }
				"GUILD_MEMBER_UPDATE" => {return Some(Event::GuildMemberUpdate(json) ); }
				"GUILD_MEMBERS_CHUNK" => {return Some(Event::GuildMembersChunk(json) ); }
				"GUILD_ROLE_CREATE" => {return Some(Event::GuildRoleCreate(json) ); }
				"GUILD_ROLE_UPDATE" => {return Some(Event::GuildRoleUpdate(json) ); }
				"GUILD_ROLE_DELETE" => {return Some(Event::GuildRoleDelete(json) ); }
				"MESSAGE_CREATE" => {

					let id = json["id"].as_str().unwrap_or("0").parse::<u64>().unwrap();
					let channel_id = json["channel_id"].as_str().unwrap_or("0").parse::<u64>().unwrap();
					let user_id = json["author"]["id"].as_str().unwrap_or("0").parse::<u64>().unwrap();
					let username = json["author"]["username"].as_str().unwrap_or("").to_string();
					let discriminator = json["author"]["discriminator"].as_str().unwrap_or("").to_string();
					let avatar = match json["author"]["avatar"].as_str(){
						Some(x) => {
							format!("https://cdn.discordapp.com/avatars/{}/{}",&user_id,x)
						}
						None => { String::new()}
					};
					let content = json["content"].as_str().unwrap_or("").to_string();

					return Some(Event::MessageCreate(DMessage{
						id: id,
						channel_id,
						author: DUser{
							id: user_id,
							username: username,
							discriminator: discriminator,
							avatar: avatar
						},
						content: content,
					}) ); }
				"MESSAGE_UPDATE" => {return Some(Event::MessageUpdate(json) ); }
				"MESSAGE_DELETE" => {return Some(Event::MessageDelete(json) ); }
				"MESSAGE_DELETE_BULK" => {return Some(Event::MessageDeleteBulk(json) ); }
				"MESSAGE_REACTION_ADD" => {return Some(Event::MessageReactionAdd(json) ); }
				"MESSAGE_REACTION_REMOVE" => {return Some(Event::MessageReactionRemove(json) ); }
				"MESSAGE_REACTION_REMOVE_ALL" => {return Some(Event::MessageReactionRemoveAll(json) ); }
//				"PRESENCE_UPDATE" => {return Some(Event::PresenceUpdate(json) ); }
				"TYPING_START" => {return Some(Event::TypingStart(json) ); }
				"USER_UPDATE" => {return Some(Event::UserUpdate(json) ); }
//				"VOICE_STATE_UPDATE" => {return Some(Event::VoiceStateUpdate(json) ); }
//				"VOICE_SERVER_UPDATE" => {return Some(Event::VoiceServerUpdate(json) ); }
//				"WEBHOOKS_UPDATE" => {return Some(Event::WebhooksUpdate(json) ); }
				_ => {return None;}
			}
		}
		return None;
	}
}
