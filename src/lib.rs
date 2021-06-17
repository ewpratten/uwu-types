//! `uwu-types` adds some UwU and OwO for your Rust code.
//!
//! This crate was inspired by 
//! [this tweet](https://twitter.com/thingskatedid/status/1404610732348477441) 
//! from [@thingskatedid](https://twitter.com/thingskatedid).
//!
//! Special credits to:
//!  - [@mycoliza](https://twitter.com/mycoliza)
//!    - Inspired the `uwusize` type
//!  - [@pH_0x05](https://twitter.com/pH_0x05)
//!    - Inspired the `owo` types for signed numbers
//!  - [@anotherwalther](https://twitter.com/anotherwalther)
//!    - Inspired the `kate!()` macro

#![no_std]

/// The 8-bit unsigned integer type
#[allow(non_camel_case_types)]
pub type uwu8 = u8;

/// The 16-bit unsigned integer type
#[allow(non_camel_case_types)]
pub type uwu16 = u16;

/// The 32-bit unsigned integer type
#[allow(non_camel_case_types)]
pub type uwu32 = u32;

/// The 64-bit unsigned integer type
#[allow(non_camel_case_types)]
pub type uwu64 = u64;

/// The 128-bit unsigned integer type
#[allow(non_camel_case_types)]
pub type uwu128 = u128;

/// The pointer-sized unsigned integer type.
///
/// The size of this primitive is how many bytes it takes to reference 
/// any location in memory. For example, on a 32 bit target, this is 4 bytes 
/// and on a 64 bit target, this is 8 bytes.
#[allow(non_camel_case_types)]
pub type uwusize = usize;

/// The 8-bit signed integer type
#[allow(non_camel_case_types)]
pub type owo8 = i8;

/// The 16-bit signed integer type
#[allow(non_camel_case_types)]
pub type owo16 = i16;

/// The 32-bit signed integer type
#[allow(non_camel_case_types)]
pub type owo32 = i32;

/// The 64-bit signed integer type
#[allow(non_camel_case_types)]
pub type owo64 = i64;

/// The 128-bit signed integer type
#[allow(non_camel_case_types)]
pub type owo128 = i128;

/// Print "Kate!" to the console
#[macro_export]
macro_rules! kate {
    () => {
        println!("Kate!");
    };
}