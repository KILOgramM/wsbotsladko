use crate::disapi::Discord;
use serde_json::{Value, Number};
use crate::conf::Config;
use crate::conf::ConfType;
use serde_json::map::Map;
use serenity::model::guild::Role;

pub struct RoleConf{
	pub val: Value,
}
impl RoleConf{

	pub fn servers_iter() -> Map<String, Value>{
		let val = match Config::get_root(ConfType::rating) {
			None => {return Map::new();}

			Some(v) => {v}
		};
		return val.as_object().expect("RoleConfig list error").clone();
	}

	fn by_serv(id: u64) -> RoleConf{
		let val = match Config::get(ConfType::rating, format!("/{}",id)){
			None => {Value::Null}

			Some(v) => {v}
		};

		RoleConf{
			val
		}
	}
	fn find_by_rating(&self, rating: u16) -> (Option<String>, Option<u64>){
		if self.val.is_null(){
			return (None, None);
		}

		let mut some_name = None;
		let mut some_id = None;

		for role in self.val.as_array().expect("Err RolCon#1: expect array"){
			let low = role["low"].as_u64().expect("Err RolCon#2: expect u64");
			let max = role["max"].as_u64().expect("Err RolCon#3: expect u64");
			if low <= rating as u64 && max >= rating as u64{
				if let Some(id) = role["id"].as_u64(){
					some_id = Some(id);
				}
				if let Some(name) = role["name"].as_str(){
					some_name = Some(name.to_string());
				}
				break;
			}
		}

		return (some_name, some_id);
	}
	fn get_list(&self) -> Vec<(Option<String>, Option<u64>)>{
		if self.val.is_null(){
			return Vec::new();
		}
		let mut vec = Vec::new();

		for role in self.val.as_array().expect("Err RolCon#4: expect array"){
			let mut el = (None, None);
			if let Some(id) = role["id"].as_u64(){
				el.1 = Some(id);
			}
			if let Some(name) = role["name"].as_str(){
				el.0 = Some(name.to_string());
			}
			vec.push(el);
		}

		return vec;
	}
	fn all_have_id(&self) -> bool{
		if self.val.is_null(){
			return false;
		}
		for role in self.val.as_array().expect("Err RolCon#5: expect array"){

			if let Some(_) = role.get("id"){
				continue;
			}
			else { return false; }

		}
		return true;
	}

	pub fn merge(&self, other: Vec<Role>, srver: u64) -> Vec<(Option<String>, Option<u64>)>{
		if self.val.is_null(){
			return Vec::new();
		}
		let server = format!("{}",srver);
		let mut list = RoleConf::servers_iter();
		let mut vec = self.get_list();
		'outer: for (i, vec_roleid) in vec.clone().iter().enumerate(){
			if vec_roleid.1.is_some(){continue;}
			'inner: for role in other.iter(){
				if let Some(ref name) = vec_roleid.0{
					if role.name.eq(name){
						vec[i] = (vec_roleid.0.clone(), Some(role.id.0));
						if let Some(s) = list.get_mut(&server){
							let len = s.as_array().unwrap().len();
							for n in 0..len{
								if let Some(a) = s.get_mut(n){
									if let Some(o) = a.as_object_mut(){
										if let Some(m) = o.get("id"){
											if let Some(id) = m.as_u64(){
												if id == role.id.0{

													if let Some(new_name) = &vec_roleid.0 {
														o.insert("name".to_string(),json!(new_name.clone()));

													}

												}
											}
										}
										else {
											if let Some(m) = o.get("name"){
												if let Some(cur_name) = m.as_str(){
													if let Some(inner_name) = &vec_roleid.0 {
														if cur_name.eq(inner_name.as_str()){
															o.insert("id".to_string(),json!(role.id.0));
														}
													}
												}
											}
										}
									}
								}
							}
						}
						break 'inner;
					}
				}
				else {
					warn!("Role #{} don't have name for search\n {:#?}",i,self.val);
					continue 'outer;
				}
			}
		}

		Config::set_in_file(ConfType::rating,Value::Object(list.clone()));
		Config::set_root(ConfType::rating,Value::Object(list));
		return vec;
	}



	/* old merge
	fn merge(&self, other: Value) -> Vec<(Option<String>, Option<u64>)>{
		if self.val.is_null(){
			return Vec::new();
		}
		let mut vec = self.get_list();
		'outer: for (i, vec_roleid) in vec.clone().iter().enumerate(){
			if vec_roleid.1.is_some(){continue;}
			'inner: for role in other.as_array().expect("Err RolCon#6: expect array"){
				if let Some(ref name) = vec_roleid.0{
					if name.eq(role["name"].as_str().expect("Err RolCon#7")){
						vec[i] = (vec_roleid.0.clone(), Some(role["id"].as_str().expect("Err RolCon#8").parse::<u64>().expect("Err RolCon#9: parse error")));
						break 'inner;
					}
				}
				else {
					continue 'outer;
				}
			}
		}

		return vec;
	}
	*/
}

