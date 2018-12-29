#![forbid(unsafe_code)]

/*!
Big array helper for serde.
The purpose of this crate is to make (de-)serializing arrays of sizes > 32 easy.
This solution is needed until [const generics](https://github.com/rust-lang/rust/issues/44580) are becoming stable.

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

# fn main() { let s = S { arr: [1; 64] }; }
```
*/

extern crate serde;
extern crate core;

#[doc(hidden)]
pub mod reex {
    pub use core::fmt;
    pub use core::marker::PhantomData;
    pub use serde::ser;
    pub use serde::ser::{Serialize, Serializer};
    pub use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, Error};
}

/**
Big array macro

This is the main macro of this crate.
Invoking it creates a trait that can be used together with a `#[serde(with = "TraitName")]` like attribute
on an array that's a member of a struct you want to (de-) serialize.
```
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate serde_big_array;
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
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate serde_big_array;
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
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate serde_big_array;
# fn main() {}
#
big_array! {
    BigArray;
    42, 300, 1234, 99999,
}

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr_a: [u8; 300],
    #[serde(with = "BigArray")]
    arr_b: [u8; 42],
}
```
*/
#[macro_export]
macro_rules! big_array {
    ($name:ident; $($len:expr,)+) => {
        pub trait $name<'de>: Sized {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: $crate::reex::Serializer;
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: $crate::reex::Deserializer<'de>;
        }
        $(
            impl<'de, T> $name<'de> for [T; $len]
                where T: Default + Copy + $crate::reex::Serialize + $crate::reex::Deserialize<'de>
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: $crate::reex::Serializer
                {
                    use $crate::reex::ser::SerializeTuple;
                    let mut seq = serializer.serialize_tuple(self.len())?;
                    for elem in &self[..] {
                        seq.serialize_element(elem)?;
                    }
                    seq.end()
                }

                fn deserialize<D>(deserializer: D) -> Result<[T; $len], D::Error>
                    where D: $crate::reex::Deserializer<'de>
                {
                    use $crate::reex::PhantomData;
                    struct ArrayVisitor<T> {
                        element: PhantomData<T>,
                    }

                    impl<'de, T> $crate::reex::Visitor<'de> for ArrayVisitor<T>
                        where T: Default + Copy + $crate::reex::Deserialize<'de>
                    {
                        type Value = [T; $len];

                        fn expecting(&self, formatter: &mut $crate::reex::fmt::Formatter) -> $crate::reex::fmt::Result {
                            formatter.write_str(concat!("an array of length ", $len))
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<[T; $len], A::Error>
                            where A: $crate::reex::SeqAccess<'de>
                        {
                            let mut arr = [T::default(); $len];
                            for i in 0..$len {
                                arr[i] = seq.next_element()?
                                    .ok_or_else(|| $crate::reex::Error::invalid_length(i, &self))?;
                            }
                            Ok(arr)
                        }
                    }

                    let visitor = ArrayVisitor { element: PhantomData };
                    deserializer.deserialize_tuple($len, visitor)
                }
            }
        )+
    };
    ($name:ident;) => {
        big_array! {
            $name;
            40, 48, 50, 56, 64, 72, 96, 100, 128, 160, 192, 200, 224, 256, 384, 512,
            768, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
        }
    }
}
