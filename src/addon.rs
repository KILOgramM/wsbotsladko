//lfg & embed
use extime::get_time;

#[macro_use]
use serde_json;
use serde_json::Value;

#[macro_use]
use serde_derive;

use std::sync::Mutex;
use crate::{Hero,
     POOL,
     REG_BTAG,
     load_by_id,
     embed_from_value,
     User,
     HeroInfoReq,
     BtagData,
//     load_btag_data,
     WSSERVER};

use std::sync::RwLock;
use mysql::from_row;
use std::fmt::Debug;
use regex::Regex;
use std::thread;
use mysql;
use crate::disapi::Discord;
use crate::EmbedStruct;
use crate::dstruct::DMessage;
use crate::dstruct::DUser;
use crate::OwData;
use crate::dishandler::send_value;

use serenity::model::id::ChannelId;
use serenity::cache::CacheRwLock;
use serenity::http::raw::Http;


lazy_static!{
    pub static ref DB: Global = Global::new();
}
//pub static DB: Global = DataB;

pub struct Global{
//    pub lfg: RwLock<Vec<LFG>>,
//    pub chat: RwLock<Vec<(u64,Chat)>>,
    pub embeds_s: RwLock<Vec<(String,Value)>>,
    pub users: RwLock<Vec<(User)>>,
    //pub admins_id: RwLock<Vec<(u64)>>,
    pub temp: RwLock<Vec<(u32, TempData)>>,
}
impl Global{
    fn new() -> Global{
        Global{
//            lfg:  RwLock::new(Vec::new()),
//            chat: RwLock::new(Vec::new()),
            embeds_s: RwLock::new(Vec::new()),
            users: RwLock::new(Vec::new()),
            temp: RwLock::new(Vec::new()),
        }
    }

/*
    pub fn ini_lfg(&self){

        let mut l: Vec<LFG> = Vec::new();
        let call = format!("SELECT * FROM lfg");
        match POOL.prepare(call.as_str()){
            Err(e) => {
                if let mysql::Error::MySqlError(my)=e{
                    if my.code == 1146{
                        let mut call = format!(r#"CREATE TABLE `lfg`
                        (`did` bigint(20) unsigned NOT NULL,
                        `data` json DEFAULT NULL,PRIMARY KEY (`did`)
                        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci"#);
                        let mut conn = POOL.get_conn().unwrap();
                        let _ = conn.query(call);
                        info!("DB>ini>LFG>try to create Table");
                        return;
                    }
                    info!("DB>ini>LFG>pool Error on call[{}]: {:?}", call, my);
                }
                    else { info!("DB>ini>LFG>pool Error on call[{}]", call);}
            }
            Ok(mut stmt) => {
                for row in stmt.execute(()).unwrap() {
                    let (did, mut string) = from_row::<(u64, String)>(row.unwrap());

                    let v: Value = serde_json::from_str(string.as_str()).unwrap();

                    match v.get("time") {
                        Some(_) => {
                            match serde_json::from_str(string.as_str()){
                                Ok(ls) => {
                                    l.push(ls);
                                }
                                Err(e) => {
                                    info!("DB>ini>LFG>serde Error on [{}]: {:?}", did, e);
                                }
                            }
                        }
                        None => {
                            let time:i64 = get_time().sec-172800;
                            let lfg = LFG{
                                did: v.get("did").unwrap().as_u64().unwrap(),
                                btag: v.get("btag").unwrap().as_str().unwrap().to_string(),
                                reg: v.get("reg").unwrap().as_str().unwrap().to_string(),
                                plat: v.get("plat").unwrap().as_str().unwrap().to_string(),
                                rating: v.get("rating").unwrap().as_u64().unwrap() as u16,
                                description: v.get("description").unwrap().as_str().unwrap().to_string(),
                                time,
                            };
                            let json = serde_json::to_string(&lfg).unwrap();
                            let mut call = format!("INSERT INTO lfg (");

                            call = format!("{} did", call);
                            call = format!("{}, data", call);

                            call = format!("{}) VALUES (", call);

                            call = format!("{} {}", call, lfg.did);
                            call = format!("{}, '{}'", call, json.clone());

                            call = format!("{}) ON DUPLICATE KEY UPDATE", call);
                            call = format!("{} data='{}'", call, json);
                            let mut conn = POOL.get_conn().unwrap();
                            if let Err(e) = conn.query(call){
                                info!("ini_lfg>lfg_edit Err: {}", e);
                            }
                            l.push(lfg);
                        }
                    }

                }
            }
        }

        loop{
            match self.lfg.try_write() {
                Ok(mut lfg) => {
                    *lfg = l;
                    break;
                }
                _ => {}
            }
        }
        info!("\'LFG\' list ini done");
    }
*/

