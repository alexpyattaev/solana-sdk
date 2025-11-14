#![feature(prelude_import)]
#![feature(print_internals)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use solana_frozen_abi_macro::frozen_abi;
use solana_frozen_abi_macro::AbiExample;
pub struct BlockhashQueue {
    pub last_hash_index: u64,
    pub a: u16,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for BlockhashQueue {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private226::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "BlockhashQueue",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "last_hash_index",
                &self.last_hash_index,
            )?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "a", &self.a)?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BlockhashQueue {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private226::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private226::Formatter,
                ) -> _serde::__private226::fmt::Result {
                    _serde::__private226::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private226::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private226::Ok(__Field::__field0),
                        1u64 => _serde::__private226::Ok(__Field::__field1),
                        _ => _serde::__private226::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private226::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "last_hash_index" => _serde::__private226::Ok(__Field::__field0),
                        "a" => _serde::__private226::Ok(__Field::__field1),
                        _ => _serde::__private226::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private226::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"last_hash_index" => _serde::__private226::Ok(__Field::__field0),
                        b"a" => _serde::__private226::Ok(__Field::__field1),
                        _ => _serde::__private226::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private226::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private226::PhantomData<BlockhashQueue>,
                lifetime: _serde::__private226::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BlockhashQueue;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private226::Formatter,
                ) -> _serde::__private226::fmt::Result {
                    _serde::__private226::Formatter::write_str(__formatter, "struct BlockhashQueue")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private226::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)? {
                        _serde::__private226::Some(__value) => __value,
                        _serde::__private226::None => {
                            return _serde::__private226::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct BlockhashQueue with 2 elements",
                            ));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<u16>(&mut __seq)? {
                        _serde::__private226::Some(__value) => __value,
                        _serde::__private226::None => {
                            return _serde::__private226::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct BlockhashQueue with 2 elements",
                            ));
                        }
                    };
                    _serde::__private226::Ok(BlockhashQueue {
                        last_hash_index: __field0,
                        a: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private226::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private226::Option<u64> =
                        _serde::__private226::None;
                    let mut __field1: _serde::__private226::Option<u16> =
                        _serde::__private226::None;
                    while let _serde::__private226::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private226::Option::is_some(&__field0) {
                                    return _serde::__private226::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "last_hash_index",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private226::Some(
                                    _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private226::Option::is_some(&__field1) {
                                    return _serde::__private226::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                    );
                                }
                                __field1 = _serde::__private226::Some(
                                    _serde::de::MapAccess::next_value::<u16>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private226::Some(__field0) => __field0,
                        _serde::__private226::None => {
                            _serde::__private226::de::missing_field("last_hash_index")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private226::Some(__field1) => __field1,
                        _serde::__private226::None => _serde::__private226::de::missing_field("a")?,
                    };
                    _serde::__private226::Ok(BlockhashQueue {
                        last_hash_index: __field0,
                        a: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["last_hash_index", "a"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "BlockhashQueue",
                FIELDS,
                __Visitor {
                    marker: _serde::__private226::PhantomData::<BlockhashQueue>,
                    lifetime: _serde::__private226::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::solana_frozen_abi::abi_example::AbiExample for BlockhashQueue {
    fn example() -> Self {
        {
            ::std::io::_print(format_args!(
                "AbiExample::example for struct: {0}\n",
                std::any::type_name::<BlockhashQueue>(),
            ));
        };
        use ::solana_frozen_abi::abi_example::AbiExample;
        BlockhashQueue {
            last_hash_index: AbiExample::example(),
            a: AbiExample::example(),
        }
    }
    fn random(rng: &mut impl ::rand::RngCore) -> Self {
        {
            ::std::io::_print(format_args!(
                "AbiExample::random for struct: {0}\n",
                std::any::type_name::<BlockhashQueue>(),
            ));
        };
        use ::solana_frozen_abi::abi_example::AbiExample;
        BlockhashQueue {
            last_hash_index: AbiExample::random(rng),
            a: AbiExample::random(rng),
        }
    }
}
impl Distribution<BlockhashQueue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockhashQueue {
        BlockhashQueue {
            last_hash_index: rng.gen(),
            a: rng.gen(),
        }
    }
}

pub fn main() {}
