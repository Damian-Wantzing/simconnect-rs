#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        assert_eq!(SIMCONNECT_UNUSED, u32::MAX);
    }
}
