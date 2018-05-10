#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate reqwest;
//extern crate rusqlite;
extern crate mysql;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate indexmap;

extern crate native_tls;
extern crate websocket;

extern crate time as extime;
//https://discordapp.com/api/oauth2/authorize?client_id=316281967375024138&scope=bot&permissions=0

use regex::Regex;
use std::io::Read;
use std::io::Write;
use std::ops::Deref;
use std::ops::Index;

//use std::env;
//use rusqlite::Connection;

pub mod addon;
//pub mod net;
//pub mod tournaments;
pub mod event;

pub mod denum;
pub mod dstruct;
pub mod dis;
pub mod disapi;

use disapi::Discord;
use dstruct::DCShell;
use denum::Event;
use denum::OutLink;
use dstruct::{DMessage,DUser};

use event::{EventChanel, EventH, EventReq, EventChanelBack, EventType};
use addon::{DB, Chat, lfg_none, Stage_LFG, Global, TempData};
use dstruct::{DiscordMain};
use serde_json::Value;

//use net::Net;

use std::{thread, time, fmt};

use std::time::{Duration, Instant, SystemTime};
use std::fmt::Debug;
use std::sync::mpsc::channel;
use mysql::from_row;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

lazy_static! {

    pub static ref POOL: mysql::conn::pool::Pool = mysql::Pool::new(build_opts()).unwrap();
    pub static ref REG_BTAG: Regex = Regex::new(r"^.{2,16}#[0-9]{2,6}$").expect("Regex btag error");
    static ref REG_TIME: Regex = Regex::new(r"(?P<n>\d){1,4} ?(?i)(?P<ntype>m|min|h|hour)").expect("Regex btag error");

    static ref START_TIME: extime::Tm = extime::now();
    static ref EVENT: EventH = EventH::create();

    pub static ref D: DiscordMain = DiscordMain::new(load_settings());
}
pub static WSSERVER: u64 = 351798277756420098; //ws = 351798277756420098 //bs = 316394947513155598
static SWITCH_NET: AtomicBool = ATOMIC_BOOL_INIT;
static DEBUG: AtomicBool = ATOMIC_BOOL_INIT;

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
    rtg: u16,
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
            rtg: 0,
            reg: String::new(),
            plat: String::new(),
            scrim_preset: Preset_Scrim::new(),
            rtg_preset: Preset_Rtg::new(),
        }
        /*
        let mut temp_user = User::empty();
        temp_user.did = autor.id.0;
        temp_user.name = autor.name;
        temp_user.disc = autor.discriminator.to_string();
        temp_user.btag = battletag.clone();
        temp_user.rtg = 0;
        temp_user.reg = region.clone();
        temp_user.plat = platform.clone();
        temp_user.scrim_preset = user.scrim_preset;
        temp_user.rtg_preset = user.rtg_preset;
        */
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
}
impl Hero{
    fn get_from_bliz_str(s: &str) -> Hero{
        match s{
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
            Hero::None => {return String::new();}
        }
    }
}

