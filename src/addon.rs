
extern crate time;

#[macro_use]
use serde_json;
use serde_json::Value;

#[macro_use]
use serde_derive;

use std::sync::Mutex;
use {Hero,
     embed,
     DIS,
     POOL,
     REG_BTAG,
     load_by_id,
     embed_from_value,
     User,
     HeroInfoReq,
     BtagData,
     load_btag_data};
use discord::model::Message;
use discord::model::Channel;
use discord::model::ChannelId;
use discord;
use std::sync::RwLock;
use mysql::from_row;
use std::fmt::Debug;
use regex::Regex;
use std::thread;
use mysql;

lazy_static!{
    pub static ref DB: Global = Global::new();
}


pub struct Global{
    pub lfg: RwLock<Vec<LFG>>,
    pub chat: RwLock<Vec<(u64,Chat)>>,
    pub embeds_s: RwLock<Vec<(String,Value)>>,
    pub users: RwLock<Vec<(User)>>,
    //pub admins_id: RwLock<Vec<(u64)>>,
    pub temp: RwLock<Vec<(u32, TempData)>>,
}
impl Global{
    fn new() -> Global{
        Global{
            lfg: RwLock::new(Vec::new()),
            chat: RwLock::new(Vec::new()),
            embeds_s: RwLock::new(Vec::new()),
            users: RwLock::new(Vec::new()),
            temp: RwLock::new(Vec::new()),
        }
    }