    /*
    pub fn ini_chat(&self){

        let mut ch: Vec<(u64,Chat)> = Vec::new();
        let mut call = format!("SELECT * FROM chat");
        match POOL.prepare(call.as_str()){
            Err(e) => {
                if let mysql::Error::MySqlError(my)=e{
                    if my.code == 1146{
                        let mut call = format!(r#"CREATE TABLE `chat`
                        (`did` bigint(20) unsigned NOT NULL,
                        `data` json DEFAULT NULL,PRIMARY KEY (`did`)
                        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci"#);
                        let mut conn = POOL.get_conn().unwrap();
                        let _ = conn.query(call);
                        info!("DB>ini>Chat>try to create Table");
                        return;
                    }
                    info!("DB>ini>Chat>pool Error on call[{}]: {:?}", call, my);
                }
                else { info!("DB>ini>Chat>pool Error on call[{}]", call);}

            }
            Ok(mut stmt) => {
                for row in stmt.execute(()).unwrap() {
                    let (did, string) = from_row::<(u64, String)>(row.unwrap());
                    match serde_json::from_str(string.as_str()){
                        Ok(c) => {ch.push((did,c));}
                        Err(e) => {
                            info!("DB>ini>Chat>serde Error on [{}]: {:?}", did, e);
                        }
                    }
                }
            }
        }


        match self.chat.write() {
            Ok(mut chat) => {
                *chat = ch;

            }
            _ => {}
        }

        info!("\'Chat\' list ini done");
    }
    */

    pub fn ini_embeds_s(&self){
        use std::fs::File;
        use std::io::Read;
        lazy_static! {
            static ref REG_FILE: Regex = Regex::new(r"(?ms)^\*\[(?P<name>.+?)\]\*").expect("Regex file error");
        }


        let mut p1 = File::open("embed.ws");
        let mut p2 = File::open("target\\debug\\embed.ws");
        let mut p3 = File::open("src\\embed.ws");
        let mut path = if let Ok(p) = p1{Some(p)}
        else if let Ok(p) = p2 { Some(p)}
        else if let Ok(p) = p3 { Some(p)}
        else {None};

        match path {
            None => {
                info!("DB>ini>Embed>File>Open Error [embed.ws]");
            }
            Some(mut f) => {
                let mut raw = String::new();

                match f.read_to_string(&mut raw) {
                    Err(e) => {
                        info!("DB>ini>Embed>File>Read Error [embed.ws]: {:?}", e);
                    }
                    _ => {

                        let mut emb: Vec<(String,Value)> = Vec::new();
                        let mut value_name = None;
                        let mut value_txt = String::new();

                        for line in raw.lines(){
                            if REG_FILE.is_match(&line){
                                if let Some(name) = value_name{
                                    match serde_json::from_str(value_txt.as_str()){
                                        Ok(embed) => {
                                            emb.push((name,embed));
                                            value_txt = String::new();
                                            value_name = None;
                                        }
                                        Err(e) => {
                                            info!("DB>ini>Embed>serde Error on [{}]: {:?}", name, e);
                                            value_txt = String::new();
                                            value_name = None;
                                        }
                                    }
                                }
                                value_name = Some(REG_FILE.captures(&line).unwrap().get(1).unwrap().as_str().to_string());
                            } else {
                                if let Some(_) = value_name{
                                    value_txt.push_str(line);
                                }
                            }
                        }
                        if let Some(name) = value_name{
                            match serde_json::from_str(value_txt.as_str()){
                                Ok(embed) => {
                                    emb.push((name,embed));
                                }
                                Err(e) => {
                                    info!("DB>ini>Embed>serde Error on [{}]: {:?}", name, e);
                                }
                            }
                        }


                        match self.embeds_s.write() {
                            Ok(mut embeds) => {
                                *embeds = emb;
                            }
                            _ => {}
                        }

                        info!("\'Embed\' list ini done");
                    }
                }


            }
        }
    }

    /*
    pub fn get_chat(&self, id: u64) -> Option<Chat>{
        use std::ops::Deref;
        let mut chat: Option<Chat> = None;
        {
            loop{
                match self.chat.try_read(){
                    Ok(chats) => {
                        let chats = chats.deref();
                        for c in chats{
                            if c.0 == id{
                                chat = Some(c.clone().1);
                                break;
                            }
                        }
                        break;
                    }
                    _=>{}
                }
            }
        }
        return chat;
    }
    */

    pub fn get_embed(&self, name: &str) -> Option<Value>{
        use std::ops::Deref;
        let mut val = None;

        match self.embeds_s.read(){
            Ok(embeds) => {
                let embeds = embeds.deref();
                for embed in embeds{
                    if embed.0 == name.to_string(){
                        val = Some(embed.clone().1);
                        break;
                    }
                }
            }
            _=>{}
        }

        return val;
    }

/*
    pub fn get_lfg(&self, id: u64) -> Option<LFG>{
        use std::ops::Deref;
        let mut lfg_main: Option<LFG> = None;


        match self.lfg.read(){
            Ok(lfg) => {
                let lfg = lfg.deref();
                for l in lfg{
                    if l.did == id{
                        lfg_main = Some(l.clone());
                        break;
                    }
                }
            }
            _=>{}
        }


        return lfg_main;
    }



    pub fn get_lfg_list(&self) -> Vec<LFG>{
        use std::ops::Deref;
        let mut lfg_main: Vec<LFG> = Vec::new();
        {
            loop{
                match self.lfg.try_read(){
                    Ok(lfg) => {
                        let lfg = lfg.deref();
                        lfg_main = lfg.clone();
                        break;
                    }
                    _=>{}
                }
            }
        }
        return lfg_main;
    }
*/

