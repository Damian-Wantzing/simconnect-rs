#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(SIMCONNECT_UNUSED, u32::MAX);
    }

    #[test]
    fn test_simconnect_open() {
        unsafe {
            // VERY IMPORTANT: we need to pass a pointer to a mutable pointer
            // so not just a null pointer itself since SimConnect will reassign what it is pointing
            // to
            let handle = &mut std::ptr::null_mut();

            let result = SimConnect_Open(
                handle,
                CString::new("Test Application").unwrap().as_ptr(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            );
            assert!(!handle.is_null());

            SimConnect_Close(*handle);

            assert!(result >= 0);
        }
    }
}
