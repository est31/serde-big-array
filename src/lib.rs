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

mod array;
mod const_generics;
pub use array::Array;
pub use const_generics::BigArray;
