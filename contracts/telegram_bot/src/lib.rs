#![feature(proc_macro_hygiene)]

#[cfg(not(test))]
extern crate wee_alloc;

#[cfg(not(test))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
extern crate alloc;
extern crate ellipticoin;
extern crate wasm_rpc_macros;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate ellipticoin_test_framework;
mod error;
pub mod telegram_bot;
