use std::ffi;

use linked_list::LinkedList;
mod linked_list;
mod bindings;

linked_list::impl_LlItem!{[bindings::attrl, bindings::batch_status]}

fn get_nodes() -> LinkedList<bindings::batch_status> {
    LinkedList::new( unsafe {
        let conn = bindings::pbs_connect(0 as *mut i8);
        // second arg is null to get all nodes, third is null to get all attributes, forth is unused
        *bindings::pbs_stathost(conn, 0 as *mut i8, 0 as *mut bindings::attrl, 0 as *mut i8)
    })
}


fn main() {
    for node in get_nodes() {
        println!("{}", unsafe {ffi::CStr::from_ptr(node.name).to_str().unwrap()});
        for attrib in LinkedList::new(unsafe{*node.attribs}) {
            let resource = unsafe {
                if attrib.resource != 0 as *mut i8 {
                    ffi::CStr::from_ptr(attrib.resource).to_str().unwrap()
                }else{
                    ""
                }
            };
            let name = unsafe { ffi::CStr::from_ptr(attrib.name).to_str().unwrap() };
            let value = unsafe { ffi::CStr::from_ptr(attrib.value).to_str().unwrap() };

            println!("\t{}:{} {}", name, resource, value);
       }
    }
}
