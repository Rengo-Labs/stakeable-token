#![no_main]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use staking_token::{self, STAKINGTOKEN};

#[derive(Default)]
struct StakingToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for StakingToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl STAKINGTOKEN<OnChainContractStorage> for StakingToken {}

impl StakingToken {
    fn constructor(
        &mut self,
        declaration_hash: Key,
        timing_hash: Key,
        helper_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        contract_hash: ContractHash,
    ) {
        STAKINGTOKEN::init(
            self,
            declaration_hash,
            timing_hash,
            helper_hash,
            bep20_hash,
            snapshot_hash,
            Key::from(contract_hash),
        );
    }
}

#[no_mangle]
fn constructor() {
    let declaration_hash: Key = runtime::get_named_arg("declaration_hash");
    let timing_hash: Key = runtime::get_named_arg("timing_hash");
    let helper_hash: Key = runtime::get_named_arg("helper_hash");
    let bep20_hash: Key = runtime::get_named_arg("bep20_hash");
    let snapshot_hash: Key = runtime::get_named_arg("snapshot_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    StakingToken::default().constructor(
        declaration_hash,
        timing_hash,
        helper_hash,
        bep20_hash,
        snapshot_hash,
        contract_hash,
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("declaration_hash", Key::cl_type()),
            Parameter::new("timing_hash", Key::cl_type()),
            Parameter::new("helper_hash", Key::cl_type()),
            Parameter::new("bep20_hash", Key::cl_type()),
            Parameter::new("snapshot_hash", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let declaration_hash: Key = runtime::get_named_arg("declaration_hash");
    let timing_hash: Key = runtime::get_named_arg("timing_hash");
    let helper_hash: Key = runtime::get_named_arg("helper_hash");
    let bep20_hash: Key = runtime::get_named_arg("bep20_hash");

    let snapshot_hash: Key = runtime::get_named_arg("snapshot_hash");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "declaration_hash"=>declaration_hash,
        "timing_hash"=>timing_hash,
        "helper_hash"=>helper_hash,
        "bep20_hash"=>bep20_hash,
        "snapshot_hash"=>snapshot_hash,
        "contract_hash" => contract_hash,
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}