#[derive(Default,Clone,Debug)]
pub struct BtagData {
    btag: String,
    reg: String,
    plat: String,
    rating: u16,
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
    time_played: Option<Time>,
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

pub fn load_btag_data(btag: String, reg: String, plat: String, req:HeroInfoReq) -> Option<BtagData> //Проверка существования профиля и подгрузка рейтинга при наличии
{
    use std::time::SystemTime;

    if btag.is_empty() || plat.is_empty(){
        return None;
    }

    let sys_time_old = SystemTime::now();

    let use_new_net: bool = SWITCH_NET.load(Ordering::Relaxed);
    let mode_debug: bool = DEBUG.load(Ordering::Relaxed);


    if mode_debug{
        println!("Start: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
    }


    let mut result = None;
    let mut url = String::new();
//        url = format!("https://playoverwatch.com/en-us/career/{}/{}/{}", plat.to_lowercase(), reg.to_lowercase(), btag.replace("#", "-"));
        url = format!("https://playoverwatch.com/en-us/career/{}/{}", plat.to_lowercase(), btag.replace("#", "-"));

        match reqwest::get(&url){
            Ok(mut resp) => {

                match resp.text(){
                    Ok(text) =>{result = Some(text);}
                    Err(e) => {
                        println!("[load_btag_data] Error while take body:\n{}", e);

                    }
                }
//                let mut content = String::new();
//                if let Err(e) = resp.read_to_string(&mut content){
//                    println!("[load_btag_data] Error while reading body:\n{}", e);
//                }
//                else {
//                    result = Some(content);
//                }
            }
            Err(e) => {
                println!("[load_btag_data] Error while get responce from url. Probaly wrong url:\n{}", e);
            }
        }



    if mode_debug{
        println!("Get respornse: {:?}",
                 SystemTime::now().duration_since(sys_time_old).unwrap());
    }

    if let Some(body) = result{
        if body.contains("h1 class=\"u-align-center\">Profile Not Found<") {
            return None;
        }

        let mut b_data = BtagData::default();
        b_data.btag = btag;
        b_data.reg = reg;
        b_data.plat = plat;
        b_data.url = url.clone();


        let avatar_url_patern = "masthead-player\"><img src=\"";
        b_data.avatar_url = match body.find(avatar_url_patern){ //Ищем URL аватара
            Some(start_pos) => {
                let mut string = String::new();
                let mut pos = start_pos + avatar_url_patern.len();
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
                string
            }
            None => {
                String::new()
            }
        };

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
                            println!("Error while parce rating:\n{}\n{}",string ,e);
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

        if mode_debug{
            println!("Get rating: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
        }

        let mut comp = String::new();
        let mut time_played = String::new();
        let mut games_won = String::new();
        let mut win_perc = String::new();
        let mut aim = String::new();
        let mut kills_per_live = String::new();
        let mut best_multiple_kills = String::new();
        let mut obj_kills = String::new();

        static COMP_STR: &str = "id=\"competitive\" data-js=\"career-category\""; //начало комп раздела, конец раздела быстрой игры
        static TIME_PLAYED_STR: &str = "data-category-id=\"overwatch.guid.0x0860000000000021\""; //начало раздела времени в игре
        static GAMES_WON_STR: &str = "data-category-id=\"overwatch.guid.0x0860000000000039\""; //начало раздела выйграных матчей
        static WIN_PERC_STR: &str = "data-category-id=\"overwatch.guid.0x08600000000003D1\""; //начало раздела процента побед
        static AIM_STR: &str = "data-category-id=\"overwatch.guid.0x086000000000002F\""; //начало раздела меткости
        static KILLS_PER_LIVE_STR: &str = "data-category-id=\"overwatch.guid.0x08600000000003D2\""; //начало раздела убийств за одну жизнь
        static BEST_MULTIPLE_KILLS_STR: &str = "data-category-id=\"overwatch.guid.0x0860000000000346\""; //начало раздела лучш. множ. убийств
        static OBJ_KILLS_STR: &str = "data-category-id=\"overwatch.guid.0x086000000000039C\""; //начало раздела убийств у объекта
        static ACHIVMENT_STR: &str = "<section id=\"achievements-section\""; //начало раздела ачивок, конец комп раздела

        if req.time_played || req.games_won || req.win_perc || req.aim
            || req.kills_per_live || req.best_multiple_kills || req.obj_kills{

            comp = cut_part_of_str(&body.to_string(), COMP_STR, ACHIVMENT_STR);
        }
        if req.time_played{
            time_played = cut_part_of_str(&comp, TIME_PLAYED_STR, GAMES_WON_STR);

            loop{
                match find_next_hero(&time_played){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {
                        //println!("3 cut");
                        let hdat = find_description(hero_data.as_str());
                        let mut time = Time::None;
                        if hdat != "--" || hdat.is_empty(){
                            let num = hdat.find(" ").unwrap();
                            let (hdat_split1,hdat_split2) = hdat.split_at(num);
                            time = match hdat_split2{
                                " hour"|" hours" => {
                                    Time::Hours(hdat_split1.parse::<u32>().unwrap())
                                }
                                " minute"|" minutes" => {
                                    Time::Min(hdat_split1.parse::<u32>().unwrap())
                                }
                                " second"|" seconds" => {
                                    Time::Sec(hdat_split1.parse::<u32>().unwrap())
                                }
                                _ =>{
                                    Time::None
                                }
                            };
                        }

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.time_played = Some(time);

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                                break;
                        }
                        else {
                            time_played = next_data;
                        }
                    }
                }
            }
        }
        if req.games_won{
            games_won = cut_part_of_str(&comp, GAMES_WON_STR, WIN_PERC_STR);

            loop{
                match find_next_hero(&games_won){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {

                        let hdat = find_description(hero_data.as_str());

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.games_won = Some(hdat.parse::<u32>().unwrap());

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                            break;
                        }
                            else {
                                games_won = next_data;
                            }
                    }
                }
            }
        }
        if req.win_perc{
            win_perc = cut_part_of_str(&comp, WIN_PERC_STR, AIM_STR);

            loop{
                match find_next_hero(&win_perc){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {

                        let hdat = find_description(hero_data.as_str());

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.win_perc = Some(hdat.trim_matches('%').parse::<u16>().unwrap());

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                            break;
                        }
                            else {
                                win_perc = next_data;
                            }
                    }
                }
            }
        }
        if req.aim{
            aim = cut_part_of_str(&comp, AIM_STR, KILLS_PER_LIVE_STR);
            loop{
                match find_next_hero(&aim){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {

                        let hdat = find_description(hero_data.as_str());

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.aim = Some(hdat.trim_matches('%').parse::<u16>().unwrap());

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                            break;
                        }
                            else {
                                aim = next_data;
                            }
                    }
                }
            }
        }
        if req.kills_per_live{
            kills_per_live = cut_part_of_str(&comp,
                                             KILLS_PER_LIVE_STR,
                                             BEST_MULTIPLE_KILLS_STR);
            loop{
                match find_next_hero(&kills_per_live){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {

                        let hdat = find_description(hero_data.as_str());

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.kills_per_live = Some(hdat.parse::<f32>().unwrap());

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                            break;
                        }
                            else {
                                kills_per_live = next_data;
                            }
                    }
                }
            }
        }
        if req.best_multiple_kills{
            best_multiple_kills = cut_part_of_str(&comp,
                                                  BEST_MULTIPLE_KILLS_STR,
                                                  OBJ_KILLS_STR);
            loop{
                match find_next_hero(&best_multiple_kills){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {

                        let hdat = find_description(hero_data.as_str());

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.best_multiple_kills = Some(hdat.parse::<u32>().unwrap());

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                            break;
                        }
                            else {
                                best_multiple_kills = next_data;
                            }
                    }
                }
            }
        }
        if req.obj_kills{
            obj_kills = cut_part_of_str(&comp,
                                        OBJ_KILLS_STR,
                                        ACHIVMENT_STR);
            loop{
                match find_next_hero(&obj_kills){
                    (Hero::None, ..) => {break;}
                    (hero, hero_data, next_data) => {

                        let hdat = find_description(hero_data.as_str());

                        let mut hero_stats = HeroStats::new(hero);
                        hero_stats.obj_kills = Some(hdat.parse::<u32>().unwrap());

                        b_data.hero_data(hero_stats);

                        if next_data.is_empty(){
                            break;
                        }
                            else {
                                obj_kills = next_data;
                            }
                    }
                }
            }
        }

        if mode_debug{
            println!("End: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
        }
        return Some(b_data);

    }
    else{
        if mode_debug{
            println!("End None: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
        }
        return None;
    }

}

