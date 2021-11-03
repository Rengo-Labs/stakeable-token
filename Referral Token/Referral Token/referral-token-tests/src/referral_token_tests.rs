use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::referral_token_instance::REFERRALTOKENInstance;
use crate::test_instance::TESTInstance;

const NAME: &str = "Referral Token";

fn deploy() -> (TestEnv, REFERRALTOKENInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let _env_factory = TestEnv::new();
    let token = REFERRALTOKENInstance::new(
        &env,
        NAME,
        Sender(owner),
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
    );
    (env, token, owner)
}


#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, owner) = deploy();
    token.constructor(
        Sender(owner),
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
    );
}