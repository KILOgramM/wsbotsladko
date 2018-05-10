use std::thread;
use std::time::Duration;
use websocket::{Message,
                message::OwnedMessage,
                result::{WebSocketError,
                         WebSocketResult}};
use websocket::client::ClientBuilder;
use websocket::stream::sync::AsTcpStream;
use serde_json::Value;


use reqwest;
use serde_json;
use dstruct::{DoubleChanel,
              HBeat,
              DiscordMain,
              DCShell};
use denum::{UniChanel,
            OutLink,
            LocalLink,
            GlobE};

use std::sync::mpsc::{Receiver,Sender,channel};

use disapi::Discord;
use denum::Event;

const GETGATEWAY: &'static str = "https://discordapp.com/api/v6/gateway";

enum Core{
    Ok,
    SendIdentify,
    SendIdentifyProceed,
    Resume,
    ResumeProceed,
    ReconectClose,
    Reconect,
    CloseConfirm,
}

fn core(dc_shell: DoubleChanel<UniChanel>){
    //переменные

    let gateway = get_gate();
    println!("[WebSocket Core] Gate: {}", &gateway);

    let mut client = ClientBuilder::new(gateway.as_str())
        .unwrap()
        .connect_secure(None)
        .unwrap();
    client.stream_ref().as_tcp().set_read_timeout(Some(Duration::from_millis(50))).unwrap();


    let (cha_se, reciv_inner) = channel::<OwnedMessage>();


    println!("[WebSocket Core] Connection established");

    let mut hbeat = HBeat::new();
    let mut state = Core::Ok;
    let mut last_seq: Option<u64> = None;
	let mut session_id: Option<String> = None;



    //цикл
    loop {

        //recieve
        match client.recv_message() {
            Ok(m) => {
                match m {
                    OwnedMessage::Text(jtext) => {
//                        let mc = jtext.clone();
//                        thread::spawn(move||{
//                            event_eater(mc);
//                        });
                        let v: Value = serde_json::from_str(&jtext).unwrap();
                        match v["op"].as_u64(){
                            Some(0) => {
                                let _ = dc_shell.send(UniChanel::Responce(v.clone()));
                                match v["t"].as_str(){
	                                Some("READY") => {
                                        if let Some(id) = v["d"]["session_id"].as_str(){
                                            session_id = Some(id.to_string());
                                        }
                                    }
                                    Some("RESUMED") => {
                                        state = Core::Ok;
                                    }
                                    Some(_) => {
                                    }
                                    None => {}
                                }
	                            if let Some(s) = v["s"].as_u64(){
		                            last_seq = Some(s);
	                            }

                            }
                            Some(1) => {}
                            Some(7) => {

                            }
                            Some(9) => {
	                            match v["t"].as_bool(){
		                            Some(true) => {
                                        state = Core::Resume;
		                            }
		                            _ => {
                                        hbeat.stop();
                                        state = Core::ReconectClose;
                                    }
	                            }
                            }
                            Some(10) => {
                                if let Some(hb) = v["d"]["heartbeat_interval"].as_u64() {
                                    hbeat.set(hb);
                                }
                                state = Core::SendIdentify;
                            }
                            Some(11) => {continue;}
                            _ => {}
                        }

                    }
                    OwnedMessage::Ping(x) => {
                        client.send_message(&OwnedMessage::Pong(x));
                    }

                    OwnedMessage::Close(e) => {
                        if let Core::CloseConfirm = state{
                            dc_shell.send(UniChanel::Close);
                            return;
                        }
                        if let Core::Reconect = state{
                        }
                        else {
                            println!("[WebSocket Core] Connection closed: {:?}", e);
                        }
                    }

                    _ => {}

                }

            },
            Err(_) => {
            }
        };

        //send
        if hbeat.is_time(){
            let mut data = json!({
                "op": 1,
                "d": null
            });
            if let Some(n) = last_seq{
                *data.get_mut("d").unwrap() = json!(n);
            }
            let mes = OwnedMessage::Text(serde_json::to_string(&data).unwrap());
            match client.send_message(&mes) {
                Ok(()) => {
//                    println!("[WebSocket Core] Send HBeat");
                    hbeat.refresh();
                }
                Err(e) => {
                    println!("[WebSocket Core] Send Err1: {:?}", e);
                    return;
                }
            }
        }



        match state{
            Core::SendIdentify => {
                let data = json!({
                    "op": 2,
                    "d": {
                        "token": Discord::token(),
                        "properties": {
                            "$os": "windows",
                            "$browser": "wsbot",
                            "$device": "wsbot"
                        },
                        "compress": false
                    }
                });
                let mes = OwnedMessage::Text(serde_json::to_string(&data).unwrap());
                match client.send_message(&mes) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("[WebSocket Core] Send Loop2: {:?}", e);
                        return;
                    }
                }
                state = Core::SendIdentifyProceed;
                continue;
            }
            Core::Resume => {
                if let Some(ref id) = session_id{
                    let mut data = json!({
                                        "op": 6,
                                        "d": {
                                            "token": Discord::token(),
                                            "session_id": id.as_str(),
                                            "seq": null
                                        }
                                    });
                    if let Some(n) = last_seq{
                        *data.get_mut("d").unwrap().get_mut("seq").unwrap() = json!(n);
                    }
                    let mes = OwnedMessage::Text(serde_json::to_string(&data).unwrap());
                    match client.send_message(&mes) {
                        Ok(()) => {
                            state = Core::ResumeProceed;
                            continue;
                        }
                        Err(e) => {
                            println!("[WebSocket Core] Send Err3: {:?}", e);
                            return;
                        }
                    }
                }
                panic!("[WebSocket Core] Cannot resume connection");
            }
            Core::Ok => {}
            Core::ReconectClose => {
                let mes = OwnedMessage::Close(None);
                match client.send_message(&mes) {
                    Ok(()) => {
                        state = Core::Reconect;
                        continue;
                    }
                    Err(e) => {
                        println!("[WebSocket Core] Send Err4: {:?}", e);
                        return;
                    }
                }
            }
            Core::Reconect => {
                let _ = client.shutdown();
                client = ClientBuilder::new(gateway.as_str())
                    .unwrap()
                    .connect_secure(None)
                    .unwrap();
                last_seq = None;
                session_id = None;
                println!("[WebSocket Core] Reconection success");
                continue;
            }
            _ => {}
        }

        match dc_shell.recv_simp(){
            UniChanel::None => {}
            UniChanel::Close => {
                let mes = OwnedMessage::Close(None);
                match client.send_message(&mes) {
                    Ok(()) => {
                        state = Core::CloseConfirm;
                        continue;
                    }
                    Err(e) => {
                        println!("[WebSocket Core] Send Err4: {:?}", e);
                        dc_shell.send(UniChanel::Close);
                        return;
                    }
                }
            }
            _ => {}
        }

    }

}

