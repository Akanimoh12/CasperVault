#![no_std]
#![no_main]

use caspervault_contracts::CvCspr;

#[no_mangle]
fn call() {
    CvCspr::deploy();
}