    pub fn send_embed(&self, cache: impl AsRef<Http>, name: &str, chanel: ChannelId){
        match self.get_embed(name){
            None => {
                info!("Embed [{}] not found", name);
            }
            Some(embed) => {
                send_value(cache,embed,chanel);
            }
        }

    }

/*
    pub fn push_lfg(&self, one_more_lfg: LFG){
        use std::ops::DerefMut;
        loop{
            match self.lfg.try_write() {
                Ok(mut lfg) => {
                    let mut lfg = lfg.deref_mut();
                    let mut some_i = None;

                    for (i,l) in lfg.iter().enumerate(){
                        if l.did == one_more_lfg.did{
                            some_i = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = some_i{
                        lfg.remove(i);
                    }
                    lfg.push(one_more_lfg);
                    break;
                }
                _ => {}
            }
        }
    }
    pub fn remove_lfg(&self, id: u64){
        use std::ops::DerefMut;
        loop{
            match self.lfg.try_write() {
                Ok(mut lfg) => {
                    let mut lfg = lfg.deref_mut();
                    let mut some_i = None;

                    for (i,l) in lfg.iter().enumerate(){
                        if l.did == id{
                            some_i = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = some_i{
                        lfg.remove(i);
                    }
                    break;
                }
                _ => {}
            }
        }
    }
*/
/*
    pub fn new_temp(&self, data: TempData) -> u32{
        extern crate rand;
        use std::ops::DerefMut;
        let mut rnd:u32 = 0;
        loop{
            rnd = rand::random::<u32>();
            if let None = self.get_temp(rnd){
                break;
            }
        }

        match self.temp.write() {
            Ok(mut temp) => {
                let mut temp = temp.deref_mut();
                temp.push((rnd, data));

            }
            _ => {}
        }

        return rnd;
    }
    pub fn get_temp(&self, id: u32) -> Option<TempData>{
        use std::ops::Deref;
        let mut temp_return: Option<TempData> = None;
        match self.temp.read(){
            Ok(temp) => {
                let temp = temp.deref();
                for c in temp{
                    if c.0 == id{
                        temp_return = Some(c.1.clone());
                        break;
                    }
                }
            }
            _=>{}
        }
        return temp_return;
    }
    pub fn set_temp(&self, id: u32, data: TempData){
        use std::ops::DerefMut;

        match self.temp.write() {
            Ok(mut temp) => {
                let mut temp = temp.deref_mut();
                let mut some_i = None;

                for (i,l) in temp.iter().enumerate(){
                    if l.0 == id{
                        some_i = Some(i);
                        break;
                    }
                }
                if let Some(i) = some_i{
                    temp.remove(i);
                }
                temp.push((id,data));
            }
            _ => {}
        }

    }
    pub fn rem_temp(&self, id: u32){
        use std::ops::DerefMut;

        match self.temp.write() {
            Ok(mut temp) => {
                let mut temp = temp.deref_mut();
                let mut some_i = None;
                for (i,l) in temp.iter().enumerate(){
                    if l.0 == id{
                        some_i = Some(i);
                        break;
                    }
                }
                if let Some(i) = some_i{
                    temp.remove(i);
                }
            }
            _ => {}
        }

    }
*/
    /*
    pub fn set_chat(&self, id: u64, chat_new: Chat){
        use std::ops::DerefMut;

        match self.chat.write() {
            Ok(mut chat) => {
                let mut chat = chat.deref_mut();
                let mut some_i = None;

                for (i,l) in chat.iter().enumerate(){
                    if l.0 == id{
                        some_i = Some(i);
                        break;
                    }
                }
                if let Some(i) = some_i{
                    chat.remove(i);
                }
                chat.push((id, chat_new.clone()));
                let json = serde_json::to_string(&chat_new).unwrap();
                let mut call = format!("INSERT INTO chat (");

                call = format!("{} did", call);
                call = format!("{}, data", call);

                call = format!("{}) VALUES (", call);

                call = format!("{} {}", call, id);
                call = format!("{}, '{}'", call, json.clone());

                call = format!("{}) ON DUPLICATE KEY UPDATE", call);
                call = format!("{} data='{}'", call, json);
                let mut conn = POOL.get_conn().unwrap();
                if let Err(e) = conn.query(call.clone()){
                    info!("set_chat Error in call [{}]:\n{}", call, e);
                }
            }
            _ => {}
        }

    }
    pub fn rem_chat(&self, id: u64){
        use std::ops::DerefMut;
        match self.chat.write() {
            Ok(mut chat) => {
                let mut chat = chat.deref_mut();
                let mut some_i = None;

                for (i,l) in chat.iter().enumerate(){
                    if l.0 == id{
                        some_i = Some(i);
                        break;
                    }
                }
                if let Some(i) = some_i{
                    chat.remove(i);
                }

                let mut call = format!("DELETE FROM chat WHERE did={}",id);
                let mut conn = POOL.get_conn().unwrap();
                if let Err(e) = conn.query(call.clone()){
                    info!("rem_chat Error in call [{}]:\n{}",call, e);
                }

            }
            _ => {}
        }
    }
    */
}

