use WSSERVER;
use embed_from_value;
use {DIS, POOL, EVENT};
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

use addon::DB;
use discord::model::ServerId;
use discord::model::ChannelId;
use std::ops::Sub;
use std::ops::Add;

use indexmap::map::IndexMap;
use serde_json;

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
                    break;
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
        req: EventReq,
    },
    RecalcEvent(String),
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
    for (key , event) in list.iter(){
        match event.event_type {
            EventType::LFGCleaning =>{
                lfg_clean_found =true;
                break;
            }
            _ => {
                continue;
            }
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
            req,
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

                    EventChanel::RecalcEvent(name) =>{
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
                if next.to_timespec() <= cur_time{
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
    req: EventReq,
}
impl EventData{

    fn create(name: String,event_type: EventType, req: EventReq) ->EventData{

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
        println!("Start Event {}", &self.name);
        let event_type = self.event_type.clone();
        let name = self.name.clone();
        let _ = thread::spawn(move || match_func(name, event_type));

        if self.req.once{
            return Some(self.name.clone());
        }
        else {
            if let Some(name) = self.calc_next(){
                return Some(name);
            }
            return None;
        }

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
        use extime::{now,
                     empty_tm,
                     get_time,
                     Duration};
        use std::cmp::Ordering;


        if let Some(ref next) = self.next_activ{
            self.last_activ = Some(next.clone());
        }

        let mut time = match self.last_activ {
            Some(ref last) => {
                if self.req.once {
                    return Some(self.name.clone());
                }
                let mut new = last.to_tm();
                if let Some(_) = self.req.sec{
                    new = new.add(Duration::seconds(1));
                }
                    else if let Some(_) = self.req.min {
                        new = new.add(Duration::seconds((60 - new.tm_sec) as i64));
                    }
                        else if let Some(_) = self.req.hour {
                            new = new.add(Duration::minutes((60 - new.tm_min) as i64));
                            new = new.add(Duration::seconds((60 - new.tm_sec) as i64));
                        }
                            else {
                                match (self.req.day_of_mouth, self.req.day_of_week) {
                                    (None,None) =>{
                                        if let Some(_) = self.req.month{
                                            let cur_month = new.tm_mon;
                                            new = new.add(Duration::seconds(60 - new.tm_sec as i64));
                                            new = new.add(Duration::minutes(60 - new.tm_min as i64));
                                            new = new.add(Duration::hours(24 - new.tm_hour as i64));
                                            if new.tm_mon == cur_month{
                                                let day_duration = Duration::days(1);
                                                loop{
                                                    new = new.add(day_duration);
                                                    if new.tm_mon != cur_month{
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        else if let Some(_) = self.req.year {
                                            new = new.add(Duration::seconds(60 - new.tm_sec as i64));
                                            new = new.add(Duration::minutes(60 - new.tm_min as i64));
                                            new = new.add(Duration::hours(24 - new.tm_hour as i64));
                                            new = new.add(Duration::days(365 - new.tm_yday as i64));
                                        }
                                    }
                                    _ =>{
                                        new = new.add(Duration::hours(24 - new.tm_hour as i64));
                                        new = new.add(Duration::minutes(60 - new.tm_min as i64));
                                        new = new.add(Duration::seconds(60 - new.tm_sec as i64));
                                    }
                                }
                            }
                new
            }
            None => {
                now()
            }
        };
        let time_zone = time.tm_utcoff;

        let mut check = false;

        loop{

            if let Some(year) = self.req.year{
                let mut year = year;
                if year >= 1900{
                    year = year - 1900;
                }
                if time.tm_year > year as i32{
                    return Some(self.name.clone());
                }
                if time.tm_year < year as i32{
                    time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                    time = time.add(Duration::minutes(60 - time.tm_min as i64));
                    time = time.add(Duration::hours(24 - time.tm_hour as i64));
                    time = time.add(Duration::days(365 - time.tm_yday as i64));
                }
            }
            if let Some(month) = self.req.month{
                if time.tm_mon > month as i32{

                    time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                    time = time.add(Duration::minutes(60 - time.tm_min as i64));
                    time = time.add(Duration::hours(24 - time.tm_hour as i64));
                    time = time.add(Duration::days(365 - time.tm_yday as i64));

                    check = true;
                }
                if time.tm_mon < month as i32{

                    time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                    time = time.add(Duration::minutes(60 - time.tm_min as i64));
                    time = time.add(Duration::hours(24 - time.tm_hour as i64));

                    if time.tm_mon != month as i32{
                        let day_duration = Duration::days(1);
                        loop{
                            time = time.add(day_duration);
                            if time.tm_mon == month as i32{
                                break;
                            }
                        }
                    }
                    check = true;
                }
            }

            match (self.req.day_of_mouth, self.req.day_of_week){
                (None,None) =>{}
                (d_m,d_w) =>{
                    let mut d_m_bool = false;
                    let mut d_w_bool = false;

                    if let Some(day_month) = d_m{
                        if time.tm_mday == day_month as i32{
                            d_m_bool = true;
                        }
                        else {
                            d_m_bool = false;
                        }
                    }
                    else{
                        d_m_bool = true;
                    }

                    if let Some(day_week) = d_w{
                        if time.tm_wday == day_week as i32{
                            d_w_bool = true;
                        }
                            else {
                                d_w_bool = false;
                            }
                    }
                        else{
                            d_w_bool = true;
                        }


                    if !d_w_bool || !d_m_bool{



                        loop{

                            time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                            time = time.add(Duration::minutes(60 - time.tm_min as i64));
                            time = time.add(Duration::hours(24 - time.tm_hour as i64));
                            if let Some(day_month) = d_m{
                                if time.tm_mday == day_month as i32{
                                    d_m_bool = true;
                                }
                                    else {
                                        d_m_bool = false;
                                    }
                            }
                                else{
                                    d_m_bool = true;
                                }

                            if let Some(day_week) = d_w{
                                if time.tm_wday == day_week as i32{
                                    d_w_bool = true;
                                }
                                    else {
                                        d_w_bool = false;
                                    }
                            }
                                else{
                                    d_w_bool = true;
                                }
                            if d_w_bool && d_m_bool{
                                break;
                            }
                        }
                        check = true;
                    }
                }
            }

            if let Some(hour) = self.req.hour{
                if time.tm_hour != hour as i32{

                    time = time.add(Duration::seconds(60 - time.tm_sec as i64));
                    time = time.add(Duration::minutes(60 - time.tm_min as i64));

                    if time.tm_hour != hour as i32{
                        let hour_duration = Duration::hours(1);
                        loop{
                            time = time.add(hour_duration);
                            if time.tm_hour == hour as i32{
                                break;
                            }
                        }
                    }
                    check = true;
                }

            }

            if let Some(min) = self.req.min{
                if time.tm_min != min as i32{
                    let mut add_to_min = Duration::seconds(60 - time.tm_sec as i64);
                    time = time.add(add_to_min);
                    if time.tm_min != min as i32{
                        let min_duration = Duration::minutes(1);
                        loop{
                            time = time.add(min_duration);
                            if time.tm_min == min as i32{
                                break;
                            }
                        }
                    }
                    check = true;
                }
            }

            if let Some(sec) = self.req.sec{
                if time.tm_sec != sec as i32{
                    let sec_duration = Duration::seconds(1);
                    loop{
                        time = time.add(sec_duration);
                        if time.tm_sec == sec as i32{
                            break;
                        }
                    }
                    check = true;
                }
            }

            if !check {break;}
            else{
                check = false;
            }
        }
        time.tm_utcoff = time_zone; //костыль слёта часового пояса
        let nex_event_time = time.to_timespec().sec;
        self.next_activ = Some(TmAlt::from(time));

        if get_time().sec > nex_event_time{
            return None;
        }
        else {
            return self.calc_next();
        }

//        match get_time().sec.cmp(&nex_event_time) {
//            Ordering::Greater => {
//                return None;
//            }
//
//            _ => {
//                return self.calc_next();
//            }
//        }

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
                    embed_from_value(ChannelId(c),v.clone());
                }
                    else {

                        match server_id{
                            Some(id) =>{
                                if let Ok(chnels) = DIS.get_server_channels(ServerId(id)){
                                    for c in chnels{
                                        if c.name == room{
                                            embed_from_value(c.id,v.clone());
                                            return;
                                        }
                                    }
                                }
                            }
                            None => {
                                if let Some(servername) = server{
                                    match DIS.get_servers() {
                                        Ok(list) => {
                                            for serv in list{
                                                if serv.name == servername{
                                                    if let Ok(chnels) = DIS.get_server_channels(serv.id){
                                                        for c in chnels{
                                                            if c.name == room{
                                                                embed_from_value(c.id,v.clone());
                                                                return;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {println!("get_servers err: {}", e)}
                                    }
                                }
                                else {
                                    println!("Event>no server data in evet: {}", name);
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
    }
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

    if let Some(servid) = serverId{
        if let Ok(chnels) = DIS.get_server_channels(ServerId(servid)){
            for c in chnels{
                if c.name == chanel{
                    return Some(c.id.0);
                }
            }
        }
        println!("{}room not found (by server Id)",err_str);

    }

    if let Some(servername) = server{
        match DIS.get_servers() {
            Ok(list) => {
                for serv in list{
                    if serv.name == servername {
                        if let Ok(chnels) = DIS.get_server_channels(serv.id){
                            for c in chnels{
                                if c.name == chanel{
                                    return Some(c.id.0);
                                }
                            }
                        }
                        println!("{}room not found (by server name)",err_str);
                        return None;
                    }
                }
                println!("{}server not found",err_str);
                return None;
            }
            Err(e) => {
                println!("{}get_servers err: {}",err_str, e);
                return None;}
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