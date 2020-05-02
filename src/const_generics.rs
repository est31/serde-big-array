use core::fmt;
use core::result;
use core::marker::PhantomData;
use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, Error};

pub trait BigArray<'de>: Sized {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer;
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>;
}
impl<'de, T, const N: usize> BigArray<'de> for [T; N]
    where T: Default + Copy + Serialize + Deserialize<'de>
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_tuple(self.len())?;
        for elem in &self[..] {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }

    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct ArrayVisitor<T> {
            element: PhantomData<T>,
        }

        impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<[T; N]>
            where T: Default + Copy + Deserialize<'de>
        {
            type Value = [T; N];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                macro_rules! write_len {
                    ($l:literal) => {
                        write!(formatter, concat!("an array of length ", $l))
                    };
                    ($l:tt) => {
                        write!(formatter, "an array of length {}", $l)
                    };
                }

                write_len!(N)
            }

            fn visit_seq<A>(self, mut seq: A) -> result::Result<[T; N], A::Error>
                where A: SeqAccess<'de>
            {
                let mut arr: [T; N] = [T::default(); N];
                for i in 0..N {
                    arr[i] = seq.next_element()?
                        .ok_or_else(|| Error::invalid_length(i, &self))?;
                }
                Ok(arr)
            }
        }

        let visitor = ArrayVisitor { element: PhantomData };
        // The allow is needed to support (32 + 33) like expressions
        #[allow(unused_parens)]
        deserializer.deserialize_tuple(N, visitor)
    }
}
