// Copyright 2020 Damir Jelić
// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![doc = include_str!("../README.md")]
#![deny(
    missing_debug_implementations,
    missing_docs,
    dead_code,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(not(any(feature = "native-tls", feature = "rustls-tls",)))]
compile_error!("one of 'native-tls' or 'rustls-tls' features must be enabled");

#[cfg(all(feature = "native-tls", feature = "rustls-tls",))]
compile_error!("only one of 'native-tls' or 'rustls-tls' features can be enabled");

#[cfg(all(feature = "sso_login", target_arch = "wasm32"))]
compile_error!("'sso_login' cannot be enabled on 'wasm32' arch");

pub use bytes;
pub use matrix_sdk_base::{
    media, Room as BaseRoom, RoomInfo, RoomMember as BaseRoomMember, RoomType, Session,
    StateChanges, StoreError,
};
pub use matrix_sdk_common::*;
pub use reqwest;
#[doc(no_inline)]
pub use ruma;

mod client;
pub mod config;
mod error;
pub mod event_handler;
mod http_client;
/// High-level room API
pub mod room;
mod room_member;
mod sync;

#[cfg(feature = "encryption")]
pub mod encryption;

pub use client::{Client, LoopCtrl};
pub use error::{Error, HttpError, HttpResult, Result};
pub use http_client::HttpSend;
pub use room_member::RoomMember;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
