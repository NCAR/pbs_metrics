use std::ffi;

mod bindings;

fn main() {
    unsafe{
        let conn = bindings::pbs_connect(0 as *mut i8);
        // second arg is null to get all nodes, third is null to get all attributes, forth is unused
        let mut status = bindings::pbs_stathost(conn, 0 as *mut i8, 0 as *mut bindings::attrl, 0 as *mut i8);
        while status != 0 as *mut bindings::batch_status {
            println!("{}", ffi::CStr::from_ptr((*status).name).to_str().unwrap());

            let mut attribs = (*status).attribs;
            while attribs != 0 as *mut bindings::attrl{
                let r = (*attribs).resource;
                let resource = {
                    if r != 0 as *mut i8 {
                        ffi::CStr::from_ptr(r).to_str().unwrap()
                    }else{""}
                };

                println!("\t{}:{} {}",
                    ffi::CStr::from_ptr((*attribs).name).to_str().unwrap(), 
                    resource,
                    ffi::CStr::from_ptr((*attribs).value).to_str().unwrap());
                attribs = (*attribs).next;
            }
            println!("\n");
            status = (*status).next;
        }
    }
}
