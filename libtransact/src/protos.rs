/*
 * Copyright 2018 Bitwise IO, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use std::error::Error as StdError;

#[derive(Debug)]
pub enum ProtoConversionError {
    SerializationError(String),
}

impl StdError for ProtoConversionError {
    fn description(&self) -> &str {
        match *self {
            ProtoConversionError::SerializationError(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            ProtoConversionError::SerializationError(_) => None,
        }
    }
}

impl std::fmt::Display for ProtoConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ProtoConversionError::SerializationError(ref s) => {
                write!(f, "SerializationError: {}", s)
            }
        }
    }
}

pub trait FromProto<P>: Sized {
    fn from_proto(other: P) -> Result<Self, ProtoConversionError>;
}

pub trait FromNative<N>: Sized {
    fn from_native(other: N) -> Result<Self, ProtoConversionError>;
}

pub trait IntoNative<T>: Sized
where
    T: FromProto<Self>,
{
    fn into_native(self) -> Result<T, ProtoConversionError> {
        FromProto::from_proto(self)
    }
}

pub trait IntoProto<T>: Sized
where
    T: FromNative<Self>,
{
    fn into_proto(self) -> Result<T, ProtoConversionError> {
        FromNative::from_native(self)
    }
}

// Includes the autogenerated protobuf messages
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