fn find_description(string: &str) -> String //class="description">
{
    let description_patern = "class=\"description\">";
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
    static hero_start: &str = "data-hero-guid=\"0x02E0000000000";
    let mut answer = (Hero::None, String::new(), String::new());

    let start = match string.find(hero_start) {
        Some(x) => {x+hero_start.chars().count()}
        None => {return answer;}
    };

    let mut hero_str = String::new();

    unsafe{
        answer.1 = string.slice_unchecked(start, string.chars().count()).to_string();
        hero_str = answer.1.slice_unchecked(0, 3).to_owned();
    }

    answer.0 = Hero::get_from_bliz_str(hero_str.as_str());


    let end = match answer.1.find(hero_start) {
        Some(x) => {x}
        None => {0}
    };

    if end > 0{
        unsafe{
            answer.2 = answer.1.slice_unchecked(end, answer.1.chars().count()).to_string();
            answer.1 = answer.1.slice_unchecked(0, end).to_string();
        }
    }

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
        return main.slice_unchecked(start, end+1).to_owned();
    }
}

fn add_to_db(user: User) //Добавление Нового профиля в БД
{
    let call = format!("INSERT INTO users (did, name, disc, btag, rtg, reg, plat, scrim_preset, rtg_preset) VALUES ({}, '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}');",
                       &user.did, &user.name, &user.disc, &user.btag, &user.rtg, &user.reg, &user.plat, serde_json::to_string(&user.scrim_preset).unwrap(), serde_json::to_string(&user.rtg_preset).unwrap());
    let mut conn = POOL.get_conn().unwrap();
    println!("[MySQL request INSERT INTO users] {}", call);
    let _ = conn.query(call);
}

fn update_in_db(user: User) //Изменение профиля в БД
{
    let call = format!("UPDATE users SET name='{}', disc='{}', btag='{}', rtg={}, reg='{}', plat='{}', scrim_preset='{}', rtg_preset='{}' WHERE did={}",
                       &user.name, &user.disc, &user.btag, &user.rtg, &user.reg, &user.plat, serde_json::to_string(&user.scrim_preset).unwrap(), serde_json::to_string(&user.rtg_preset).unwrap(), &user.did);
    let mut conn = POOL.get_conn().unwrap();
    let _ = conn.query(call);
}

pub fn load_by_id(id: u64) -> Option<User> //Получение профиля из базы по DiscordId
{
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
}

pub fn load_settings() -> String //Загрузка DiscordId
{
    let mut ta = String::new();
    let mut stmt = POOL.prepare("SELECT distoken FROM bottoken").unwrap();
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

fn reg_user(mut reg_str: Vec<&str>, autor: DUser, chan: u64) //Диалог создания профиля
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
    let mut rating: u16 = 0;
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

        if !no_btag {
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

            let answer = load_btag_data(battletag.to_string(), region.to_string(), platform.to_string(), req);

            if let Some(an) = answer{
                rating = an.rating;
                //println!("rating: {}", rating);
                thumbnail = an.avatar_url.clone();
                roleruler = role_ruler_text(WSSERVER,
                           autor.id,
                           RoleR::rating(rating));
            }
            else {
                acc_not_found = true;
                rating = 0;
            }
        }

        title = "Регистрация пройдена";


        if no_btag || no_plat || no_reg {

            des = "Но вы не указали полные данные :worried:";

            if no_btag {fields.push(("BattleTag".to_string(),"Не указан".to_string(),false))}
                else { fields.push(("BattleTag".to_string(), battletag.clone(), false)) }
            if no_reg {fields.push(("Регион".to_string(),"По умолчанию (EU)".to_string(),false))}
                else { fields.push(("Регион".to_string(), region.clone(), false)) }
            if no_plat {fields.push(("Платформа".to_string(),"По умолчанию (PC)".to_string(),false))}
                else { fields.push(("Платформа".to_string(), platform.clone(), false)) }

            if acc_not_found {
                des = "Мы не смогли найти ваш профиль Overwtach по заданным параметрам.\nВозможно вы ошиблись или указали недостаточно данных.";
                footer = "Вы можете указать корректные данные позже с помощью комманды !wsreg";
            } else {
                if rating > 0 {fields.push(("Рейтинг".to_string(), format!("{}",rating), false));}

                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";


            }
        } else {
            if acc_not_found {
                des = "Похоже мы не смогли найти ваш профиль Overwtach по заданным параметрам. \nВозможно вы ошиблись или указали недостаточно данных.";
                fields.push(("BattleTag".to_string(), battletag.clone(), false));
                fields.push(("Регион".to_string(), region.clone(), false));
                fields.push(("Платформа".to_string(), platform.clone(), false));
                footer ="Вы можете добавить их позже с помощью комманды !wsreg";
            } else {
                des = "Информация успешно добавлена :wink:";
                fields.push(("BattleTag".to_string(), battletag.clone(), false));
                fields.push(("Регион".to_string(), region.clone(), false));
                fields.push(("Платформа".to_string(), platform.clone(), false));
                if rating > 0 {fields.push(("Рейтинг".to_string(), format!("{}",rating), false))}

                    footer = "Изменить BattleTag, Регион и Платформу вы можете используя комманду !wsreg";

            }
        }

        if acc_not_found {
            let mut temp_user = User::empty();
            temp_user.did = autor.id;
            temp_user.name = autor.username;
            temp_user.disc = autor.discriminator;
            add_to_db(temp_user);
        } else {
            let mut temp_user = User::empty();
            temp_user.did = autor.id;
            temp_user.name = autor.username;
            temp_user.disc = autor.discriminator;
            temp_user.btag = battletag.to_string();
            temp_user.rtg = rating;
            temp_user.reg = region.to_string();
            temp_user.plat = platform.to_string();
            add_to_db(temp_user);
        }
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
		.send(chan);
}

