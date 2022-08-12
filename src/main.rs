mod bindings;

fn main() {
    println!("Hello, world!");
    unsafe{
        let conn = bindings::pbs_connect(0 as *mut i8);
        println!("{}", conn);
    }
}

// include <pbs_error.h>
// include <pbs_ifl.h>

/*
mod ffi {
    #[repr(C)]
    struct attrl {
        name: *mut c_char
        char *name
        char *resource
        char *value
        struct attrl *next
    }

    #[repr(C)]
    struct batch_status {
        struct batch_status *next
        char *name
        struct attrl *attribs
        char *text
    }

    extern "C" {
        include!("<pbs_error.h>");
        include!("<pbs_ifl.h>");

        fn pbs_connect(char *server) -> int;
        fn pbs_statfree(struct batch_status *psj)
        fn pbs_stathost(int connect, char *target, struct attrl *output_attribs, char *extend) -> struct batch_status *
    }
}
*/
