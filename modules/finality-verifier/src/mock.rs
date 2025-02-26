// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

use crate::pallet::{BridgedHeader, Config};
use bp_runtime::{BlockNumberOf, Chain};
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{
	testing::{Header, H256},
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

pub type AccountId = u64;
pub type TestHeader = BridgedHeader<TestRuntime>;
pub type TestNumber = BlockNumberOf<<TestRuntime as Config>::BridgedChain>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TestRuntime;

impl_outer_origin! {
	pub enum Origin for TestRuntime where system = frame_system {}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for TestRuntime {
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type DbWeight = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
}

impl pallet_substrate_bridge::Config for TestRuntime {
	type BridgedChain = TestBridgedChain;
}

parameter_types! {
	pub const MaxElementsInSingleProof: Option<u32> = Some(5);
}

impl crate::pallet::Config for TestRuntime {
	type BridgedChain = TestBridgedChain;
	type HeaderChain = pallet_substrate_bridge::Module<TestRuntime>;
	type AncestryProof = Vec<<Self::BridgedChain as Chain>::Header>;
	type AncestryChecker = Checker<<Self::BridgedChain as Chain>::Header, Self::AncestryProof>;
}

#[derive(Debug)]
pub struct TestBridgedChain;

impl Chain for TestBridgedChain {
	type BlockNumber = <TestRuntime as frame_system::Config>::BlockNumber;
	type Hash = <TestRuntime as frame_system::Config>::Hash;
	type Hasher = <TestRuntime as frame_system::Config>::Hashing;
	type Header = <TestRuntime as frame_system::Config>::Header;
}

#[derive(Debug)]
pub struct Checker<H, P>(std::marker::PhantomData<(H, P)>);

impl<H> bp_header_chain::AncestryChecker<H, Vec<H>> for Checker<H, Vec<H>> {
	fn are_ancestors(_ancestor: &H, _child: &H, proof: &Vec<H>) -> bool {
		!proof.is_empty()
	}
}

pub fn run_test<T>(test: impl FnOnce() -> T) -> T {
	sp_io::TestExternalities::new(Default::default()).execute_with(test)
}

pub fn test_header(num: TestNumber) -> TestHeader {
	// We wrap the call to avoid explicit type annotations in our tests
	bp_test_utils::test_header(num)
}
