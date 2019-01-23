use crate::WSSERVER;
use crate::embed_from_value;
use crate::{POOL, EVENT, User, load_btag_data, HeroInfoReq, OwData};
use mysql::from_row;
use mysql;
use std::thread;
use std::sync::mpsc::{channel,
                      Sender,
                      Receiver};
use extime::Timespec;
use extime::Tm;
use extime;
use std::sync::Mutex;

use crate::addon::DB;
use std::ops::Sub;
use std::ops::Add;

use indexmap::map::IndexMap;
use serde_json;
use crate::disapi::Discord;

pub struct EventH{
    sender: Mutex<Sender<EventChanel>>,
    receiver: Mutex<Receiver<EventChanelBack>>,
}
impl EventH{
    pub fn create() -> EventH{
        let (send, receiv) = channel::<EventChanel>();
        let (send2, receiv2) = channel::<EventChanelBack>();
        let thread_child = thread::Builder::new()
            .name("Event_Engine".to_string())
            .spawn(move || event_engine(receiv,send2)).unwrap();

        EventH{
            sender: Mutex::new(send),
            receiver: Mutex::new(receiv2),
        }

    }

    pub fn send(&self, enm: EventChanel){
        use std::ops::Deref;
        loop{
            match self.sender.try_lock(){
                Ok(sender) => {
                    let sender = sender.deref();
                    match sender.send(enm){
                        Err(_) => {
                            println!("Event_Engine>Sender Error while send data");
                        }
                        _ => {}
                    }
                    break;
                }
                _=>{}
            }
        }
    }

