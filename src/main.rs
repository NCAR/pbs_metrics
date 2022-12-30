use std::collections::HashMap;
use pbs::{Server, Resource, Status};
use clap::{Parser};

use std::str::FromStr;

#[derive(Debug, Parser)]
struct Cli {
    resource: Resource,
    server: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let srv = if let Some(s) = args.server {
        Server::connect_to(&s)
    } else {
        Server::new()
    };
    let data: Vec<HashMap<String,serde_json::Value>> = srv.stat(args.resource.clone(), None, vec!())
        .map(parse_status).collect();


    println!("{}", serde_json::json!({"measurement": r_to_string(&args.resource), "datapoints": data}));
}

pub fn r_to_string(r: &Resource) -> String {
    match *r {
        Resource::Hostname => "hostname".to_string(),
        Resource::Que => "que".to_string(),
        Resource::Job => "job".to_string(),
        Resource::Reservation => "reservation".to_string(),
        Resource::Resource => "resource".to_string(),
        Resource::Scheduler => "scheduler".to_string(),
        Resource::Server => "server".to_string(),
        Resource::Vnode => "vnode".to_string(),
    }
}

fn parse_status(status: Status) -> HashMap<String, serde_json::Value> {
    let mut parsed = status.attribs_iter()
    .map(|attrib| {
        let value = {
            //memory string to bytes conversion parsing
            if let Some(res) = attrib.resource() {
                if res.contains("mem"){
                    if attrib.value().ends_with("gb") {
                        (&attrib.value()[..attrib.value().len()-2].parse::<usize>().unwrap()*1000000000).to_string()
                    }else if attrib.value().ends_with("mb") {
                        (&attrib.value()[..attrib.value().len()-2].parse::<usize>().unwrap()*1000000).to_string()
                    }else if attrib.value().ends_with("kb") {
                        (&attrib.value()[..attrib.value().len()-2].parse::<usize>().unwrap()*1000).to_string()
                    }else if attrib.value().ends_with('b') {
                        attrib.value()[..attrib.value().len()-1].to_string()
                    }else{
                    attrib.value().to_string()
                    }
                }else{
                    attrib.value().to_string()
                }
            }else{
                attrib.value().to_string()
            }
        };
        let mut key = attrib.name().to_owned();

        if let Some(res) = attrib.resource() {
            key.push('_');
            key.push_str(res);
        }
        let val = if let Ok(v) = isize::from_str(&value) {serde_json::Value::Number(v.into()) }
            else {serde_json::Value::String(value)};
        (key, val)
    })
    .collect::<HashMap<String,serde_json::Value>>();
    parsed.insert("name".to_string(), serde_json::Value::String(status.name().to_string()));
    parsed
}
