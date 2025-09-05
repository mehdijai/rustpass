#[macro_export]
/// ## Dispatch import macro
/// import module and re-export them all.
macro_rules! dis_import {
    ($arg:ident) => {
        mod $arg;
        pub use $arg::*;
    };
}

pub mod core;
