#![no_std]
#![no_main]

use caspervault_contracts::LstCspr;

#[no_mangle]
fn call() {
    LstCspr::deploy();
}
