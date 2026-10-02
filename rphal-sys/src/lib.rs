#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
pub mod robot;
pub mod types;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
