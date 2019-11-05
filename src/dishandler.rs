use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::cache::CacheRwLock;
use serenity::http::raw::Http;
use serde_json::Value;

pub struct DisHandler;

use crate::reg_check;
use crate::reg_user;
use crate::edit_user;
use crate::{wsstats,
            add_to_db,
            delete_user};
use crate::EmbedStruct;
use crate::event::rating_updater;
use crate::event::{EventChanel,EventChanelBack};
use crate::addon::event_add;
use crate::{User,
            DEBUG,
            SWITCH_NET,
			EVENT};
use crate::START_TIME;
use crate::conf::Config;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

use crate::addon::DB;
//use websocket::futures::future::err;


impl EventHandler for DisHandler {
	fn message(&self, _ctx: Context, _new_message: Message){
		use std::thread;
		thread::spawn(move || {


			if _new_message.content.starts_with('!') {
				let content = _new_message.content.clone();
				let channel =_new_message.channel_id.clone();
				let mes_split: Vec<&str> = content.as_str().split_whitespace().collect();
				match mes_split[0].to_lowercase().as_str() {
					"!wsreg" => {
						if let Err(e) = channel.broadcast_typing(&_ctx){
							error!("Trying broadcat typing: {}",e);
						}

						match reg_check(_new_message.author.id.0) {
							false => {
								reg_user(mes_split.clone(), _new_message.author.clone().into(), channel, &_ctx);
							}
							true => { edit_user(mes_split.clone(), _new_message.author.clone().into(), channel, &_ctx); }
						}
					}

					"!wsstats" => {
						info!("wsstats");
						if let Err(e) = channel.broadcast_typing(&_ctx){
							error!("Trying broadcat typing: {}",e);
						}
						wsstats(mes_split.clone(), _new_message.author.id.0, channel,&_ctx);
					}

					"!wstour" => {
						info!("wstour");
						DB.send_embed(&_ctx,"tourneys",channel);
					}

					"!wshelp" => {
						info!("wshelp");
						DB.send_embed(&_ctx,"help",channel);
					}
					"!wscmd" => {
						info!("wscmd");
						DB.send_embed(&_ctx,"cmd",channel);
					}
					/*
					"!wslfg" => {
						info!("wslfg");
						lfg_none(mes.clone());
					}
					*/
					_ => {}
				}


				//ADMIN COMMANDS

				if _new_message.author.id.0 == 193759349531869184 || _new_message.author.id.0 == 222781446971064320{
					match mes_split[0].to_lowercase().as_str() {
						"!rup" => {
							rating_updater(&_ctx);
							channel.say(&_ctx,"Рейтинг обновлён");
						}

						"!ahelp" => {
							DB.send_embed(&_ctx,"admin_commands",channel);
						}

						"!event" =>{

							match mes_split.get(1){
								Some(&"add") =>{
									//11

									event_add(_new_message.content.clone());
								}
								Some(&"retime") =>{
									match mes_split.get(2){
										Some(name) =>{
											EVENT.send(EventChanel::RecalcEventTime(name.to_string()));
										}
										_ =>{

										}
									}
								}
								Some(&"rechan") =>{
									match mes_split.get(2){
										Some(name) =>{
											EVENT.send(EventChanel::RecalcEventChanel(name.to_string()));
										}
										_ =>{

										}
									}
								}
								Some(&"rem") =>{
									match mes_split.get(2){
										Some(name) =>{
											EVENT.send(EventChanel::RemEvent(name.to_string()));
											let mut embed = EmbedStruct::empty();
											let field_name = format!("Удаление эвента");
											let mut field_text = format!("Эвент `{}` удалён", name);
											embed.fields.push((field_name, field_text, false));
											embed.send(&_ctx,channel);
										}
										_ =>{
											let mut embed = EmbedStruct::empty();
											let field_name = format!("Удаление эвента");
											let mut field_text = format!("Имя не указано");
											embed.fields.push((field_name, field_text, false));
											embed.send(&_ctx,channel);
										}
									}
								}
								_ =>{


								}
							}

							EVENT.send(EventChanel::GetList);
							match EVENT.recive(){
								EventChanelBack::Error =>{
									let mut embed = EmbedStruct::empty();
									let field_name = format!("\u{FEFF}");
									let mut field_text = format!("Unexpected Reciver Error");
									embed.fields.push((field_name, field_text, false));
									embed.send(&_ctx,channel);
								}
								EventChanelBack::List(list) =>{
									let mut embed = EmbedStruct::empty();
									let field_name = format!("Event List");
									let mut field_text = format!("```\n");
									let mut max_len = 0;
									for (name, _) in list.clone(){
										if name.len() > max_len{
											max_len = name.len();
										}
									}
									for (name, tmalt) in list{
										field_text = format!("{}{}",field_text,name);
										for _ in 0..(max_len - name.len()){
											field_text.push(' ');
										}
										//info!("{:?}",tmalt.to_tm());
										field_text = format!("{}: {}\n",field_text,tmalt.to_tm().ctime());
									}
									field_text = format!("{}```\n",field_text);
									embed.fields.push((field_name, field_text, false));
									embed.send(&_ctx,channel);
								}
							}
						}

						"!test" => {
							let mut test_user: User = User::empty();
							test_user.did = _new_message.author.id.0;
							test_user.name = _new_message.author.name;
							test_user.disc = format!("{}",_new_message.author.discriminator);
							add_to_db(test_user);
						}
						"!test2" => {
							if let Some(id_str) = mes_split.get(1){
								if let Ok(id) = id_str.parse::<u64>(){

									delete_user(id);
									channel.say(&_ctx, format!("{} удалён",id));
								}
								channel.say(&_ctx, format!("Неизвестный параметр:`{}`",id_str));

							}
							else{
								delete_user(_new_message.author.id.0);
								channel.say(&_ctx, format!("{} удалён",_new_message.author.id.0));

							}

						}
						"!ini" =>{
							if mes_split.len() > 1{
								match mes_split[1].to_lowercase().as_str(){
									"embed" => {
										DB.ini_embeds_s();
										channel.say(&_ctx, "Embed-ы инициализированы");
									}
//                                            "lfg" => {
//                                                DB.ini_lfg();
//                                                Discord::send_mes(mes.channel_id, "Вектор LFG инициализирован", "", false);
//                                            }
//                                            "chat" => {
//                                                DB.ini_chat();
//                                                Discord::send_mes(mes.channel_id, "Вектор Chat инициализирован", "", false);
//                                            }
									"config" => {
										Config::init();
										channel.say(&_ctx, "Config инициализирован");
									}
									_ => {
										channel.say(&_ctx, "Опция не определена");
									}
								}

							}
							else {
								channel.say(&_ctx, "Перезагрузить embed, lfg или chat");
							}

						}
						/*"!serverlist" => {
							let string = format!("==Начало списка==");
							Discord::send_mes(mes.channel_id, string.as_str(), "", false);
							if let Some(value) = Discord::get_servers(){

							}

							for s in state_clone.servers(){
								let thum = match s.icon_url(){
									None => { String::new()}
									Some(s) => {s}
								};
								let title = &s.name;
								let mut des = format!("Id: {:?}\n",s.id.0);
								des = format!("{}Owner: <@{}>\n",des,s.owner_id);
								des = format!("{}Region: {}\n",des,s.region);
								des = format!("{}Members Count: {}\n",des,s.member_count);
								des = format!("{}Joined At: {}",des,s.joined_at);
								EmbedStruct::empty()
									.title(&title)
									.des(&des)
									.thumbnail(&thum)
									.send(mes.channel_id);
							}
							let string = format!("==Конец списка==");
							Discord::send_mes(mes.channel_id, string.as_str(), "", false);

						}*/
						"!debug" => {
							if mes_split.len() > 1{
								match mes_split[1].to_lowercase().as_str(){
									"on" => {
										DEBUG.store(true, Ordering::Relaxed);
										channel.say(&_ctx, "Debug Включен");
									}
									"off" => {
										DEBUG.store(false, Ordering::Relaxed);
										channel.say(&_ctx, "Debug Выключен");
									}
									_ => {
										let string = format!("Debug статус: {}", DEBUG.load(Ordering::Relaxed));
										channel.say(&_ctx, string.as_str());
									}
								}
							}
							else {
								let string = format!("Debug статус: {}", DEBUG.load(Ordering::Relaxed));
								channel.say(&_ctx, string.as_str());
							}
						}
						"!new_net" => {
							if mes_split.len() > 1{
								match mes_split[1].to_lowercase().as_str(){
									"on" => {
										SWITCH_NET.store(true, Ordering::Relaxed);
										channel.say(&_ctx, "new_net Включен");
									}
									"off" => {
										SWITCH_NET.store(false, Ordering::Relaxed);
										channel.say(&_ctx, "new_net Выключен");
									}
									_ => {
										let string = format!("new_net статус: {}", SWITCH_NET.load(Ordering::Relaxed));
										channel.say(&_ctx, string.as_str());
									}
								}
							}
							else {
								let string = format!("new_net статус: {}", SWITCH_NET.load(Ordering::Relaxed));
								channel.say(&_ctx,string.as_str());
							}
						}


						"!shver" => {
							use std::ops::Add;
							use std::ops::Sub;
							let start_clone:extime::Tm = START_TIME.clone();

							let cur_time = extime::now();
							let start_day = match START_TIME.tm_mday{
								0..=9 =>{ format!("0{}",START_TIME.tm_mday)}
								_ => {format!("{}",START_TIME.tm_mday)}
							};
							let start_mon = match START_TIME.tm_mon+1{
								0..=9 =>{ format!("0{}",START_TIME.tm_mon+1)}
								_ => {format!("{}",START_TIME.tm_mon+1)}
							};
							let start_h = match START_TIME.tm_hour{
								0..=9 =>{ format!("0{}",START_TIME.tm_hour)}
								_ => {format!("{}",START_TIME.tm_hour)}
							};
							let start_m = match START_TIME.tm_min{
								0..=9 =>{ format!("0{}",START_TIME.tm_min)}
								_ => {format!("{}",START_TIME.tm_min)}
							};
							let start_s = match START_TIME.tm_sec{
								0..=9 =>{ format!("0{}",START_TIME.tm_sec)}
								_ => {format!("{}",START_TIME.tm_sec)}
							};

							let dur_time = cur_time - start_clone;
							let mut dif_time = dur_time.num_seconds();



							let up_d = dif_time / 86400;
							dif_time = dif_time - (up_d * 86400);


							let up_h = dif_time / 3600;
							dif_time = dif_time - (up_h * 3600);
							let up_hour = match up_h{
								0..=9 =>{ format!("0{}",up_h)}
								_ => {format!("{}",up_h)}
							};

							let up_m = dif_time / 60;
							dif_time = dif_time - (up_m * 60);
							let up_min = match up_m{
								0..=9 =>{ format!("0{}",up_m)}
								_ => {format!("{}",up_m)}
							};

							let up_sec = match dif_time{
								0..=9 =>{ format!("0{}",dif_time)}
								_ => {format!("{}",dif_time)}
							};


							let title = "Bot Info";
							let ver = env!("CARGO_PKG_VERSION");
							let mut des = format!("Ver: {}\n",ver);
							des = format!("{}Start time: {}:{}:{} {}.{}.{}\n",des,
							              start_h,start_m,start_s,
							              start_day,start_mon,START_TIME.tm_year+1900,);
							des = format!("{}Up time: {}:{}:{}:{}\n",des,
							              up_d, up_hour, up_min, up_sec);
							EmbedStruct::empty()
								.title(&title)
								.des(&des)
								.send(&_ctx,channel);

						}

						_=>{}
					}
				}

			}
			else {

			}
		});
	}
}

pub fn send_value(cache: impl AsRef<Http>, embed: Value, chanel: ChannelId){
	if let Err(e) = cache.as_ref().send_message(chanel.0, &embed){
		error!("Trying send embed: {}",e);
	}
}

