use std::collections::HashMap;
use std::env;
use pbs::{self, ResourceType, Status};

fn main() {
    if let Some(rtype) = ResourceType::from_str(env::args().nth(1).expect("Requires an arg, options are: hosts, ques, jobs, reservations, resources, schedulers, servers, vnodes").as_str()){
       let srv = pbs::Server::new();
       let datapoints: Vec<HashMap<String,String>> = srv.stat(&rtype)
           .map(|x| parse_status(x, rtype.to_string())).collect();
       let measurment =  match rtype {
           ResourceType::Hostname => "pbs_stathost",
           ResourceType::Que => "pbs_statque",
           ResourceType::Job => "pbs_statjob",
           ResourceType::Reservation => "pbs_statresv",
           ResourceType::Resource => "pbs_statrsc",
           ResourceType::Scheduler => "pbs_statsched",
           ResourceType::Server => "pbs_statserver",
           ResourceType::Vnode => "pbs_vnode_stat"
        };
        print!("{{ \"measurement\": \"{}\", \"datapoints\": ", measurment);
        print!("{}", serde_json::to_string(&datapoints).unwrap());
        println!("}}");
    }else{
        panic!("requires an arg for what to stat")
    }
}

fn parse_status(status: Status, name: String) -> HashMap<String, String> {
    let mut parsed = status.attribs()
    .map(|attrib| {
        let value = {
            if let Some(res) = attrib.resource {
                if res.contains("mem"){
                    if attrib.value.ends_with("gb") {
                        (&attrib.value[..attrib.value.len()-2].parse::<usize>().unwrap()*1000000000).to_string()
                    }else if attrib.value.ends_with("mb") {
                        (&attrib.value[..attrib.value.len()-2].parse::<usize>().unwrap()*1000000).to_string()
                    }else if attrib.value.ends_with("kb") {
                        (&attrib.value[..attrib.value.len()-2].parse::<usize>().unwrap()*1000).to_string()
                    }else if attrib.value.ends_with("b") {
                        attrib.value[..attrib.value.len()-1].to_string()
                    }else{
                    attrib.value.to_string()
                    }
                }else{
                    attrib.value.to_string()
                }
            }else{
                attrib.value.to_string()
            }
        };
        let mut key = attrib.name.to_owned();

        if let Some(res) = attrib.resource {
            key.push_str("_");
            key.push_str(res);
        }
        (key, value)
    })
    .collect::<HashMap<String,String>>();
    parsed.insert(name.to_string(), status.name().to_string());
    parsed
}