fn edit_user(mut reg_str: Vec<&str>, autor: DUser,chan: u64) //Диалог на запрос редактирования профиля
{
    let mut battletag: String = String::new();
    let mut region: String = String::new();
    let mut platform: String = String::new();
    let user = load_by_id(autor.id).unwrap();
    let mut rating: u16 = 0;
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
                    temp_user.rtg = 0;
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
                    let answer = load_btag_data(battletag.to_string(), region.to_string(), platform.to_string(), req);

                    if let Some(an) = answer{
                        rating = an.rating;
                        //println!("rating: {}", rating);
                        thumbnail = an.avatar_url.clone();
                        roleruler = role_ruler_text(WSSERVER,
                                   autor.id,
                                   RoleR::rating(rating));
                    }
                        else {
                            acc_not_found = true;
                            rating = 0;
                        }

                    if acc_not_found {
                        if force {
                            let mut temp_user = User::empty();
                            temp_user.did = autor.id;
                            temp_user.name = autor.username;
                            temp_user.disc = autor.discriminator;
                            temp_user.btag = battletag.clone();
                            temp_user.rtg = 0;
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
                    } else {
                        let mut temp_user = User::empty();
                        temp_user.did = autor.id;
                        temp_user.name = autor.username;
                        temp_user.disc = autor.discriminator;
                        temp_user.btag = battletag.clone();
                        temp_user.rtg = rating;
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
                        if rating > 0 {fields.push(("Рейтинг".to_string(), format!("{}",rating), false))}

                            footer = "Убедитесь, что верно ввели парамтры на изменение ваших данных";




                    }
                }
            } else {
                title = "Изменение данных";
                des = "Ваши текущие данные совпадают с введёнными";
                fields.push(("BattleTag".to_string(), battletag.clone(), false));
                fields.push(("Регион".to_string(), region.clone(), false));
                fields.push(("Платформа".to_string(), platform.clone(), false));
                if rating > 0 {fields.push(("Рейтинг".to_string(), format!("{}",rating), false))}



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
		.send(chan);
}

//Отправная функция по всем запросам касательно скримов
/*
fn scrim_starter(mut mes: &str, autor: discord::model::User)
{
    lazy_static! {
        static ref REG_RTG: Regex = Regex::new(r"^([0-9]{1,3}|[1-4][0-9]{1,3}|5000)$").unwrap();
    }

    let mut mes_str: Vec<&str> = mes.split_whitespace().collect();
    let mut battletag: String = String::new();
    let mut platform: String = String::new();
    let mut rating: u16 = 0;
    let mut unnone = false;
    let mut botmess: String = "     Поиск скримов".to_string();
    let mut timer: u64 = 900; //Вообще надо бы сделать значение с точкой но это потом
    let mut live_time: u64 = 900;
    let mut hide: bool = false;
    let mut show_btag: bool = false;
    let mut help_str = "\n```markdown\n!wsscrim {Время поиска: m|H} {Платформа: PC|P4|XB} {Рейтинг группы (по умолчанию будет взят ваш рейтинг)} {Сохранить шаблон: save}\n```";
    //let is_reg = reg_check(autor.id);
    let user: User = match load_by_id(autor.id.0) {
        Some(u) => { u }
        _ => { User::empty() }
    };
    if user.did != 0 && !user.btag.is_empty() {
        rating = match load_overwatch_rating(user.btag.clone(), user.reg.clone(), user.plat.clone()) {
            6000 => {
                0
            }
            x => { x }
            _ => { 0 }
        }
    }
    let mut save_preset: bool = false;
    let mut preset: Preset_Scrim;

    if mes_str.capacity() > 1 {
        for s in mes_str {
            match s.to_uppercase().as_str() {
                "PC" | "P4" | "XB" => {
                    platform = s.to_uppercase();
                }
                "SAVE" => {
                    save_preset = true;
                }
                _ => {
                    if REG_BTAG.is_match(s) {
                        battletag = s.to_string();
                    } else if REG_RTG.is_match(s) { rating = s.parse::<u16>().unwrap(); } else { unnone = true; }
                }
            }
        }
        if unnone {
            if REG_TIME.is_match(mes) {
                match REG_TIME.captures(mes).unwrap().name("ntype").unwrap().as_str().to_uppercase().as_str() {
                    "M" | "MIN" => {
                        timer = REG_TIME.captures(mes).unwrap().name("n").unwrap().as_str().parse::<u64>().unwrap() * 60;
                    }
                    "H" | "HOUR" => {
                        timer = REG_TIME.captures(mes).unwrap().name("n").unwrap().as_str().parse::<u64>().unwrap() * 3600;
                    }
                    _ => {}
                }
                if timer <= 7200 {
                    live_time = timer;
                } else {
                    live_time = 7200;
                    //текст о превышении лимита
                }
            }
        }
        let no_btag = match (battletag.is_empty(), user.btag.clone().is_empty()) {
            (true, true) => { true }
            (false, _) => { false }
            (true, false) => {
                battletag = user.btag.clone();
                true
            }
            _ => { false }
        };

        let no_plat: bool = match (platform.is_empty(), user.plat.clone().is_empty()) {
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
    } else {
        if user.scrim_preset.plat.is_empty() {
            if user.did == 0 {
                //проверка на регистрацию
            } else {
                if rating == 0 {} else {}
            }
        } else {
            let mut scrim = Scrim::new();

            scrim.user = autor.id.0;
            scrim.plat = user.scrim_preset.plat;
            scrim.rtg = user.scrim_preset.rtg;
            scrim.live_time = user.scrim_preset.live_time;
            scrim.hide = hide;
            scrim.show_btag = show_btag;
        }
    }

    let mut scrim = Scrim::new();

    scrim.user = autor.id.0;
    scrim.plat = platform;
    scrim.rtg = rating;
    scrim.live_time = live_time;
    scrim.hide = hide;
    scrim.show_btag = show_btag;

    match DIS.create_invite(DIS.create_private_channel(autor.id).unwrap().id, 600, 0, true) {
        Ok(chan) => {
            println!("[code] {}", chan.code);
            println!("[server_id] {:?}", chan.server_id);
            println!("[server_name] {}", chan.server_name);
            botmess.push_str(format!("\nПлатформа: {}", scrim.plat.clone()).as_str());
            botmess.push_str(format!("\nРейтинг: {:?}", scrim.rtg).as_str());
            botmess.push_str(format!("\nВремя поиска: {:?}", live_time * 60).as_str());
            //let _ = DIS.send_message(, botmess.as_str(), "", false);
            scrim_queue(&scrim);
        }
        Err(e) => { println!("[CreatingChanelErr] {:?}", e) }
    }
}*/

