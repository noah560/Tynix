#![no_std]

// pub mod process;
pub mod allocator;
// pub mod devices;
// pub mod builtins;
mod kernel_heap;
mod file;
pub use file::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
