/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

//! Contains all logic of the state provisioning mechanism
//! including the remote attestation and tls / tcp connection part.

use std::convert::TryFrom;

mod authentication;
pub mod seal_handler;
mod tls_ra_client;
mod tls_ra_server;

#[cfg(feature = "test")]
pub mod tests;

#[cfg(feature = "test")]
pub mod mocks;

/// Header of an accompanied payloard. Indicates the
/// length an the type (opcode) of the following payload.
#[derive(Clone, Debug)]
pub struct TcpHeader {
	pub opcode: Opcode,
	pub payload_length: u64,
}

impl TcpHeader {
	fn new(opcode: Opcode, payload_length: u64) -> Self {
		Self { opcode, payload_length }
	}
}

/// Indicates the payload content type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opcode {
	ShieldingKey = 0,
	StateKey = 1,
	State = 2,
}

impl TryFrom<u8> for Opcode {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Opcode::ShieldingKey),
			1 => Ok(Opcode::StateKey),
			2 => Ok(Opcode::State),
			_ => Err(()),
		}
	}
}

impl Opcode {
	pub fn to_bytes(self) -> [u8; 1] {
		(self as u8).to_be_bytes()
	}
}
