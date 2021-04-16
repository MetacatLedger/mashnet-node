// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use crate as delegation;
use crate::*;
use ctype::mock as ctype_mock;

use frame_support::{parameter_types, weights::constants::RocksDbWeight};
use kilt_primitives::{AccountId, Signature};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};

pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Ctype: ctype::{Pallet, Call, Storage, Event<T>},
		Delegation: delegation::{Pallet, Call, Storage, Event<T>},
		Did: did::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const SS58Prefix: u8 = 38;
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = ();

	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl Config for Test {
	type Event = ();
	type WeightInfo = ();
	type DelegationNodeId = H256;
}

impl ctype::Config for Test {
	type Event = ();
	type WeightInfo = ();
}

impl did::Config for Test {
	type Event = ();
	type WeightInfo = ();
	type DidIdentifier = AccountId;
}

pub type TestDelegationNodeId = <Test as Config>::DelegationNodeId;
pub type TestCtypeHash = <Test as frame_system::Config>::Hash;
pub type TestDidIdentifier = <Test as did::Config>::DidIdentifier;

pub(crate) const DEFAULT_ACCOUNT: AccountId = AccountId::new([0u8; 32]);

pub struct ExtBuilder {
	ctype_builder: Option<ctype_mock::ExtBuilder>,
	root_delegations_stored: Vec<(TestDelegationNodeId, DelegationRoot<Test>)>,
	delegations_stored: Vec<(TestDelegationNodeId, DelegationNode<Test>)>,
	children_stored: Vec<(TestDelegationNodeId, Vec<TestDelegationNodeId>)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			ctype_builder: None,
			root_delegations_stored: vec![],
			delegations_stored: vec![],
			children_stored: vec![],
		}
	}
}

impl From<ctype_mock::ExtBuilder> for ExtBuilder {
	fn from(ctype_builder: ctype_mock::ExtBuilder) -> Self {
		let mut instance = Self::default();
		instance.ctype_builder = Some(ctype_builder);
		instance
	}
}

impl ExtBuilder {
	pub fn with_root_delegations(mut self, root_delegations: Vec<(TestDelegationNodeId, DelegationRoot<Test>)>) -> Self {
		self.root_delegations_stored = root_delegations;
		self
	}

	pub fn with_delegations(mut self, delegations: Vec<(TestDelegationNodeId, DelegationNode<Test>)>) -> Self {
		self.delegations_stored = delegations;
		self
	}

	pub fn with_children(mut self, children: Vec<(TestDelegationNodeId, Vec<TestDelegationNodeId>)>) -> Self {
		self.children_stored = children;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut ext = if let Some(ctype_builder) = self.ctype_builder.clone() {
			ctype_builder.build()
		} else {
			let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
			sp_io::TestExternalities::new(storage)
		};

		if !self.root_delegations_stored.is_empty() {
			ext.execute_with(|| {
				self.root_delegations_stored.iter().for_each(|root_delegation| {
					delegation::Roots::<Test>::insert(root_delegation.0, root_delegation.1.clone());
				})
			});
		}

		if !self.delegations_stored.is_empty() {
			ext.execute_with(|| {
				self.delegations_stored.iter().for_each(|del| {
					delegation::Delegations::<Test>::insert(del.0, del.1.clone());
				})
			});
		}

		if !self.children_stored.is_empty() {
			ext.execute_with(|| {
				self.children_stored.iter().for_each(|child| {
					delegation::Children::<Test>::insert(child.0, child.1.clone());
				})
			});
		}

		ext
	}
}

pub(crate) fn hash_to_u8<T: Encode>(hash: T) -> Vec<u8> {
	hash.encode()
}

const DEFAULT_ROOT_ID_SEED: u64 = 1u64;
const ALTERNATIVE_ROOT_ID_SEED: u64 = 2u64;

pub fn get_delegation_root_id(default: bool) -> H256 {
	if default {
		H256::from_low_u64_be(DEFAULT_ROOT_ID_SEED)
	} else {
		H256::from_low_u64_be(ALTERNATIVE_ROOT_ID_SEED)
	}
}
