#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(unused_must_use)]
#![allow(non_shorthand_field_patterns)]
#![allow(non_camel_case_types)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate reqwest;
//extern crate rusqlite;
#[macro_use]
extern crate mysql;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate indexmap;

//extern crate native_tls;
//extern crate websocket;
//extern crate net2;

extern crate time as extime;

#[macro_use]
extern crate log;
extern crate simplelog;

extern crate serenity;
//https://discordapp.com/api/oauth2/authorize?client_id=316281967375024138&scope=bot&permissions=0

use regex::Regex;
use std::io::Read;
use std::io::Write;
use std::ops::Deref;
use std::ops::Index;

use serenity::client::Client;
use serenity::http::raw::Http;
use serenity::model::id::ChannelId;
use dishandler::DisHandler;

pub mod addon;
pub mod conf;
pub mod roles;
pub mod event;
pub mod multirolefix;
//pub mod owparcer;


pub mod denum;
pub mod dstruct;
//pub mod dis;
pub mod disapi;
pub mod dishandler;

#[cfg(test)]
mod tests;



use disapi::Discord;
use dstruct::DCShell;
use denum::Event;
use denum::OutLink;
use dstruct::{DMessage,DUser};

use event::{EventChanel, EventH, EventReq, EventChanelBack, EventType};
use addon::{DB,
//            Chat,
//            lfg_none,
//              Stage_LFG,
            Global, TempData};
use dstruct::{DiscordMain};
use serde_json::Value;

use std::{thread, time, fmt};

use std::time::{Duration, Instant, SystemTime};
use std::fmt::Debug;
use std::sync::mpsc::channel;
use mysql::from_row;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

use multirolefix::Rating;
use multirolefix::load_btag_data_multirole;

use roles::RoleR;
use roles::RoleChange;
use roles::role_ruler_text;

lazy_static! {

    pub static ref POOL: mysql::Pool = mysql::Pool::new(build_opts()).expect("MySQL Pool not open in lazy_static");
    pub static ref REG_BTAG: Regex = Regex::new(r"^.{2,16}#[0-9]{2,6}$").expect("Regex btag error");
    static ref REG_TIME: Regex = Regex::new(r"(?P<n>\d){1,4} ?(?i)(?P<ntype>m|min|h|hour)").expect("Regex REG_TIME error");

    static ref START_TIME: extime::Tm = extime::now();
    pub static ref EVENT: EventH = EventH::create();

//    pub static ref D: DiscordMain = DiscordMain::new(load_settings());
}




pub static WSSERVER: u64 = 351798277756420098; //ws = 351798277756420098 //bs = 316394947513155598
static SWITCH_NET: AtomicBool = ATOMIC_BOOL_INIT;
static DEBUG: AtomicBool = ATOMIC_BOOL_INIT;
static DEBUG_LOCKED_THREADS: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Preset_Scrim {
    plat: String,
    live_time: u64,
    btag: String,
    rtg: u16,
}

