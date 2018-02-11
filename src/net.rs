

use hyper;
use hyper::Client;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::FutureResponse;
use std::fmt;
use std;
use native_tls;

//lazy_static = "1.0"

use futures::stream::Stream;
use futures::Future;

pub struct Net{
}

impl Net{

    pub fn get(link: String) -> Result<(hyper::StatusCode, String),NetError>{

        match Core::new() {
            Ok(mut core) =>{
                let handle = core.handle();

                let https = match HttpsConnector::new(4, &handle){
                    Ok(x) => {x}
                    Err(e) => {
                        return Err(NetError::HttpsConnectorCreating(e));
                    }
                };

                let client = Client::configure()
                    .connector(https)
                    .keep_alive(false)
                    .build(&handle);

                let uri = match link.parse(){
                    Ok(x) => {x}
                    Err(e) => {
                        return Err(NetError::ParceLink(e));
                    }
                };

                let work = client.get(uri)
                    .and_then(|res| {
                    let stat = res.status();
                    res.body().concat2().and_then(move |body| {
                        let v = String::from_utf8_lossy(&body);
                        //println!("BODY: \n{}\n\n", v);
                        Ok((stat,v.into_owned()))
                    })
                });
                match core.run(work){
                    Ok((res,body)) =>{
                        let mut body_str = String::new();
                        //let status = res.status();
//                        res.body().concat2().and_then(|body: hyper::Chunk| {
//                            let stringify = String::from_utf8_lossy(&body);
//                            println!("{}", stringify);
//                            body_str = stringify.into_owned();
//                            Ok(())
//                        });
                        return Ok((res,body));
                    }
                    Err(e) =>{
                        return Err(NetError::CoreRun(e));
                    }
                }

            }
            Err(e)=>{
                return Err(NetError::TokioCoreCreating(e));
            }
        }

    }
}

#[derive(Debug)]
pub enum NetError{
    TokioCoreCreating(std::io::Error),
    HttpsConnectorCreating(native_tls::Error),
    ParceLink(hyper::error::UriError),
    CoreRun(hyper::Error),
}

impl NetError{
    pub fn get_des(&self) -> String{
        match self{
            &NetError::TokioCoreCreating(_) => {
                format!("Net Error:\nWhile creating Tokio Core in Net impl")
            }
            &NetError::HttpsConnectorCreating(_) => {
                format!("Net Error:\nWhile creating Https Connector in Net impl")
            }
            &NetError::ParceLink(_) => {
                format!("Net Error:\nWhile Parcing Link in Net impl")
            }
            &NetError::CoreRun(_) => {
                format!("Net Error:\nWhile Core Run in Net impl")
            }
        }
    }
}
impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            &NetError::TokioCoreCreating(ref x) => {
                write!(f, "   {}\n---Error---\n{}---Error---", self.get_des(), x)
            }
            &NetError::HttpsConnectorCreating(ref x) => {
                write!(f, "   {}\n---Error---\n{}---Error---", self.get_des(), x)
            }
            &NetError::ParceLink(ref x) => {
                write!(f, "   {}\n---Error---\n{}---Error---", self.get_des(), x)
            }
            &NetError::CoreRun(ref x) => {
                write!(f, "   {}\n---Error---\n{}---Error---", self.get_des(), x)
            }
        }
    }
}