    pub fn ini_lfg(&self){

        let mut l: Vec<LFG> = Vec::new();
        let mut call = format!("SELECT * FROM lfg");
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
                        println!("DB>ini>LFG>try to create Table");
                        return;
                    }
                    println!("DB>ini>LFG>pool Error on call[{}]: {:?}", call, my);
                }
                    else { println!("DB>ini>LFG>pool Error on call[{}]", call);}
            }
            Ok(mut stmt) => {
                for row in stmt.execute(()).unwrap() {
                    let (did, string) = from_row::<(u64, String)>(row.unwrap());
                    match serde_json::from_str(string.as_str()){
                        Ok(ls) => {l.push(ls);}
                        Err(e) => {
                            println!("DB>ini>LFG>serde Error on [{}]: {:?}", did, e);
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
        println!("\'LFG\' list ini done");
    }
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
                        println!("DB>ini>Chat>try to create Table");
                        return;
                    }
                    println!("DB>ini>Chat>pool Error on call[{}]: {:?}", call, my);
                }
                else { println!("DB>ini>Chat>pool Error on call[{}]", call);}

            }
            Ok(mut stmt) => {
                for row in stmt.execute(()).unwrap() {
                    let (did, string) = from_row::<(u64, String)>(row.unwrap());
                    match serde_json::from_str(string.as_str()){
                        Ok(c) => {ch.push((did,c));}
                        Err(e) => {
                            println!("DB>ini>Chat>serde Error on [{}]: {:?}", did, e);
                        }
                    }
                }
            }
        }

        loop{
            match self.chat.try_write() {
                Ok(mut chat) => {
                    *chat = ch;
                    break;
                }
                _ => {}
            }
        }
        println!("\'Chat\' list ini done");
    }
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
                println!("DB>ini>Embed>File>Open Error [embed.ws]");
            }
            Some(mut f) => {
                let mut raw = String::new();

                match f.read_to_string(&mut raw) {
                    Err(e) => {
                        println!("DB>ini>Embed>File>Read Error [embed.ws]: {:?}", e);
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
                                            println!("DB>ini>Embed>serde Error on [{}]: {:?}", name, e);
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
                                    println!("DB>ini>Embed>serde Error on [{}]: {:?}", name, e);
                                }
                            }
                        }

                        loop{
                            match self.embeds_s.write() {
                                Ok(mut embeds) => {
                                    *embeds = emb;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        println!("\'Embed\' list ini done");
                    }
                }


            }
        }
    }

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
    pub fn get_embed(&self, name: &str) -> Option<Value>{
        use std::ops::Deref;
        let mut val = None;
        {
            loop{
                match self.embeds_s.try_read(){
                    Ok(embeds) => {
                        let embeds = embeds.deref();
                        for embed in embeds{
                            if embed.0 == name.to_string(){
                                val = Some(embed.clone().1);
                                break;
                            }
                        }
                        break;
                    }
                    _=>{}
                }
            }
        }
        return val;
    }
    pub fn get_lfg(&self, id: u64) -> Option<LFG>{
        use std::ops::Deref;
        let mut lfg_main: Option<LFG> = None;
        {
            loop{
                match self.lfg.try_read(){
                    Ok(lfg) => {
                        let lfg = lfg.deref();
                        for l in lfg{
                            if l.did == id{
                                lfg_main = Some(l.clone());
                                break;
                            }
                        }
                        break;
                    }
                    _=>{}
                }
            }
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
    pub fn send_embed(&self, name: &str, chanel: ChannelId){
        match self.get_embed(name){
            None => {
                println!("Embed [{}] not found", name);
            }
            Some(embed) => {
                embed_from_value(chanel,embed);

            }
        }

    }
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
        loop{
            match self.temp.try_write() {
                Ok(mut temp) => {
                    let mut temp = temp.deref_mut();
                    temp.push((rnd, data));
                    break;
                }
                _ => {}
            }
        }
        return rnd;
    }
    pub fn get_temp(&self, id: u32) -> Option<TempData>{
        use std::ops::Deref;
        let mut temp_return: Option<TempData> = None;
        {
            loop{
                match self.temp.try_read(){
                    Ok(temp) => {
                        let temp = temp.deref();
                        for c in temp{
                            if c.0 == id{
                                temp_return = Some(c.1.clone());
                                break;
                            }
                        }
                        break;
                    }
                    _=>{}
                }
            }
        }
        return temp_return;
    }
    pub fn set_temp(&self, id: u32, data: TempData){
        use std::ops::DerefMut;
        loop{
            match self.temp.try_write() {
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
                    break;
                }
                _ => {}
            }
        }
    }
    pub fn rem_temp(&self, id: u32){
        use std::ops::DerefMut;
        loop{
            match self.temp.try_write() {
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
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn set_chat(&self, id: u64, chat_new: Chat){
        use std::ops::DerefMut;
        loop{
            match self.chat.try_write() {
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
                        println!("set_chat Error in call [{}]:\n{}", call, e);
                    }
                    break;
                }
                _ => {}
            }
        }
    }
    pub fn rem_chat(&self, id: u64){
        use std::ops::DerefMut;
        loop{
            match self.chat.try_write() {
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
                        println!("rem_chat Error in call [{}]:\n{}",call, e);
                    }
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn prep_user(id: u64){
        thread::spawn( move|| {
            use std::ops::DerefMut;
            let mut some_i = None;
            let mut some_u = User::empty();
            loop{
                match DB.users.try_write() {
                    Ok(mut users) => {
                        let mut users = users.deref_mut();

                        for (i,l) in users.iter().enumerate(){
                            if l.did == id{
                                some_i = Some(i);
                                some_u = l.clone();
                                break;
                            }
                        }
                        if let Some(i) = some_i{
                            users.remove(i);
                        }
                        break;
                    }
                    _ => {}
                }
            }
            if let None = some_i{
                match load_by_id(id){
                    Some(user) =>{
                        some_u = user.clone();
                        let mut req = HeroInfoReq::empty();
                        req.rating = true;
                        match load_btag_data(user.btag,user.reg,user.plat,req){
                            None => {
                            }
                            Some(data) => {
                                some_u.rtg = data.rating;
                            }
                        }
                    }
                    None => {

                    }
                }
            }
            if some_u.did != 0{
                loop {
                    match DB.users.try_write() {
                        Ok(mut users) => {
                            let mut users = users.deref_mut();
                            users.push(some_u);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        });
    }
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Chat{
    LFG(Stage_LFG),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct LFG{
    pub did: u64,
    btag: String,
    reg: String,
    plat: String,
    rating: u16,
    description: String,
}
impl LFG{
    pub fn def_table(debug: bool, chanel: ChannelId) -> Vec<(String, String,bool)>{
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
    fn to_line(&self, chanel: ChannelId) -> (String,String){

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
    pub fn to_line_debug(&self, chanel: ChannelId) -> (String,String){

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

#[derive(Clone, Debug)]
pub enum Role{
    DPS,
    Def,
    Tank,
    Sup,
    Flex,
}


pub fn lfg(mes: Message, stage: Stage_LFG){
    let color: u64 = 37595;
    let private_c = DIS.create_private_channel(mes.author.id).unwrap().id;
    DIS.broadcast_typing(mes.channel_id);
    match stage{
        Stage_LFG::None => { lfg_none(mes)}
        Stage_LFG::ConfirmRemove => {

            let (mut mes_data, _) = split_mes(mes.content.clone());
            for m in mes_data{
                let mut s = m;
                match s.to_uppercase().as_str() {
                    "!WSLFG" => {continue;}
                    "N" | "NO" | "НЕТ" | "ОТМЕНА" => {
                        match DB.get_lfg(mes.author.id.0){
                            Some(lfg) => {
                                let title = "Объявление осталось на месте:";
                                let (tstring, dstring) = lfg.to_line(mes.channel_id);
                                if let Err(e) = embed(mes.channel_id,"",title,"",String::new(),color,
                                                      (String::new(),""),vec![(tstring,dstring,false)],("","",""),String::new(),String::new()){
                                    println!("Message Error: {:?}", e);
                                }
                            }
                            None => {
                                DB.send_embed("lfg_not_found_WTF",mes.channel_id);
                            }
                        }
                        DB.rem_chat(mes.author.id.0);
                        return;
                    }
                    "YES" | "ДА" | "Y" => {
                        lfg_rem(mes.channel_id,mes.author.id.0);
                        DB.rem_chat(mes.author.id.0);
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
                        match DB.get_lfg(mes.author.id.0){
                            Some(lfg) => {
                                let title = "Объявление осталось преждним:";
                                let (tstring, dstring) = lfg.to_line(mes.channel_id);
                                if let Err(e) = embed(mes.channel_id,"",title,"",String::new(),color,
                                                      (String::new(),""),vec![(tstring,dstring,false)],("","",""),String::new(),String::new()){
                                    println!("Message Error: {:?}", e);
                                }
                            }
                            None => {
                                DB.send_embed("lfg_not_found_WTF",mes.channel_id);
                            }
                        }
                        DB.rem_chat(mes.author.id.0);
                        return;
                    }
                    "YES" | "ДА" | "Y" => {
                        lfg_add(lfg.clone());

                        let title = "Ваше объявление обновлено:";
                        let (tstring, dstring) = lfg.to_line(mes.channel_id);
                        if let Err(e) = embed(mes.channel_id,"",title,"",String::new(),color,
                                              (String::new(),""),vec![(tstring,dstring,false)],("","",""),String::new(),String::new()){
                            println!("Message Error: {:?}", e);
                        }
                        DB.rem_chat(mes.author.id.0);
                        return;
                    }
                    _=>{}
                }
            }
        }
    }

}

fn lfg_none(mes: Message){
    let mut user = User::empty();

    let err_color: u64 = 13369344;
    let err_title = ":no_entry: Упс...";
    let color: u64 = 37595;
    let yellow_color: u64 = 15651330;
    let title = "Ваше объявление будет вглядеть так:".to_string();

    let private_c = DIS.create_private_channel(mes.author.id).unwrap().id;

    
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
                if let Err(e) = embed(mes.channel_id,"",title,"",String::new(),color,
                                      (String::new(),""),fields.clone(),("","",""),String::new(),String::new()){
                    println!("Message Error [!wslfg]: \n{:?}\nFields: \n{:?}\n", e, fields);
                }
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

					lfg_rem(mes.channel_id,mes.author.id.0);
                    /*match DB.get_lfg(mes.author.id.0){
                        Some(lfg) => {
							
                            
							/*DB.set_chat(mes.author.id.0, Chat::LFG(Stage_LFG::ConfirmRemove));
                            let mut vec = Vec::new();

                            let title = "Текущее:".to_string();
                            vec.push((title,lfg.to_line(),false));


                            if let Err(e) = embed(mes.channel_id,"","Удалить объявление из базы?","(Y/N/Отмена)",String::new(),color,
                                                  (String::new(),""),vec,("","",""),String::new(),String::new()){
                                println!("Message Error: {:?}", e);
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

        match load_by_id(mes.author.id.0) {
            None => {
                DB.send_embed("lfg_user_not_reg",mes.channel_id);
                return;}
            Some(u) => {
                user = u;
            }
        }

        if let Some(mut old_lfg) = DB.get_lfg(mes.author.id.0){



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
                    None => {
                        DB.send_embed("lfg_wrong_btag",mes.channel_id);
                        return;
                    }
                    Some(data) => {
                        old_lfg.btag = btag;
                        old_lfg.rating = data.rating;
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
			if let Err(e) = embed(mes.channel_id,"",title,"",String::new(),color,
								  (String::new(),""),vec![(tstring,dstring,false)],("","",""),String::new(),String::new()){
				println!("Message Error: {:?}", e);
			}
			/*
            DB.set_chat(mes.author.id.0, Chat::LFG(Stage_LFG::ConfirmUpdate(lfg.clone())));
            let mut vec = Vec::new();

            let title = "Текущее:".to_string();
            vec.push((title,old_lfg.to_line(),false));

            let title = "Новое:".to_string();
            vec.push((title,lfg.to_line(),false));

            if let Err(e) = embed(mes.channel_id,"","У вас уже размещено объявление","Хотете изменить? (Y/N/Отмена)",String::new(),color,
                                  (String::new(),""),vec,("","",""),String::new(),String::new()){
                println!("Message Error: {:?}", e);
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
                match load_btag_data(btag.clone(),reg.clone(),plat.clone(),req){
                    None => {
                        DB.send_embed("lfg_wrong_btag",mes.channel_id);
                        return;
                    }
                    Some(data) => {
                        rating = data.rating;
                    }
                }

                let lfg = LFG{
                    did: mes.author.id.0,
                    rating,
                    btag,
                    plat,
                    reg,
                    description: des,
                };

                lfg_add(lfg.clone());

                let title = "Ваше объявление добавлено в базу:";
                let (tstring, dstring) = lfg.to_line(mes.channel_id);
                if let Err(e) = embed(mes.channel_id,"",title,"",String::new(),color,
                                      (String::new(),""),vec![(tstring,dstring,false)],("","",""),String::new(),String::new()){
                    println!("Message Error: {:?}", e);
                }
                return;
            }

    }

}

fn lfg_rem(chanel: ChannelId,id: u64){
    let color: u64 = 37595;
    match DB.get_lfg(id){
        Some(lfg) => {
            let title = "Ваше объявление удалено:";
            let (tstring, dstring) = lfg.to_line(chanel);
            if let Err(e) = embed(chanel,"",title,"",String::new(),color,
                                  (String::new(),""),vec![(tstring,dstring,false)],("","",""),String::new(),String::new()){
                println!("Message Error: {:?}", e);
            }
            let mut call = format!("DELETE FROM lfg WHERE did={}",id);
            let mut conn = POOL.get_conn().unwrap();
            if let Err(e) = conn.query(call){
                println!("lfg_rem Err: {}", e);
            }
            DB.remove_lfg(id);
        }
        None => {
            DB.send_embed("lfg_del_notfound",chanel);
            return;
        }
    }

}

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
        println!("lfg_add Err: {}", e);
    }

    DB.push_lfg(lfg);
}

fn match_merge(chanel: ChannelId, did: u64) -> String{

    let user = match DIS.create_private_channel(discord::model::UserId(did)){
        Ok(x) => {
            x.recipient
        }
        _ => {return format!("<@{}>",did);}
    };
    if let Ok(chan) = DIS.get_channel(chanel){
        match chan{
            Channel::Group(chan_group) => {
                let servers = match DIS.get_servers() {
                    Ok(x) => {x}
                    _ => {return format!("{}#{}", user.name, user.discriminator);}
                };
                let mut users_id = Vec::new();
                for u in chan_group.recipients{
                    users_id.push(u.id);
                }
                users_id.push(user.id);
                //users_id.push(chan_group.owner_id);
                let mut br = false;
                'outer: for server in servers{
                    br = false;
                    'inner: for id in users_id.clone(){
                        match DIS.get_member(server.id,id){
                            Ok(_) =>{continue;
                            }
                            _ => {br = true; break 'inner;
                            }
                        }
                    }
                    if !br{
                        return format!("<@{}>",did);
                    }
                }
                return format!("{}#{}", user.name, user.discriminator);
            }
            Channel::Private(chan_private) => {
                let servers = match DIS.get_servers() {
                    Ok(x) => {x}
                    _ => {return format!("{}#{}", user.name, user.discriminator);}
                };
                let mut users_id = Vec::new();
                users_id.push(user.id);
                users_id.push(chan_private.recipient.id);
                let mut br = false;
                'outer: for server in servers{
                    br = false;
                    'inner: for id in users_id.clone(){
                        match DIS.get_member(server.id,id){
                            Ok(_) =>{continue;
                            }
                            _ => {br = true; break 'inner;
                            }
                        }
                    }
                    if !br{
                        return format!("<@{}>",did);
                    }
                }
                return format!("{}#{}", user.name, user.discriminator);
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
    }
}