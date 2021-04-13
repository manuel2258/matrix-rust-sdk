// Copyright 2021 Damir Jelić
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

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub(super) inner: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreKeyMessage {
    pub(super) inner: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OlmMessage {
    Normal(Message),
    PreKey(PreKeyMessage),
}

impl OlmMessage {
    pub fn from_type_and_ciphertext(message_type: usize, ciphertext: String) -> Result<Self, ()> {
        match message_type {
            0 => Ok(Self::PreKey(PreKeyMessage { inner: ciphertext })),
            1 => Ok(Self::Normal(Message { inner: ciphertext })),
            _ => Err(()),
        }
    }

    pub fn ciphertext(&self) -> &str {
        match self {
            OlmMessage::Normal(m) => &m.inner,
            OlmMessage::PreKey(m) => &m.inner,
        }
    }

    pub fn message_type(&self) -> MessageType {
        match self {
            OlmMessage::Normal(_) => MessageType::Normal,
            OlmMessage::PreKey(_) => MessageType::PreKey,
        }
    }

    pub fn to_tuple(self) -> (usize, String) {
        let message_type = self.message_type();

        match self {
            OlmMessage::Normal(m) => (message_type.into(), m.inner),
            OlmMessage::PreKey(m) => (message_type.into(), m.inner),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum MessageType {
    Normal,
    PreKey,
}

impl TryFrom<usize> for MessageType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::PreKey),
            1 => Ok(MessageType::Normal),
            _ => Err(()),
        }
    }
}

impl Into<usize> for MessageType {
    fn into(self) -> usize {
        match self {
            Self::PreKey => 0,
            Self::Normal => 1,
        }
    }
}

use std::convert::TryFrom;

#[cfg(test)]
use olm_rs::session::OlmMessage as LibolmMessage;
#[cfg(test)]
impl From<LibolmMessage> for OlmMessage {
    fn from(other: LibolmMessage) -> Self {
        let (message_type, ciphertext) = other.to_tuple();

        match message_type {
            olm_rs::session::OlmMessageType::PreKey => {
                Self::PreKey(PreKeyMessage { inner: ciphertext })
            }
            olm_rs::session::OlmMessageType::Message => Self::Normal(Message { inner: ciphertext }),
        }
    }
}

#[cfg(test)]
impl Into<LibolmMessage> for OlmMessage {
    fn into(self) -> LibolmMessage {
        let ciphertext = self.ciphertext().to_owned();

        match self {
            OlmMessage::Normal(_) => {
                LibolmMessage::from_type_and_ciphertext(1, ciphertext).unwrap()
            }
            OlmMessage::PreKey(_) => {
                LibolmMessage::from_type_and_ciphertext(0, ciphertext).unwrap()
            }
        }
    }
}