pub fn shell(dc_global: DoubleChanel<GlobE>){

    //let (dc_local_s, dc_local_r) = DoubleChanel::<LocalLink>::new();

    let (dc_core, dc_to_core) = DoubleChanel::<UniChanel>::new();

    thread::spawn(move || core(dc_to_core));

    let mut dis_links: Vec<DCShell> = Vec::new();

    loop {
        match dc_core.recv_simp(){
            UniChanel::None =>{}
            UniChanel::Close =>{
                dc_global.send(GlobE::Drop);
                return;}
            UniChanel::Responce(value) => {
                if let Some(event) = Event::frome_json(value){
                    for link in dis_links.iter(){
                        link.send(OutLink::Event(event.clone()));
                    }
                }
            }
        }
        match dc_global.recv_simp() {
            GlobE::GetChanel(dc) =>{
                dis_links.push(DCShell::from_dc(dc));
            }
            GlobE::Drop => {
                dc_core.send(UniChanel::Close);
            }
            _ =>{}
        }
//        for link in dis_links{
//            match link.get(){
//                _ =>{}
//            }
//        }

    }


}

fn get_gate() -> String {
    //println!("Get frome {}", GETGATEWAY);

    match reqwest::get(GETGATEWAY){
        Ok(mut resp) => {
            match resp.text(){
                Ok(text) =>{
                    let v: Value = serde_json::from_str(&text).unwrap();
                    return v["url"].as_str().unwrap().to_string();
                }
                Err(e) => {
                    panic!("[reqwest] Error while take body:\n{}", e);
                }
            }
        }
        Err(e) => {
            panic!("[reqwest] Error while get responce from url. Probaly wrong url:\n{}", e);
        }
    };

}

fn event_eater(ev: String) {
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
        println!("{}\n\n", mess);
}