pub enum RoleR{
	rating(u16),
}
#[derive(Debug)]
pub enum RoleChange{
	add(String),
	rem(String)
}

use serenity::http::raw::Http;
use serenity::model::id::GuildId;

pub fn role_ruler_text(cache: impl AsRef<Http>, server_id: u64, user_id: u64, cmd: RoleR) -> String{
	let mut answer = String::new();
	let mut removed = Vec::new();
	let mut added = Vec::new();
	for role in role_ruler(cache, server_id, user_id, cmd){
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

pub fn role_ruler(cache: impl AsRef<Http>, server_id: u64, user_id: u64, cmd: RoleR) -> Vec<RoleChange>{
//	info!("SERV ID [{}]", server_id);
	let mut answer: Vec<RoleChange> = Vec::new();

	match cmd{
		RoleR::rating(r) => {

//			let conf = RoleConf::by_serv(server_id);

			for (id_conf_serv, val) in RoleConf::servers_iter(){
				let id_conf_serv: u64 = id_conf_serv.parse::<u64>().unwrap();
//				info!("---\n {}\n---\n{}\n---\n",id_conf_serv, serde_json::to_string_pretty(&val).unwrap());
				let conf = RoleConf{
					val: val
				};



				if conf.val.is_null(){return answer;}

				if let Ok(member) = cache.as_ref().get_member(id_conf_serv,user_id) {

//					if !member["roles"].is_array() {
//						continue; //Отбрасывает тех кого нет на сервере
//					}

					let roles_list = match conf.all_have_id(){
						true => {conf.get_list()}
						false => {
							conf.merge(cache.as_ref().get_guild_roles(id_conf_serv).expect("Getting guild roles list"),id_conf_serv)

						}
					};

					let have_role = match conf.find_by_rating(r) {
						(option_name, Some(role_id)) => {
							let name = match option_name {
								Some(n) => {n}
								None => {
									let mut name = String::new();
									for r in roles_list.iter(){
										if let Some(i) = r.1{
											if i == role_id{
												if let Some(ref role) = r.0{
													name = role.clone();
												}
												break;
											}
										}
									}
									name
								}
							};
							Some((name,role_id))
						}
						(Some(name), None) => {
							let mut role_id = 0;

							for r in roles_list.iter(){
								if let Some(ref role) = r.0{
									if role.eq(&name){
										if let Some(i) = r.1{
											role_id = i;
										}
										break;
									}
								}
							}
							Some((name,role_id))
						}
						(None, None) => {None}
					};

					let mut already_have_role = false;

					for role in roles_list{
						'inner1: for member_role in member.roles.iter(){
							let member_role_id = member_role.0;
							if member_role_id == role.1.expect("Err RolRul#5"){
								if let Some((_, role_id)) = have_role{
									if role_id == member_role_id{
										already_have_role = true;
									}
										else {
											if let Err(e) = cache.as_ref().remove_member_role(id_conf_serv,user_id,member_role_id){
												warn!("Error #1 while removing member role: {}",e);
											}
//											Discord::rem_member_role(id_conf_serv,user_id,member_role_id);
											if let Some(role_name) = role.0{
												if id_conf_serv == server_id{
													answer.push(RoleChange::rem(role_name));
												}
											}
										}
								}
									else {
										if let Err(e) = cache.as_ref().remove_member_role(id_conf_serv,user_id,member_role_id){
											warn!("Error #2 while removing member role: {}",e);
										}
//										Discord::rem_member_role(id_conf_serv,user_id,member_role_id);
										if let Some(role_name) = role.0{
											if id_conf_serv == server_id{
												answer.push(RoleChange::rem(role_name));
											}

										}
									}

								break 'inner1;


							}
						}
					}
					if !already_have_role{
						if let Some((name, role_id)) = have_role{
							if let Err(e) = cache.as_ref().add_member_role(id_conf_serv, user_id, role_id){
								warn!("Error while adding member role: {}",e);
							}
//							Discord::add_member_role(id_conf_serv,user_id,role_id);
							if id_conf_serv == server_id{
								answer.push(RoleChange::add(name));
							}

						}

					}

				}

			}



		}
	}

	return answer;
}
