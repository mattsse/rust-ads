#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
extern crate bincode;
#[allow(dead_code)]
extern crate rust_ads;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate std as std;

use bincode::{deserialize, serialize, Infinite};

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use rust_ads::prelude::*;


struct Entity {
    x: i8,
    y: Vec<u8>,
    a: i8,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Entity: () = {
    extern crate serde as _serde;

    #[automatically_derived]
    impl _serde::Serialize for Entity {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state =
                    // 8 bytes for the length of the vector,
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "Entity",
                                                               0 + 1 + 1 + 1)
                        {
                        ::result::Result::Ok(val) => val,
                        ::result::Result::Err(err) => {
                            return ::result::Result::Err(::convert::From::from(err))
                        }
                    };

            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "x", &self.x) {
                ::result::Result::Ok(val) => val,
                ::result::Result::Err(err) => {
                    return ::result::Result::Err(::convert::From::from(err))
                }
            };

            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "y", &self.y) {
                ::result::Result::Ok(val) => val,
                ::result::Result::Err(err) => {
                    return ::result::Result::Err(::convert::From::from(err))
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "a", &self.a) {
                ::result::Result::Ok(val) => val,
                ::result::Result::Err(err) => {
                    return ::result::Result::Err(::convert::From::from(err))
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Entity: () = {


    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Entity {

        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 3",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "x" => _serde::export::Ok(__Field::__field0),
                        "y" => _serde::export::Ok(__Field::__field1),
                        "a" => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"x" => _serde::export::Ok(__Field::__field0),
                        b"y" => _serde::export::Ok(__Field::__field1),
                        b"a" => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<Entity>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Entity;
                fn expecting(
                    &self,
                    formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(formatter, "struct Entity")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<i8>(&mut __seq)
                    {
                        ::result::Result::Ok(val) => val,
                        ::result::Result::Err(err) => {
                            return ::result::Result::Err(::convert::From::from(err))
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"tuple of 3 elements",
                            ));
                        }
                    };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<Vec<u8>>(&mut __seq) {
                            ::result::Result::Ok(val) => val,
                            ::result::Result::Err(err) => {
                                return ::result::Result::Err(::convert::From::from(err))
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"tuple of 3 elements",
                                ));
                            }
                        };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<i8>(&mut __seq)
                    {
                        ::result::Result::Ok(val) => val,
                        ::result::Result::Err(err) => {
                            return ::result::Result::Err(::convert::From::from(err))
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"tuple of 3 elements",
                            ));
                        }
                    };
                    _serde::export::Ok(Entity {
                        x: __field0,
                        y: __field1,
                        a: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<i8> = _serde::export::None;
                    let mut __field1: _serde::export::Option<Vec<u8>> = _serde::export::None;
                    let mut __field2: _serde::export::Option<i8> = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            ::result::Result::Ok(val) => val,
                            ::result::Result::Err(err) => {
                                return ::result::Result::Err(::convert::From::from(err))
                            }
                        } {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("x"),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<i8>(&mut __map) {
                                        ::result::Result::Ok(val) => val,
                                        ::result::Result::Err(err) => {
                                            return ::result::Result::Err(::convert::From::from(err))
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("y"),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Vec<u8>>(&mut __map) {
                                        ::result::Result::Ok(val) => val,
                                        ::result::Result::Err(err) => {
                                            return ::result::Result::Err(::convert::From::from(err))
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::export::Option::is_some(&__field2) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                    );
                                }
                                __field2 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<i8>(&mut __map) {
                                        ::result::Result::Ok(val) => val,
                                        ::result::Result::Err(err) => {
                                            return ::result::Result::Err(::convert::From::from(err))
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    ::result::Result::Ok(val) => val,
                                    ::result::Result::Err(err) => {
                                        return ::result::Result::Err(::convert::From::from(err))
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => match _serde::private::de::missing_field("x") {
                            ::result::Result::Ok(val) => val,
                            ::result::Result::Err(err) => {
                                return ::result::Result::Err(::convert::From::from(err))
                            }
                        },
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => match _serde::private::de::missing_field("y") {
                            ::result::Result::Ok(val) => val,
                            ::result::Result::Err(err) => {
                                return ::result::Result::Err(::convert::From::from(err))
                            }
                        },
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => match _serde::private::de::missing_field("a") {
                            ::result::Result::Ok(val) => val,
                            ::result::Result::Err(err) => {
                                return ::result::Result::Err(::convert::From::from(err))
                            }
                        },
                    };
                    _serde::export::Ok(Entity {
                        x: __field0,
                        y: __field1,
                        a: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["x", "y", "a"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Entity",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<Entity>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Entity {
    #[inline]
    fn eq(&self, __arg_0: &Entity) -> bool {
        match *__arg_0 {
            Entity {
                x: ref __self_1_0,
                y: ref __self_1_1,
                a: ref __self_1_2,
            } => match *self {
                Entity {
                    x: ref __self_0_0,
                    y: ref __self_0_1,
                    a: ref __self_0_2,
                } => {
                    true && (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Entity) -> bool {
        match *__arg_0 {
            Entity {
                x: ref __self_1_0,
                y: ref __self_1_1,
                a: ref __self_1_2,
            } => match *self {
                Entity {
                    x: ref __self_0_0,
                    y: ref __self_0_1,
                    a: ref __self_0_2,
                } => {
                    false || (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Entity {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Entity {
                x: ref __self_0_0,
                y: ref __self_0_1,
                a: ref __self_0_2,
            } => {
                let mut builder = __arg_0.debug_struct("Entity");
                let _ = builder.field("x", &&(*__self_0_0));
                let _ = builder.field("y", &&(*__self_0_1));
                let _ = builder.field("a", &&(*__self_0_2));
                builder.finish()
            }
        }
    }
}
fn dummy() {
    let x = Entity {
        x: 20,
        y: <[_]>::into_vec(box [0, 0, 0, 1]),
        a: 21,
    };
    let encoded = serialize(&x, Infinite).unwrap();
    ::io::_print(::std::fmt::Arguments::new_v1_formatted(
        &["", "\n"],
        &match (&encoded,) {
            (__arg0,) => [::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Debug::fmt)],
        },
        &[
            ::std::fmt::rt::v1::Argument {
                position: ::std::fmt::rt::v1::Position::At(0usize),
                format: ::std::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                    flags: 0u32,
                    precision: ::std::fmt::rt::v1::Count::Implied,
                    width: ::std::fmt::rt::v1::Count::Implied,
                },
            },
        ],
    ));
    let decoed: Entity = deserialize(encoded.as_slice()).unwrap();
    ::io::_print(::std::fmt::Arguments::new_v1_formatted(
        &["", "\n"],
        &match (&decoed,) {
            (__arg0,) => [::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Debug::fmt)],
        },
        &[
            ::std::fmt::rt::v1::Argument {
                position: ::std::fmt::rt::v1::Position::At(0usize),
                format: ::std::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                    flags: 0u32,
                    precision: ::std::fmt::rt::v1::Count::Implied,
                    width: ::std::fmt::rt::v1::Count::Implied,
                },
            },
        ],
    ));
}
fn main() {
    dummy();
    return;
    let endpoint = ::fmt::format(::std::fmt::Arguments::new_v1_formatted(
        &["127.0.0.1:"],
        &match (&router::PORT_BASE1,) {
            (__arg0,) => [
                ::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Display::fmt),
            ],
        },
        &[
            ::std::fmt::rt::v1::Argument {
                position: ::std::fmt::rt::v1::Position::At(0usize),
                format: ::std::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::std::fmt::rt::v1::Alignment::Unknown,
                    flags: 0u32,
                    precision: ::std::fmt::rt::v1::Count::Implied,
                    width: ::std::fmt::rt::v1::Count::Implied,
                },
            },
        ],
    ));
    let listener = TcpListener::bind(endpoint.as_str()).unwrap();
    ::io::_print(::std::fmt::Arguments::new_v1(
        &["listening started, ready to accept\n"],
        &match () {
            () => [],
        },
    ));
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"Hello World\r\n").unwrap();
            let mut content = String::new();
            stream.read_to_string(&mut content);
            ::io::_print(::std::fmt::Arguments::new_v1_formatted(
                &["", "\n"],
                &match (&content,) {
                    (__arg0,) => [
                        ::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Display::fmt),
                    ],
                },
                &[
                    ::std::fmt::rt::v1::Argument {
                        position: ::std::fmt::rt::v1::Position::At(0usize),
                        format: ::std::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                            flags: 0u32,
                            precision: ::std::fmt::rt::v1::Count::Implied,
                            width: ::std::fmt::rt::v1::Count::Implied,
                        },
                    },
                ],
            ));
        });
    }
}
