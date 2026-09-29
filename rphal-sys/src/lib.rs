#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
pub mod error;
pub mod digital;
pub mod robot;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