impl Preset_Scrim {
    fn new() -> Preset_Scrim {
	    Preset_Scrim {
		    plat: String::new(),
		    live_time: 0,
            btag: String::new(),
		    rtg: 6000,
	    }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
struct Preset_Rtg {
    plat: String,
    live_time: u64,
    btag: String,
    rtg: u16,
}

impl Preset_Rtg {
    fn new() -> Preset_Rtg {
        Preset_Rtg {
            plat: String::new(),
            live_time: 0,
            btag: String::new(),
            rtg: 6000,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Scrim {
    user: u64,
    plat: String,
    rtg: u16,
    live_time: u64,
    hide: bool,
    show_btag: bool,
}

impl Scrim {
    fn new() -> Scrim {
        Scrim {
            user: 0,
            plat: "PC".to_string(),
            rtg: 6000,
            live_time: 900,
            hide: false,
            show_btag: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    did: u64,
    //discord id?
    name: String,
    //discord name?
    disc: String,
    //name#disc цифры после ника
    btag: String,
    //battlenet tag
    rtg: Rating,
    //OW rating
    reg: String,
    //OW region
    plat: String,
    //OW plaform
    scrim_preset: Preset_Scrim,
    rtg_preset: Preset_Rtg,
}

impl User {
    pub fn empty() -> User {
        User {
            did: 0,
            name: String::new(),
            disc: String::new(),
            btag: String::new(),
            rtg: Rating::empty(),
            reg: String::new(),
            plat: String::new(),
            scrim_preset: Preset_Scrim::new(),
            rtg_preset: Preset_Rtg::new(),
        }
    }
}

pub struct Server {
    dsid: u64,
    //discord server id?
}

impl Server {
    pub fn empty() -> Server {
        Server {
            dsid: 0,
        }
    }
}
fn build_opts() -> mysql::Opts //Конструктор для БД
{
    let mut builder = mysql::OptsBuilder::new();
    builder.user(Some("bot")).pass(Some("1234")).db_name(Some("wsowbot")); //wsowbot //ows
    return mysql::Opts::from(builder);
}

#[derive(Debug,Clone)]
enum Time{
    Hours(u32),
    Min(u32),
    Sec(u32),
    None
}

#[derive(PartialEq,Clone,Debug)]
pub enum Hero{
    None,
    Winston,
    Tracer,
    Pharah,
    Genji,
    Zenyatta,
    Reinhardt,
    Mercy,
    Lucio,
    Soldier,
    DVa,
    Reaper,
    Hanzo,
    Torbjorn,
    Widowmaker,
    Bastion,
    Symmetra,
    Roadhog,
    McCree,
    Junkrat,
    Zarya,
    Mei,
    Sombra,
    Doomfist,
    Ana,
    Orisa,
    Moira,
    Brigitte,
    Wrecking_Ball,
    Ashe,
	Baptiste,

}
impl Hero{
    fn get_from_bliz_str(s: &str) -> Hero{
        match s{
            "Winston" => {return Hero::Winston;}
            "Tracer" => {return Hero::Tracer;}
            "Pharah" => {return Hero::Pharah;}
            "Genji" => {return Hero::Genji;}
            "Zenyatta" => {return Hero::Zenyatta;}
            "Reinhardt" => {return Hero::Reinhardt;}
            "Mercy" => {return Hero::Mercy;}
            "Lúcio" => {return Hero::Lucio;}
            "Soldier: 76" => {return Hero::Soldier;}
            "D.Va" => {return Hero::DVa;}
            "Reaper" => {return Hero::Reaper;}
            "Hanzo" => {return Hero::Hanzo;}
            "Torbjörn" => {return Hero::Torbjorn;}
            "Widowmaker" => {return Hero::Widowmaker;}
            "Bastion" => {return Hero::Bastion;}
            "Symmetra" => {return Hero::Symmetra;}
            "Roadhog" => {return Hero::Roadhog;}
            "McCree" => {return Hero::McCree;}
            "Junkrat" => {return Hero::Junkrat;}
            "Zarya" => {return Hero::Zarya;}
            "Mei" => {return Hero::Mei;}
            "Sombra" => {return Hero::Sombra;}
            "Doomfist" => {return Hero::Doomfist;}
            "Ana" => {return Hero::Ana;}
            "Orisa" => {return Hero::Orisa;}
            "Moira" => {return Hero::Moira;}
            "Brigitte" => {return Hero::Brigitte;}
            "Ashe" => {return Hero::Ashe;}
            "Wrecking Ball" => {return Hero::Wrecking_Ball;}
	        "Baptiste" => {return Hero::Baptiste;}
            _ => {{return Hero::None;}}
        }
       /* match s{
            "009" => {return Hero::Winston;}
            "003" => {return Hero::Tracer;}
            "008" => {return Hero::Pharah;}
            "029" => {return Hero::Genji;}
            "020" => {return Hero::Zenyatta;}
            "007" => {return Hero::Reinhardt;}
            "004" => {return Hero::Mercy;}
            "079" => {return Hero::Lucio;}
            "06E" => {return Hero::Soldier;}
            "07A" => {return Hero::DVa;}
            "002" => {return Hero::Reaper;}
            "005" => {return Hero::Hanzo;}
            "006" => {return Hero::Torbjorn;}
            "00A" => {return Hero::Widowmaker;}
            "015" => {return Hero::Bastion;}
            "016" => {return Hero::Symmetra;}
            "040" => {return Hero::Roadhog;}
            "042" => {return Hero::McCree;}
            "065" => {return Hero::Junkrat;}
            "068" => {return Hero::Zarya;}
            "0DD" => {return Hero::Mei;}
            "12E" => {return Hero::Sombra;}
            "12F" => {return Hero::Doomfist;}
            "13B" => {return Hero::Ana;}
            "13E" => {return Hero::Orisa;}
            "1A2" => {return Hero::Moira;}
            "195" => {return Hero::Brigitte;}
            _ => {{return Hero::None;}}
        }
      */
    }
    fn name_eng(&self) -> String{
        match self{
             &Hero::Winston => {return String::from("Winston");}
             &Hero::Tracer => {return String::from("Tracer");}
             &Hero::Pharah => {return String::from("Pharah");}
             &Hero::Genji => {return String::from("Genji");}
             &Hero::Zenyatta => {return String::from("Zenyatta");}
             &Hero::Reinhardt => {return String::from("Reinhardt");}
             &Hero::Mercy => {return String::from("Mercy");}
             &Hero::Lucio => {return String::from("Lucio");}
             &Hero::Soldier => {return String::from("Soldier: 76");}
             &Hero::DVa => {return String::from("D.Va");}
             &Hero::Reaper => {return String::from("Reaper");}
             &Hero::Hanzo => {return String::from("Hanzo");}
             &Hero::Torbjorn => {return String::from("Torbjorn");}
             &Hero::Widowmaker => {return String::from("Widowmaker");}
             &Hero::Bastion => {return String::from("Bastion");}
             &Hero::Symmetra => {return String::from("Symmetra");}
             &Hero::Roadhog => {return String::from("Roadhog");}
             &Hero::McCree => {return String::from("McCree");}
             &Hero::Junkrat => {return String::from("Junkrat");}
             &Hero::Zarya => {return String::from("Zarya");}
             &Hero::Mei => {return String::from("Mei");}
             &Hero::Sombra => {return String::from("Sombra");}
             &Hero::Doomfist => {return String::from("Doomfist");}
             &Hero::Ana => {return String::from("Ana");}
             &Hero::Orisa => {return String::from("Orisa");}
             &Hero::Moira => {return String::from("Moira");}
             &Hero::Brigitte => {return String::from("Brigitte");}
             &Hero::Ashe => {return String::from("Ashe");}
             &Hero::Wrecking_Ball => {return String::from("Wrecking Ball");}
	         &Hero::Baptiste => {return String::from("Baptiste");}
             &Hero::None => {return String::new();}
        }
    }
    pub  fn name_rus(self) -> String{
        match self{
            Hero::Winston => {return String::from("Уинстон");}
            Hero::Tracer => {return String::from("Трейсер");}
            Hero::Pharah => {return String::from("Фарра");}
            Hero::Genji => {return String::from("Гэндзи");}
            Hero::Zenyatta => {return String::from("Дзенъятта");}
            Hero::Reinhardt => {return String::from("Райнхардт");}
            Hero::Mercy => {return String::from("Ангел");}
            Hero::Lucio => {return String::from("Лусио");}
            Hero::Soldier => {return String::from("Солдат-76");}
            Hero::DVa => {return String::from("D.Va");}
            Hero::Reaper => {return String::from("Жнец");}
            Hero::Hanzo => {return String::from("Хандзо");}
            Hero::Torbjorn => {return String::from("Торбьорн");}
            Hero::Widowmaker => {return String::from("Роковая вдова");}
            Hero::Bastion => {return String::from("Бастион");}
            Hero::Symmetra => {return String::from("Симметра");}
            Hero::Roadhog => {return String::from("Турбосвин");}
            Hero::McCree => {return String::from("Маккри");}
            Hero::Junkrat => {return String::from("Крысавчик");}
            Hero::Zarya => {return String::from("Заря");}
            Hero::Mei => {return String::from("Мэй");}
            Hero::Sombra => {return String::from("Сомбра");}
            Hero::Doomfist => {return String::from("Кулак Смерти");}
            Hero::Ana => {return String::from("Ана");}
            Hero::Orisa => {return String::from("Ориса");}
            Hero::Moira => {return String::from("Мойра");}
            Hero::Brigitte => {return String::from("Бригитта");}
            Hero::Ashe => {return String::from("Эш");}
            Hero::Wrecking_Ball => {return String::from("Таран");}
	        Hero::Baptiste => {return String::from("Батист");}
            Hero::None => {return String::new();}
        }
    }
}

#[derive(Default,Clone,Debug)]
pub struct BtagData {
    btag: String,
    reg: String,
    plat: String,
    rating: Rating,
    url: String,
    avatar_url: String,
    rank_url: String,
    heroes: Vec<HeroStats>,
}
impl BtagData{
   fn hero_data(&mut self, hero_stats: HeroStats){
        for hero in &mut self.heroes{
            if hero.hero == hero_stats.hero{
                if let Some(x) = hero_stats.time_played{
                    hero.time_played = Some(x);
                }
                if let Some(x) = hero_stats.games_won{
                    hero.games_won = Some(x);
                }
                if let Some(x) = hero_stats.win_perc{
                    hero.win_perc = Some(x);
                }
                if let Some(x) = hero_stats.aim{
                    hero.aim = Some(x);
                }
                if let Some(x) = hero_stats.kills_per_live{
                    hero.kills_per_live = Some(x);
                }
                if let Some(x) = hero_stats.best_multiple_kills{
                    hero.best_multiple_kills = Some(x);
                }
                if let Some(x) = hero_stats.obj_kills{
                    hero.obj_kills = Some(x);
                }

                return;
            }
        }
       self.heroes.push(hero_stats);
   }
}


#[derive(Debug)]
pub enum OwData{
    NotFound,
    ClosedProfile{
        btag: String,
        reg: String,
        plat: String,
        url: String,
        avatar_url: String,
    },
    Full(BtagData)
}

#[derive(Default,Debug,Clone)]
pub struct HeroInfoReq{
    num: i16,
    rating: bool,
    time_played: bool,
    games_won: bool,
    win_perc: bool,
    aim: bool,
    kills_per_live: bool,
    best_multiple_kills: bool,
    obj_kills: bool,
}
impl HeroInfoReq{
    pub fn empty() -> HeroInfoReq{
        HeroInfoReq{
            num: 0,
            rating: false,
            time_played: false,
            games_won: false,
            win_perc: false,
            aim: false,
            kills_per_live: false,
            best_multiple_kills: false,
            obj_kills: false,
        }
    }
}

#[derive(Clone,Debug)]
struct HeroStats{
    hero: Hero,
    time_played: Option<String>,
    games_won: Option<u32>,
    win_perc: Option<u16>,
    aim: Option<u16>,
    kills_per_live: Option<f32>,
    best_multiple_kills: Option<u32>,
    obj_kills: Option<u32>,
}
impl HeroStats{
    fn new(h: Hero) -> HeroStats{
        HeroStats{
            hero: h,
            time_played: None,
            games_won: None,
            win_perc: None,
            aim: None,
            kills_per_live: None,
            best_multiple_kills: None,
            obj_kills: None,
        }
    }
}

/*
pub fn load_btag_data(btag: String, reg: String, plat: String, req:HeroInfoReq) -> OwData //Проверка существования профиля и подгрузка рейтинга при наличии
{
    lazy_static! {
        static ref REG_AVATAR: Regex = Regex::new(r#"player-portrait"\s??src="(?P<url>[^"]+?)""#).expect("Regex avatar url error");
    }
    use std::time::SystemTime;
    use self::OwData::*;
    if btag.is_empty() || plat.is_empty(){
        return NotFound;
    }

    let sys_time_old = SystemTime::now();

    let use_new_net: bool = SWITCH_NET.load(Ordering::Relaxed);
    let mode_debug: bool = DEBUG.load(Ordering::Relaxed);


    if mode_debug{
        info!("Start: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
    }


    let mut result: Option<String> = None;
    let mut url = String::new();
        url = format!("https://playoverwatch.com/en-us/career/{}/{}", plat.to_lowercase(), btag.replace("#", "-"));

        match reqwest::get(&url){
            Ok(mut resp) => {

                match resp.text(){
                    Ok(text) =>{
                        result = Some(text);}
                    Err(e) => {
                        info!("[load_btag_data] Error while take body:\n{}", e);

                    }
                }
            }
            Err(e) => {
                info!("[load_btag_data] Error while get responce from url. Probaly wrong url:\n{}", e);
            }
        }



//    if mode_debug{
//        info!("Get respornse: {:?}",
//                 SystemTime::now().duration_since(sys_time_old).unwrap());
//    }

    if let Some(body) = result{
        if body.contains("h1 class=\"u-align-center\">Profile Not Found<") {
            return NotFound;
        }

        let mut b_data = BtagData::default();
        b_data.btag = btag;
        b_data.reg = reg;
        b_data.plat = plat;
        b_data.url = url.clone();
        b_data.avatar_url = String::new();


        if let Some(avatar) = REG_AVATAR.captures(&body){
            b_data.avatar_url = avatar.name("url").expect("Avatar url").as_str().to_string();
        }

//        if mode_debug{
//            info!("Get rating: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
//        }

        if body.contains("masthead-permission-level-text\">Private Profile<"){

            return ClosedProfile {
                btag: b_data.btag,
                reg: b_data.reg,
                plat: b_data.plat,
                url: b_data.url,
                avatar_url: b_data.avatar_url,
            };
        }
        else {
            if req.rating{
                let rating_patern = "class=\"u-align-center h5\">";
                match body.find(rating_patern){  //Ищем рейтинг
                    Some(start_pos) => {

                        let mut string = String::new();
                        let mut pos = start_pos+rating_patern.chars().count();
                        loop{

                            let c = body.index(pos..pos+1).chars().next().unwrap();
                            if c == '<'{
                                break;
                            }
                            else {
                                pos += 1;
                                string.push(c);
                                continue;
                            }
                        }
                        b_data.rating = match string.parse::<u16>(){

                            Ok(x) => {x}

                            Err(e) => {
                                info!("Error while parce rating:\n{}\n{}",string ,e);
                                0
                            }
                        };
                        let comp_rang_patern = "class=\"competitive-rank\"><img src=\"";
                        match body.find(comp_rang_patern){  //Ищем URL иконки рейтинга
                            Some(start_pos) => {
                                let mut string = String::new();
                                let mut pos = start_pos + comp_rang_patern.len();
                                loop{
                                    let c = body.index(pos..pos+1).chars().next().unwrap();

                                    if c == '\"'{
                                        break;
                                    }
                                    else {
                                        string.push(c);
                                        pos += 1;
                                        continue;
                                    }

                                }

                                b_data.rank_url = string;
                            }
                            None => {
                                b_data.rank_url = String::new();
                            }
                        }

                    }
                    None => {
                        b_data.rating = 0;
                        b_data.rank_url = String::new();
                    }
                }

            }
            let mut comp = String::new();
            let mut time_played = String::new();
            let mut games_won = String::new();
            let mut win_perc = String::new();
            let mut aim = String::new();
            let mut kills_per_live = String::new();
            let mut best_multiple_kills = String::new();
            let mut obj_kills = String::new();

            static COMP_STR: &str = "id=\"competitive\""; //начало комп раздела, конец раздела быстрой игры
            static TIME_PLAYED_STR: &str = "data-category-id=\"0x0860000000000021\""; //начало раздела времени в игре
            static GAMES_WON_STR: &str = "data-category-id=\"0x0860000000000039\""; //начало раздела выйграных матчей
            static AIM_STR: &str = "data-category-id=\"0x086000000000002F\""; //начало раздела меткости
            static WIN_PERC_STR: &str = "data-category-id=\"0x08600000000003D1\""; //начало раздела процента побед
            static KILLS_PER_LIVE_STR: &str = "data-category-id=\"0x08600000000003D2\""; //начало раздела убийств за одну жизнь
            static AIM_CRIT_STR: &str = "data-category-id=\"0x08600000000003E2\""; //начало раздела убийств за одну жизнь
            static BEST_MULTIPLE_KILLS_STR: &str = "data-category-id=\"0x0860000000000346\""; //начало раздела лучш. множ. убийств
            static OBJ_KILLS_STR: &str = "data-category-id=\"0x086000000000031C\""; //начало раздела убийств у объекта
            static ACHIVMENT_STR: &str = "id=\"achievements-section\""; //начало раздела ачивок, конец комп раздела

            if req.time_played || req.games_won || req.win_perc || req.aim
                || req.kills_per_live || req.best_multiple_kills || req.obj_kills {
                comp = cut_part_of_str(&body.to_string(), COMP_STR, ACHIVMENT_STR);
            }
            if req.time_played {
                time_played = cut_part_of_str(&comp, TIME_PLAYED_STR, GAMES_WON_STR);
                loop {
                    match find_next_hero(&time_played) {
                        (Hero::None, ..) => {break;}
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());
                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.time_played = Some(hdat);

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                time_played = next_data;
                            }
                        }
                    }
                }
            }
            if req.games_won {
                games_won = cut_part_of_str(&comp, GAMES_WON_STR, AIM_STR);

                loop {
                    match find_next_hero(&games_won) {
                        (Hero::None, ..) => { break; }
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());

                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.games_won = Some(hdat.parse::<u32>().expect("Err #13"));

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                games_won = next_data;
                            }
                        }
                    }
                }
            }
            if req.aim {
                aim = cut_part_of_str(&comp, AIM_STR, WIN_PERC_STR);
                loop {
                    match find_next_hero(&aim) {
                        (Hero::None, ..) => { break; }
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());

                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.aim = Some(hdat.trim_matches('%').parse::<u16>().expect("Err #15"));

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                aim = next_data;
                            }
                        }
                    }
                }
            }
            if req.win_perc {
                win_perc = cut_part_of_str(&comp, WIN_PERC_STR, KILLS_PER_LIVE_STR);

                loop {
                    match find_next_hero(&win_perc) {
                        (Hero::None, ..) => { break; }
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());

                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.win_perc = Some(hdat.trim_matches('%').parse::<u16>().expect("Err #14"));

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                win_perc = next_data;
                            }
                        }
                    }
                }
            }
            if req.kills_per_live {
                kills_per_live = cut_part_of_str(&comp,
                                                 KILLS_PER_LIVE_STR,
                                                 AIM_CRIT_STR);
                loop {
                    match find_next_hero(&kills_per_live) {
                        (Hero::None, ..) => { break; }
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());

                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.kills_per_live = Some(hdat.parse::<f32>().expect("Err #16"));

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                kills_per_live = next_data;
                            }
                        }
                    }
                }
            }
            if req.best_multiple_kills {
                best_multiple_kills = cut_part_of_str(&comp,
                                                      BEST_MULTIPLE_KILLS_STR,
                                                      OBJ_KILLS_STR);
                loop {
                    match find_next_hero(&best_multiple_kills) {
                        (Hero::None, ..) => { break; }
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());

                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.best_multiple_kills = Some(hdat.parse::<u32>().expect("Err #17"));

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                best_multiple_kills = next_data;
                            }
                        }
                    }
                }
            }
            if req.obj_kills {
                obj_kills = cut_part_of_str(&comp,
                                            OBJ_KILLS_STR,
                                            ACHIVMENT_STR);
                loop {
                    match find_next_hero(&obj_kills) {
                        (Hero::None, ..) => { break; }
                        (hero, hero_data, next_data) => {
                            let hdat = find_description(hero_data.as_str());

                            let mut hero_stats = HeroStats::new(hero);
                            hero_stats.obj_kills = Some(hdat.parse::<u32>().expect("Err #18"));

                            b_data.hero_data(hero_stats);

                            if next_data.is_empty() {
                                break;
                            } else {
                                obj_kills = next_data;
                            }
                        }
                    }
                }
            }

            if mode_debug {
                info!("End: {:?}", SystemTime::now().duration_since(sys_time_old).expect("Err #19"));
            }
            return Full(b_data);
        }

    }
    else{
        if mode_debug{
            info!("End None: {:?}", SystemTime::now().duration_since(sys_time_old).expect("Err #20"));
        }
        return NotFound;
    }

}
*/

fn find_description(string: &str) -> String //class="description">
{
    let description_patern = "class=\"ProgressBar-description\">";
    match string.find(description_patern){ //Ищем URL аватара
        Some(start_pos) => {
            let mut answer = String::new();
            let mut pos = start_pos + description_patern.len();
            loop{
                let c = string.index(pos..pos+1).chars().next().unwrap();

                if c == '<'{
                    break;
                }
                    else {
                        pos += 1;
                        answer.push(c);
                        continue;
                    }

            }
            answer
        }
        None => {
            String::new()
        }
    }
}

fn find_next_hero(string: &String) -> (Hero, String, String) //берёт целевую строку, возвращает имя героя + его блок + остаток
{
    //static hero_start: &str = "data-hero-guid=\"0x02E0000000000";
    static hero_start: &str = "data-hero=\"";
    let mut answer = (Hero::None, String::new(), String::new());


    let start = match string.find(hero_start) {
        Some(x) => {x+hero_start.len()}
        None => {return answer;}
    };
    let mut s_wo_start = "";
    let mut hero_str = "";
    unsafe {
        s_wo_start = string.slice_unchecked(start, string.len());
        hero_str = s_wo_start.slice_unchecked(0, s_wo_start.find('"').unwrap_or(1));
    }
    answer.0 = Hero::get_from_bliz_str(hero_str);
    match s_wo_start.find(hero_start) {
        Some(x) => {
            let (answer1, answer2) = s_wo_start.split_at(x-1);
            answer.1 = answer1.to_owned();
            answer.2 = answer2.to_owned();
        }
        None => {
            answer.1 =s_wo_start.to_owned();
        }
    };
    return answer;
}

fn cut_part_of_str(main: &String, wall_1: &str, wall_2: &str) -> String
{

    let start = match main.find(wall_1){
        Some(x) => {x}
        None => { return String::new();}
    };
    let end = match main.find(wall_2){
        Some(x) => {x+wall_2.len()}
        None => {return String::new();}
    };
    unsafe {

        if start > end {
            info!("errr start > end: {} {}",wall_1, wall_2);
            return String::new(); }
        let temp = main.slice_unchecked(start, end).to_owned();
        return temp;
    }

}
fn add_dsid_to_db(server: Server) //Добавление Нового профиля в БД
{
    for mut stmt in POOL.prepare(r"INSERT INTO dservers
                                       (dsid)
                                   VALUES
                                       (:dsid)").into_iter() {
        stmt.execute(params!{
                "dsid" => server.dsid,
            }).expect("[MySQL add_dsid_to_db error]");
    }
}

fn add_to_db(user: User) //Добавление Нового профиля в БД
{
    use mysql::Params;
    use mysql::Value;
	use std::collections::HashMap;
    use core::hash::BuildHasherDefault;
    use twox_hash::XxHash;

    let mut hash: HashMap<String, Value, BuildHasherDefault<XxHash>> = Default::default();
    hash.insert("did".to_string(), Value::UInt(user.did));
    hash.insert("name".to_string(), Value::Bytes(user.name.into_bytes()));
    hash.insert("disc".to_string(), Value::Bytes(user.disc.into_bytes()));
    hash.insert("btag".to_string(), Value::Bytes(user.btag.into_bytes()));
    hash.insert("rtg".to_string(), Value::Int(user.rtg.higest_rating() as i64));
    hash.insert("reg".to_string(), Value::Bytes(user.reg.into_bytes()));
    hash.insert("plat".to_string(), Value::Bytes(user.plat.into_bytes()));
    hash.insert("scrim_preset".to_string(), Value::Bytes(serde_json::to_vec(&user.scrim_preset).unwrap()));
    hash.insert("rtg_preset".to_string(), Value::Bytes(serde_json::to_vec(&user.rtg_preset).unwrap()));

    POOL.prep_exec(r"INSERT INTO users
                                       (did, name, disc, btag, rtg, reg, plat, scrim_preset, rtg_preset)
                                   VALUES
                                       (:did, :name, :disc, :btag, :rtg, :reg, :plat, :scrim_preset, :rtg_preset)",
                   Params::Named(hash)
    ).expect("[MySQL add_to_db error]");



//            stmt.execute(params!{
//                "did" => user.did,
//                "name" => &user.name,
//                "disc" => &user.disc,
//                "btag" => &user.btag,
//                "rtg" => &user.rtg,
//                "reg" => &user.reg,
//                "plat" => &user.plat,
//                "scrim_preset" => serde_json::to_string(&user.scrim_preset).unwrap(),
//                "rtg_preset" => serde_json::to_string(&user.rtg_preset).unwrap(),
//            }).expect("[MySQL add_to_db error]");


}

fn update_in_db(user: User) //Изменение профиля в БД
{

    use mysql::Params;
    use mysql::Value;
    use std::collections::HashMap;
    use core::hash::BuildHasherDefault;
    use twox_hash::XxHash;

    let mut hash: HashMap<String, Value, BuildHasherDefault<XxHash>> = Default::default();
    hash.insert("did".to_string(), Value::UInt(user.did));
    hash.insert("name".to_string(), Value::Bytes(user.name.into_bytes()));
    hash.insert("disc".to_string(), Value::Bytes(user.disc.into_bytes()));
    hash.insert("btag".to_string(), Value::Bytes(user.btag.into_bytes()));
    hash.insert("rtg".to_string(), Value::Int(user.rtg.have_rating() as i64));
    hash.insert("reg".to_string(), Value::Bytes(user.reg.into_bytes()));
    hash.insert("plat".to_string(), Value::Bytes(user.plat.into_bytes()));
    hash.insert("scrim_preset".to_string(), Value::Bytes(serde_json::to_vec(&user.scrim_preset).unwrap()));
    hash.insert("rtg_preset".to_string(), Value::Bytes(serde_json::to_vec(&user.rtg_preset).unwrap()));

    POOL.prep_exec(r"UPDATE
                                users
                            SET
                                name=:name, disc=:disc, btag=:btag, rtg=:rtg, reg=:reg, plat=:plat, scrim_preset=:scrim_preset, rtg_preset=:rtg_preset
                            WHERE
                                did=:did",
                   Params::Named(hash)
    ).expect("[MySQL add_to_db error]");

    /*
    for mut stmt in POOL.prepare(r"UPDATE users
                                    SET
                                       name=:name, disc=:disc, btag=:btag, rtg=:rtg, reg=:reg, plat=:plat, scrim_preset=:scrim_preset, rtg_preset=:rtg_preset
                                   WHERE
                                       did=:did").into_iter() {
        stmt.execute(params!{
                "name" => &user.name,
                "disc" => &user.disc,
                "btag" => &user.btag,
                "rtg" => &user.rtg,
                "reg" => &user.reg,
                "plat" => &user.plat,
                "scrim_preset" => serde_json::to_string(&user.scrim_preset).unwrap(),
                "rtg_preset" => serde_json::to_string(&user.rtg_preset).unwrap(),
                "did" => &user.did,
            }).expect("[MySQL update_in_db error]");
    }
    */



/*
    let call = format!("UPDATE users SET name='{}', disc='{}', btag='{}', rtg={}, reg='{}', plat='{}', scrim_preset='{}', rtg_preset='{}' WHERE did={}",
                       &user.name, &user.disc, &user.btag, &user.rtg, &user.reg, &user.plat, serde_json::to_string(&user.scrim_preset).unwrap(), serde_json::to_string(&user.rtg_preset).unwrap(), &user.did);
    let mut conn = POOL.get_conn().unwrap();
    let _ = conn.query(call);
    */
}
/*
//pub fn load_by_dsid(dsid: u64) -> Option<Server> //Получение сервера из базы по Discord server Id
//{
 //   let mut conn = POOL.get_conn().unwrap();
 //   let command = format!("SELECT dsid FROM servers WHERE dsid = {}", &dsid);
 //   let mut stmt = conn.prepare(command).unwrap();
 //   let mut server:Option<Server> = None;

 //   for row in stmt.execute(()).unwrap() {
 //       let dsid = mysql::from_row::<(u64)>(row.unwrap());
 //       let mut s = Server::empty();
 //       s.dsid = dsid;
 //       server = Some(s);
 //   }
 //   return server;
//}
*/
pub fn load_by_id(id: u64) -> Option<User> //Получение профиля из базы по DiscordId
{

        POOL.prep_exec("SELECT did, name, disc, btag, rtg, reg, plat, scrim_preset, rtg_preset FROM users WHERE did = :a", params!{"a" => id})
            .map(|result| {
                result.map(|x| x.expect("load_by_id MySQL reqwest error")).map(|row| {

                    let (udid, uname, udisc, ubtag, urtg, ureg,
                        uplat, scrim_preset, rtg_preset) = mysql::from_row::<
                        (u64, String, String, String, u16, String, String, String, String)>(row);
                    let mut u = User::empty();
                    u.did = udid;
                    u.name = uname;
                    u.disc = udisc;
                    u.btag = ubtag;
                    u.rtg = Rating::from1(urtg);
                    u.reg = ureg;
                    u.plat = uplat;
                    u.scrim_preset = serde_json::from_str(&scrim_preset).unwrap();
                    u.rtg_preset = serde_json::from_str(&rtg_preset).unwrap();
                    u
                }).next()
            }).unwrap()



/*
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("SELECT did, name, disc, btag, rtg, reg, plat, scrim_preset, rtg_preset FROM users WHERE did = {}", &id);
    let mut stmt = conn.prepare(command).unwrap();
    let mut user:Option<User> = None;

    for row in stmt.execute(()).unwrap() {
        let (udid, uname, udisc, ubtag, urtg, ureg,
            uplat, scrim_preset, rtg_preset) = mysql::from_row::<
            (u64, String, String, String, u16, String, String, String, String)>(row.unwrap());
        let mut u = User::empty();
        u.did = udid;
        u.name = uname;
        u.disc = udisc;
        u.btag = ubtag;
        u.rtg = urtg;
        u.reg = ureg;
        u.plat = uplat;
        u.scrim_preset = serde_json::from_str(&scrim_preset).unwrap();
        u.rtg_preset = serde_json::from_str(&rtg_preset).unwrap();
        user = Some(u);
    }
    return user;
    */
}

pub fn load_settings() -> String //Загрузка DiscordId
{
    let mut ta = String::new();
    let mut stmt = POOL.prepare("SELECT distoken FROM bottoken").expect("Error while prepare POOL in load_settings()");
    for row in stmt.execute(()).unwrap() {
        ta = from_row::<String>(row.unwrap());
    }
    return ta;
}


fn user_exist(id: u64) -> bool //Проверка существования профиля в базе
{
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("SELECT EXISTS (SELECT * FROM users WHERE did = {})", &id);
    let mut stmt = conn.prepare(command).unwrap();
    let mut answer: bool = false;

    for row in stmt.execute(()).unwrap() {
        answer = mysql::from_row::<bool>(row.unwrap());
    }


    return answer;
}

fn delete_user(id: u64) //Удаление рпофиля (пока только для тестов)
{
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("DELETE FROM users WHERE did = {}", &id);
    let mut stmt = conn.prepare(command).unwrap();
    //let mut answer: bool = false;
    let _ = stmt.execute(());
}

fn reg_check(id: u64) -> bool //Проверка наличия профиля и BattleTag у профиля в БД
{
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("SELECT EXISTS (SELECT * FROM users WHERE did = {})", &id);
    let mut stmt = conn.prepare(command).unwrap();
    let mut exist: bool = false;


    for row in stmt.execute(()).unwrap() {
        exist = mysql::from_row::<bool>(row.unwrap());
    }

    return exist;
}

fn reg_user(mut reg_str: Vec<&str>, autor: DUser, chan: ChannelId, cache: impl AsRef<Http>) //Диалог создания профиля
{
    let err_color: u64 = 13369344;
    let err_title = ":no_entry: Упс...";
    let color: u64 = 37595;
    let thumbnail_ws = "http://winspirit.org/sites/default/files/full-quad-200px.png";

    let mut title = "";
    let mut des = "";
    let mut thumbnail:String = String::new();
    let mut footer = "";
    let mut fields: Vec<(String, String, bool)> = Vec::new();


    let mut battletag: String = String::new();
    let mut region: String = String::new();
    let mut platform: String = String::new();
    let mut rating = Rating::empty();
    let mut unnone = false;
    let mut botmess: String = String::new();
    let mut roleruler = String::new();

    if reg_str.capacity() > 1 {
        reg_str.remove(0);

        for s in reg_str {
            match s.to_uppercase().as_str() {
                "KR" | "EU" | "US" => {
                    region = s.to_uppercase();
                }
                "PC" | "P4" | "XB" => {
                    platform = s.to_uppercase();
                }
                _ => {
                    if REG_BTAG.is_match(s) {
                        battletag = s.to_string();
                    } else { unnone = true; }
                }
            }
        }
        let no_btag = battletag.is_empty();
        let no_plat: bool = match platform.is_empty() {
            true => {
                platform = "PC".to_string();
                true
            }
            _ => { false }
        };
        let no_reg: bool = match region.is_empty() {
            true => {
                region = "EU".to_string();
                true
            }
            _ => { false }
        };

        let mut acc_not_found = false;

        let mut req = HeroInfoReq::default();
        req.num = 0;
        req.rating = true;
        req.time_played = false;
        req.games_won = false;
        req.aim = false;
        req.kills_per_live = false;
        req.win_perc = false;
        req.best_multiple_kills = false;
        req.obj_kills = false;

        let answer = load_btag_data_multirole(battletag.to_string(), region.to_string(), platform.to_string(), req);

        if !no_btag {

            match answer{
                OwData::NotFound => {
                    acc_not_found = true;
//                    rating = 0;
                }
                OwData::ClosedProfile { ref avatar_url, ..
                } => {
//                    rating = 0;
                    thumbnail = avatar_url.clone();
                },
                OwData::Full(ref BData) => {
                    rating = BData.rating.clone();
                    //info!("rating: {}", rating);
                    thumbnail = BData.avatar_url.clone();


                    let server_id = if let Ok(channel) = cache.as_ref().get_channel(chan.0){
                        match channel.guild() {
                            Some(guild_lock) => {
                                guild_lock.read().guild_id.0
                            },
                            None => { 0u64 }
                        }
                    }
                    else { 0u64 };
//                    let server_id = match Discord::get_chanel(chan){
//                        None => {0}
//                        Some(json) => {
//                            match json.get("guild_id"){
//                                None => {0}
//                                Some(jtext) => {
//                                    jtext.as_str().unwrap_or("0").parse::<u64>().unwrap_or(0)
//                                }
//                            }
//                        }
//                    };

                    roleruler = role_ruler_text(&cache,
                                                server_id,
                                                autor.id,
                                                RoleR::rating(rating.higest_rating()));
                }
            }
        }

        title = "Регистрация пройдена";


        if no_btag || no_plat || no_reg {

            des = "Не хватает некоторых данных, но ничего страшного :worried:";

            if no_btag {fields.push(("BattleTag".to_string(),"Не указан".to_string(),false))}
                else { fields.push(("BattleTag".to_string(), battletag.clone(), false)) }
            if no_reg {fields.push(("Регион".to_string(),"По умолчанию (EU)".to_string(),false))}
                else { fields.push(("Регион".to_string(), region.clone(), false)) }
            if no_plat {fields.push(("Платформа".to_string(),"По умолчанию (PC)".to_string(),false))}
                else { fields.push(("Платформа".to_string(), platform.clone(), false)) }



            match answer{
                OwData::NotFound => {
                    des = "Мы не смогли найти ваш профиль Overwtach по заданным параметрам.\nВозможно вы ошиблись или указали недостаточно данных.";
                    footer = "Вы можете указать корректные данные позже с помощью комманды !wsreg";
                }
                OwData::ClosedProfile {
                    ..
                } => {
                    des = "Мы нашли ваш профиль, но он скрыт и мы не сможем находить ваш рейтинг.\nЧто бы бот мог видеть ваш рейтинг вым надо открыть свой профиль.";
                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";
                },
                OwData::Full(_) => {
                    if rating.have_rating() {
//                        fields.push(("Рейтинг".to_string(), rating.as_str(), false));
                        fields.append(rating.as_fields().as_mut());
                    }

                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";
                }
            }




//            if acc_not_found {
//                des = "Мы не смогли найти ваш профиль Overwtach по заданным параметрам.\nВозможно вы ошиблись или указали недостаточно данных.";
//                footer = "Вы можете указать корректные данные позже с помощью комманды !wsreg";
//            } else {
//                if rating > 0 {fields.push(("Рейтинг".to_string(), format!("{}",rating), false));}
//
//                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";
//
//
//            }
        }
        else {

            match answer{
                OwData::NotFound => {
                    des = "Похоже мы не смогли найти ваш профиль Overwtach по заданным параметрам. \nВозможно вы ошиблись или указали недостаточно данных.";
                    fields.push(("BattleTag".to_string(), battletag.clone(), false));
                    fields.push(("Регион".to_string(), region.clone(), false));
                    fields.push(("Платформа".to_string(), platform.clone(), false));
                    footer ="Вы можете добавить их позже с помощью комманды !wsreg";
                }
                OwData::ClosedProfile {
                    ..
                } => {
                    des = "Мы нашли ваш профиль, но он скрыт и мы не сможем находить ваш рейтинг.\nЧто бы бот мог видеть ваш рейтинг вым надо открыть свой профиль.";
                    fields.push(("BattleTag".to_string(), battletag.clone(), false));
                    fields.push(("Регион".to_string(), region.clone(), false));
                    fields.push(("Платформа".to_string(), platform.clone(), false));
                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";
                },
                OwData::Full(_) => {
                    des = "Информация успешно добавлена :wink:";
                    fields.push(("BattleTag".to_string(), battletag.clone(), false));
                    fields.push(("Регион".to_string(), region.clone(), false));
                    fields.push(("Платформа".to_string(), platform.clone(), false));
                    if rating.have_rating() {
//                        fields.push(("Рейтинг".to_string(), rating.as_str(), false))
                        fields.append(rating.as_fields().as_mut());
                    }

                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";
                }
            }
//            if acc_not_found {
//                des = "Похоже мы не смогли найти ваш профиль Overwtach по заданным параметрам. \nВозможно вы ошиблись или указали недостаточно данных.";
//                fields.push(("BattleTag".to_string(), battletag.clone(), false));
//                fields.push(("Регион".to_string(), region.clone(), false));
//                fields.push(("Платформа".to_string(), platform.clone(), false));
//                footer ="Вы можете добавить их позже с помощью комманды !wsreg";
//            } else {
//                des = "Информация успешно добавлена :wink:";
//                fields.push(("BattleTag".to_string(), battletag.clone(), false));
//                fields.push(("Регион".to_string(), region.clone(), false));
//                fields.push(("Платформа".to_string(), platform.clone(), false));
//                if rating > 0 {fields.push(("Рейтинг".to_string(), format!("{}",rating), false))}
//
//                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";
//
//            }
        }

        let mut temp_user = User::empty();
        match answer{
            OwData::NotFound => {
                temp_user.did = autor.id;
                temp_user.name = autor.username;
                temp_user.disc = autor.discriminator;
                add_to_db(temp_user);
                }
            OwData::ClosedProfile {
                ref avatar_url, ref btag, ref reg, ref plat, ..
            } => {
                temp_user.did = autor.id;
                temp_user.name = autor.username;
                temp_user.disc = autor.discriminator;
                temp_user.btag = btag.clone();
                temp_user.rtg = rating;
                temp_user.reg = reg.clone();
                temp_user.plat = plat.clone();
                add_to_db(temp_user);
                },
            OwData::Full(ref BData) => {
                temp_user.did = autor.id;
                temp_user.name = autor.username;
                temp_user.disc = autor.discriminator;
                temp_user.btag = BData.btag.clone();
                temp_user.rtg = rating;
                temp_user.reg = BData.reg.clone();
                temp_user.plat = BData.plat.clone();
                add_to_db(temp_user);
               }
        }


//        if acc_not_found {
//            let mut temp_user = User::empty();
//            temp_user.did = autor.id;
//            temp_user.name = autor.username;
//            temp_user.disc = autor.discriminator;
//            add_to_db(temp_user);
//        } else {
//            let mut temp_user = User::empty();
//            temp_user.did = autor.id;
//            temp_user.name = autor.username;
//            temp_user.disc = autor.discriminator;
//            temp_user.btag = battletag.to_string();
//            temp_user.rtg = rating;
//            temp_user.reg = region.to_string();
//            temp_user.plat = platform.to_string();
//            add_to_db(temp_user);
//        }
    } else {
        title = "Регистрация пройдена";
        des ="Но вы не указали никакой информации. Совсем :worried:";
        footer ="Вы можете добавить её позже с помощью комманды !wsreg";
        let mut temp_user = User::empty();
        temp_user.did = autor.id;
        temp_user.name = autor.username;
        temp_user.disc = autor.discriminator;

        add_to_db(temp_user);
    }
    if roleruler.is_empty(){
        roleruler = footer.to_string();
    }

	EmbedStruct::empty()
		.title(&title)
		.des(&des)
		.thumbnail(&thumbnail)
		.col(color)
		.footer((String::new(),&roleruler))
		.fields(fields)
		.send(cache, chan);
}

fn edit_user(mut reg_str: Vec<&str>, autor: DUser,chan: ChannelId, cache: impl AsRef<Http>) //Диалог на запрос редактирования профиля
{
    let mut battletag: String = String::new();
    let mut region: String = String::new();
    let mut platform: String = String::new();
    let user = load_by_id(autor.id).unwrap();
    let mut rating = Rating::empty();
    let mut unnone = false;
    let mut botmess: String = String::new();
    let mut force: bool = false;

    let mut title = "";
    let mut des = "";
    let mut thumbnail:String = String::new();
    let mut footer = "";
    let mut fields: Vec<(String, String, bool)> = Vec::new();
    let color: u64 = 37595;
    let mut roleruler = String::new();

    if reg_str.capacity() > 1 {
        reg_str.remove(0);

        for s in reg_str {
            match s.to_uppercase().as_str() {
                "KR" | "EU" | "US" => {
                    region = s.to_uppercase();
                }
                "PC" | "P4" | "XB" => {
                    platform = s.to_uppercase();
                }
                "FORCE" => {
                    force = true;
                }
                _ => {
                    if REG_BTAG.is_match(s) {
                        battletag = s.to_string();
                    } else { unnone = true; }
                }
            }
        }
        let no_btag = match (battletag.is_empty(), user.btag.is_empty()) {
            (true, true) => { true }
            (false, _) => { false }
            (true, false) => {
                battletag = user.btag.clone();
                true
            }
            _ => { false }
        };

        let no_plat: bool = match (platform.is_empty(), user.plat.is_empty()) {
            (true, true) => {
                platform = "PC".to_string();
                true
            }
            (false, _) => { false }
            (true, false) => {
                platform = user.plat.clone();
                true
            }
            _ => { false }
        };

        let no_reg: bool = match (region.is_empty(), user.reg.is_empty()) {
            (true, true) => {
                region = "EU".to_string();
                true
            }
            (false, _) => { false }
            (true, false) => {
                region = user.reg.clone();
                true
            }
            _ => { false }
        };

        let mut acc_not_found = false;
        let mut new_data = false;
        if no_btag && no_plat && no_reg {
            unnone = false;
            title = ":rolling_eyes: Упс..";
            des = "К сожалению, не удалось определить праметры заданные вами";
        } else {
            if !no_btag && !battletag.eq(&user.btag) { new_data = true; }
            if !no_plat && !platform.eq(&user.plat) { new_data = true; }
            if !no_reg && !region.eq(&user.reg) { new_data = true; }
            if new_data {
                if battletag.is_empty() {
                    let mut temp_user = User::empty();
                    temp_user.did = autor.id;
                    temp_user.name = autor.username;
                    temp_user.disc = autor.discriminator;
                    temp_user.btag = battletag.clone();
                    temp_user.rtg = Rating::empty();
                    temp_user.reg = region.clone();
                    temp_user.plat = platform.clone();
                    temp_user.scrim_preset = user.scrim_preset;
                    temp_user.rtg_preset = user.rtg_preset;

                    update_in_db(temp_user);

                    fields.push(("Мы обновили ваши данные".to_string(),
                                 "Но мы не сможем узнать ваш рейтинг без указания BattleTag\nУбедитесь, что верно ввели парамтры на изменение ваших данных".to_string(),
                                 false));
                } else {
                    let mut req = HeroInfoReq::default();
                    req.num = 0;
                    req.rating = true;
                    req.time_played = false;
                    req.games_won = false;
                    req.aim = false;
                    req.kills_per_live = false;
                    req.win_perc = false;
                    req.best_multiple_kills = false;
                    req.obj_kills = false;
                    let answer = load_btag_data_multirole(battletag.to_string(), region.to_string(), platform.to_string(), req);

                    match answer{
                        OwData::NotFound => {
                            acc_not_found = true;
//                            rating = 0;
                        }
                        OwData::ClosedProfile {
                            ref avatar_url, ..
                        } => {
                            thumbnail = avatar_url.clone();
//                            rating = 0;
                        },
                        OwData::Full(ref BData) => {

                            rating = BData.rating.clone();
                            //info!("rating: {}", rating);
                            thumbnail = BData.avatar_url.clone();
                            use serenity::model::channel::Channel;
                            let server_id = match cache.as_ref().get_channel(chan.0){
                                Ok(channel_enum) => {
                                    match channel_enum {
                                        Channel::Guild(c) =>{
                                            c.read().guild_id.0
                                        }
                                        _ => {0}
                                    }
                                }
                                _ => {0}
                            };

//                            let server_id = match Discord::get_chanel(chan){
//                                None => {0}
//                                Some(json) => {
//                                    match json.get("guild_id"){
//                                        None => {0}
//                                        Some(jtext) => {
//                                            jtext.as_str().unwrap_or("0").parse::<u64>().unwrap_or(0)
//                                        }
//                                    }
//                                }
//                            };
                            roleruler = role_ruler_text(&cache,
                                                        server_id,
                                                        autor.id,
                                                        RoleR::rating(rating.higest_rating()));
                        }
                    }


/*                    if let Some(an) = answer{
//                        rating = an.rating;
//                        //info!("rating: {}", rating);
//                        thumbnail = an.avatar_url.clone();
//                        let server_id = match Discord::get_chanel(chan){
//                            None => {0}
//                            Some(json) => {
//                                match json.get("guild_id"){
//                                    None => {0}
//                                    Some(jtext) => {
//                                        jtext.as_str().unwrap_or("0").parse::<u64>().unwrap_or(0)
//                                    }
//                                }
//                            }
//                        };
//                        roleruler = role_ruler_text(server_id,
//                                   autor.id,
//                                   RoleR::rating(rating));
//                    }
//                        else {
//                            acc_not_found = true;
//                            rating = 0;
//                        }
*/



                    match answer{
                        OwData::NotFound => {
                            if force {
                                let mut temp_user = User::empty();
                                temp_user.did = autor.id;
                                temp_user.name = autor.username;
                                temp_user.disc = autor.discriminator;
                                temp_user.btag = battletag.clone();
                                temp_user.rtg = Rating::empty();
                                temp_user.reg = region.clone();
                                temp_user.plat = platform.clone();
                                temp_user.scrim_preset = user.scrim_preset;
                                temp_user.rtg_preset = user.rtg_preset;

                                update_in_db(temp_user);
                                title = "Данные обновлены";
                                des = "Мы принудително обновили ваши данные";

                                fields.push(("BattleTag".to_string(), battletag.clone(), false));
                                fields.push(("Регион".to_string(), region.clone(), false));
                                fields.push(("Платформа".to_string(),
                                             format!("{} \n\nУбедитесь, что верно ввели парамтры на изменение ваших данных",platform.clone()), false));


                            } else {
                                title = "Изменение данных";
                                des = "Мы не смогли найти ваш профиль Overwatch по заданным параметрам";
                                fields.push(("BattleTag".to_string(), battletag.clone(), false));
                                fields.push(("Регион".to_string(), region.clone(), false));
                                fields.push(("Платформа".to_string(),
                                             format!("{} \n\nУбедитесь, что верно ввели парамтры на изменение ваших данных\nНо если вы настаиваете, то добавте FORCE в конец, для изменения данных",platform.clone()),
                                             false));

                            }
                        }
                        OwData::ClosedProfile {..} => {
                            let mut temp_user = User::empty();
                            temp_user.did = autor.id;
                            temp_user.name = autor.username;
                            temp_user.disc = autor.discriminator;
                            temp_user.btag = battletag.clone();
                            temp_user.rtg = Rating::empty();
                            temp_user.reg = region.clone();
                            temp_user.plat = platform.clone();
                            temp_user.scrim_preset = user.scrim_preset;
                            temp_user.rtg_preset = user.rtg_preset;

                            update_in_db(temp_user);
                            title = "Данные обновлены";
                            des = "Профиль OW скрыт";
                            fields.push(("BattleTag".to_string(), battletag.clone(), false));
                            fields.push(("Регион".to_string(), region.clone(), false));
                            fields.push(("Платформа".to_string(), platform.clone(), false));

                            footer = "Мы не можем выдеть ваш ретинг, когда профиль скрыт";
                        },
                        OwData::Full(_) => {
                            let mut temp_user = User::empty();
                            temp_user.did = autor.id;
                            temp_user.name = autor.username;
                            temp_user.disc = autor.discriminator;
                            temp_user.btag = battletag.clone();
                            temp_user.rtg = rating.clone();
                            temp_user.reg = region.clone();
                            temp_user.plat = platform.clone();
                            temp_user.scrim_preset = user.scrim_preset;
                            temp_user.rtg_preset = user.rtg_preset;

                            update_in_db(temp_user);
                            title = "Данные обновлены";
                            des = "Профиль OW найден";
                            fields.push(("BattleTag".to_string(), battletag.clone(), false));
                            fields.push(("Регион".to_string(), region.clone(), false));
                            fields.push(("Платформа".to_string(), platform.clone(), false));
                            if rating.have_rating() {
//                                fields.push(("Рейтинг".to_string(), rating.as_str(), false))
                                fields.append(rating.as_fields().as_mut());
                            }

                            footer = "Убедитесь, что верно ввели парамтры на изменение ваших данных";

                        }
                    }
                }
            } else {
                title = "Изменение данных";
                des = "Ваши текущие данные совпадают с введёнными";
                fields.push(("BattleTag".to_string(), battletag.clone(), false));
                fields.push(("Регион".to_string(), region.clone(), false));
                fields.push(("Платформа".to_string(), platform.clone(), false));
                if rating.have_rating() {
//                    fields.push(("Рейтинг".to_string(), rating.as_str(), false))
                    fields.append(rating.as_fields().as_mut());
                }



            }
        }

    } else {
        title = "Вы уже зарегестрированны";
        des = "Что бы добаваить или изменить данные о вашем профиле, укажите их вместе с командой !wsreg";
    }
    if footer.is_empty(){footer = "!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}";}

    if roleruler.is_empty(){
        roleruler = footer.to_string();
    }

	EmbedStruct::empty()
		.title(&title)
		.des(&des)
		.thumbnail(&thumbnail)
		.col(color)
		.footer((String::new(),&roleruler))
		.fields(fields)
		.send(&cache,chan);
}


fn insert(name: &str, var: &String) {
    let mut call = format!("INSERT INTO variables (name,var) VALUES(");
    call = format!("{}'{}'", call, name);
    call = format!("{},'{}'", call, var);
    call = format!("{}) ON DUPLICATE KEY UPDATE", call);
    call = format!("{} var='{}'", call, var);
    let mut conn = POOL.get_conn().unwrap();
    let _ = conn.query(call);
}

fn get_db(name: &str) -> String {
    let mut string = String::new();
    let mut call = format!("SELECT var FROM variables WHERE");
    call = format!("{} name='{}'", call, name);
    let mut stmt = POOL.prepare(call.as_str()).unwrap();
    for row in stmt.execute(()).unwrap() {
        string = from_row::<String>(row.unwrap());
    }
    return string;
}


fn get_arg_from_mes(mut reg_str: Vec<&str>) -> User{
    let mut u = User::empty();
    if reg_str.capacity() > 1 {
        reg_str.remove(0);
        for s in reg_str {
            match s.to_uppercase().as_str() {
                "KR" | "EU" | "US" => {
                    u.reg = s.to_uppercase();
                }
                "PC" | "P4" | "XB" => {
                    u.plat = s.to_uppercase();
                }
                _ => {
                    if REG_BTAG.is_match(s) {
                        u.btag = s.to_string();
                    } else {  }
                }
            }
        }


    }
    return u;
}

fn wsstats(mes: Vec<&str>, autor_id: u64, chanel: ChannelId, cache: impl AsRef<Http>){

    let mut err_end = false;

    let err_color: u64 = 13369344;
    let err_title = ":no_entry: Упс...";
    let color: u64 = 37595;
    let rating_field_name = "Рейтинг";
    let hero_list_titles = vec!["Обычно","Но в душе","Иногда","Реже","И в крайнем случае"];

    let mut u = User::empty();
    let mut m = User::empty();
    let mut update = false;

    let mut req = HeroInfoReq::default();
    req.num = 5;
    req.rating = true;
    req.time_played = true;
    req.games_won = true;
    req.aim = false;
    req.kills_per_live = false;
    req.win_perc = true;
    req.best_multiple_kills = false;
    req.obj_kills = false;

    let mut roleruler = String::new();

    if mes.capacity() > 1 {
        m = get_arg_from_mes(mes);
        u = match load_by_id(autor_id) {
            Some(u) => { u }
            _ => { User::empty() }
        };
        if u.btag.is_empty() && m.btag.is_empty() {

            //Ошибка: Вы не указали BTag при регистрации и в сообщении.
            let botmess = "Вы не указали BTag при регистрации и в сообщении";
	        EmbedStruct::empty()
		        .title(err_title)
		        .des(botmess)
		        .col(err_color)
		        .send(&cache, chanel);
	        err_end = true;

        } else if u.btag == m.btag {
            if m.plat.is_empty() { m.plat = "PC".to_string(); }
            if m.reg.is_empty() { m.reg = "EU".to_string(); }
            if u.plat == m.plat && u.reg == m.reg { update = true; } else { u = m; }
        } else {
            if m.btag.is_empty() && m.plat.is_empty() && m.reg.is_empty() {

                //Ошибка: Параметры не распознаны.
                let botmess = "Параметры не распознаны";
	            EmbedStruct::empty()
		            .title(err_title)
		            .des(botmess)
		            .col(err_color)
		            .send(&cache, chanel);
                err_end = true;

            } else {
                if !m.btag.is_empty() { u.btag = m.btag; }
                if !m.plat.is_empty() { u.plat = m.plat; }
                if !m.reg.is_empty() { u.reg = m.reg; }
            }
        }
    } else {
        match reg_check(autor_id) {
            false => {

                //Ошибка: Вы не зарегестрированы и не указали BTag в сообщении.
                let botmess = "Вы не зарегестрированы и не указали BTag в сообщении";
	            EmbedStruct::empty()
		            .title(err_title)
		            .des(botmess)
		            .col(err_color)
		            .send(&cache, chanel);
                err_end = true;
            }
            true => {

                u = match load_by_id(autor_id) {
                    Some(u) => { u }
                    _ => { User::empty() }
                };
                if u.btag.is_empty() {

                    //Ошибка: Вы не указали BTag при регистрации и в сообщении.
                    let botmess = "Вы не указали BTag при регистрации и в сообщении";
	                EmbedStruct::empty()
		                .title(err_title)
		                .des(botmess)
		                .col(err_color)
		                .send(&cache, chanel);
                    err_end = true;

                } else { update = true; }

            }
        }
    }
    if !err_end {
        if u.plat.is_empty() { u.plat = "PC".to_string(); }
        if u.reg.is_empty() { u.reg = "EU".to_string(); }

        let answer = load_btag_data_multirole(u.btag.to_string(), u.reg.to_string(), u.plat.to_string(), req);
        match answer{
            OwData::NotFound => {
                u.rtg = Rating::empty();
            }
            OwData::ClosedProfile {
                ..
            } => {
                u.rtg = Rating::empty();
            },
            OwData::Full(ref BData) => {
                u.rtg = BData.rating.clone();
            }
        }
        if update {

            use serenity::model::channel::Channel;
            let server_id = match cache.as_ref().get_channel(chanel.0){
                Ok(channel_enum) => {
                    match channel_enum {
                        Channel::Guild(c) =>{
                            c.read().guild_id.0
                        }
                        _ => {0}
                    }
                }
                _ => {0}
            };

//            let server_id = match Discord::get_chanel(chanel){
//                None => {0}
//                Some(json) => {
//                    match json.get("guild_id"){
//                        None => {0}
//                        Some(jtext) => {
//                            jtext.as_str().unwrap_or("0").parse::<u64>().unwrap_or(0)
//                        }
//                    }
//                }
//            };
            roleruler = role_ruler_text(&cache, server_id,
                       autor_id,
                       RoleR::rating(u.rtg.higest_rating()));
            update_in_db(u.clone());
        }

        match answer{
            OwData::NotFound => {
                //Ошибка: Такой игрок не найден.
                let botmess = "Такой игрок не найден";
                EmbedStruct::empty()
                    .title(err_title)
                    .des(botmess)
                    .col(err_color)
                    .send(&cache, chanel);
            }
            OwData::ClosedProfile {
                btag, reg, plat, url, avatar_url
            } => {
                let botmess = format!("{} {} {} Профиль скрыт", u.btag, u.reg, u.plat);
                let des = format!("[Ссылка на профиль]({})", url);
                EmbedStruct::empty()
                    .title(&botmess)
                    .des(&des)
                    .col(color)
                    .thumbnail(&avatar_url.clone())
                    .footer((String::new(),&roleruler))
                    .send(&cache, chanel);
            },
            OwData::Full(BData) => {
                if !u.rtg.have_rating() {
                    //Рейтинг отсутствует
                    let botmess = format!("{} {} {}", u.btag, u.reg, u.plat);
                    let des = format!("[Ссылка на профиль]({})", BData.url);
                    EmbedStruct::empty()
                        .title(&botmess)
                        .des(&des)
                        .col(color)
                        .thumbnail(&BData.avatar_url.clone())
                        .footer((String::new(),&roleruler))
                        .send(&cache, chanel);
                }
                else {
                    let botmess = format!("{} {} {}", u.btag, u.reg, u.plat);
                    let des = format!("[Ссылка на профиль]({})", BData.url);

                    let mut fields_vec: Vec<(String, String , bool)> = BData.rating.as_fields();
                    if hero_list_titles.len()>BData.heroes.len(){}
                    else{
                        for (enumerat,l) in hero_list_titles.iter().enumerate(){

                            let ref an = BData.heroes[enumerat];
                            let mut itre = an.clone().hero.name_rus();
                            let name = format!("{} {}",l,itre);

                            let mut value = String::new();
                            let mut f = true;

                            value = format!("[{}]",an.clone().time_played.unwrap_or(String::new()));


                            if let Some(x)= an.win_perc{
                                if !f{value = format!("{},",value)}
                                else{f=false}

                                value = format!("{} {}% побед",value,x);
                            }

                            if let Some(x)= an.games_won{
                                if !f{value = format!("{},",value)}
                                else{f=false}

                                value = format!("{} {} побед(а)",value,x);
                            }

                            fields_vec.push((name, value, false));
                        }
                    }

                    EmbedStruct::empty()
                        .title(&botmess)
                        .des(&des)
                        .thumbnail(&BData.avatar_url.clone())
                        .col(color)
                        .footer((String::new(),&roleruler))
                        .fields(fields_vec)
                        .send(&cache, chanel);

                }

            }
        }
    }
}

struct EmbedStruct<'a>{
    text: &'a str,
    title: &'a str,
    des: &'a str,
    thumbnail: String,
    col: u64,
    footer: (String, &'a str),
    fields: Vec<(String, String , bool)>,
    author: (&'a str,&'a str,&'a str),
    url: String,
    image: String,
}
impl<'a> EmbedStruct<'a> {
    fn empty() -> EmbedStruct<'a> {
        EmbedStruct{
            text: "",
            title: "",
            des: "",
            thumbnail: String::new(),
            col: 0,
            footer: (String::new(), ""),
            fields: Vec::new(),
            author: ("","",""),
            url: String::new(),
            image: String::new(),
        }
    }
	fn text(self, text: &'a str) -> Self{
		let mut s = self;
		s.text = text;
		s
	}
	fn title(self, title: &'a str) -> Self{
		let mut s = self;
		s.title = title;
		s
	}
	fn des(self, des: &'a str) -> Self{
		let mut s = self;
		s.des = des;
		s
	}
	fn thumbnail(self, thumbnail: &str) -> Self{
		let mut s = self;
		s.thumbnail = thumbnail.into();
		s
	}
	fn col(self, col: u64) -> Self{
		let mut s = self;
		s.col = col;
		s
	}
	fn footer(self, footer: (String, &'a str)) -> Self{
		let mut s = self;
		s.footer = footer;
		s
	}
	fn fields(self, fields: Vec<(String, String , bool)>) -> Self{
		let mut s = self;
		s.fields = fields;
		s
	}
	fn author(self, author: (&'a str,&'a str,&'a str)) -> Self{
		let mut s = self;
		s.author = author;
		s
	}
	fn url(self, url: &str) -> Self{
		let mut s = self;
		s.url = url.into();
		s
	}
	fn image(self, image: &str) -> Self{
		let mut s = self;
		s.image = image.into();
		s
	}
    fn send(self, cache: impl AsRef<Http>, chanel: ChannelId){
	    use serde_json::Map;
        let mut json = Map::new();
	    if !self.text.is_empty(){
		    let text:&str = if self.text.len()>2000{
			    &self.text[0..2000]
		    }
		    else{
			    self.text
		    };
		    json.insert("content".into(), json!(text));
	    }
	    let mut embed = Map::new();

	    if !self.title.is_empty(){
		    embed.insert("title".into(), json!(&self.title));
	    }
	    if !self.des.is_empty(){
		    embed.insert("description".into(), json!(&self.des));
	    }
	    if !self.thumbnail.is_empty(){
		    embed.insert("thumbnail".into(), json!({"url":&self.thumbnail}));
	    }

        embed.insert("color".into(), json!(&self.col));

	    if !self.footer.0.is_empty() || !self.footer.1.is_empty(){
		    let mut footer = Map::new();
		    if !self.footer.0.is_empty(){
			    footer.insert("icon_url".into(), json!(&self.footer.0));
		    }
		    if !self.footer.1.is_empty(){
			    footer.insert("text".into(), json!(&self.footer.1));
		    }
		    embed.insert("footer".into(), json!(footer));
	    }
	    if !self.image.is_empty(){
		    embed.insert("image".into(), json!({"url":&self.image}));
	    }
	    if !self.url.is_empty(){
		    embed.insert("url".into(), json!(&self.url));
	    }
	    if !self.author.0.is_empty() || !self.author.1.is_empty() || !self.author.2.is_empty(){
		    let mut author = Map::new();
		    if !self.author.0.is_empty(){
			    author.insert("name".into(), json!(&self.author.0));
		    }
		    if !self.author.1.is_empty(){
			    author.insert("url".into(), json!(&self.author.1));
		    }
		    if !self.author.2.is_empty(){
			    author.insert("icon_url".into(), json!(&self.author.2));
		    }
		    embed.insert("author".into(), json!(author));
	    }
	    if !self.fields.is_empty() {
		    let mut fields = Vec::new();
		    for (name, text, inline) in self.fields {
			    let mut field = Map::new();
			    if !name.is_empty() {
				    field.insert("name".into(), json!(name));
			    }
			    if !text.is_empty() {
				    field.insert("value".into(), json!(text));
			    }
			    field.insert("inline".into(), json!(inline));
			    fields.push(json!(field));
		    }
		    embed.insert("fields".into(), json!(fields));
	    }
	    json.insert("embed".into(), json!(embed));
	    Discord::send_embed(cache,chanel,json!(json));
    }
}

pub fn embed_from_value(cache: impl AsRef<Http>, chanel: ChannelId, val: Value){
    Discord::send_embed(cache, chanel,val);
}

fn main() {
    {
        use std::panic;
        use std::ops::Deref;

        panic::set_hook(Box::new(|panic_info| {
            let (filename, line) =
                panic_info.location().map(|loc| (loc.file(), loc.line()))
                          .unwrap_or(("<unknown>", 0));

            let cause = panic_info.payload().downcast_ref::<String>().map(String::deref);

            let cause = cause.unwrap_or_else(||
                panic_info.payload().downcast_ref::<&str>().map(|s| *s)
                          .unwrap_or("<cause unknown>")
            );

            error!("A panic occurred at {}:{}: {}", filename, line, cause);
        }));
    }

    {
        use simplelog::*;
        use std::fs::OpenOptions;
        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Info, ConfigBuilder::new().set_time_format_str("[%F %a] %T").build(), TerminalMode::Mixed).unwrap(),
                WriteLogger::new(LevelFilter::Info, ConfigBuilder::new().set_time_format_str("[%F %a] %T").build(), OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("wsbot.log")
                    .expect("Err while open log file to write")),
            ]
        ).unwrap();
    }
    use conf::Config;
    use addon::event_add;

//    let dcshell:DCShell = Discord::get_event_reciever();

    DB.ini_embeds_s();
//    DB.ini_lfg();
//    DB.ini_chat();
    Config::init();



    let mut client = Client::new(load_settings(),DisHandler)
        .expect("Error creating client");
    EVENT.send(EventChanel::Start(client.cache_and_http.http.clone()));

    info!("[Status] Main loop start");
    info!("{}", START_TIME.ctime());
    if let Err(why) = client.start() {
        error!("An error occurred while running the client: {:?}", why);
    }


/*
    loop {


        let event = match dcshell.get_wait(){
            OutLink::Event(e) => {e}
            OutLink::None => {continue;}
        };


        match event {
            Event::MessageCreate(mes) => {

                thread::spawn(move || {


                    if mes.content.as_str().starts_with('!') {
                        let content = mes.content.clone();
                        let mes_split: Vec<&str> = content.as_str().split_whitespace().collect();
                        match mes_split[0].to_lowercase().as_str() {
                            "!wsreg" => {
                                Discord::send_typing(mes.channel_id);
                                match reg_check(mes.author.id) {
                                    false => {
                                        reg_user(mes_split.clone(), mes.author.clone(), mes.channel_id);
                                    }
                                    true => { edit_user(mes_split.clone(), mes.author.clone(), mes.channel_id); }
                                }
                            }

                            "!wsstats" => {
                                info!("wsstats");
                                Discord::send_typing(mes.channel_id);
                                wsstats(mes_split.clone(), mes.author.id, mes.channel_id);
                            }

                            "!wstour" => {
                                info!("wstour");
                                DB.send_embed("tourneys",mes.channel_id);
                            }

                            "!wshelp" => {
                                info!("wshelp");
                                DB.send_embed("help",mes.channel_id);
                            }
                            "!wscmd" => {
                                info!("wscmd");
                                DB.send_embed("cmd",mes.channel_id);
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

                        if mes.author.id == 193759349531869184 || mes.author.id == 222781446971064320{
                            match mes_split[0].to_lowercase().as_str() {
	                            "!rup" => {
		                            use event::rating_updater;
		                            rating_updater();
		                            Discord::send_mes(mes.channel_id, "Опция не определена", "", false);
	                            }

                                "!ahelp" => {
                                    DB.send_embed("admin_commands",mes.channel_id);
                                }

                                "!event" =>{

                                    match mes_split.get(1){
                                        Some(&"add") =>{
                                            //11

                                            event_add(mes.content.clone());
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
                                                    embed.send(mes.channel_id);
                                                }
                                                _ =>{
                                                    let mut embed = EmbedStruct::empty();
                                                    let field_name = format!("Удаление эвента");
                                                    let mut field_text = format!("Имя не указано");
                                                    embed.fields.push((field_name, field_text, false));
                                                    embed.send(mes.channel_id);
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
                                            embed.send(mes.channel_id);
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
                                            embed.send(mes.channel_id);
                                        }
                                    }
                                }

                                "!test" => {
                                    let mut test_user: User = User::empty();
                                    test_user.did = mes.author.id;
                                    test_user.name = mes.author.username;
                                    test_user.disc = mes.author.discriminator;
                                    add_to_db(test_user);
                                }
                                "!test2" => {
                                    if let Some(id_str) = mes_split.get(1){
                                        if let Ok(id) = id_str.parse::<u64>(){

                                            delete_user(id);
                                            Discord::send_mes(mes.channel_id, &format!("{} удалён",id), "", false);
                                        }
                                        Discord::send_mes(mes.channel_id, &format!("Неизвестный параметр:`{}`",id_str), "", false);

                                    }
                                    else{
                                        delete_user(mes.author.id);
                                        Discord::send_mes(mes.channel_id, &format!("{} удалён",mes.author.id), "", false);

                                    }

                                }
                                "!ini" =>{
                                    if mes_split.len() > 1{
                                        match mes_split[1].to_lowercase().as_str(){
                                            "embed" => {
                                                DB.ini_embeds_s();
                                                Discord::send_mes(mes.channel_id, "Embed-ы инициализированы", "", false);
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
                                                Discord::send_mes(mes.channel_id, "Config инициализирован", "", false);
                                            }
                                            _ => {
                                                Discord::send_mes(mes.channel_id, "Опция не определена", "", false);
                                            }
                                        }

                                    }
                                    else {
                                        Discord::send_mes(mes.channel_id, "Перезагрузить embed, lfg или chat", "", false);
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
                                                Discord::send_mes(mes.channel_id, "Debug Включен", "", false);
                                            }
                                            "off" => {
                                                DEBUG.store(false, Ordering::Relaxed);
                                                Discord::send_mes(mes.channel_id, "Debug Выключен", "", false);
                                            }
                                            _ => {
                                                let string = format!("Debug статус: {}", DEBUG.load(Ordering::Relaxed));
                                                Discord::send_mes(mes.channel_id, string.as_str(), "", false);
                                            }
                                        }
                                    }
                                        else {
                                            let string = format!("Debug статус: {}", DEBUG.load(Ordering::Relaxed));
                                            Discord::send_mes(mes.channel_id, string.as_str(), "", false);
                                        }
                                }
                                "!new_net" => {
                                    if mes_split.len() > 1{
                                        match mes_split[1].to_lowercase().as_str(){
                                            "on" => {
                                                SWITCH_NET.store(true, Ordering::Relaxed);
                                                Discord::send_mes(mes.channel_id, "new_net Включен", "", false);
                                            }
                                            "off" => {
                                                SWITCH_NET.store(false, Ordering::Relaxed);
                                                Discord::send_mes(mes.channel_id, "new_net Выключен", "", false);
                                            }
                                            _ => {
                                                let string = format!("new_net статус: {}", SWITCH_NET.load(Ordering::Relaxed));
                                                Discord::send_mes(mes.channel_id, string.as_str(), "", false);
                                            }
                                        }
                                    }
                                        else {
                                            let string = format!("new_net статус: {}", SWITCH_NET.load(Ordering::Relaxed));
                                            Discord::send_mes(mes.channel_id, string.as_str(), "", false);
                                        }
                                }

                                /*
                                "!lfg" => {
                                    use addon::LFG;
                                    let color = 0;
                                    let mut rem = false;
                                    let mut remall = false;
                                    let mut id = None;
                                    let mut first_element = true;
                                    for mes in mes_split{
                                         if first_element{
                                             first_element = false;
                                             continue;
                                         }
                                        match mes.to_lowercase().as_str(){
                                            "remall" | "delall" => {
                                                remall = true;
                                            }
                                            "rem" | "del" => {
                                                rem = true;
                                            }
                                            x => {
                                                if let Ok(num) = x.parse::<u64>(){
                                                    id = Some(num);
                                                }
                                            }
                                        }
                                    }

                                    match (remall, rem, id) {
                                        (true, _, _) => {
                                            let lfg_list = DB.get_lfg_list();
                                            for lfg in lfg_list{
                                                let mut call = format!("DELETE FROM lfg WHERE did={}",lfg.did);
                                                let mut conn = POOL.get_conn().unwrap();
                                                if let Err(e) = conn.query(call){
                                                    info!("lfg_rem Err: {}", e);
                                                }
                                                DB.remove_lfg(lfg.did);
                                            }
                                            Discord::send_mes(mes.channel_id, "Список LFG очищен", "", false);
                                        }

                                        (false,true,Some(did)) => {
                                            match DB.get_lfg(did){
                                                Some(lfg) => {
                                                    let title = "Объявление удалено:";
                                                    let (tstring, dstring) = lfg.to_line_debug(mes.channel_id);
	                                                EmbedStruct::empty()
		                                                .title(&title)
		                                                .col(color)
		                                                .fields(vec![(tstring,dstring,false)])
		                                                .send(mes.channel_id);

                                                    let mut call = format!("DELETE FROM lfg WHERE did={}",did);
                                                    let mut conn = POOL.get_conn().unwrap();
                                                    if let Err(e) = conn.query(call){
                                                        info!("lfg_rem Err: {}", e);
                                                    }
                                                    DB.remove_lfg(did);
                                                }
                                                None => {
                                                    Discord::send_mes(mes.channel_id, "По указаному DID не найдено", "", false);
                                                    return;
                                                }
                                            }
                                        }

                                        (false,false,Some(did)) => {
                                            match DB.get_lfg(did){
                                                Some(lfg) => {

                                                    let (tstring, dstring) = lfg.to_line_debug(mes.channel_id);
	                                                EmbedStruct::empty()
		                                               // .title(&title)
		                                                .col(color)
		                                                .fields(vec![(tstring,dstring,false)])
		                                                .send(mes.channel_id);

                                                }
                                                None => {
                                                    Discord::send_mes(mes.channel_id, "По указаному DID не найдено", "", false);
                                                    return;
                                                }
                                            }
                                        }


                                        (false, _, None) => {
                                            let fields:Vec<(String,String,bool)> = LFG::def_table(true,mes.channel_id);
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

                                    }



                                }
                                */

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
                                        .send(mes.channel_id);

                                }

                                _=>{}
                            }
                        }

                    }
                    else {

                    }
                });
            }
            Event::Ready(json) => {
                info!("READY:\n{}", serde_json::to_string_pretty(&json).unwrap_or(String::new()));
            }
            Event::GuildCreate(json) => {

                info!("GuildCreate:\nName: {}", json["name"].as_str().unwrap_or(""));
                info!("Id: {}", json["id"].as_str().unwrap_or(""));
 //               let dservid = json["id"].as_str().unwrap().parse::<u64>().unwrap()("");
//
  //              if load_by_dsid(dservid) = dservid.as_str().unwrap_or("0").parse::<u64>().unwrap()
   //             {}
   //             else {add_dsid_to_db(dservid)};

                info!("Member Count: {}", json["member_count"].as_u64().unwrap_or(0));
                if let Some(user) = Discord::get_user(json["owner_id"].as_str().unwrap_or("0").parse::<u64>().unwrap()){
                    info!("Owner Id: {}", user.id);
                    info!("Owner Username: {}", user.username);
                    info!("Owner Discriminator: {}", user.discriminator);
                }


                Config::exec_fn(move |rwlock| {
                    use conf::ConfType;
                    use std::ops::DerefMut;
                    //Апдейт конфига ролей (если есть)

                    let config = rwlock.deref_mut();
                    let mut need_update = false;
                    let mut rating = Value::Null;

                    if let Some(rating_root) = config.get(&ConfType::rating){
                        rating = rating_root.clone();
                        if let Some(text_id) = json["id"].as_str(){
                            if let Some(guild_list_of_roles) = json["roles"].as_array(){
                                if let Some(guild_conf) = rating_root.pointer(format!("/{}",text_id).as_str()){
                                    for (pos, conf_role) in guild_conf.as_array().expect("Err main#4").iter().enumerate(){
                                        'inner: for guild_role in guild_list_of_roles{
                                            let id = guild_role["id"].as_str().expect("Err main#0").parse::<u64>().expect("Err main#1");
                                            let name = guild_role["name"].as_str().expect("Err main#3");

                                            if let Some(conf_id) = conf_role["id"].as_u64(){
                                                if id == conf_id{
                                                    if let Some(conf_name) = conf_role["name"].as_str(){
                                                        if conf_name.eq(name){
                                                        }
                                                            else {
                                                                rating.pointer_mut(format!("/{}/{}",text_id,pos).as_str()).unwrap().as_object_mut().unwrap().insert("name".to_string(), Value::String(name.to_string()));
                                                                need_update = true;
                                                            }
                                                    }
                                                    break 'inner;
                                                }
                                            }
                                                else{
                                                    if let Some(conf_name) = conf_role["name"].as_str(){
                                                        if conf_name.eq(name){
                                                            rating.pointer_mut(format!("/{}/{}",text_id,pos).as_str()).unwrap().as_object_mut().unwrap().insert("id".to_string(), json!(id));
                                                            need_update = true;
                                                            break 'inner;
                                                        }
                                                    }
                                                }

                                        }

                                    }

                                }
                            }
                        }
                    }
                    if need_update{
                        Config::set_in_file(ConfType::rating, rating.clone());
                        config.insert(ConfType::rating, rating);
                    }
                });

            }
            Event::GuildRoleUpdate(json) => {
                Config::exec_fn(move |rwlock| {
                    use conf::ConfType;
                    use std::ops::DerefMut;
                    //Апдейт конфига ролей (если есть)

                    let config = rwlock.deref_mut();
                    let mut need_update = false;
                    let mut rating = Value::Null;
                    if let Some(rating_root) = config.get(&ConfType::rating){
                        rating = rating_root.clone();
                        if let Some(text_id) = json["guild_id"].as_str(){
                            if let Some(guild_role) = json.get("role"){
                                if let Some(guild_conf) = rating_root.pointer(format!("/{}",text_id).as_str()){
                                    let id = guild_role["id"].as_str().expect("Err main#0").parse::<u64>().expect("Err main#1");
                                    let name = guild_role["name"].as_str().expect("Err main#3");
                                    for (pos, conf_role) in guild_conf.as_array().expect("Err main#4").iter().enumerate(){
                                        if let Some(conf_id) = conf_role["id"].as_u64(){
                                            if id == conf_id{
                                                if let Some(conf_name) = conf_role["name"].as_str(){
                                                    if conf_name.eq(name){
                                                    }
                                                        else {
                                                            rating.pointer_mut(format!("/{}/{}",text_id,pos).as_str()).unwrap().as_object_mut().unwrap().insert("name".to_string(), Value::String(name.to_string()));
                                                            need_update = true;
                                                        }
                                                }
                                                break;
                                            }
                                        }
                                            else{
                                                if let Some(conf_name) = conf_role["name"].as_str(){
                                                    if conf_name.eq(name){
                                                        rating.pointer_mut(format!("/{}/{}",text_id,pos).as_str()).unwrap().as_object_mut().unwrap().insert("id".to_string(), json!(id));
                                                        need_update = true;
                                                        break;
                                                    }
                                                }
                                            }
                                    }
                                }
                            }
                        }
                    }
                    if need_update{
                        Config::set_in_file(ConfType::rating, rating.clone());
                        config.insert(ConfType::rating, rating);
                    }
                });

            }
            _ => {}

            //END OF MAIN THREAD
        }
    }
    */
}