#[derive(Clone, Debug)]
pub enum TempData{
    None,
    OwRating(u16),
    Bool(bool),
}
impl TempData{
    pub fn is_true(&self) -> bool{
        match self{
            &TempData::Bool(b) => {return b;}
            _ => {return false;}
        }
    }
}

//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub enum Chat{
//    LFG(Stage_LFG),
//}

/*
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct LFG{
    pub did: u64,
    btag: String,
    reg: String,
    plat: String,
    rating: u16,
    description: String,
    pub time: i64,
}
impl LFG{
    pub fn def_table(debug: bool, chanel: u64) -> Vec<(String, String,bool)>{
        let mut lfg_list: Vec<LFG> = DB.get_lfg_list();
        let mut fields:Vec<(String, String,bool)> = Vec::new();
        for i in 0..25{
            if let Some(lfg) = lfg_list.pop(){

                let (string, mut des) = match debug {
                    true => {lfg.to_line_debug(chanel)}
                    false => {lfg.to_line(chanel)}
                };
                let num = if i+1<10{
                    format!("0{}",i+1)
                }
                else { format!("{}",i+1) };
                des = format!("#{} | {}", num, des);
                fields.push((string,des,false));
            }
            else {
                break;
            }
        }
        return fields;
    }
    fn to_line(&self, chanel: u64) -> (String,String){

        let des = if self.description.is_empty(){
            String::new()
        }
        else {
            format!("\n```\n{}\n```",self.description)
        };
        let u:User = load_by_id(self.did).unwrap();
        return (format!("\u{FEFF}"),
                format!("{} | [{}](https://playoverwatch.com/en-us/career/{}/{}/{}) | {} SR {}",
                        match_merge(chanel, self.did), self.btag, self.plat.to_lowercase(), self.reg.to_lowercase(), self.btag.replace("#", "-"), self.rating, des));

    }
    pub fn to_line_debug(&self, chanel: u64) -> (String,String){

        let des = if self.description.is_empty(){
            String::new()
        }
            else {
                format!("\n```\n{}\n```",self.description)
            };
        let u:User = load_by_id(self.did).unwrap();
        return (format!("{}", self.did),
                format!("{} | [{}](https://playoverwatch.com/en-us/career/{}/{}/{}) | {} SR {}",
                        match_merge(chanel, self.did), self.btag, self.plat.to_lowercase(), self.reg.to_lowercase(), self.btag.replace("#", "-"), self.rating, des));

    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Stage_LFG{
    None,
    ConfirmUpdate(LFG),
    ConfirmRemove,
}

*/

#[derive(Clone, Debug)]
pub enum Role{
    DPS,
    Def,
    Tank,
    Sup,
    Flex,
}

