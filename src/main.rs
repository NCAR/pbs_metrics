use std::ffi;
use std::collections::HashMap;
use serde_json::json;

mod pbs;
mod linked_list;

use pbs::bindings;
use linked_list::LinkedList;

const METRICS: [&str; 3] =  ["state", "resources_available", "resources_assigned"];
const RESOURCES: [&str; 3] = ["ncpus", "mem", "ngpus"];

linked_list::impl_LlItem!{[bindings::attrl, bindings::batch_status]}

fn get_nodes() -> LinkedList<bindings::batch_status> {
    LinkedList::new( unsafe {
        let conn = bindings::pbs_connect(0 as *mut i8);
        // second arg is null to get all nodes, third is null to get all attributes, forth is unused
        *bindings::pbs_stathost(conn, 0 as *mut i8, 0 as *mut bindings::attrl, 0 as *mut i8)
    })
}

fn get_attribs(attrl: bindings::attrl) -> HashMap<String, String> {
    let mut attribs: HashMap<String, String> = LinkedList::new(attrl)
    .map(|attrib| {
        let name = unsafe { ffi::CStr::from_ptr(attrib.name).to_str().unwrap() };
        if METRICS.contains(&name) {
        let resource = unsafe {
            if attrib.resource != 0 as *mut i8 {
                ffi::CStr::from_ptr(attrib.resource).to_str().unwrap()
            }else{
                ""
            }
        };
        if RESOURCES.contains(&resource) || resource == "" {
        let value = unsafe { ffi::CStr::from_ptr(attrib.value).to_str() }.unwrap().to_string();
        let mut key = name.to_owned();

        if resource != "" {
            key.push_str("_");
            key.push_str(resource);
        }
        Some((key, value))
        }else{None}
        }else{None}
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect();

    for resource in RESOURCES {
        let mut k = "resources_available_".to_owned();
        k.push_str(resource);
        if !attribs.contains_key(&k) {
            let mut drop_k = "resources_assigned_".to_owned();
            drop_k.push_str(resource);
            attribs.remove(&drop_k);
        }
    } 
    return attribs;
}



fn main() {
    for node in get_nodes() {
        let hostname = unsafe {ffi::CStr::from_ptr(node.name).to_str().unwrap()};
        println!("{hostname}");
        let mut attribs = get_attribs(unsafe{*node.attribs});
        attribs.insert("hostname".to_string(), hostname.to_string());
        println!("{}", json!(attribs));
    }
}
