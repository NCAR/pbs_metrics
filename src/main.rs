use std::env;
use pbs::{self, ResourceType};

fn main() {
    if let Some(rtype) = ResourceType::from_str(env::args().nth(1).expect("Requires an arg, options are: hosts, ques, jobs, reservations, resources, schedulers, servers, vnodes").as_str()){
       let datapoints = pbs::stat(&rtype);
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
