#![forbid(unsafe_code)]

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
