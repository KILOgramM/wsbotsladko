//connect to server & token
use std::thread;
use std::time::Duration;
use websocket::{Message,
                message::OwnedMessage,
                result::{WebSocketError,
                         WebSocketResult}};
use websocket::client::ClientBuilder;
use websocket::stream::sync::AsTcpStream;
use serde_json::Value;
use std::io::ErrorKind;
use extime;


use reqwest;
use serde_json;
use crate::dstruct::{DoubleChanel,
              HBeat,
              DiscordMain,
              DCShell};
use crate::denum::{UniChanel,
            OutLink,
            LocalLink,
            GlobE};

use std::sync::mpsc::{Receiver,Sender,channel};

use crate::disapi::Discord;
use crate::denum::Event;

const GETGATEWAY: &'static str = "https://discordapp.com/api/v6/gateway";

#[derive(PartialEq)]
enum Core{
    Ok,
    SendIdentify,
    SendIdentifyProceed,
    Resume,
    ResumeProceed,
    ReconectClose,
    Reconect,
    ReconnectProceed,
    CloseConfirm,
}

fn core(dc_shell: DoubleChanel<UniChanel>){
//    use net2::TcpStreamExt;
    //переменные

    let gateway = get_gate();
    info!("[WebSocket Core] Gate: {}", &gateway);

    let mut client = ClientBuilder::new(gateway.as_str())
        .expect("[WebSocket Core] Make Client Builder")
        .connect_secure(None)
        .expect("[WebSocket Core] Create Connection from Builder");
//    client.stream_ref().as_tcp().set_nonblocking(true);
    //
	{
        use net2::TcpStreamExt;
        client.stream_ref().as_tcp().set_read_timeout(Some(Duration::from_millis(150))).expect("[WebSocket Core] Set Read Timeout");
        client.stream_ref().as_tcp().set_keepalive(Some(Duration::from_millis(300))).expect("[WebSocket Core] Set Keepalive");
	}
//
//    info!("[WebSocket Core] read_timeout is: {:?}", client.stream_ref().as_tcp().read_timeout());
//    info!("[WebSocket Core] keepalive is: {:?}", client.stream_ref().as_tcp().keepalive_ms());
    let (cha_se, reciv_inner) = channel::<OwnedMessage>();


    info!("[WebSocket Core] Connection established");

    let mut hbeat = HBeat::new();
    let mut state = Core::Ok;
    let mut last_seq: Option<u64> = None;
	let mut session_id: Option<String> = None;



    //цикл
    'coreloop: loop{

        //recieve
        match client.recv_message() {
            Ok(m) => {
                match m {
                    OwnedMessage::Text(jtext) => {
//                        let mc = jtext.clone();
//                        thread::spawn(move||{
//                            event_eater(mc);
//                        });
                        let v: Value = serde_json::from_str(&jtext).expect("[WebSocket Core] Serialize Message from WebSocket");
                        match v["op"].as_u64(){
                            Some(0) => {
                                let _ = dc_shell.send(UniChanel::Responce(v.clone()));
                                match v["t"].as_str(){
	                                Some("READY") => {
                                        if let Some(id) = v["d"]["session_id"].as_str(){
                                            session_id = Some(id.to_string());
                                        }
                                        state = Core::Ok;
                                    }
                                    Some("RESUMED") => {
                                        info!("[WebSocket Core] Discord Connection Resumed successfully");
                                        state = Core::Ok;
                                    }
                                    _ => {
                                        state = Core::Ok;
                                    }

                                }
	                            if let Some(s) = v["s"].as_u64(){
		                            last_seq = Some(s);
	                            }

                            }
                            Some(7) => {
                                info!("[WebSocket Core] Discord ask to reconnect");
                                hbeat.stop();
                                state = Core::ReconectClose;
                            }
                            Some(9) => {
                                info!("[WebSocket Core] Discord Invalid Session Reconnecting in 5 sec");
                                thread::sleep(Duration::from_secs(5));
                                if state == Core::Reconect || state == Core::ReconnectProceed {
                                    session_id = None;
                                    hbeat.stop();
                                    state = Core::ReconectClose;
                                }
                                else {
                                    match v["t"].as_bool(){
                                        Some(true) => {
                                            hbeat.stop();
                                            state = Core::ReconectClose;
                                        }
                                        _ => {
                                            session_id = None;
                                            hbeat.stop();
                                            state = Core::ReconectClose;
                                        }
                                    }
                                }

                            }
                            Some(10) => {
                                info!("[WebSocket Core] Discord Hello Message");
                                if let Some(hb) = v["d"]["heartbeat_interval"].as_u64() {
                                    hbeat.set(hb);
                                }

                                match state {
                                    Core::ReconnectProceed => {}
                                    Core::Reconect => {}
                                    _ => {state = Core::SendIdentify;}
                                }
                                continue 'coreloop;
                            }
                            Some(11) => {continue;}
                            _ => {}
                        }

                    }
                    OwnedMessage::Ping(x) => {
                        client.send_message(&OwnedMessage::Pong(x));
                    }

                    OwnedMessage::Close(e) => {
                        hbeat.stop();
                        if let Core::CloseConfirm = state{
                            dc_shell.send(UniChanel::Close);
                            return;
                        }
                        if let Core::Reconect = state{

                        }
                        else {
                            error!("[WebSocket Core] Connection closed: {:?}", e);
                            info!("[WebSocket Core] WebSocket Retry in 5 sec");
                            thread::sleep(Duration::from_secs(5));
                            state = Core::Reconect;

                        }
                    }

                    _ => {}

                }

            },
            Err(e) => {
                match &e {
                    WebSocketError::IoError(err) => {
                        if err.kind() == ErrorKind::TimedOut{
                        }
                        else {
                            match state {
                                Core::Reconect => {}
                                Core::ReconectClose => {}
                                _ => {
                                    error!("[WebSocket Core] WebSocket Error: {:?}", e);
                                    info!("[WebSocket Core] WebSocket Retry in 5 sec");
                                    thread::sleep(Duration::from_secs(5));
                                    state = Core::Reconect;
                                    hbeat.stop();
                                }
                            }
                        }
                    }
                    _ => {
                        match state {
                            Core::Reconect => {}
                            Core::ReconectClose => {}
                            _ => {
                                error!("[WebSocket Core] WebSocket Error: {:?}", &e);
                                info!("[WebSocket Core] WebSocket Retry in 5 sec");
                                thread::sleep(Duration::from_secs(5));
                                state = Core::Reconect;
                                hbeat.stop();
                            }
                        }
                    }
                }
            }
        };

        //send
        if hbeat.is_time(){
            let mut data = json!({
                "op": 1,
                "d": null
            });
            if let Some(n) = last_seq{
                *data.get_mut("d").expect("[WebSocket Core] Get data Row in HBeat") = json!(n);
            }
            let mes = OwnedMessage::Text(serde_json::to_string(&data).expect("[WebSocket Core] Serialize HBeat Message"));
            match client.send_message(&mes) {
                Ok(()) => {
//                    info!("[WebSocket Core] Send HBeat");
                    hbeat.refresh();
                }
                Err(e) => {
                    info!("[WebSocket Core] Send Err1 ({}): {:?}",extime::now().ctime(), e);
                    if let WebSocketError::IoError(err) = e {
                        if let ErrorKind::ConnectionAborted = err.kind(){
                            info!("[WebSocket Core] Try reconect");
                            let _ = client.shutdown();
                            client = ClientBuilder::new(gateway.as_str())
                                .expect("[WebSocket Core] Make Client Builder for Reconect")
                                .connect_secure(None)
                                .expect("[WebSocket Core] Make Connection for Reconect");
                            last_seq = None;
                            session_id = None;
                            //info!("[WebSocket Core] Reconection success");
                            continue;
                        }
                    }
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
                let mes = OwnedMessage::Text(serde_json::to_string(&data).expect("[WebSocket Core] Serialize Identify Message"));
                match client.send_message(&mes) {
                    Ok(()) => (),
                    Err(e) => {
                        info!("[WebSocket Core] Send Loop2: {:?}", e);
                        return;
                    }
                }
                state =Core::SendIdentifyProceed;
                continue 'coreloop;
            }
            Core::Resume => {
                if let Some(ref id) = session_id{

                }
                warn!("[WebSocket Core] Cannot resume connection. Reconnecting");
                state = Core::ReconectClose;
            }
            Core::Ok => {}
            Core::ReconectClose => {
                hbeat.stop();
                let mes = OwnedMessage::Close(None);
                match client.send_message(&mes) {
                    Ok(()) => {
                        state = Core::Reconect;
                        continue 'coreloop;
                    }
                    Err(e) => {
                        info!("[WebSocket Core] Send Err4: {:?}", e);
                        return;
                    }
                }
            }
            Core::Reconect => {
                let _ = client.shutdown();
                client = ClientBuilder::new(gateway.as_str())
                    .expect("[WebSocket Core] Make Client Builder for Reconect")
                    .connect_secure(None)
                    .expect("[WebSocket Core] Make Connection for Reconect");
//                client.stream_ref().as_tcp().set_nonblocking(true);
                {
                    use net2::TcpStreamExt;
                    client.stream_ref().as_tcp().set_read_timeout(Some(Duration::from_millis(50))).expect("[WebSocket Core] Set Read Timeout");
                    client.stream_ref().as_tcp().set_keepalive(Some(Duration::from_millis(300))).expect("[WebSocket Core] Set Keepalive");
                }
                last_seq = None;
                session_id = None;
                info!("[WebSocket Core] Reconection success");
                hbeat.start();
                hbeat.refresh();
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
                        *data.get_mut("d").expect("[WebSocket Core] Get data in Resume").get_mut("seq").expect("[WebSocket Core] Get Seq in data in Resume") = json!(n);
                    }
                    let mes = OwnedMessage::Text(serde_json::to_string(&data).expect("[WebSocket Core] Serialize Resume Message"));
                    match client.send_message(&mes) {
                        Ok(()) => {
                            state = Core::ResumeProceed;
                            continue 'coreloop;
                        }
                        Err(e) => {
                            info!("[WebSocket Core] Send Err3: {:?}", e);
                            return;
                        }
                    }
                }
                state = Core::ReconnectProceed;
                continue 'coreloop;
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
                        info!("[WebSocket Core] Send Err4: {:?}", e);
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


    let (dc_core, dc_to_core) = DoubleChanel::<UniChanel>::new();
    thread::Builder::new()
            .name("Websocket Core".to_string())
            .spawn(move || core(dc_to_core));

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


    }


}

fn get_gate() -> String {
    //info!("Get frome {}", GETGATEWAY);

    match reqwest::get(GETGATEWAY){
        Ok(mut resp) => {
            match resp.text(){
                Ok(text) =>{
                    let v: Value = serde_json::from_str(&text).expect("[WebSocket Core] Serialize gate Respoce");
                    return v["url"].as_str().expect("[WebSocket Core] Trying Get Gate URL").to_string();
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
        info!("{}\n\n", mess);
}
