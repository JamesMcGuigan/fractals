// #![feature(iter_intersperse)]  // #![feature]` may not be used on the stable release channel
// #![allow(unstable_name_collisions)]
// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// #![cfg_attr(debug_assertions, allow(dead_code))]
extern crate yew;

mod _wee_alloc;
pub mod components;
pub mod elements;
pub mod services;
pub mod mathematics;