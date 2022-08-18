mod pbs;

use std::env;

fn main() {
    let measurment;
    let datapoints;
    match env::args().nth(1).expect("Requires an arg, nodestat or questat").as_str() {
    "nodestat" => {
        datapoints = pbs::statnodes();
        measurment = "pbs_nodestat";
    },
    "questat" => {
        datapoints = pbs::statques();
        measurment = "pbs_questat";
    },
    _ => panic!("requires an arg, nodestat or questat")
    }
    print!("{{ \"measurement\": \"{}\", \"datapoints\": ", measurment);
    print!("{}", serde_json::to_string(&datapoints).unwrap());
    println!("}}");
}
