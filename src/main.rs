mod pbs;

use std::env;

fn main() {
    let measurment;
    let datapoints;
    match env::args().nth(1).expect("Requires an arg, options are: hosts, ques, jobs, reservations, resources, schedulers, servers, vnodes").as_str() {
    "hosts" => {
        datapoints = pbs::stat_hosts();
        measurment = "pbs_stathost";
    },
    "ques" => {
        datapoints = pbs::stat_ques();
        measurment = "pbs_statque";
    },
    "jobs" => {
        datapoints = pbs::stat_jobs();
        measurment = "pbs_statjob";
    },
    "reservations" => {
        datapoints = pbs::stat_reservations();
        measurment = "pbs_statresv";
    },
    "resources" => {
        datapoints = pbs::stat_resources();
        measurment = "pbs_statrsc";
    }
    "schedulers" => {
        datapoints = pbs::stat_schedulers();
        measurment = "pbs_statsched";
    },
    "servers" => {
        datapoints = pbs::stat_servers();
        measurment = "pbs_statserver";
    }
    "vnodes" => {
        datapoints = pbs::stat_vnodes();
        measurment = "pbs_vnode_stat";
    },
    _ => panic!("requires an arg for what to stat")
    }
    print!("{{ \"measurement\": \"{}\", \"datapoints\": ", measurment);
    print!("{}", serde_json::to_string(&datapoints).unwrap());
    println!("}}");
}
