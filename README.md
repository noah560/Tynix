# Tynix
A super modular microkernel written in Rust that
you can build for pretty much any platform.

(WORK IN PROGRESS: STILL IN ALPHA AND NOT FEATURE COMPLETE YET)

## Features
Working Features:

Work-in-Progress Features:
 - Memory management
 - Modular IO device support
 - Support for built-in programs that
   are frozen into the kernel binary
 - Bytecode interpreter

Planned features:
 - Modular memory management code
 - Ability to swap in custom bytecode
   interpreters without patching the library
 - Ability to swap in custom memory management
   code for managing RAM pages and program breaks.

## Configuration
 - Memory: For devices under 2MB memory, don't activate either.
   For devices under 32MB memory, activate the feature `mid-memory`.
   For devices with even more memory, activate the feature `high-memory`;

## Example
Note: This example doesn't work since
the features it uses aren't ready yet
```rust
use tynix::*;

struct MyDevices;
impl tynix::device::Devices for MyDevices {
    fn get_count() -> usize { 1 }
    fn get_name(_id: usize) -> &'static [u8] { "output".as_bytes() }
    fn get_available_bytes(_id: usize) -> usize { 0 }
    fn read(_id: usize) -> u8 { 0 }
    fn write(_id: usize, data: u8) -> bool {
        print!("{}", char::from(data));
        true
    }
}

struct MyBuiltins;
impl tynix::builtin::Builtins for MyBuiltins {
    fn get_count() -> usize { 1 }
    fn get_name(_id: usize) -> &'static [u8] { "init".as_bytes() }
    fn get_data(_id: usize) -> &'static [u8] { include_bytes!("demo_software.bin") }
}

/// 1 MB of RAM
static mut kernel: Tynix<MyBuiltins, MyDevices, 1024*1024> = Default::default();

fn main() {
    unsafe { kernel.run() };
}
```

