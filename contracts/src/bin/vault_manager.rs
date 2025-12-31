#![no_std]
#![no_main]

use caspervault_contracts::VaultManager;

#[no_mangle]
fn call() {
    VaultManager::deploy();
}