/*
fn scrim_queue(scrim: &Scrim) {
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("SELECT COUNT(*) FROM scrim_queue");
    let mut stmt = conn.prepare(command).unwrap();
    let mut answer: u64 = 0;

    for row in stmt.execute(()).unwrap() {
        answer = mysql::from_row::<u64>(row.unwrap());
    }

    println!("[MySQL request SELECT COUNT(*)] {:?}", answer);

    if answer == 0 {
        add_to_scrim_queue(&scrim);
    } else {
        let mut call = format!("SELECT");

        call = format!("{} user", call);
        call = format!("{}, plat", call);
        call = format!("{}, rtg", call);
        call = format!("{}, live_time", call);
        call = format!("{}, hide", call);
        call = format!("{}, show_btag", call);

        call = format!("{} FROM scrim_queue WHERE", call);

        call = format!("{} plat='{}'", call, scrim.plat);

        call = format!("{} AND", call);

        call = format!("{} rtg BETWEEN {:?} AND {:?}", call, scrim.rtg - 200, scrim.rtg + 200);

        call = format!("{} ORDER BY createdtime LIMIT 1", call);
        let mut conn = POOL.get_conn().unwrap();
        let mut stmt = conn.prepare(call.as_str()).unwrap();
        let mut founded: Scrim = Scrim::new();


        for row in stmt.execute(()).unwrap() {
            let (user, plat, rtg, live_time, hide, show_btag) = mysql::from_row::<(u64, String, u16, u64, bool, bool)>(row.unwrap());
            founded.user = user;
            founded.plat = plat;
            founded.rtg = rtg;
            founded.live_time = live_time;
            founded.hide = hide;
            founded.show_btag = show_btag;
        }
        let mut botmess: String = "Найдено".to_string();
        botmess.push_str(format!("\nUser: {}", discord::model::UserId(founded.user).mention()).as_str());
        botmess.push_str(format!("\nПлатформа: {}", founded.plat).as_str());


        call = format!("SELECT");
        call = format!("{} ABS(UNIX_TIMESTAMP(endtime) - UNIX_TIMESTAMP(CURRENT_TIMESTAMP))", call);
        call = format!("{}, CASE WHEN endtime >= CURRENT_TIMESTAMP THEN 1 ELSE 0 END", call);
        call = format!("{} FROM scrim_queue WHERE", call);
        call = format!("{} user={}", call, scrim.user);

        let mut conn = POOL.get_conn().unwrap();
        let mut stmt = conn.prepare(call.as_str()).unwrap();
        let mut answer: u64 = 0;
        let mut trigger: i16 = 0;

        for row in stmt.execute(()).unwrap() {
            let (first, second) = mysql::from_row::<(u64, i16)>(row.unwrap());
            answer = first;
            trigger = second;
        }
        if trigger == 1 {
            botmess.push_str(format!("\nОсталось: {:?} секунд", answer).as_str());
        } else {
            if answer == 0 {
                botmess.push_str(format!("\nПрошло: ровно").as_str());
            }
            {
                botmess.push_str(format!("\nПрошло: {:?} секунд", answer).as_str());
            }
        }

        let _ = DIS.send_message(DIS.create_private_channel(discord::model::UserId(founded.user)).unwrap().id, botmess.as_str(), "", false);
    }
}*/

/*
fn add_to_scrim_queue(scrim: &Scrim) {
    let time_str = format!("{:?}:{:?}:{:?}", scrim.live_time / 3600, (scrim.live_time / 60) % 60, scrim.live_time % 60);
    println!("[time_str] {}", time_str);

    let mut call = format!("INSERT INTO scrim_queue (");

    call = format!("{} user", call);
    call = format!("{}, plat", call);
    call = format!("{}, rtg", call);
    call = format!("{}, live_time", call);
    call = format!("{}, hide", call);
    call = format!("{}, show_btag", call);
    call = format!("{}, endtime", call);

    call = format!("{}) VALUES (", call);

    call = format!("{} {:?}", call, scrim.user);
    call = format!("{}, '{}'", call, scrim.plat);
    call = format!("{}, {}", call, scrim.rtg);
    call = format!("{}, {}", call, scrim.live_time);
    call = format!("{}, {}", call, match scrim.hide {
        true => { 1 }
        false => { 0 }
    });
    call = format!("{}, {}", call, match scrim.show_btag {
        true => { 1 }
        false => { 0 }
    });
    call = format!("{}, TIMESTAMP(CURRENT_TIMESTAMP,'{}')", call, time_str);

    call = format!("{});", call);

    println!("[MySQL request INSERT INTO scrim_queue] {}", call);
    let mut conn = POOL.get_conn().unwrap();
    let _ = conn.query(call);
}
*/

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

/*
fn event_eater(ev: String){
    thread::spawn(move || {

        let mut mess = String::new();
        let mut tabs = 0;
        for char in ev.chars() {
            match char {
                '(' => {
                    mess.push(char);
                    mess.push('\n');
                    tabs = tabs + 1;
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                '{' => {
                    mess.push(char);
                    mess.push('\n');
                    tabs = tabs + 1;
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                ',' => {
                    mess.push(char);
                    mess.push('\n');
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                ')' => {
                    mess.push(char);
                    mess.push('\n');
                    tabs = tabs - 1;
                    if tabs < 0 { tabs = 0; }
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                '}' => {
                    mess.push('\n');
                    mess.push(char);
                    tabs = tabs - 1;
                    if tabs < 0 { tabs = 0; }
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                '[' => {
                    mess.push(char);
                    mess.push('\n');
                    tabs = tabs + 1;
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                ']' => {
                    mess.push('\n');
                    mess.push(char);
                    tabs = tabs - 1;
                    if tabs < 0 { tabs = 0; }
                    for _ in 0..tabs {
                        mess.push('\t');
                    }
                }
                _ => {
                    mess.push(char);
                }
            }
        }
        println!("{}", mess);
    });
}
*/

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

