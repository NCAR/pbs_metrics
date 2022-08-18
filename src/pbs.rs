use std::collections::HashMap;
use std::ptr::{null,NonNull};
use std::ffi::CStr;

mod bindings;
mod linked_list;

use linked_list::LinkedList;

linked_list::impl_LlItem!{[bindings::attrl, bindings::batch_status]}

fn parse_status(status: bindings::batch_status, name: &str) -> HashMap<String, String> {
    let mut parsed = LinkedList::new(unsafe{*status.attribs})
    .map(|attrib| {
        let name = unsafe { CStr::from_ptr(attrib.name).to_str().unwrap() };
        let resource = unsafe {
            if let Some(_) = NonNull::new(attrib.resource) {
                CStr::from_ptr(attrib.resource).to_str().unwrap()
            }else{
                ""
            }
        };
        let value = unsafe { CStr::from_ptr(attrib.value).to_str() }.unwrap().to_string();
        let mut key = name.to_owned();

        if resource != "" {
            key.push_str("_");
            key.push_str(resource);
        }
        (key, value)
    })
    .collect::<HashMap<String,String>>();
    parsed.insert(name.to_string(), unsafe{CStr::from_ptr(status.name).to_str()}.unwrap().to_string());
    return parsed;
}

fn stat_pbs(f: &dyn Fn(i32) -> bindings::batch_status, name: &str) -> Vec<HashMap<String,String>> {
    let conn = unsafe{bindings::pbs_connect(null::<i8>() as *mut i8)};
    let resp = LinkedList::new(f(conn));
    unsafe{bindings::pbs_disconnect(conn)};

    //make sure to insert resource name into metric
    resp.map(|x| parse_status(x, name)).collect()
} 
    

pub fn statnodes() -> Vec<HashMap<String, String>> {
    // second arg is null to get all nodes, third is null to get all attributes, forth is unused
    stat_pbs( &|conn| unsafe {*bindings::pbs_stathost(conn, null::<i8>() as *mut i8, null::<bindings::attrl>() as *mut bindings::attrl, null::<i8>() as *mut i8)}, "hostname")
}

pub fn statques() -> Vec<HashMap<String, String>> {
    stat_pbs( &|conn| unsafe {*bindings::pbs_statque(conn, null::<i8>() as *mut i8, null::<bindings::attrl>() as *mut bindings::attrl, null::<i8> as *mut i8)}, "que")
}

