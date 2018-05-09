use reqwest;
use reqwest::{Response,Result};
use D;
use serde_json::{Value, Map};
use dstruct::DUser;
use dstruct::DCShell;
use dstruct::DServerBig;
use EmbedStruct;

pub const DAPI: &'static str = "https://discordapp.com/api/v6";
pub const UAGENT: &'static str = "DiscordBot (https://github.com/KILOgramM/wsbotsladko, 0.5.0)";

pub struct Discord{
}
impl Discord{
	pub fn token() -> &'static String{
		D.get_token()
	}
	pub fn get_event_reciever() -> DCShell{
		DCShell::from_dc(D.get_chanel())
	}
	pub fn send_embed(chanel_id: u64, embed: Value){

		if let Err(e)  = dpool(&format!("/channels/{}/messages", chanel_id),Some(embed)){
			println!("[Embed Send] Error\nError:\n{}",e);
		}

	}
	pub fn send_typing(chanel_id: u64){

		if let Err(e)  = dpool(&format!("/channels/{}/typing", chanel_id),None){
			println!("[Send Typing] Error:\n{}",e);
		}

	}
	pub fn set_member_roles(server_id: u64,user_id: u64, role_id: Vec<Value>){
		if let Err(e)  = dpatch(&format!("/guilds/{}/members/{}", server_id,user_id),json!({ "roles": role_id })){
			println!("[Set Member Roles] Error\nError:\n{}",e);
		}
	}

	pub fn get_servers() -> Option<Value>{
		match dget(&format!("/users/@me/guilds"),None){
			Err(e)  => {
				println!("[Get Servers] Error:\n{}",e);
				return None;
			}
			Ok(mut k) => {
				return k.json().unwrap();
			}
		}
	}
	pub fn get_chanel(chanel_id: u64) -> Option<Value>{

		match dget(&format!("/channels/{}", chanel_id),None){
			Err(e)  => {
				println!("[Get Channel] Error:\n{}",e);
				return None;
			}
			Ok(mut k) => {
				return k.json().unwrap();
			}
		}

	}
	pub fn get_server_channels(server_id: u64) -> Option<Value>{

		match dget(&format!("/guilds/{}/channels", server_id),None){
			Err(e)  => {
				println!("[Get Channels] Error:\n{}",e);
				return None;
			}
			Ok(mut k) => {
				return k.json().unwrap();
			}
		}

	}
	pub fn get_roles_list(server_id: u64) -> Option<Value>{

		match dget(&format!("/guilds/{}/roles", server_id),None){
			Err(e)  => {
				println!("[Get Roles] Error:\n{}",e);
				return None;
			}
			Ok(mut k) => {
				return k.json().unwrap();
			}
		}

	}

	pub fn get_user(user_id: u64) -> Option<DUser>{
		match dget(&format!("/users/{}", user_id),None){
			Err(e)  => {
				println!("[Get User] Error:\n{}",e);
				return None;
			}
			Ok(mut k) => {
				let json:Value = k.json().unwrap();
				let mut user = DUser::empty()
					.id(json["id"].as_str().unwrap().parse::<u64>().unwrap())
					.username(json["username"].as_str().unwrap())
					.discriminator(json["discriminator"].as_str().unwrap());
				if let Some(avatar) = json["avatar"].as_str(){
					user = user.avatar_raw(avatar);
				}
				return Some(user);
			}
		}
	}
	pub fn get_member(server_id: u64,user_id: u64) -> Option<Value>{

		match dget(&format!("/guilds/{}/members/{}", server_id,user_id),None){
			Err(e)  => {
				println!("[Get Member] Error:\n{}",e);
				return None;
			}
			Ok(mut k) => {
				return k.json().unwrap();
			}
		}

	}
	pub fn send_mes(chanel_id: u64,text:&str,nonce:&str,tts:bool){
		let mut json = Map::new();
		if !text.is_empty(){
			let t: &str = if text.len()>2000{
				&text[0..2000]
			}
				else{
					text
				};
			json.insert("content".into(), json!(&t));
		}
		if !nonce.is_empty(){
			json.insert("nonce".into(), json!(&nonce));
		}
		json.insert("tts".into(), json!(&tts));

		if let Err(e)  = dpool(&format!("/channels/{}/messages", chanel_id),Some(json!(&json))){
			println!("[Message Send] Error\nEmbed:\n{:?}\nError:\n{}",json,e);
		}
	}

}


fn dpatch(cmd: &str, json:Value) -> Result<Response>{
	let url = format!("{}{}",DAPI,cmd);
	let mut headers = reqwest::header::Headers::new();
	headers.set_raw("Authorization",format!("Bot {}", Discord::token()));
	headers.set_raw("User-Agent",UAGENT);
	reqwest::Client::new()
		.patch(&url)
		.headers(headers)
		.json(&json)
		.send()
}

fn dget(cmd: &str, json:Option<Value>) -> Result<Response>{
	let url = format!("{}{}",DAPI,cmd);
	let mut headers = reqwest::header::Headers::new();
	headers.set_raw("Authorization",format!("Bot {}", Discord::token()));
	headers.set_raw("User-Agent",UAGENT);
	let mut req = reqwest::Client::new()
		.get(&url);
	let mut req = req.headers(headers);
	if let Some(j) = json{
		req.json(&j);
	}

	req.send()
}

fn dpool(cmd: &str, json:Option<Value>) -> Result<Response>{
	let url = format!("{}{}",DAPI,cmd);
	let mut headers = reqwest::header::Headers::new();
	headers.set_raw("Authorization",format!("Bot {}", Discord::token()));
	headers.set_raw("User-Agent",UAGENT);
	let mut req = reqwest::Client::new()
		.post(&url);
	let mut req = req.headers(headers);
	if let Some(j) = json{
		req.json(&j);
	}

	req.send()
}