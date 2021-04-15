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
#[macro_use]
extern crate serde_big_array;

big_array! { BigArray; }

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

If you enable the `const-generics` feature, you won't have to invoke the `big_array` macro any more:

```Rust
#[macro_use]
extern crate serde_derive;
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
}

# fn main() {}
```
*/
#![no_std]

mod const_generics;
pub use const_generics::BigArray;

/**
Big array macro

This is the main macro of this crate.
Invoking it creates a trait that can be used together with a `#[serde(with = "TraitName")]` like attribute
on an array that's a member of a struct you want to (de-) serialize.
```
# use serde_derive::{Serialize, Deserialize};
# use serde_big_array::big_array;
# fn main() {}
#
big_array! { BigArray; }

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr: [u8; 128],
}
```
The name of the added trait is your choice.

The macro doesn't automatically implement the trait for all possible array lengths.
Instead, the trait is implemented for a pre-specified set of numbers.
The default way to invoke the macro is by specifying the name only, like:
```
# use serde_derive::{Serialize, Deserialize};
# use serde_big_array::big_array;
# fn main() {}
#
big_array! {
    BigArray;
}
```
Then, the trait will be implemented for a pre-defined set of interesting array lengths.
Currently, the numbers are:
```ignore
40, 48, 50, 56, 64, 72, 96, 100, 128, 160, 192, 200, 224, 256, 384, 512,
768, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
```
These are the same numbers that the `arrayvec` crate uses as well,
and should cover most places this macro is used.

If this default setting is not suiting your use case, the macro has you covered as well.
You can specify a custom set of numbers by using the second way to invoke the macro:

```
# use serde_derive::{Serialize, Deserialize};
# use serde_big_array::big_array;
# fn main() {}
#
big_array! {
    BigArray;
    +42, 300, 1234, 99999,
}

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr_a: [u8; 300],
    #[serde(with = "BigArray")]
    arr_b: [u8; 42],
}
```

If the `+` is specified like in the example above, the trait is also implemented for the
pre-defined set of array lengths. If omitted, it's implemented for the specified numbers only.
*/
#[macro_export]
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
