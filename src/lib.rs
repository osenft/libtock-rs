#![feature(
    asm,
    alloc_error_handler,
    core_intrinsics,
    lang_items,
    llvm_asm,
    naked_functions,
)]
#![cfg_attr(any(target_arch = "arm", target_arch = "riscv32"), no_std)]

mod callback;

pub mod adc;
pub mod ble_composer;
pub mod ble_parser;
pub mod buttons;
pub mod console;
pub mod debug;
pub mod electronics;
pub mod futures;
pub mod gpio;
pub mod led;
pub mod memop;
pub mod result;
pub mod rng;
pub mod sensors;
pub mod shared_memory;
pub mod simple_ble;
pub mod temperature;
pub mod timer;
pub mod unwind_symbols;

#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
pub mod entry_point;

#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
mod lang_items;

pub mod syscalls;

pub use libtock_codegen::main;

// Dummy structure to force importing the panic_handler and other no_std elements when nothing else
// is imported.
pub struct LibTock;
