use std::sync::mpsc::{Receiver,Sender,channel};
use std::time::{Instant, Duration};
use std::ops::Deref;

use std::sync::{Mutex,
                mpsc::{
                    SendError,
                    TryRecvError,
                    RecvError
                }
};

use serde_json::Value;


use dis::shell;
use denum::{OutLink,UniChanel,GlobE};

use websocket::{Message, OwnedMessage};

pub struct DoubleChanel<T: Default>{
    pub sender: Sender<T>,
    pub reciver: Receiver<T>
}
impl<T> DoubleChanel<T> where T: Default{
    pub fn new() -> (DoubleChanel<T>, DoubleChanel<T>){
        let (s1, r1) = channel::<T>();
        let (s2, r2) = channel::<T>();
        let dc1 = DoubleChanel{
            sender: s1,
            reciver: r2
        };
        let dc2 = DoubleChanel{
            sender: s2,
            reciver: r1
        };
        return (dc1, dc2);
    }
    fn new_self() -> DoubleChanel<T>{
        let (s1, r1) = channel::<T>();
        let dc1 = DoubleChanel{
            sender: s1,
            reciver: r1
        };
        return dc1;
    }
    pub fn send(&self, data: T) -> Result<(), SendError<T>>{
        self.sender.send(data)
    }
    fn recv(&self) -> Result<T, TryRecvError>{
        self.reciver.try_recv()
    }
    pub fn recv_simp(&self) -> T{
        match self.reciver.try_recv(){
            Ok(data) => {
                return data;
            }
            Err(_) =>{
                return Default::default();
            }
        }
    }
	pub fn recv_sleep_simp(&self) -> T{
		match self.reciver.recv(){
			Ok(data) => {
				return data;
			}
			Err(e) =>{
				panic!("[DC Receive] Error. Chanel Dead:\n{}",e);
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct DMessage{
    pub id: u64,
    pub channel_id: u64,
    pub author: DUser,
    pub content: String,

}
impl DMessage{
    pub fn empty() -> Self{
        DMessage{
            id: 0,
            channel_id: 0,
            author: DUser::empty(),
            content: String::new(),
        }
    }
    pub fn id(self, id: u64) -> Self{
        let mut s = self;
        s.id = id;
        s
    }
    pub fn channel_id(self, channel_id: u64) -> Self{
        let mut s = self;
        s.channel_id = channel_id;
        s
    }
    pub fn author(self, author: DUser) -> Self{
        let mut s = self;
        s.author = author;
        s
    }
    pub fn content(self, content: &str) -> Self{
        let mut s = self;
        s.content = content.into();
        s
    }
}

#[derive(Clone, Debug)]
pub struct DUser{
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}
impl DUser{
    pub fn empty() -> Self{
        DUser{
             id: 0,
             username: String::new(),
             discriminator: String::new(),
             avatar: String::new(),
        }
    }
    pub fn id(self, id: u64) -> Self{
        let mut s = self;
        s.id = id;
        s
    }
    pub fn username(self, username: &str) -> Self{
        let mut s = self;
        s.username = username.into();
        s
    }
    pub fn discriminator(self, discriminator: &str) -> Self{
        let mut s = self;
        s.discriminator = discriminator.into();
        s
    }
    pub fn avatar(self, avatar: &str) -> Self{
        let mut s = self;
        s.avatar = avatar.into();
        s
    }
    pub fn avatar_raw(self, avatar: &str) -> Self{
        let mut s = self;
        s.avatar = format!("https://cdn.discordapp.com/avatars/{}/{}",&s.id,avatar);
        s
    }
}

pub struct DServerBig{
	pub id: u64,
	pub name: String,

}

pub struct DiscordMain{
    dc: Mutex<DoubleChanel<GlobE>>,
	token: String,
}

impl DiscordMain{
    pub fn new(token: String) -> DiscordMain{
        use std::thread;
        let (dc_sender, dc_reciever) = DoubleChanel::<GlobE>::new();
	thread::Builder::new()
            .name("Websocket".to_string())
            .spawn(move || shell(dc_reciever));
        DiscordMain{
            dc: Mutex::new(dc_sender),
	        token,
        }
    }

	pub fn get_token(&self) -> &String{
		&self.token
	}

    pub fn get_chanel(&self) -> DoubleChanel<OutLink>{
        let (dc_sender, dc_reciever) = DoubleChanel::<OutLink>::new();
        use std::ops::Deref;
        loop{
            match self.dc.try_lock(){
                Ok(r) => {
                    let r = r.deref();
                    r.send(GlobE::GetChanel(dc_reciever));
                    break;
                }
                _=>{}
            }
        }
        return dc_sender;
    }



    fn recv_wait_simp(&self) -> GlobE{
        use std::ops::Deref;
        loop{
            match self.dc.try_lock(){
                Ok(r) => {
                    let r = r.deref();
                    return r.recv_sleep_simp();
                }
                _=>{return GlobE::None;}
            }
        }
    }
}
impl Drop for DiscordMain{
    fn drop(&mut self){
        loop{
            match self.dc.try_lock(){
                Ok(r) => {
                    let r = r.deref();
                    r.send(GlobE::Drop);
                    let instatin = Instant::now();
                    let dur = Duration::from_secs(1);
                    loop{
                        match r.recv_simp() {
                            GlobE::Drop => {
                                return;
                            }
                            _ =>{
                                let d = instatin.elapsed();
                                if dur.ge(&d){
                                    return;
                                }
                            }
                        }
                    }


                    break;
                }
                _=>{}
            }
        }
    }
}

pub struct DCShellConf{
    pub send_events: bool,

}

pub struct DCShell{
    dc: DoubleChanel<OutLink>,
    conf: DCShellConf
}
impl DCShell{
    pub fn from_dc(dc: DoubleChanel<OutLink>)
        -> Self{
        let dconf = DCShellConf{
            send_events: false,

        };

        DCShell{
            dc,
            conf: dconf
        }
    }
	pub fn get(&self) -> OutLink{
		self.dc.recv_simp()
	}

	pub fn get_wait(&self) -> OutLink{
		self.dc.recv_sleep_simp()
	}

	pub fn send(&self, data: OutLink){
		let _ = self.dc.send(data);
	}

}


pub struct HBeat{
    time: Instant,
    interval: u64,
    pub switch: bool,
}
impl HBeat {
    pub fn new() -> HBeat {
        HBeat {
            time: Instant::now(),
            interval: 0,
            switch: false,
        }
    }
    pub fn set(&mut self, interval: u64) {
        self.interval = interval -100;
        self.switch = true;
        self.time = Instant::now();
    }
    pub fn refresh(&mut self) {
        self.time = Instant::now();
    }
    pub fn is_time(&self) -> bool {
        if !self.switch { return false; }
        let duration = self.time.elapsed();
        let mut mil = duration.as_secs() * 1000;
        mil += (duration.subsec_nanos() / 1_000_000) as u64;
        if mil < self.interval {
            return false;
        }
        return true;
    }
    pub fn stop(&mut self) {
        self.switch = false;
    }
    pub fn start(&mut self) {
        self.switch = true;
    }
}