/*
pub fn lfg(mes: DMessage, stage: Stage_LFG){
    let color: u64 = 37595;
    //let private_c = DIS.create_private_channel(mes.author.id).unwrap().id;
	Discord::send_typing(mes.channel_id);
    match stage{
        Stage_LFG::None => { lfg_none(mes)}
        Stage_LFG::ConfirmRemove => {

            let (mut mes_data, _) = split_mes(mes.content.clone());
            for m in mes_data{
                let mut s = m;
                match s.to_uppercase().as_str() {
                    "!WSLFG" => {continue;}
                    "N" | "NO" | "НЕТ" | "ОТМЕНА" => {
                        match DB.get_lfg(mes.author.id){
                            Some(lfg) => {
                                let title = "Объявление осталось на месте:";
                                let (tstring, dstring) = lfg.to_line(mes.channel_id);
	                            EmbedStruct::empty()
		                            .title(&title)
		                            .col(color)
		                            .fields(vec![(tstring,dstring,false)])
		                            .send(mes.channel_id);
                            }
                            None => {
                                DB.send_embed("lfg_not_found_WTF",mes.channel_id);
                            }
                        }
                        DB.rem_chat(mes.author.id);
                        return;
                    }
                    "YES" | "ДА" | "Y" => {
                        lfg_rem(mes.channel_id,mes.author.id);
                        DB.rem_chat(mes.author.id);
                        return;
                    }
                    _=>{}
                }
            }
        }
        Stage_LFG::ConfirmUpdate(lfg) => {
            let (mut mes_data, _) = split_mes(mes.content.clone());
            for m in mes_data{
                let mut s = m;
                match s.to_uppercase().as_str() {
                    "!WSLFG" => {continue;}
                    "N" | "NO" | "НЕТ" | "ОТМЕНА" => {
                        match DB.get_lfg(mes.author.id){
                            Some(lfg) => {
                                let title = "Объявление осталось преждним:";
                                let (tstring, dstring) = lfg.to_line(mes.channel_id);
	                            EmbedStruct::empty()
		                            .title(&title)
		                            .col(color)
		                            .fields(vec![(tstring,dstring,false)])
		                            .send(mes.channel_id);
                            }
                            None => {
                                DB.send_embed("lfg_not_found_WTF",mes.channel_id);
                            }
                        }
                        DB.rem_chat(mes.author.id);
                        return;
                    }
                    "YES" | "ДА" | "Y" => {
                        lfg_add(lfg.clone());

                        let title = "Ваше объявление обновлено:";
                        let (tstring, dstring) = lfg.to_line(mes.channel_id);
	                    EmbedStruct::empty()
		                    .title(&title)
		                    .col(color)
		                    .fields(vec![(tstring,dstring,false)])
		                    .send(mes.channel_id);
                        DB.rem_chat(mes.author.id);
                        return;
                    }
                    _=>{}
                }
            }
        }
    }

}

pub fn lfg_none(mes: DMessage){
    let mut user = User::empty();

    let err_color: u64 = 13369344;
    let err_title = ":no_entry: Упс...";
    let color: u64 = 37595;
    let yellow_color: u64 = 15651330;
    let title = "Ваше объявление будет вглядеть так:".to_string();

    //let private_c = DIS.create_private_channel(mes.author.id).unwrap().id;

    
    let (mut mes_data, des) = split_mes(mes.content.clone());


    let _ = mes_data.remove(0);

    if mes_data.is_empty() && des.is_empty(){
        let fields:Vec<(String,String,bool)> = LFG::def_table(false,mes.channel_id);
        let title = "Список игроков:";
        if fields.is_empty(){
            DB.send_embed("lfg_list_empty",mes.channel_id);
            return;
        }
            else {
	            EmbedStruct::empty()
		            .title(&title)
		            .col(color)
		            .fields(fields)
		            .send(mes.channel_id);
            }
    }
    else {

//        let was_private_c = match DIS.get_channel(mes.channel_id) {
//            Ok(Channel::Private(_)) => {true}
//            _ => {false}
//        };


        let mut btag = String::new();
        let mut reg = String::new();
        let mut plat = String::new();
        let mut rating: u16 = 0;
        let mut fields: Vec<(String, String, bool)> = Vec::new();

        for m in mes_data{
            let mut s = m.as_str();

            match s.to_uppercase().as_str() {
                "KR" | "EU" | "US" => {
                    reg = s.to_uppercase();
                }
                "PC" | "P4" | "XB" => {
                    plat = s.to_uppercase();
                }
                "DEL" | "REMOVE" | "REM" | "DELETE" => {

					lfg_rem(mes.channel_id,mes.author.id);
                    /*match DB.get_lfg(mes.author.id.0){
                        Some(lfg) => {
							
                            
							/*DB.set_chat(mes.author.id.0, Chat::LFG(Stage_LFG::ConfirmRemove));
                            let mut vec = Vec::new();

                            let title = "Текущее:".to_string();
                            vec.push((title,lfg.to_line(),false));


                            if let Err(e) = embed(mes.channel_id,"","Удалить объявление из базы?","(Y/N/Отмена)",String::new(),color,
                                                  (String::new(),""),vec,("","",""),String::new(),String::new()){
                                info!("Message Error: {:?}", e);
                            }*/
                            return;
                        }
                        None => {
                            DB.send_embed("lfg_del_notfound",mes.channel_id);
                            return;
                        }
                    }*/
                    return;
                }
                "HELP" => {
                    DB.send_embed("lfg_help",mes.channel_id);
                    return;
                }
                _ => {
                    if REG_BTAG.is_match(s) {
                        btag = s.to_string();
                    }
                }
            }
        }

        match load_by_id(mes.author.id) {
            None => {
                DB.send_embed("lfg_user_not_reg",mes.channel_id);
                return;}
            Some(u) => {
                user = u;
            }
        }

        if let Some(mut old_lfg) = DB.get_lfg(mes.author.id){

            if !btag.is_empty() || !reg.is_empty() || !plat.is_empty(){

                if btag.is_empty(){
                    btag = old_lfg.btag;
                }

                if reg.is_empty(){
                    reg = old_lfg.reg;
                }
                if plat.is_empty(){
                    plat = old_lfg.plat;
                }

                let mut req = HeroInfoReq::empty();
                req.rating = true;


                match load_btag_data(btag.clone(),reg.clone(),plat.clone(),req){
                    OwData::NotFound | OwData::ClosedProfile {..} => {
                        DB.send_embed("lfg_wrong_btag",mes.channel_id);
                        return;
                    },
                    OwData::Full(BData) => {
                        old_lfg.btag = btag;
                        old_lfg.rating = BData.rating;
                        old_lfg.reg = reg;
                        old_lfg.plat = plat;
                    }
                }
            }

            if !des.is_empty(){
                old_lfg.description = des;
            }

			lfg_add(old_lfg.clone());

			let title = "Ваше объявление обновлено:";
            let (tstring, dstring) = old_lfg.to_line(mes.channel_id);
	        EmbedStruct::empty()
		        .title(&title)
		        .col(color)
		        .fields(vec![(tstring,dstring,false)])
		        .send(mes.channel_id);

			/*
            DB.set_chat(mes.author.id.0, Chat::LFG(Stage_LFG::ConfirmUpdate(lfg.clone())));
            let mut vec = Vec::new();

            let title = "Текущее:".to_string();
            vec.push((title,old_lfg.to_line(),false));

            let title = "Новое:".to_string();
            vec.push((title,lfg.to_line(),false));

            if let Err(e) = embed(mes.channel_id,"","У вас уже размещено объявление","Хотете изменить? (Y/N/Отмена)",String::new(),color,
                                  (String::new(),""),vec,("","",""),String::new(),String::new()){
                info!("Message Error: {:?}", e);
            }*/
            return;

        }
            else {

                if btag.is_empty(){
                    if user.btag.is_empty() {
                        DB.send_embed("lfg_user_no_btag",mes.channel_id);
                        return;
                    }
                        else {
                            btag = user.btag;
                        }
                }

                match (reg.is_empty(), user.reg.is_empty()) {
                    (true, true) => {
                        reg = "EU".to_string();
                    }
                    (true, false) => {
                        reg = user.reg.clone();
                    }
                    _ => {}
                };

                match (plat.is_empty(), user.reg.is_empty()) {
                    (true, true) => {
                        plat = "PC".to_string();
                    }
                    (true, false) => {
                        plat = user.plat.clone();
                    }
                    _ =>{}

                };

                let mut req = HeroInfoReq::empty();
                req.rating = true;

                match load_btag_data(btag.clone(),reg.clone(),plat.clone(),req) {
                    OwData::NotFound | OwData::ClosedProfile { .. } => {
                        DB.send_embed("lfg_wrong_btag", mes.channel_id);
                        return;
                    },
                    OwData::Full(BData) => {
                        rating = BData.rating;
                    }
                }

                let lfg = LFG{
                    did: mes.author.id,
                    rating,
                    btag,
                    plat,
                    reg,
                    description: des,
                    time: get_time().sec,
                };

                lfg_add(lfg.clone());

                let title = "Ваше объявление добавлено в базу:";
                let (tstring, dstring) = lfg.to_line(mes.channel_id);
	            EmbedStruct::empty()
		            .title(&title)
		            .col(color)
		            .fields(vec![(tstring,dstring,false)])
		            .send(mes.channel_id);

                return;
            }

    }

}

