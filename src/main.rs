#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate regex;
extern crate reqwest;
//extern crate rusqlite;
extern crate mysql;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

//https://discordapp.com/api/oauth2/authorize?client_id=316281967375024138&scope=bot&permissions=0
use discord::{Discord, ChannelRef, State};
use discord::model::Event;
use regex::Regex;
use std::io::Read;
//use std::env;
//use rusqlite::Connection;


use std::{thread, time, fmt};

use std::time::{Duration, Instant, SystemTime};
use std::fmt::Debug;
use std::sync::mpsc::channel;
use mysql::from_row;

lazy_static! {
    static ref DIS: discord::Discord = Discord::from_bot_token(load_settings().as_str()).expect("login failed");
    static ref POOL: mysql::conn::pool::Pool = mysql::Pool::new(build_opts()).unwrap();
    static ref REG_BTAG: Regex = Regex::new(r"^.{2,16}#[0-9]{2,6}$").expect("Regex btag error");
    static ref REG_TIME: Regex = Regex::new(r"(?P<n>\d){1,4} ?(?i)(?P<ntype>m|min|h|hour)").expect("Regex btag error");
}

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
struct User {
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
    fn empty() -> User {
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
    builder.user(Some("bot")).pass(Some("1234")).db_name(Some("wsowbot"));
    return mysql::Opts::from(builder);
}

fn load_overwatch_rating(btag: String, reg: String, plat: String) -> u16 //Проверка существование профил и подгрузка рейтинга при наличее
{
    lazy_static! {
        static ref RE: Regex = Regex::new("<div class=\"u-align-center h5\">(\\d+)</div>").unwrap();
    }

    let url = &format!("https://playoverwatch.com/en-us/career/{}/{}/{}", plat.to_lowercase(), reg.to_lowercase(), btag.replace("#", "-"));
    //println!("сам урл есть - {}", &url);
    let mut resp = reqwest::get(url).expect("Wrong url");
    //println!("Запрос УРЛ успешен");
    let mut content = String::new();
    //println!("новая строка?");
    resp.read_to_string(&mut content).expect("OW player page downloading error");
    //println!("весь контент страницы в строке?");
    if content.contains("<h1 class=\"u-align-center\">Page Not Found</h1>") {
        return 6000;
    }
    let result = RE.captures(&content);
    if result.is_none() {
        return 0;
    }
    return result.unwrap().get(1).unwrap().as_str().parse::<u16>().unwrap();
    //println!("нашли б таг в строке");
}
fn load_overwatch_comphero_played(btag: String, reg: String, plat: String) -> u16 //Проверка существование профил и подгрузка рейтинга при наличее
{
    lazy_static! {
        static ref RE: Regex = Regex::new("<div id=\"competitive\"(.*)</div></div></div></div>").unwrap();
        static ref HERO: Regex = Regex::new("0x02E0000000000(.*)\">").unwrap();
    }

    let url = &format!("https://playoverwatch.com/en-us/career/{}/{}/{}", plat.to_lowercase(), reg.to_lowercase(), btag.replace("#", "-"));
    //println!("сам урл есть - {}", &url);
    let mut resp = reqwest::get(url).expect("Wrong url");
    //println!("Запрос УРЛ успешен");
    let mut content = String::new();
    //println!("новая строка?");
    resp.read_to_string(&mut content).expect("OW player page downloading error");
    //println!("весь контент страницы в строке?");
    if content.contains("<h1 class=\"u-align-center\">Page Not Found</h1>") {
        return 6000;
    }
    let result = HERO.captures(&content);
    if result.is_none() {
        return 0;
    }
    return result.unwrap().get(1).unwrap().as_str().parse::<u16>().unwrap();
    //println!("нашли героя в строке);
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

fn load_by_id(id: u64) -> User //Получение профиля из базы по DiscordId
{
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("SELECT did, name, disc, btag, rtg, reg, plat, scrim_preset, rtg_preset FROM users WHERE did = {}", &id);
    let mut stmt = conn.prepare(command).unwrap();
    let mut u = User::empty();

    for row in stmt.execute(()).unwrap() {
        let (udid, uname, udisc, ubtag, urtg, ureg, uplat, scrim_preset, rtg_preset) = mysql::from_row::<(u64, String, String, String, u16, String, String, String, String)>(row.unwrap());
        u.did = udid;
        u.name = uname;
        u.disc = udisc;
        u.btag = ubtag;
        u.rtg = urtg;
        u.reg = ureg;
        u.plat = uplat;
        u.scrim_preset = serde_json::from_str(&scrim_preset).unwrap();
        u.rtg_preset = serde_json::from_str(&rtg_preset).unwrap();
    }
    return u;
}

fn load_settings() -> String //Загрузка DiscordId
{
    let mut ta = String::new();
    let mut stmt = POOL.prepare("SELECT distoken FROM bottoken").unwrap();
    for row in stmt.execute(()).unwrap() {
        ta = from_row::<String>(row.unwrap());
    }
    return ta;
}


fn user_exist(id: discord::model::UserId) -> bool //Проверка существования профиля в базе
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

fn delete_user(id: discord::model::UserId) //Удаление рпофиля (пока только для тестов)
{
    let mut conn = POOL.get_conn().unwrap();
    let command = format!("DELETE FROM users WHERE did = {}", &id);
    let mut stmt = conn.prepare(command).unwrap();
    //let mut answer: bool = false;
    let _ = stmt.execute(());
}

fn reg_check(id: discord::model::UserId) -> bool //Проверка наличия профиля и BattleTag у профиля в БД
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

fn reg_user(mut reg_str: Vec<&str>, autor: discord::model::User) //Диалог создания профиля
{
    let mut battletag: String = String::new();
    let mut region: String = String::new();
    let mut platform: String = String::new();
    let mut rating: u16 = 0;
    let mut unnone = false;
    let mut botmess: String = String::new();

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
            rating = match load_overwatch_rating(battletag.to_string(), region.to_string(), platform.to_string()) {
                6000 => {
                    acc_not_found = true;
                    0
                }
                x => { x }
                _ => { 0 }
            };
        }

        botmess.push_str("Теперь Вы с нами!");
        if no_btag || no_plat || no_reg {
            botmess.push_str("\nНо, к сожалению, мы не нашли ваш");

            match (no_btag, no_plat, no_reg) {
                (true, true, true) => { botmess.push_str(" BattleTag, платформу и регион"); }
                (true, true, false) => { botmess.push_str(" BattleTag и платформу"); }
                (true, false, true) => { botmess.push_str(" BattleTag и регион"); }
                (true, false, false) => { botmess.push_str(" BattleTag"); }
                (false, true, true) => { botmess.push_str("у платформу и регион"); }
                (false, true, false) => { botmess.push_str("у платформу"); }
                (false, false, true) => { botmess.push_str(" регион"); }
                _ => {}
            }
            botmess.push_str(" в вашем сообщении.");
            if acc_not_found {
                botmess.push_str("\nТакже мы не смогли найти ваш профиль Overwtach по заданным параметрам. Возможно вы ошиблись или указали недостаточно данных.");
                botmess.push_str("\nВы можете добавить их позже с помощью комманды \n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
            } else {
                if no_reg { botmess.push_str("\nРегион установлен по умолчанию: EU"); }
                if no_plat { botmess.push_str("\nПлатформа установленна по умолчанию: PC"); }
                if rating > 0 { botmess.push_str(format!("\nВаш рейтинг определён: {}", rating).as_str()); }
                botmess.push_str("\nИзменить BattleTag, Регион и Платформу вы можете повторно введя комманду \n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
            }
        } else {
            if acc_not_found {
                botmess.push_str("\nТакже мы не смогли найти ваш профиль Overwtach по заданным параметрам. Возможно вы ошиблись или указали недостаточно данных.");
                botmess.push_str("\nВы можете добавить их позже с помощью комманды \n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
            } else {
                botmess.push_str("\nВаш аккаунт успешно найден");
                if rating > 0 { botmess.push_str(format!("\nВаш рейтинг определён: {}", rating).as_str()); }
                botmess.push_str("\nИзменить BattleTag, Регион и Платформу вы можете повторно введя комманду \n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
            }
        }

        if acc_not_found {
            let mut temp_user = User::empty();
            temp_user.did = autor.id.0;
            temp_user.name = autor.name;
            temp_user.disc = autor.discriminator.to_string();
            add_to_db(temp_user);
        } else {
            let mut temp_user = User::empty();
            temp_user.did = autor.id.0;
            temp_user.name = autor.name;
            temp_user.disc = autor.discriminator.to_string();
            temp_user.btag = battletag.to_string();
            temp_user.rtg = rating;
            temp_user.reg = region.to_string();
            temp_user.plat = platform.to_string();
            add_to_db(temp_user);
        }
    } else {
        botmess.push_str("Теперь вы с нами!");
        botmess.push_str("\nВы не указали никакой информации, но вы всегда можете добавить их с помощью комманды \n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");

        let mut temp_user = User::empty();
        temp_user.did = autor.id.0;
        temp_user.name = autor.name;
        temp_user.disc = autor.discriminator.to_string();

        add_to_db(temp_user);
    }
    let _ = DIS.send_message(DIS.create_private_channel(autor.id).unwrap().id, botmess.as_str(), "", false);
}

fn edit_user(mut reg_str: Vec<&str>, autor: discord::model::User) //Диалог на запрос редактирования профиля
{
    let mut battletag: String = String::new();
    let mut region: String = String::new();
    let mut platform: String = String::new();
    let user = load_by_id(autor.id.0);
    let mut rating: u16 = 0;
    let mut unnone = false;
    let mut botmess: String = String::new();
    let mut force: bool = false;


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
            botmess = "К сожалению, не удалось определить праметры заданные вами.".to_string();
            botmess.push_str("\nУбедитесь, что верно ввели парамтры на изменение ваших данных\n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
        } else {
            if !no_btag && !battletag.eq(&user.btag) { new_data = true; }
            if !no_plat && !platform.eq(&user.plat) { new_data = true; }
            if !no_reg && !region.eq(&user.reg) { new_data = true; }
            if new_data {
                if battletag.is_empty() {
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

                    update_in_db(temp_user);

                    botmess = "Мы обновили ваши данные.".to_string();
                    botmess.push_str("\nК сожалению, мы не сможем узнать ваш рейтинг без указания BattleTag.");
                    botmess.push_str(format!("\nРегион: {}", region).as_str());
                    botmess.push_str(format!("\nПлатформа: {}", platform).as_str());
                    botmess.push_str("\nУбедитесь, что верно ввели парамтры на изменение ваших данных");
                } else {
                    rating = match load_overwatch_rating(battletag.clone(), region.clone(), platform.clone()) {
                        6000 => {
                            acc_not_found = true;
                            0
                        }
                        x => { x }
                        _ => { 0 }
                    };
                    if acc_not_found {
                        if force {
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

                            update_in_db(temp_user);
                            botmess = "Мы принудително обновили ваши данные.".to_string();
                            botmess.push_str(format!("\nBattleTag: {}", battletag).as_str());
                            botmess.push_str(format!("\nРегион: {}", region).as_str());
                            botmess.push_str(format!("\nПлатформа: {}", platform).as_str());
                            botmess.push_str("\nУбедитесь, что верно ввели парамтры на изменение ваших данных");
                        } else {
                            botmess = "Мы не смогли найти ваш профиль Overwatch по заданным параметрам. Возможно вы ошиблись или указали недостаточно данных.".to_string();
                            botmess.push_str(format!("\nBattleTag: {}", battletag).as_str());
                            botmess.push_str(format!("\nРегион: {}", region).as_str());
                            botmess.push_str(format!("\nПлатформа: {}", platform).as_str());
                            botmess.push_str("\nУбедитесь, что верно ввели парамтры на изменение ваших данных\n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
                            botmess.push_str("\nНо если вы настаиваете, то добавте FORCE в конец, для изменения данных.");
                        }
                    } else {
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

                        update_in_db(temp_user);
                        botmess = "Мы обновили ваши данные.".to_string();
                        botmess.push_str(format!("\nBattleTag: {}", battletag).as_str());
                        botmess.push_str(format!("\nРегион: {}", region).as_str());
                        botmess.push_str(format!("\nПлатформа: {}", platform).as_str());
                        if rating > 0 { botmess.push_str(format!("\nРейтинг: {}", rating).as_str()); }
                        botmess.push_str("\nУбедитесь, что верно ввели парамтры на изменение ваших данных");
                    }
                }
            } else {
                botmess = "Ваши текущие данные совпадают с введёнными.".to_string();
                botmess.push_str("\nУбедитесь, что верно ввели парамтры на изменение ваших данных\n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
            }
        }
        if unnone {
            botmess.push_str("\nНе все настройки удалось определить, перепроверьте ваше сообщение.");
        }
    } else {
        botmess = "Вы уже зарегестрированны.".to_string();
        botmess.push_str("\nЧто бы добаваить или изменить данные о вашем профиле, укажите их вместе с командой\n```markdown\n!wsreg {Ваш BTag} {Регион EU|US|KR} {Платформа PC|P4|XB}\n```");
    }
    let _ = DIS.send_message(DIS.create_private_channel(autor.id).unwrap().id, botmess.as_str(), "", false);
}


fn scrim_starter(mut mes: &str, autor: discord::model::User) //Отправная функция по всем запросам касательно скримов
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
    let is_reg = reg_check(autor.id);
    let user: User = match is_reg {
        true => { load_by_id(autor.id.0) }
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
}

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
}

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
    let mut str = String::new();
    let mut call = format!("SELECT var FROM variables WHERE");
    call = format!("{} name='{}'", call, name);
    let mut stmt = POOL.prepare(call.as_str()).unwrap();
    for row in stmt.execute(()).unwrap() {
        str = from_row::<String>(row.unwrap());
    }
    return str;
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

fn main() {
    let (mut connection, ready) = DIS.connect().expect("connect failed");

    let mut state_t = State::new(ready);
    println!("[Status] Ready");
    loop {
        let event = match connection.recv_event() {
            Ok(event) => event,
            Err(discord::Error::Closed(code, body)) => {
                println!("[Error] Connection closed with status {:?}: {}", code, body);
                break
            }
            Err(err) => {
                println!("[Warning] Receive error: {:?}", err);
                continue
            }
        };
        state_t.update(&event);

        match event {
            Event::MessageCreate(message) => {
                /*                    match state_t.find_channel(message.channel_id) {
                //
                //                        Some(ChannelRef::Public(server, channel)) => {
                //
                //                            let  mes = format!("[{} #{}] {}: {}", server.name, channel.name, message.author.name, message.content);
                //                            //println!();
                //                            tx.send(
                //                                Container{
                //                                message: mes.to_string(),
                //                                chanel_id: message.channel_id}
                //                            ).unwrap();
                //
                //                        }
                //                        Some(ChannelRef::Group(group)) => {
                //                            println!("[Group {}] {}: {}", group.name(), message.author.name, message.content);
                //                        }
                //                        Some(ChannelRef::Private(channel)) => {
                //                            if message.author.name == channel.recipient.name {
                //                                println!("[Private] {}: {}", message.author.name, message.content);
                //                            } else {
                //                                println!("[Private] To {}: {}", channel.recipient.name, message.content);
                //                            }
                //                        }
                //                        None => println!("[Unknown Channel] {}: {}", message.author.name, message.content),
                //                    }*/

                if message.content.as_str().starts_with('!') {
                    let mut mes: discord::model::Message = message.clone();

                    thread::spawn(move || {
                        let mes_split: Vec<&str> = mes.content.as_str().split_whitespace().collect();
                        match mes_split[0].to_lowercase().as_str() {
                            "!wsreg" => {
                                match reg_check(mes.author.id) {
                                    false => {
                                        reg_user(mes_split, mes.author);
                                    }
                                    true => { edit_user(mes_split, mes.author); }
                                }
                            }
                            "!wsstats" => {
                                let mut err_end = false;
                                let mut u = User::empty();
                                let mut m = User::empty();
                                let mut update = false;
                                if mes_split.capacity() > 1 {
                                    m = get_arg_from_mes(mes_split);
                                    u = load_by_id(mes.author.id.0);
                                    if u.btag.is_empty() && m.btag.is_empty() {
                                        let botmess = "Ошибка: Вы не указали BTag при регистрации и в сообщении.";
                                        let _ = DIS.send_message(message.channel_id, botmess, "", false);
                                        err_end = true;
                                    }
                                    else if u.btag == m.btag {
                                        if u.plat == m.plat && u.reg == m.reg{ update = true;}
                                        else { u = m;}

                                    } else {

                                        if m.btag.is_empty() && m.plat.is_empty() && m.reg.is_empty(){
                                            let botmess = "Ошибка: Параметры не распознаны.";
                                            let _ = DIS.send_message(message.channel_id, botmess, "", false);
                                            err_end = true;
                                        }
                                        else {
                                            if !m.btag.is_empty() { u.btag = m.btag;}
                                            if !m.plat.is_empty() { u.plat = m.plat;}
                                            if !m.reg.is_empty() { u.reg = m.reg;}
                                        }

                                    }

                                }
                                else {
                                    match reg_check(mes.author.id) {
                                        false => {
                                            let botmess = "Ошибка: Вы не зарегестрированы и не указали BTag в сообщении.";
                                            let _ = DIS.send_message(message.channel_id, botmess, "", false);
                                            err_end = true;
                                        }
                                        true => {
                                            u = load_by_id(mes.author.id.0);
                                            if u.btag.is_empty() {
                                                let botmess = "Ошибка: Вы не указали BTag при регистрации и в сообщении.";
                                                let _ = DIS.send_message(message.channel_id, botmess, "", false);
                                                err_end = true;
                                            }
                                            else { update = true; }
                                        }
                                    }
                                }
                                if !err_end {
                                    if u.plat.is_empty() { u.plat = "PC".to_string(); }
                                    if u.reg.is_empty() { u.reg = "EU".to_string(); }

                                    u.rtg = match load_overwatch_rating(u.btag.to_string(), u.reg.to_string(), u.plat.to_string()) {
                                        6000 => {
                                            6000
                                        }
                                        x => { x }
                                        _ => { 0 }
                                    };
                                    if update { update_in_db(u.clone()); }
                                    if u.rtg == 6000 {
                                        let botmess = "Ошибка: Такой игрок не найден.";
                                        let _ = DIS.send_message(message.channel_id, botmess, "", false);
                                    } else if u.rtg == 0 {
                                        let botmess = format!("Рейтинг - Отсутствует, регион - {}, платформа {}", u.reg, u.plat);
                                        let _ = DIS.send_message(message.channel_id, botmess.as_str(), "", false);
                                    } else {
                                        let botmess = format!("Рейтинг акаунта {} - {}, регион - {}, платформа {}", u.btag, u.rtg, u.reg, u.plat);
                                        let _ = DIS.send_message(message.channel_id, botmess.as_str(), "", false);
                                    }

                                    let hero = load_overwatch_comphero_played(u.btag.to_string(), u.reg.to_string(), u.plat.to_string());
                                    let botmess = format!("Основные герои: {}", hero);
                                    let _ = DIS.send_message(message.channel_id, botmess.as_str(), "", false);
                                }
                                    }
                            "!wsscrim" => {
                                scrim_starter(mes.content.as_str(), mes.author);
                            }

                            "!bothelp" => {
                                let botmess = "Коммамнда !bothelp";
                                let _ = DIS.send_message(message.channel_id, botmess, "", false);
                            }
                            "!wscmd" => {
                                let wscmd = include_str!("cmd.ws");
                                let _ = DIS.send_message(message.channel_id, wscmd, "", false);
                            }

                            "!wshelp" => {
                                let wscmd = include_str!("help.ws");
                                let _ = DIS.send_message(message.channel_id, wscmd, "", false);
                            }

                            "!botreg" => {}

                            "!test" => {
                                let mut test_user: User = User::empty();
                                test_user.did = mes.author.id.0;
                                test_user.name = mes.author.name;
                                test_user.disc = mes.author.discriminator.to_string();
                                add_to_db(test_user);
                            }
                            "!test2" => {
                                let botmess = ("Ваш рейтинг - {}, регион - {}, платформа {}");
                                let _ = DIS.send_message(message.channel_id, botmess, "", false);
                                delete_user(mes.author.id); }

                            _ => {}
                        }
                    });
                }
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

            Event::Unknown(name, data) => {
                // log unknown event types for later study
                println!("[Unknown Event] {}: {:?}", name, data);
            }
            _ => {}
            //let m: String = format!("{:?}", event);
            //event_eater(m);}
            //println!("[Some Event] {:?}", event);} // discard other known events
        }
    }

    //END OF MAIN THREAD
}