/*!
Big array helper for serde.
The purpose of this crate is to make (de-)serializing arrays of sizes > 32 easy.
This solution is needed until [serde adopts const generics support](https://github.com/serde-rs/serde/issues/1937).

## Example
```
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_big_array;

use serde_big_array::BigArray;

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr: [u8; 64],
}

#[test]
fn test() {
    let s = S { arr: [1; 64] };
    let j = serde_json::to_string(&s).unwrap();
    let s_back = serde_json::from_str::<S>(&j).unwrap();
    assert!(&s.arr[..] == &s_back.arr[..]);
    assert!(false);
}

# fn main() {}
```
*/
#![no_std]

mod const_generics;
pub use const_generics::BigArray;

/**
Big array macro

The macro exists for legacy reasons, to make moving to the pure `const-generics` mode easier.
Instead of this macro, please use the [`BigArray`] trait directly.
*/
#[macro_export]
#[deprecated(note = "deprecated in favour of the BigArray trait")]
macro_rules! big_array {
    ($name:ident; $($len:expr),+ $(,)?) => {
        pub use $crate::BigArray as $name;
    };
    ($name:ident; + $($len:expr),* $(,)?) => {
        big_array! {
            $name;
            40, 48, 50, 56, 64, 72, 96, 100, 128, 160, 192, 200, 224, 256, 384, 512,
            768, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
            $($len,)*
        }
    };
    ($name:ident;) => {
        big_array! {
            $name; +
        }
    }
}