fn lfg_rem(chanel: u64,id: u64){
    let color: u64 = 37595;
    match DB.get_lfg(id){
        Some(lfg) => {
            let title = "Ваше объявление удалено:";
            let (tstring, dstring) = lfg.to_line(chanel);
	        EmbedStruct::empty()
		        .title(&title)
		        .col(color)
		        .fields(vec![(tstring,dstring,false)])
		        .send(chanel);
            let mut call = format!("DELETE FROM lfg WHERE did={}",id);
            let mut conn = POOL.get_conn().unwrap();
            if let Err(e) = conn.query(call){
                info!("lfg_rem Err: {}", e);
            }
            DB.remove_lfg(id);
        }
        None => {
            DB.send_embed("lfg_del_notfound",chanel);
            return;
        }
    }

}
*/

fn split_mes(mes: String) ->(Vec<String>, String){
    let mut des = String::new();
    let mut con = String::new();
    let mut mes_data: Vec<String> = Vec::new();
    let mut is_des = false;
    for c in mes.chars(){
        if c.eq(&'\"'){
            if is_des{
                is_des = false;
                continue;
            }
                else {
                    is_des = true;
                    continue;
                }
        }
        if is_des{
            des.push(c);
            continue;
        }
            else {
                if c.eq(&' '){
                    if !con.is_empty(){mes_data.push(con);}
                    con = String::new();
                    continue;
                }
                    else {
                        con.push(c);
                        continue;
                    }
            }
    }
    if !con.is_empty(){mes_data.push(con);}
    return (mes_data, des);
}

/*
fn lfg_add(lfg: LFG){
    let json = serde_json::to_string(&lfg).unwrap();
    let mut call = format!("INSERT INTO lfg (");

    call = format!("{} did", call);
    call = format!("{}, data", call);

    call = format!("{}) VALUES (", call);

    call = format!("{} {}", call, lfg.did);
    call = format!("{}, '{}'", call, json.clone());

    call = format!("{}) ON DUPLICATE KEY UPDATE", call);
    call = format!("{} data='{}'", call, json);
    let mut conn = POOL.get_conn().unwrap();
    if let Err(e) = conn.query(call){
        info!("lfg_add Err: {}", e);
    }

    DB.push_lfg(lfg);
}
*/

