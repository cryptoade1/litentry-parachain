// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use crate::{
	get_layer_two_nonce,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_accountid_from_str, get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli,
};
use codec::Decode;
use ita_stf::{Index, TrustedCall, TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::Assertion;
use log::*;
use sp_core::Pair;

#[derive(Parser)]
pub struct BuildAssertionCommand {
	/// AccountId in ss58check format
	account: String,
	assertion: String,
}

impl BuildAssertionCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) {
		let who = get_accountid_from_str(&self.account);
		let root = get_pair_from_str(trusted_cli, "//Alice");

		let (mrenclave, shard) = get_identifiers(trusted_cli);
		let nonce = get_layer_two_nonce!(root, cli, trusted_cli);

		let top: TrustedOperation =
			TrustedCall::build_assertion(root.public().into(), who, Assertion::A1, shard, 1)
				.sign(&KeyPair::Sr25519(Box::new(root)), nonce, &mrenclave, &shard)
				.into_trusted_operation(trusted_cli.direct);
		perform_trusted_operation(cli, trusted_cli, &top);
	}
}
