
use indexmap::map::IndexMap;
use serde_json;
use serde_json::Value;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

const RATING_CONF_PATH: &'static str = "rating_conf.json";


lazy_static!{
    static ref CONF: Config = Config::new();
}




#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ConfType{
	rating,
}


pub struct Config{
	map: RwLock<IndexMap<ConfType, Value>>,
}
impl Config{
	fn new() -> Config{
		Config{
			map: RwLock::new(IndexMap::new())
		}
	}

	pub fn init(){
		use std::fs::File;
		use std::io::Read;
		let mut map: IndexMap<ConfType,Value> = IndexMap::new();
		if let Ok(mut rating) = File::open(RATING_CONF_PATH){
			let mut raw = String::new();
			match rating.read_to_string(&mut raw) {
				Err(e) => {
					println!("Config>Init>Rating>File>Read Error [{}]: {:?}", RATING_CONF_PATH, e);
				}
				_ => {
					match serde_json::from_str(raw.as_str()){
						Ok(json) => {
							map.insert(ConfType::rating,json);
						}
						Err(e) => {
							println!("Config>Init>Rating>Serde Error in [{}]: {:?}", RATING_CONF_PATH, e);
						}
					}
				}
			}
		}
		else {
			println!("Config>Init>Rating>File>Open Error [{}]", RATING_CONF_PATH);
		}




		loop{
			match CONF.map.write() {
				Ok(mut conf) => {
					*conf = map;
					break;
				}
				_ => {}
			}
		}

		println!("[Config] Init done");
	}

	pub fn get(t: ConfType, path: String) -> Option<Value>{
		use std::ops::Deref;
		loop{
			match CONF.map.read() {
				Ok(conf) => {
					let conf = conf.deref();
					match conf.get(&t) {
						None => {return None;}

						Some(j) => {
							match j.pointer(path.as_str()) {
								None => { return None;}

								Some(val) => {
									return Some(val.clone());
								}
							}
						}
					};

				}
				_ => {}
			}
		}
	}

	pub fn get_root(t: ConfType) -> Option<Value>{
		use std::ops::Deref;
		loop{
			match CONF.map.read() {
				Ok(conf) => {
					let conf = conf.deref();
					match conf.get(&t) {
						None => {return None;}

						Some(j) => {
							return Some(j.clone());
						}
					};

				}
				_ => {}
			}
		}
	}

	pub fn set_in_file(t: ConfType, json: Value){
		use std::fs::OpenOptions;
		use std::io::Write;

		let file = match t {

			ConfType::rating => {

				if let Ok(mut rating) = OpenOptions::new()
														.write(true)
														.create(true)
														.truncate(true)
														.open(RATING_CONF_PATH){

					match rating.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes()) {
						Err(e) => {
							println!("Config>Set>Rating>File>Write Error [{}]: {:?}", RATING_CONF_PATH, e);
						}
						_ => {}
					}
				}
					else {
						println!("Config>Set>Rating>File>Open Error [{}]", RATING_CONF_PATH);
					}
			}
		};



	}

	pub fn exec_fn<F>(func: F)
		where F: Fn(&mut RwLockWriteGuard<IndexMap<ConfType, Value>>) {
		loop{
			match CONF.map.write() {
				Ok(mut conf) => {
					func(&mut conf);
					return;
				}
				_ => {}
			}
		}
	}
}