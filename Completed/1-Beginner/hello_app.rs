#![allow(unused)]
use local_ip_address::local_ip;
use std::process;
use serde_derive::{ Serialize, Deserialize };

use reqwest::Url;
use thiserror;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Generic {0}")]
    Generic(String),
}

impl std::convert::From<String> for Error {
    fn from(string: String) -> Error {
        Error::Generic(string)
    }
}
impl std::convert::From<reqwest::Error> for Error {
    fn from(req: reqwest::Error) -> Error {
        Error::Generic(format!("{req}"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let test_salut = Salut::get()
        .await?;
    println!("{:?}", test_salut);
    test_salut.parse();

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Salut {
    lang: String,
}

impl Salut {
    async fn get() -> Result<Self, Error> {
        let ip = local_ip_address::local_ip().unwrap_or_else(|e| {
            println!("could not get ip address quitting {:?}", e);
            process::exit(1);
        });
        //println!("get salut ip: {:?}", ip);

        let url = Url::parse("https://hellosalut.stefanbohacek.dev/?lang=pt").unwrap();
        //let url = Url::parse(format!("https://stefanbohacek.com/hellosalut/?ip={}", ip.to_string()).as_str()).unwrap();
        //println!("get salut url: {:?}", url);

        let lang = reqwest::get(url)
            .await?
            .text()
            .await?;
        //println!("get salut lang: {:?}", lang);

        Ok(Self { 
            lang,
        })
    }
    fn parse(&self) -> String {
        let hello = self.lang
            .split(',').collect::<Vec<&str>>()[1]
            .split(':').collect::<Vec<&str>>()[1];
        let hello = &hello[1..hello.len() - 2];
        println!("{hello:?}");
        "asd".to_string()
    }
}
