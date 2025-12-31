#![no_std]
#![no_main]

use caspervault_contracts::LiquidStaking;

#[no_mangle]
fn call() {
    LiquidStaking::deploy();
}