pub fn event_add(mut data: String){
//    let mut data = mes.content.clone();
    use crate::EventReq;
    use crate::EVENT;
    use crate::EventChanel;
    use crate::EventType;
    if data.len() > 11{
        let mut event_name = String::new();
        let mut server: Option<String> = None;
        let mut server_id: Option<u64> = None;
        let mut room: String = String::new();
        let mut chanel: Option<u64> = None;
        let mut embed: String = String::new();
        let mut req_list: Vec<EventReq> = Vec::new();
        let mut req = EventReq::empty();

        let mut data = data.split_off(11);
        let mut buf: Vec<(String,String)> = Vec::new();

        let mut push = false;
        let mut switch = false;
        let mut space_check = false;
        let mut no_chars = false;
        let mut long_param = false;

        let mut option = String::new();
        let mut option_rez = String::new();
        let mut settings = String::new();
        let mut num_elements = data.len() - 1;

        for (i, c) in data.as_str().chars().enumerate(){

            match c{
                '=' | ':' => {
                    if switch {
                        settings.push(c);
                    }
                    else if !long_param{
                        switch = true;
                        no_chars = true;
                        space_check = false;
                    }
                    else{
                        if switch {
                            settings.push(c);
                        }
                        else{
                            option.push(c);
                        }
                    }
                }

                ' ' => {
                    if !no_chars && !long_param{
                        if switch {push = true;
                        }
                        else {
                            space_check = true;
                        }
                    }
                    if long_param{
                        if switch {
                            settings.push(c);
                        }
                        else{
                            option.push(c);
                        }
                    }
                }

                '"' => {
                    if long_param {long_param = false;}
                    else { long_param = true;}
                }

                ',' | '|' => {
                    if !long_param{
                        push = true;
                    }
                    else{
                        if switch {
                            settings.push(c);
                        }
                        else{
                            option.push(c);
                        }
                    }
                }

                '\n' | '\r' => {
                    if long_param{
                        long_param = false;
                    }
                    push = true;
                }

                x => {
                    if space_check{
                        option_rez.push(x);
                        push = true;
                    }
                    else {
                        if switch {
                            settings.push(x);
                        }
                        else{
                            option.push(x);
                        }
                    }
                    no_chars = false;

                }
            }
            if push{
                push = false;
                switch = false;
                no_chars = true;
                space_check = false;
                if option.is_empty() &&
                    option_rez.is_empty() &&
                    settings.is_empty(){
                    continue;
                }
                buf.push((option, settings));
                option = option_rez;
                option_rez = String::new();
                settings = String::new();
            }
        }
        if !option.is_empty(){
            buf.push((option, settings));
        }


        for (opt_namer, opt_par) in buf{
            if opt_namer.is_empty(){
                continue;
            }
            match opt_namer.as_str(){
                "once" => {
                    match opt_par.as_str(){
                        "false" => { req.once = false;}
                        _ => { req.once = true;}
                    }
                }
                "name" => {
                    event_name = opt_par;
                }

                "embed" => {
                    embed = opt_par;
                }

                "room" => {
                    info!("room - {}",&opt_par);
                    room = opt_par;
                }

                "server" => {
                    match opt_par.parse::<u64>(){
                        Ok(x) => { server_id = Some(x);}
                        Err(_) => { server = Some(opt_par);}
                    }
                }

                "year" | "y" => {
                    if let Ok(n) = opt_par.parse::<u16>(){
                        req.year = Some(n);
                    }
                }

                "month" | "mon" => {
                    match opt_par.as_str(){
                        "янв" | "январь"  => { req.month = Some(0);}
                        "фев" | "ферваль"  => { req.month = Some(1);}
                        "мар" | "март"  => { req.month = Some(2);}
                        "апр" | "апрель"  => { req.month = Some(3);}
                        "май"  => { req.month = Some(4);}
                        "июн" | "июнь"  => { req.month = Some(5);}
                        "июл" | "июль"  => { req.month = Some(6);}
                        "авг" | "август"  => { req.month = Some(7);}
                        "сен" | "сентябрь"  => { req.month = Some(8);}
                        "окт" | "октябрь"  => { req.month = Some(9);}
                        "ноя" | "ноябрь"  => { req.month = Some(10);}
                        "дек" | "декабрь"  => { req.month = Some(11);}
                        x => { if let Ok(n) = x.parse::<u8>(){
                            if n < 13{
                                req.month = Some(n-1);
                            }
                        }
                        }
                    }
                }

                "day_of_mouth" | "mday" => {
                    if let Ok(n) = opt_par.parse::<u8>(){
                        if n < 32{
                            req.day_of_mouth = Some(n);
                        }
                    }
                }

                "day_of_week" | "wday" => {
                    match opt_par.as_str(){
                        "пн" | "понедельник"  => { req.day_of_week = Some(1);}
                        "вт" | "вторник"  => { req.day_of_week = Some(2);}
                        "ср" | "среда"  => { req.day_of_week = Some(3);}
                        "чт" | "четверг"  => { req.day_of_week = Some(4);}
                        "пт" | "пятница"  => { req.day_of_week = Some(5);}
                        "сб" | "суббота"  => { req.day_of_week = Some(6);}
                        "вс" | "воскресенье"  => { req.day_of_week = Some(7);}
                        x => { if let Ok(n) = x.parse::<u8>(){
                            if n < 8{
                                req.day_of_week = Some(n);
                            }
                        }
                        }
                    }
                }

                "hour" | "h" | "hours" => {
                    if let Ok(n) = opt_par.parse::<u8>(){
                        req.hour = Some(n);
                    }
                }
                "min" | "minute" | "minutes"=> {
                    if let Ok(n) = opt_par.parse::<u8>(){
                        req.min = Some(n);
                    }
                }
                "sec" | "s" | "second" | "seconds"=> {
                    if let Ok(n) = opt_par.parse::<u8>(){
                        req.sec = Some(n);
                    }
                }

                "[trig]" =>{
                    if !req.eq_alt(&EventReq::empty()){
                        req_list.push(req);
                        req = EventReq::empty();
                    }
                }
                "time" => {
                    let mut hour = String::new();
                    let mut min = String::new();
                    let mut sec = String::new();
                    for c in opt_par.as_str().chars(){
                        match c {
                            ':' => {
                                hour = min;
                                min = sec;
                                sec = String::new();}
                            '0'|'1'|'2'|'3'|'4'
                            |'5'|'6'|'7'|'8'|'9' =>{sec.push(c);}
                            _ => {

                            }
                        }
                    }
                    if let Ok(n) = hour.parse::<u8>(){
                        req.hour = Some(n);
                    }
                    if let Ok(n) = min.parse::<u8>(){
                        req.min = Some(n);
                    }
                    if let Ok(n) = sec.parse::<u8>(){
                        req.sec = Some(n);
                    }
                }
                _ => {}
            }
        }
        if !req.eq_alt(&EventReq::empty()){
            req_list.push(req);
        }

        let event_type = EventType::CustomEmbed {
            server,
            server_id,
            room,
            chanel,
            embed
        };

        EVENT.send(EventChanel::AddEvent {
            name: event_name,
            event_type,
            req: req_list,
        });

    }

}

