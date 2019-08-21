
use crate::HeroInfoReq;
use crate::OwData;
use crate::BtagData;
use crate::cut_part_of_str;
use crate::Regex;
use crate::find_next_hero;
use crate::Hero;
use crate::HeroStats;
use crate::find_description;



#[derive(Default,Clone,Debug,Serialize,Deserialize)]
pub struct Rating{
	pub tank: Option<u16>,
	pub damage: Option<u16>,
	pub support: Option<u16>,
	fake: bool
}
impl Rating{
	pub fn from1(r: u16) -> Rating{
		Rating{
			tank: Some(r),
			damage: Some(r),
			support: Some(r),
			fake: true
		}
	}
	pub fn empty() -> Rating{
		Rating{
			tank: None,
			damage: None,
			support: None,
			fake: false
		}
	}
	pub fn have_rating(&self) -> bool{
		return self.damage.is_some() || self.tank.is_some() || self.support.is_some();
	}
	pub fn higest_rating(&self) -> u16{
		use std::cmp::max;
		return max(
			self.tank.unwrap_or(0),
			max(
				self.damage.unwrap_or(0),
				self.support.unwrap_or(0)
			)
		);
	}
	pub fn as_simple_str(&self) -> String{
		match self.fake{
			false => { format!("(T: {}, D: {}, S: {})",self.tank.unwrap_or(0),self.damage.unwrap_or(0),self.support.unwrap_or(0))}
			true => { format!("{}",self.tank.unwrap_or(0))}
		}

	}
	pub fn as_str(&self) -> String{
		match self.fake{
			false => {
				let mut text = String::new();
				if let Some(r) = self.tank{
					text.push_str(format!("Танк: {}\n", r).as_str());
				}
				if let Some(r) = self.damage{
					text.push_str(format!("Урон: {}\n", r).as_str());
				}
				if let Some(r) = self.support{
					text.push_str(format!("Поддержка: {}\n", r).as_str());
				}
				text
			}
			true => { format!("{}",self.tank.unwrap_or(0))}
		}

	}
	pub fn as_fields(&self) -> Vec<(String, String, bool)>{
		if self.have_rating(){
			match self.fake{
				false => {
					let mut vec = Vec::new();
					if let Some(r) = self.tank{
						vec.push(("Танк".to_string(), format!("{}",r), true));
					}
					if let Some(r) = self.damage{
						vec.push(("Урон".to_string(), format!("{}",r), true));
					}
					if let Some(r) = self.support{
						vec.push(("Поддержка".to_string(), format!("{}",r), true));
					}
					vec
				}
				true => { vec![("Рейтинг".to_string(), format!("{}",self.tank.unwrap_or(0)), false)]}
			}
		}
		else {
			Vec::new()
		}
	}
}

pub fn load_btag_data_multirole(btag: String, reg: String, plat: String, req:HeroInfoReq) -> OwData //Проверка существования профиля и подгрузка рейтинга при наличии + фикс для ролей
{
	lazy_static! {
        static ref REG_AVATAR: Regex = Regex::new(r#"player-portrait"\s??src="(?P<url>[^"]+?)""#).expect("Regex avatar url error");
        static ref REG_TANK_RANK: Regex = Regex::new(r#"Tank Skill Rating.+?rank-level">(?P<rank>[0-9]{1,4}?)<"#).expect("Regex tank rank error");
		static ref REG_DPS_RANK: Regex = Regex::new(r#"Damage Skill Rating.+?rank-level">(?P<rank>[0-9]{1,4}?)<"#).expect("Regex damage rank error");
		static ref REG_SUP_RANK: Regex = Regex::new(r#"Support Skill Rating.+?rank-level">(?P<rank>[0-9]{1,4}?)<"#).expect("Regex support rank error");

    }
	use std::time::SystemTime;
	use self::OwData::*;
	if btag.is_empty() || plat.is_empty(){
		return NotFound;
	}

	let sys_time_old = SystemTime::now();

	let mode_debug: bool = false;


	if mode_debug{
		println!("Start: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
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
					println!("[load_btag_data] Error while take body:\n{}", e);
				}
			}
		}
		Err(e) => {
			println!("[load_btag_data] Error while get responce from url. Probaly wrong url:\n{}", e);
		}
	}


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
		b_data.rating = Rating::empty();



		if let Some(avatar) = REG_AVATAR.captures(&body){
			b_data.avatar_url = avatar.name("url").expect("Avatar url").as_str().to_string();
		}

//        if mode_debug{
//            println!("Get rating: {:?}", SystemTime::now().duration_since(sys_time_old).unwrap());
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
				let mut temp_rating = Rating::empty();
				if let Some(rating) = REG_TANK_RANK.captures(&body){
					temp_rating.tank = Some(rating.name("rank").expect("Tank rank").as_str().parse::<u16>().expect("Parce err tank rank"));
				}
				if let Some(rating) = REG_DPS_RANK.captures(&body){
					temp_rating.damage = Some(rating.name("rank").expect("damage rank").as_str().parse::<u16>().expect("Parce err damage rank"));
				}
				if let Some(rating) = REG_SUP_RANK.captures(&body){
					temp_rating.support = Some(rating.name("rank").expect("support rank").as_str().parse::<u16>().expect("Parce err support rank"));
				}
				b_data.rating = temp_rating;

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
				println!("End: {:?}", SystemTime::now().duration_since(sys_time_old).expect("Err #19"));
			}
			return Full(b_data);
		}

	}
	else{
		if mode_debug{
			println!("End None: {:?}", SystemTime::now().duration_since(sys_time_old).expect("Err #20"));
		}
		return NotFound;
	}

}