    pub fn recive(&self) -> EventChanelBack{
        use std::ops::Deref;
        use std::time::Duration;
        loop{
            match self.receiver.try_lock(){
                Ok(receiver) => {
                    let receiver = receiver.deref();
                    match receiver.recv_timeout(Duration::from_secs(2)){
                        Ok(data) => {
                            return data;
                        }
                        _ => {return EventChanelBack::Error;}
                    }

                }
                _=>{}
            }
        }
        return EventChanelBack::Error;
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EventChanelBack{
    List(Vec<(String, TmAlt)>),
    Error,
}

pub enum EventChanel {
    AddEvent{
        name: String,
        event_type: EventType,
        req: Vec<EventReq>,
    },
    RecalcEventTime(String),
    RecalcEventChanel(String),
    RemEvent(String),
    Check,
    GetList,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EventType{
    CustomEmbed{
        server: Option<String>,
        server_id: Option<u64>,
        room: String,
        chanel: Option<u64>,
        embed: String,
    },
    LFGCleaning,
    RatingUpdate,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EventReq{
    pub sec: Option<u8>,
    pub min: Option<u8>,
    pub hour: Option<u8>,
    pub day_of_week: Option<u8>, // [0, 6] [Вс, Сб]
    pub day_of_mouth: Option<u8>,// [1, 31]
    pub month: Option<u8>, // [0, 11]
    pub year: Option<u16>,//2018 or 118
    pub once: bool,
}

impl EventReq{
    pub fn empty() -> EventReq{
        EventReq{
            sec: None,
            min: None,
            hour: None,
            day_of_week: None, // [0, 6]
            day_of_mouth: None,// [1, 31]
            month: None, // [0, 11]
            year: None,//2018 or 118
            once: false
        }
    }

    pub fn eq_alt(&self, other: &EventReq) -> bool {
        if !self.sec.eq(&other.sec){return false;}
        if !self.min.eq(&other.min){return false;}
        if !self.hour.eq(&other.hour){return false;}
        if !self.day_of_week.eq(&other.day_of_week){return false;}
        if !self.day_of_mouth.eq(&other.day_of_mouth){return false;}
        if !self.month.eq(&other.month){return false;}
        if !self.year.eq(&other.year){return false;}
        if !self.once.eq(&other.once){return false;}
        return true;
    }
}


fn event_engine(reseiv: Receiver<EventChanel>, sender: Sender<EventChanelBack>){
    use std::collections::hash_map::RandomState;
    use std::time::Duration;
    use std::sync::mpsc::RecvTimeoutError;

    let sleep_time = Duration::from_secs(1);

    let s = RandomState::new();
    let mut list:IndexMap<String,EventData,RandomState> = IndexMap::with_hasher(s);

    let mut call = format!("SELECT * FROM events");

    match POOL.prepare(call.as_str()){
        Err(e) => {
            if let mysql::Error::MySqlError(my)=e{
                if my.code == 1146{
                    let mut call = format!(r#"CREATE TABLE `events`
                        (`name` VARCHAR(40) NOT NULL,
                        `data` json DEFAULT NULL,PRIMARY KEY (`name`)
                        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4
                        COLLATE=utf8mb4_unicode_ci"#);
                    let mut conn = POOL.get_conn().unwrap();
                    let _ = conn.query(call);
                    println!("EventEng>try to create Table");
                }
                println!("EventEng>pool Error on call[{}]: {:?}", call, my);
            }
                else { println!("EventEng>pool Error on call[{}]", call);}
        }
        Ok(mut stmt) => {
            for row in stmt.execute(()).unwrap() {
                use serde_json;
                let (name, string) = from_row::<(String, String)>(row.unwrap());
                match serde_json::from_str(string.as_str()){
                    Ok(ls) => {let _ = list.insert(name, ls);}
                    Err(e) => {
                        println!("EventEng>serde Error on [{}]: {:?}", name, e);
                    }
                }
            }
        }
    }
    let mut lfg_clean_found = false;
    let mut rating_update = false;

    for (key , event) in list.iter(){
        match event.event_type {
            EventType::LFGCleaning =>{
                lfg_clean_found = true;
            }
            EventType::RatingUpdate =>{
                rating_update = true;
            }
            _ => {
                continue;
            }
        }
        if lfg_clean_found && rating_update{
            break;
        }
    }

    if !lfg_clean_found {
        let mut req = EventReq::empty();
        let name = "LFGCleaner".to_string();
        req.hour = Some(4);
        req.day_of_week = Some(3);
        EVENT.send(EventChanel::AddEvent {
            name,
            event_type: EventType::LFGCleaning,
            req: vec![req],
        });

    }

    if !rating_update {
        let mut req = EventReq::empty();
        let name = "RatingUpdate".to_string();
        req.hour = Some(3);
        req.min = Some(45);
        req.day_of_week = Some(3);
        EVENT.send(EventChanel::AddEvent {
            name,
            event_type: EventType::RatingUpdate,
            req: vec![req],
        });

    }

    //println!("Event start loop");
    loop{

        match reseiv.recv_timeout(sleep_time){
            Ok(data) => {
                match data{
                    EventChanel::AddEvent{
                        name,
                        event_type,
                        req} => {

                        let eventdata = EventData::create(name.clone(),event_type,req);
                        let json = serde_json::to_string(&eventdata).unwrap();
                        let mut call = format!("INSERT INTO events (");

                        call = format!("{} name", call);
                        call = format!("{}, data", call);

                        call = format!("{}) VALUES (", call);

                        call = format!("{} '{}'", call, name.clone());
                        call = format!("{}, '{}'", call, json.clone());

                        call = format!("{}) ON DUPLICATE KEY UPDATE", call);
                        call = format!("{} data='{}'", call, json);
                        let mut conn = POOL.get_conn().unwrap();
                        if let Err(e) = conn.query(call){
                            println!("Event>Add MySQL Err: {}", e);
                        }
                        let _ = list.remove(&name);
                        let _ = list.insert(name.clone(), eventdata);
                    }

                    EventChanel::RemEvent(name) => {
                        let mut call = format!("DELETE FROM events WHERE name='{}'",name);
                        let mut conn = POOL.get_conn().unwrap();
                        if let Err(e) = conn.query(call){
                            println!("Event>Rem Err: {}", e);
                        }
                        let _ = list.remove(&name);
                        println!("Event {} removed", name);
                    }

                    EventChanel::Check =>{
                        println!("Event>Check done");
                    }

                    EventChanel::RecalcEventTime(name) =>{
                        if let Some(ref mut event) = list.get_mut(&name){
                            event.calc_next();
                        }
                    }

                    EventChanel::RecalcEventChanel(name) =>{
                        if let Some(ref mut event) = list.get_mut(&name){
                            event.calc_chanel_id();
                        }
                    }

                    EventChanel::GetList => {
                        let mut l: Vec<(String, TmAlt)> = Vec::new();

                        for (_, event) in list.iter(){
                            let time = match event.next_activ{
                                Some(ref n) => {
                                    n.to_tm()
                                }
                                None => {
                                    extime::empty_tm()
                                }
                            };
                            l.push((event.name.clone(), TmAlt::from(time)));
                        }

                        match sender.send(EventChanelBack::List(l)){
                            Err(_) => {
                                println!("Event_Engine>Sender Error while send data");
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(RecvTimeoutError::Disconnected) =>{
                println!("EventEng>chanel disconected, cancel thread");
                return;
            }
            _ => {}
        }

        let cur_time:Timespec = extime::get_time();

        let mut remove_list = Vec::new();

        for (key, event) in list.iter_mut(){
            if let Some(next) = event.next_activ.clone(){
                if next.to_timespec().sec <= cur_time.sec{
                    if let Some(name) = event.start(){
                        remove_list.push(name);
                    }
                }
            }
            else {
                remove_list.push(key.clone());
            }

        }

        for rem in remove_list{
            let mut call = format!("DELETE FROM events WHERE name='{}'",rem);
            let mut conn = POOL.get_conn().unwrap();
            if let Err(e) = conn.query(call){
                println!("Event>Rem Err: {}", e);
            }
            let _ = list.remove(&rem);
            println!("Event {} removed", rem);
        }

    }

}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EventData{
    name: String,
    last_activ: Option<TmAlt>,
    next_activ: Option<TmAlt>,
    event_type: EventType,
    req: Vec<EventReq>,
}
impl EventData{

    fn create(name: String,event_type: EventType, req: Vec<EventReq>) ->EventData{

        let mut data = EventData{
            name,
            last_activ: None,
            next_activ: None,
            event_type,
            req
        };
        data.calc_next();
        data.calc_chanel_id();
        println!("New event: {}", data.name.clone());
        data
    }

    fn start(&mut self) -> Option<String>{

        let mut output = format!("-----------");
        output = format!("{}\nStart Event: {}",output, self.name);
        output = format!("{}\n-         -",output);
        output = format!("{}\nPlan time: {}",output, extime::now().ctime());

        if let Some(ref next) = self.next_activ{
            output = format!("{}\nStart time: {}",output, next.to_tm().ctime())
        }

        let event_type = self.event_type.clone();
        let name = self.name.clone();
        let _ = thread::spawn(move || match_func(name, event_type));


        if let Some(name) = self.calc_next(){
            output = format!("{}\nInfo: {:?}",output, &self.event_type);
            output = format!("{}\n-----------",output);
            println!("{}",output);
            return Some(name);
        }
        if let Some(ref next) = self.next_activ{
            output = format!("{}\nNext time: {}",output, next.to_tm().ctime())
        }
        output = format!("{}\nInfo: {:?}",output, &self.event_type);
        output = format!("{}\n-----------",output);
        println!("{}",output);
        return None;
    }

    fn calc_chanel_id(&mut self){
        match self.event_type {
            EventType::CustomEmbed {
                ref server,
                server_id,
                ref room,
                ref mut chanel,
                ..
            } => {
                *chanel = get_chanel_id(server.clone(),server_id.clone(),room.clone());
            }
            _ =>{}
        }
    }

    fn calc_next(&mut self) -> Option<String>{
        use extime::{now_utc,
                     empty_tm,
                     get_time,
                     Duration};
        use std::cmp::Ordering;


        if let Some(ref next) = self.next_activ{
            self.last_activ = Some(next.clone());
        }

        let mut time_list:Vec<(TmAlt,bool,usize)> = Vec::new();

        for (req_num, req) in self.req.iter().enumerate() {
            let mut time = match self.last_activ {
                Some(ref last) => {
                    if get_time().sec < last.sec {
                        now_utc()
                    } else {
                        let mut new = last.to_tm().to_utc();
                        if let Some(_) = req.sec {
                            new = new.add(Duration::seconds(1));
                        } else if let Some(_) = req.min {
                            new = new.add(Duration::seconds((60 - new.tm_sec) as i64));
                        } else if let Some(_) = req.hour {
                            new = new.add(Duration::minutes((60 - new.tm_min) as i64));
                            new = new.add(Duration::seconds((60 - new.tm_sec) as i64));
                        } else {
                            match (req.day_of_mouth, req.day_of_week) {
                                (None, None) => {
                                    if let Some(_) = req.month {
                                        let cur_month = new.tm_mon;
                                        new = new.add(Duration::seconds(60 - new.tm_sec as i64));
                                        new = new.add(Duration::minutes(60 - new.tm_min as i64));
                                        new = new.add(Duration::hours(24 - new.tm_hour as i64));
                                        if new.tm_mon == cur_month {
                                            let day_duration = Duration::days(1);
                                            loop {
                                                new = new.add(day_duration);
                                                if new.tm_mon != cur_month {
                                                    break;
                                                }
                                            }
                                        }
                                    } else if let Some(_) = req.year {
                                        new = new.add(Duration::seconds(60 - new.tm_sec as i64));
                                        new = new.add(Duration::minutes(60 - new.tm_min as i64));
                                        new = new.add(Duration::hours(24 - new.tm_hour as i64));
                                        new = new.add(Duration::days(365 - new.tm_yday as i64));
                                    }
                                }
                                _ => {
                                    new = new.add(Duration::hours(24 - new.tm_hour as i64));
                                    new = new.add(Duration::minutes(60 - new.tm_min as i64));
                                    new = new.add(Duration::seconds(60 - new.tm_sec as i64));
                                }
                            }
                        }
                        new
                    }
                }
                None => {
                    now_utc()
                }
            };


            let mut check = false;

            loop {
                if let Some(year) = req.year {
                    let mut year = year;
                    if year >= 1900 {
                        year = year - 1900;
                    }
                    if time.tm_year > year as i32 {
                        return Some(self.name.clone());
                    }
                    if time.tm_year < year as i32 {
                        time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                        time = time.add(Duration::minutes(60 - time.tm_min as i64));
                        time = time.add(Duration::hours(24 - time.tm_hour as i64));
                        time = time.add(Duration::days(365 - time.tm_yday as i64));
                    }
                }
                if let Some(month) = req.month {
                    if time.tm_mon > month as i32 {
                        time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                        time = time.add(Duration::minutes(60 - time.tm_min as i64));
                        time = time.add(Duration::hours(24 - time.tm_hour as i64));
                        time = time.add(Duration::days(365 - time.tm_yday as i64));

                        check = true;
                    }
                    if time.tm_mon < month as i32 {
                        time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                        time = time.add(Duration::minutes(60 - time.tm_min as i64));
                        time = time.add(Duration::hours(24 - time.tm_hour as i64));

                        if time.tm_mon != month as i32 {
                            let day_duration = Duration::days(1);
                            loop {
                                time = time.add(day_duration);
                                if time.tm_mon == month as i32 {
                                    break;
                                }
                            }
                        }
                        check = true;
                    }
                }

                match (req.day_of_mouth, req.day_of_week) {
                    (None, None) => {}
                    (d_m, d_w) => {
                        let mut d_m_bool = false;
                        let mut d_w_bool = false;

                        if let Some(day_month) = d_m {
                            if time.tm_mday == day_month as i32 {
                                d_m_bool = true;
                            } else {
                                d_m_bool = false;
                            }
                        } else {
                            d_m_bool = true;
                        }

                        if let Some(day_week) = d_w {
                            if time.tm_wday == day_week as i32 {
                                d_w_bool = true;
                            } else {
                                d_w_bool = false;
                            }
                        } else {
                            d_w_bool = true;
                        }


                        if !d_w_bool || !d_m_bool {
                            loop {
                                time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                                time = time.add(Duration::minutes(60 - time.tm_min as i64));
                                time = time.add(Duration::hours(24 - time.tm_hour as i64));
                                if let Some(day_month) = d_m {
                                    if time.tm_mday == day_month as i32 {
                                        d_m_bool = true;
                                    } else {
                                        d_m_bool = false;
                                    }
                                } else {
                                    d_m_bool = true;
                                }

                                if let Some(day_week) = d_w {
                                    if time.tm_wday == day_week as i32 {
                                        d_w_bool = true;
                                    } else {
                                        d_w_bool = false;
                                    }
                                } else {
                                    d_w_bool = true;
                                }
                                if d_w_bool && d_m_bool {
                                    break;
                                }
                            }
                            check = true;
                        }
                    }
                }

                if let Some(hour) = req.hour {
                    let mut hour = hour + 24;
                    hour = hour - 3;
                    if hour > 23 { hour = hour - 24; }
                    if time.tm_hour != hour as i32 {
                        time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                        time = time.add(Duration::minutes(60 - time.tm_min as i64));

                        if time.tm_hour != hour as i32 {
                            let hour_duration = Duration::hours(1);
                            loop {
                                time = time.add(hour_duration);
                                if time.tm_hour == hour as i32 {
                                    break;
                                }
                            }
                        }
                        check = true;
                    }
                }

                if let Some(min) = req.min {
                    if time.tm_min != min as i32 {
                        let mut add_to_min = Duration::seconds(60 - time.tm_sec as i64);
                        time = time.add(add_to_min);
                        if time.tm_min != min as i32 {
                            let min_duration = Duration::minutes(1);
                            loop {
                                time = time.add(min_duration);
                                if time.tm_min == min as i32 {
                                    break;
                                }
                            }
                        }
                        check = true;
                    }
                }

                if let Some(sec) = req.sec {
                    if time.tm_sec != sec as i32 {
                        let sec_duration = Duration::seconds(1);
                        loop {
                            time = time.add(sec_duration);
                            if time.tm_sec == sec as i32 {
                                break;
                            }
                        }
                        check = true;
                    }
                }

                if !check { break; } else {
                    check = false;
                }
            }
            time_list.push((TmAlt::from(time.to_local()),req.once, req_num));
        }
        let mut time_min = match time_list.get(0){
            Some(t) => {t.clone()}
            None => {
                return Some(self.name.clone())
            }
        };
        loop{
            let mut check = false;

            for t in time_list.clone(){
                if time_min.0.sec > t.0.sec{
                    time_min = t;
                    check = true;
                }
            }

            if !check { break; }
        }


        self.next_activ = Some(time_min.0.clone());

        if get_time().sec < time_min.0.sec{
            if time_min.1{
                let _ = self.req.remove(time_min.2);
            }
            let json = serde_json::to_string(&self).unwrap();
            let mut call = format!("INSERT INTO events (");

            call = format!("{} name", call);
            call = format!("{}, data", call);

            call = format!("{}) VALUES (", call);

            call = format!("{} '{}'", call, self.name);
            call = format!("{}, '{}'", call, json.clone());

            call = format!("{}) ON DUPLICATE KEY UPDATE", call);
            call = format!("{} data='{}'", call, json);
            let mut conn = POOL.get_conn().unwrap();
            if let Err(e) = conn.query(call){
                println!("Event>Add MySQL Err: {}", e);
            }
            return None;
        }
            else {
                return self.calc_next();
            }
    }
}

fn match_func(name: String, event_type: EventType){
    match event_type {
        EventType::CustomEmbed {
            server: server,
            server_id: server_id,
            room: room,
            chanel: chanel,
            embed: embed,
        } => {
            if let Some(v) = DB.get_embed(embed.as_str()){

                if let Some(c) = chanel{
                    embed_from_value(c,v.clone());
                }
                    else {
                        let room_name = json!(room);

                        match server_id{
                            Some(id) =>{
                                if let Some(ch) = Discord::get_server_channels(id){
                                    for c in ch.as_array().unwrap(){
                                        if c["name"].eq(&room_name){
                                            embed_from_value(c["id"].as_str().unwrap().parse::<u64>().unwrap(),v.clone());
                                            return;
                                        }
                                    }
                                }
                            }
                            None => {
                                if let Some(servern) = server{
                                    let server_name = json!(servern);

                                    if let Some(list) = Discord::get_servers(){
                                        for server_val in list.as_array().unwrap(){
                                            if server_name.eq(&server_val["name"]){
                                                if let Some(ch) = Discord::get_server_channels(server_val["id"].as_str().unwrap().parse::<u64>().unwrap()){
                                                    for c in ch.as_array().unwrap(){
                                                        if c["name"].eq(&room_name){
                                                            embed_from_value(c["id"].as_str().unwrap().parse::<u64>().unwrap(),v.clone());
                                                            return;
                                                        }
                                                    }
                                                }
                                            }

                                        }
                                    }
                                }
                                else {
                                    println!("Event>no server data in event: {}", name);
                                }
                            }
                        };

                    }
            }
                else {
                    println!("Event>embed not found: {}", embed);
                }

        }

        EventType::LFGCleaning => {
            let lfg_list = DB.get_lfg_list();
            for lfg in lfg_list{
                use extime::get_time;
                let cur_sec = get_time().sec;
                if lfg.time > cur_sec{
                    continue;
                }
                else {
                    if (cur_sec - lfg.time) < 172800 { //172800 = 2 дня
                        continue;
                    }
                    else {
                        let mut call = format!("DELETE FROM lfg WHERE did={}",lfg.did);
                        let mut conn = POOL.get_conn().unwrap();
                        if let Err(e) = conn.query(call){
                            println!("lfg_rem Err: {}", e);
                        }
                        DB.remove_lfg(lfg.did);
                    }
                }
            }
            println!("LFG Cleaning End");
        }

        EventType::RatingUpdate => {
            rating_updater();
        }
    }
}

pub fn rating_updater(){
    use crate::roles::role_ruler;
    use crate::RoleR;

    let begun_time = extime::get_time().sec;

    let hreq = HeroInfoReq{
        num: 0,
        rating: true,
        time_played: false,
        games_won: false,
        win_perc: false,
        aim: false,
        kills_per_live: false,
        best_multiple_kills: false,
        obj_kills: false,
    };

    let mut conn = POOL.get_conn().expect("Err in rating_updater on POOL.get_conn() #1");
    let command = format!("SELECT did, btag, plat FROM users");
    let mut stmt = conn.prepare(command).expect("Err in rating_updater on conn.prepare() #2");

    let mut counter_all = 0;
    let mut counter_bad_btag = 0;
    let mut counter_close_prof = 0;
    let mut counter_ok = 0;

    for row in stmt.execute(()).unwrap() {
        counter_all += 1;
        let mut row = row.expect("Err in rating_updater on row unpack #8");
        let did: u64 = row.take("did").expect("Err in rating_updater on row.take(\"did\") #3");
        let btag: String = row.take("btag").expect("Err in rating_updater on row.take(\"btag\") #4");
        let plat: String = row.take("plat").expect("Err in rating_updater on row.take(\"plat\") #5");


        match load_btag_data(btag.clone(),"EU".to_string(),plat,hreq.clone()){
            OwData::NotFound => {
                let call = format!("UPDATE users SET rtg={} WHERE did={}",
                                   0,  did);
                let mut conn = POOL.get_conn().expect("Err in rating_updater on POOL.get_conn() #7");
                let _ = conn.query(call);
                let _ = role_ruler(WSSERVER,did,RoleR::rating(0));
                println!("[{}] Rating of {} now {}", extime::now().ctime(),btag,0);
                counter_bad_btag += 1;
            }
            OwData::ClosedProfile {
                ..
            } => {
                let call = format!("UPDATE users SET rtg={} WHERE did={}",
                                   0,  did);
                let mut conn = POOL.get_conn().expect("Err in rating_updater on POOL.get_conn() #7");
                let _ = conn.query(call);
                let _ = role_ruler(WSSERVER,did,RoleR::rating(0));
                println!("[{}] Rating of {} now {} [Closed profile]", extime::now().ctime(),btag,0);
                counter_close_prof += 1;
            },
            OwData::Full(BData) => {
                let call = format!("UPDATE users SET rtg={} WHERE did={}",
                                   BData.rating,  did);

                let mut conn = POOL.get_conn().expect("Err in rating_updater on POOL.get_conn() #6");
                let _ = conn.query(call);
                let _ = role_ruler(WSSERVER,did,RoleR::rating(BData.rating));
                println!("[{}] Rating of {} now {}", extime::now().ctime(),btag,BData.rating);
                counter_ok += 1;
            }
        }
    }

    let mut taken_time = extime::get_time().sec - begun_time;
    let hours = taken_time/3600;
    taken_time = taken_time - (hours*3600);
    let minutes = taken_time/60;
    taken_time = taken_time - (minutes*60);
    let sec = taken_time;

    let mut output = format!("-----------");
    output = format!("{}\nRating Update Done",output);
    output = format!("{}\n-         -",output);
    output = format!("{}\nTime: {}:{}:{}",output, hours, minutes, sec);
    output = format!("{}\nAll users: {}",output, counter_all);
    output = format!("{}\nSuccess: {}",output, counter_ok);
    output = format!("{}\nWrong BTag: {}",output, counter_bad_btag);
    output = format!("{}\nClosed Profiles: {}",output, counter_close_prof);
    output = format!("{}\n-----------",output);
    println!("{}",output);

}



fn get_chanel_id(server: Option<String>, serverId: Option<u64>, chanel: String) -> Option<u64>{
    let mut err_str = format!("get_chanel_id[\n");
    if let Some(serv) = server.clone(){
        err_str = format!("{}   Srever Name: {}\n",err_str,serv);
    }
    if let Some(servid) = serverId{
        err_str = format!("{}   Srever Id: {}\n",err_str,servid);
    }
    err_str = format!("{}   Room: {}]\n> ",err_str,chanel);
    let chanel_name = json!(chanel);

    if let Some(servid) = serverId{
        if let Some(ch) = Discord::get_server_channels(servid){
            for c in ch.as_array().unwrap(){
                if c["name"].eq(&chanel_name){
                    return Some(c["id"].as_str().unwrap().parse::<u64>().unwrap());
                }
            }
        }
        println!("{}room not found (by server Id)",err_str);
    }

    if let Some(servername) = server{
        let server_name = json!(servername);

        if let Some(list) = Discord::get_servers(){
            for server_val in list.as_array().unwrap(){
                if server_name.eq(&server_val["name"]){
                    if let Some(ch) = Discord::get_server_channels(server_val["id"].as_str().unwrap().parse::<u64>().unwrap()){
                        for c in ch.as_array().unwrap(){
                            if c["name"].eq(&chanel_name){
                                return Some(c["id"].as_str().unwrap().parse::<u64>().unwrap());
                            }
                        }
                    }
                    println!("{}room not found (by server name)",err_str);
                    return None;
                }
                println!("{}server not found",err_str);
                return None;
            }
        }
    }
    return None;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TmAlt{ //костыль для Serialize
    sec: i64,
    nsec: i32,
}
impl TmAlt{
    pub fn to_timespec(&self) -> Timespec{
        Timespec{
            sec: self.sec,
            nsec: self.nsec
        }
    }

    pub fn to_tm(&self) -> Tm{
        use extime::at;
        at(self.to_timespec())
    }

}
impl From<Timespec> for TmAlt {
    fn from(i: Timespec) -> TmAlt {
        TmAlt{
            sec: i.sec,
            nsec: i.nsec
        }
    }
}
impl From<Tm> for TmAlt {
    fn from(i: Tm) -> TmAlt {
        TmAlt::from(i.to_timespec())
    }
}