/*
fn match_merge(chanel: u64, did: u64) -> String{

//    let user = match DIS.create_private_channel(discord::model::UserId(did)){
//        Ok(x) => {
//            x.recipient
//        }
//        _ => {
//            let servers = match DIS.get_servers() {
//                    Ok(x) => {x}
//                    _ => {return format!("<@{}>",discord::model::UserId(did));}
//                };
//            let mut u = None;
//            for server in servers{
//                match DIS.get_member(server.id,discord::model::UserId(did)){
//                        Ok(x) =>{u = Some(x.user); break;
//                        }
//                        _ => {continue;
//                        }
//                }
//            }
//            if let Some(x) = u{
//                x
//            }
//            else{
//                return format!("<@{}>",did);
//            }
//
//        }
//    };
    if let Some(user) = Discord::get_user(did){
        if let Some(value) = Discord::get_chanel(chanel){
            let ty = json!(0);
            if ty.eq(&value["type"]){
                if let Some(_) = Discord::get_member(value["guild_id"].as_str().unwrap().parse::<u64>().unwrap(),did){
                    return format!("<@{}>",did);
                }
            }

        }
        return format!("{}#{}", user.username, user.discriminator);
    }
    else {
        return format!("<@{}>",did);
    }


/*
if let Ok(chan) = DIS.get_channel(chanel){
        match chan{
            Channel::Group(chan_group) => {
                return format!("{}#{}", user.name, user.discriminator);

//                let servers = match DIS.get_servers() {
//                    Ok(x) => {x}
//                    _ => {return format!("{}#{}", user.name, user.discriminator);}
//                };
//                let mut users_id = Vec::new();
//                for u in chan_group.recipients{
//                    users_id.push(u.id);
//                }
//                users_id.push(user.id);
//                let server = ServerId(WSSERVER);
//                let mut br = false;
//                for id in users_id{
//                    match DIS.get_member(server,id){
//                        Ok(_) =>{continue;
//                        }
//                        _ => {br = true; break;
//                        }
//                    }
//                }
//                if br{
//                    return format!("{}#{}", user.name, user.discriminator);
//                }
//                    else {
//                        return format!("<@{}>",did);
//                    }
                //users_id.push(chan_group.owner_id);
//                let mut br = false;
//                'outer: for server in servers{
//                    br = false;
//                    'inner: for id in users_id.clone(){
//                        match DIS.get_member(server.id,id){
//                            Ok(_) =>{continue;
//                            }
//                            _ => {br = true; break 'inner;
//                            }
//                        }
//                    }
//                    if !br{
//                        return format!("<@{}>",did);
//                    }
//                }
//                return format!("{}#{}", user.name, user.discriminator);
            }
            Channel::Private(chan_private) => {
                return format!("{}#{}", user.name, user.discriminator);

//                let servers = match DIS.get_servers() {
//                    Ok(x) => {x}
//                    _ => {return format!("{}#{}", user.name, user.discriminator);}
//                };
//                let mut users_id = Vec::new();
//                users_id.push(user.id);
//                users_id.push(chan_private.recipient.id);
//
//                let server = ServerId(WSSERVER);
//                let mut br = false;
//                for id in users_id{
//                    match DIS.get_member(server,id){
//                        Ok(_) =>{continue;
//                        }
//                        _ => {br = true; break;
//                        }
//                    }
//                }
//                if br{
//                    return format!("{}#{}", user.name, user.discriminator);
//                }
//                else {
//                    return format!("<@{}>",did);
//                }


//                let mut br = false;
//                'outer: for server in servers{
//                    br = false;
//                    'inner: for id in users_id.clone(){
//                        match DIS.get_member(server.id,id){
//                            Ok(_) =>{continue;
//                            }
//                            _ => {br = true; break 'inner;
//                            }
//                        }
//                    }
//                    if !br{
//                        return format!("<@{}>",did);
//                    }
//                }

            }
            Channel::Public(chan_public) => {
                match DIS.get_member(chan_public.server_id,user.id){
                    Ok(_) =>{return format!("<@{}>",did);
                    }
                    _ => {return format!("{}#{}", user.name, user.discriminator);
                    }
                }
            }
        }
    }
    else {
        return format!("{}#{}", user.name, user.discriminator);
    }*/
}
*/