fn wsstats(mes: Vec<&str>, autor_id: u64, chanel: u64){

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
		        .send(chanel);
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
		            .send(chanel);
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
		            .send(chanel);
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
		                .send(chanel);
                    err_end = true;

                } else { update = true; }

            }
        }
    }
    if !err_end {
        if u.plat.is_empty() { u.plat = "PC".to_string(); }
        if u.reg.is_empty() { u.reg = "EU".to_string(); }

        let answer = load_btag_data(u.btag.to_string(), u.reg.to_string(), u.plat.to_string(), req);

        if answer.is_none(){
            u.rtg = 6000;
        }
        else {
            u.rtg = answer.clone().unwrap().rating;
        }
        if update {
            roleruler = role_ruler_text(WSSERVER,
                       autor_id,
                       RoleR::rating(u.rtg));
            update_in_db(u.clone()); }



        if u.rtg == 6000 {

            //Ошибка: Такой игрок не найден.
            let botmess = "Такой игрок не найден";
	        EmbedStruct::empty()
		        .title(err_title)
		        .des(botmess)
		        .col(err_color)
		        .send(chanel);
        } else if u.rtg == 0 {
            let aun:BtagData = answer.unwrap();
            //KILOgramM#2947 EU PC Рейтинг отсутствует
            let botmess = format!("{} {} {} Рейтинг отсутствует", u.btag, u.reg, u.plat);
            let des = format!("[Ссылка на профиль]({})", aun.url);
	        EmbedStruct::empty()
		        .title(&botmess)
		        .des(&des)
		        .col(color)
		        .footer((String::new(),&roleruler))
		        .send(chanel);

        } else {
            let aun:BtagData = answer.unwrap();
            let botmess = format!("{} {} {} Рейтинг {}", u.btag, u.reg, u.plat, aun.rating);
            let des = format!("[Ссылка на профиль]({})", aun.url);

            let mut fields_vec: Vec<(String, String , bool)> = Vec::new();
            if hero_list_titles.len()>aun.heroes.len(){}
            else{
                for (enumerat,l) in hero_list_titles.iter().enumerate(){

                    let ref an = aun.heroes[enumerat];
                    let mut itre = an.clone().hero.name_rus();
                    let name = format!("{} {}",l,itre);

                    let mut value = String::new();
                    let mut f = true;
                    if let Some(ref x) = aun.heroes[enumerat].time_played{
                        match x{
                            &Time::Hours(t) => {
                                if !f{value = format!("{},",value)}
                                    else{f=false}
                                value = format!("{}ч.",t);}
                            &Time::Min(t) => {
                                if !f{value = format!("{},",value)}
                                    else{f=false}
                                value = format!("{}мин.",t);}
                            &Time::Sec(t) => {
                                if !f{value = format!("{},",value)}
                                    else{f=false}
                                value = format!("{}сек.",t);}
                            &Time::None => {}
                        }
                    }
                    if let Some(x)= aun.heroes[enumerat].win_perc{
                        if !f{value = format!("{},",value)}
                            else{f=false}

                        value = format!("{} {}% побед",value,x);
                    }

                    if let Some(x)= aun.heroes[enumerat].games_won{
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
		        .thumbnail(&aun.avatar_url)
		        .col(color)
		        .footer((String::new(),&roleruler))
		        .fields(fields_vec)
		        .send(chanel);

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
    fn send(self, chanel: u64){
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
	    Discord::send_embed(chanel,json!(json));
    }
}
/*
pub fn embed(chanel: u64, text: &str, title: &str, des: &str,
         thumbnail: String, col: u64, footer: (String, &str), fields: Vec<(String, String , bool)>,
             author: (&str,&str,&str), url: String, image: String) -> discord::Result<discord::model::Message>{

    return DIS.send_embed(chanel, text, |e| {
        let mut a = e.color(col);
        if !title.is_empty() {a = a.title(title);}
        if !des.is_empty() {a = a.description(des);}
        if !thumbnail.is_empty() {a = a.thumbnail(thumbnail.as_str());}
        if !url.is_empty() {a = a.url(url.as_str());}
        if !image.is_empty() {a = a.image(image.as_str());}

        if !footer.0.is_empty() || !footer.1.is_empty()
            {
                a = a.footer(|f| {
                    let mut foo = f;
                    if !footer.0.is_empty() { foo = foo.icon_url(footer.0.as_str()); }
                    if !footer.1.is_empty() { foo = foo.text(footer.1); }
                    foo
                });
            }
        if !author.0.is_empty() || !author.1.is_empty() || !author.2.is_empty()
            {a = a.author(|au| {
                let mut aut = au;
                if !author.0.is_empty() {aut = aut.name(author.0);}
                if !author.1.is_empty() {aut = aut.url(author.1);}
                if !author.2.is_empty() {aut = aut.icon_url(author.2);}
                aut
            })}
        if fields.len() > 0 {
            a = a.fields(|z| {
                let mut w = z;
                for (name, text, inline) in fields{
                    w = w.field(name.as_str(), text.as_str(), inline);
                }
                w
            });
        }
        a
    });
}

fn embed_builder(e: EmbedBuilder,botmess: &str, des: &str, col: u64, answer: BtagData, hero_list_titles: Vec<&str>) -> EmbedBuilder{

    let mut b = e.title(botmess).description(des).color(col);

    b = b.thumbnail(answer.avatar_url.as_str());
    if hero_list_titles.len() == 0{return b;}

    b.fields(|z| embed_field_builder(z,answer,hero_list_titles))

}

fn embed_field_builder(z: discord::builders::EmbedFieldsBuilder, answer: BtagData, hero_list_titles: Vec<&str>) -> discord::builders::EmbedFieldsBuilder{
    let mut zz = z;

    if hero_list_titles.len()>answer.heroes.len(){ return zz;}

    for (enumerat,l) in hero_list_titles.iter().enumerate(){

        let ref an = answer.heroes[enumerat];
        let mut itre = an.clone().hero.name_rus();
        let name = format!("{} {}",l,itre);

        let mut value = String::new();
        let mut f = true;
        if let Some(ref x) = answer.heroes[enumerat].time_played{
            match x{
                &Time::Hours(t) => {
                    if !f{value = format!("{},",value)}
                    else{f=false}
                    value = format!("{}ч.",t);}
                &Time::Min(t) => {
                    if !f{value = format!("{},",value)}
                    else{f=false}
                    value = format!("{}мин.",t);}
                &Time::Sec(t) => {
                    if !f{value = format!("{},",value)}
                    else{f=false}
                    value = format!("{}сек.",t);}
                &Time::None => {}
            }
        }
        if let Some(x)= answer.heroes[enumerat].win_perc{if !f{value = format!("{},",value)}else{f=false} value = format!("{} {}% побед",value,x);}
        if let Some(x)= answer.heroes[enumerat].games_won{if !f{value = format!("{},",value)}else{f=false} value = format!("{} {} побед(а)",value,x);}
        zz = zz.field(name.as_str(), value.as_str(), false);
    }
    return zz;
}
*/
pub fn embed_from_value(chanel: u64, val: Value){
    Discord::send_embed(chanel,val);
}


enum RoleR{
    rating(u16),
}

enum RoleChange{
    add(String),
    rem(String)
}

fn role_ruler_text(server_id: u64, user_id: u64, cmd: RoleR) -> String{
    let mut answer = String::new();
    let mut removed = Vec::new();
    let mut added = Vec::new();
    for role in role_ruler(server_id, user_id, cmd){
        match role {
            RoleChange::add(s) =>{
                added.push(s);
            }
            RoleChange::rem(s) =>{
                removed.push(s);
            }
        };
    }
    answer = match removed.len() {
        0 =>{
            match added.len() {
                0 =>{
                    String::new()
                }
                1 =>{
                    format!("Добавлена роль \'{}\'", added[0])
                }
                _ =>{
                    let mut temp = "Добавлены роли".to_string();
                    let mut first = true;
                    for r in added{
                        if first {first = false;}
                        else {temp = format!("{},",temp);}
                        temp = format!("{} \'{}\'", temp, r);
                    }
                    temp
                }
            }
        }
        1 =>{
            match added.len() {
                0 =>{
                    format!("Роль \'{}\' убрана", removed[0])
                }
                1 =>{
                    format!("Смена ролей: с \'{}\' на \'{}\'", removed[0], added[0])
                }
                _ =>{
                    let mut temp = format!("Роль \'{}\' заменена ролями", removed[0]);
                    let mut first = true;
                    for r in added{
                        if first {first = false;}
                            else {temp = format!("{},",temp);}
                        temp = format!("{} \'{}\'", temp, r);
                    }
                    temp
                }
            }
        }
        _ =>{
            match added.len() {
                0 =>{
                    let mut temp = format!("Роли");
                    let mut first = true;
                    for r in removed.clone(){
                        if first {first = false;}
                            else {temp = format!("{},",temp);}
                        temp = format!("{} \'{}\'", temp, r);
                    }
                    temp = format!("{} убраны",temp);
                    temp
                }
                1 =>{
                    let mut temp = format!("Роли");
                    let mut first = true;
                    for r in removed.clone(){
                        if first {first = false;}
                            else {temp = format!("{},",temp);}
                        temp = format!("{} \'{}\'", temp, r);
                    }
                    temp = format!("{} заменены ролью \'{}\'", temp, added[0]);
                    temp
                }
                _ =>{

                    let mut temp = format!("Роли");
                    let mut first = true;
                    for r in removed.clone(){
                        if first {first = false;}
                            else {temp = format!("{},",temp);}
                        temp = format!("{} \'{}\'", temp, r);
                    }
                    temp = format!("{} заменены ролями", temp);
                    let mut first = true;
                    for r in added.clone(){
                        if first {first = false;}
                            else {temp = format!("{},",temp);}
                        temp = format!("{} \'{}\'", temp, r);
                    }
                    temp
                }
            }
        }
    };
    return answer;
}

fn role_ruler(server_id: u64, user_id: u64, cmd: RoleR) -> Vec<RoleChange>{
    lazy_static! {
        static ref ROLES: Vec<String> = vec![
            String::from("ТОП500"),
            String::from("Грандмастер"),
            String::from("Мастер"),
            String::from("Алмаз"),
            String::from("Платина"),
            String::from("Золото")
            ];
    }
    let mut answer: Vec<RoleChange> = Vec::new();


	if let Some(member) = Discord::get_member(server_id,user_id){
		if let Some(roles) = Discord::get_roles_list(server_id){
			match cmd{
				RoleR::rating(r) => {
					let mut find_role = String::new();
					let mut done = false;
					let mut change = false;
					let mut new_roles:Vec<Value> = Vec::new();


                    match r{
                        1...2499 =>{ find_role = ROLES.get(5).unwrap().to_string()}
                        2500...2999 =>{ find_role = ROLES.get(4).unwrap().to_string()}
                        3000...3499 =>{ find_role = ROLES.get(3).unwrap().to_string()}
                        3500...3999 =>{ find_role = ROLES.get(2).unwrap().to_string()}
                        4000...4499 =>{ find_role = ROLES.get(1).unwrap().to_string()}
                        4500...5000 =>{ find_role = ROLES.get(0).unwrap().to_string()}
                        _ =>{}
                    }



					'outer: for roleid in member["roles"].as_array().unwrap(){
						'inner: for role in roles.as_array().unwrap(){
							if roleid.as_str().unwrap().eq(role["id"].as_str().unwrap()){
								let mut is_find = false;
								let role_name: &str = role["name"].as_str().unwrap();

								for ROLE in ROLES.clone(){
									if ROLE.eq(role_name){
										is_find = true;
										if find_role.eq(role_name){
											done = true;
											new_roles.push(roleid.clone());
										}
											else {
												change = true;
												answer.push(RoleChange::rem(ROLE));
											}

									}
								}
								if is_find == false{
									new_roles.push(roleid.clone());
								}
								break 'inner;
							}
						}
					}
					if !done || change {
						if !done{
							for role in roles.as_array().unwrap(){
								let role_name: &str = role["name"].as_str().unwrap();
								if find_role.eq(role_name){
									answer.push(RoleChange::add(find_role));
									new_roles.push(role["id"].clone());
									break;
								}
							}
						}

						Discord::set_member_roles(server_id,user_id,new_roles);

					}

				}
			}
		}
	}
    return answer;
}

fn main() {
    let dcshell:DCShell = Discord::get_event_reciever();

    DB.ini_embeds_s();
    DB.ini_lfg();
    DB.ini_chat();
    EVENT.send(EventChanel::Check);
    println!("[Status] Main loop start");
    println!("{}", START_TIME.ctime());

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
                                Discord::send_typing(mes.channel_id);
                                wsstats(mes_split.clone(), mes.author.id, mes.channel_id);
                            }

                            "!wstour" => {
                                DB.send_embed("tourneys",mes.channel_id);
                            }

                            "!wshelp" => {
                                DB.send_embed("help",mes.channel_id);
                            }
                            "!wscmd" => {
                                DB.send_embed("cmd",mes.channel_id);
                            }
                            "!wslfg" => {
                                lfg_none(mes.clone());
                            }
                            _ => {}
                        }


                        //ADMIN COMMANDS

                        if mes.author.id == 193759349531869184 || mes.author.id == 222781446971064320{
                            match mes_split[0].to_lowercase().as_str() {
                                "!ahelp" => {
                                    DB.send_embed("admin_commands",mes.channel_id);
                                }

                                "!event" =>{

                                    match mes_split.get(1){
                                        Some(&"add") =>{
                                            //11

                                            let mut data = mes.content.clone();

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
                                                    if push || (i == num_elements){
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
                                                //println!("{:?}",tmalt.to_tm());
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
                                    delete_user(mes.author.id);
                                }
                                "!test3" =>{


                                }
                                "!ini" =>{
                                    if mes_split.len() > 1{
                                        match mes_split[1].to_lowercase().as_str(){
                                            "embed" => {
                                                DB.ini_embeds_s();
                                                Discord::send_mes(mes.channel_id, "Embed-ы инициализированы", "", false);
                                            }
                                            "lfg" => {
                                                DB.ini_lfg();
                                                Discord::send_mes(mes.channel_id, "Вектор LFG инициализирован", "", false);
                                            }
                                            "chat" => {
                                                DB.ini_chat();
                                                Discord::send_mes(mes.channel_id, "Вектор Chat инициализирован", "", false);
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
                                                    println!("lfg_rem Err: {}", e);
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
                                                        println!("lfg_rem Err: {}", e);
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

                                "!shver" => {
                                    use std::ops::Add;
                                    use std::ops::Sub;
                                    let start_clone:extime::Tm = START_TIME.clone();

                                    let cur_time = extime::now();
                                    let start_day = match START_TIME.tm_mday{
                                        0...9 =>{ format!("0{}",START_TIME.tm_mday)}
                                        _ => {format!("{}",START_TIME.tm_mday)}
                                    };
                                    let start_mon = match START_TIME.tm_mon+1{
                                        0...9 =>{ format!("0{}",START_TIME.tm_mon+1)}
                                        _ => {format!("{}",START_TIME.tm_mon+1)}
                                    };
                                    let start_h = match START_TIME.tm_hour{
                                        0...9 =>{ format!("0{}",START_TIME.tm_hour)}
                                        _ => {format!("{}",START_TIME.tm_hour)}
                                    };
                                    let start_m = match START_TIME.tm_min{
                                        0...9 =>{ format!("0{}",START_TIME.tm_min)}
                                        _ => {format!("{}",START_TIME.tm_min)}
                                    };
                                    let start_s = match START_TIME.tm_sec{
                                        0...9 =>{ format!("0{}",START_TIME.tm_sec)}
                                        _ => {format!("{}",START_TIME.tm_sec)}
                                    };

                                    let dur_time = cur_time - start_clone;
                                    let mut dif_time = dur_time.num_seconds();



                                    let up_d = dif_time / 86400;
                                    dif_time = dif_time - (up_d * 86400);


                                    let up_h = dif_time / 3600;
                                    dif_time = dif_time - (up_h * 3600);
                                    let up_hour = match up_h{
                                        0...9 =>{ format!("0{}",up_h)}
                                        _ => {format!("{}",up_h)}
                                    };

                                    let up_m = dif_time / 60;
                                    dif_time = dif_time - (up_m * 60);
                                    let up_min = match up_m{
                                        0...9 =>{ format!("0{}",up_m)}
                                        _ => {format!("{}",up_m)}
                                    };

                                    let up_sec = match dif_time{
                                        0...9 =>{ format!("0{}",dif_time)}
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
                        /*if let Ok(discord::model::Channel::Private(_)) = DIS.get_channel(mes.channel_id){
                            if let Some(c) = DB.get_chat(mes.author.id.0){
                                match c{
                                    Chat::LFG(stage) => {lfg(mes,stage);}
                                }
                            }
                        }*/
//						if let Some(c) = DB.get_chat(mes.author.id){
//                                match c{
//                                    Chat::LFG(stage) => {lfg(mes,stage);}
//                                }
//                            }
                    }
                });
            }
            /*
                        Event::ServerCreate(x) => {
                            match x {
                                discord::model::PossibleServer::Offline(_) => {}
                                discord::model::PossibleServer::Online(y) => {
                                    if y.name.eq("Bobin\'sTestPoligon") {
                                        insert("BobinServerId",&format!("{:?}",y.id.0))
                                    }
                                }
                            }
                        }
            */
            _ => {}
            //let m: String = format!("{:?}", event);
            //event_eater(m);}
            //println!("[Some Event] {:?}", event);} // discard other known events



            //END OF MAIN THREAD
        }
    }
}



