// //! Constants used by the WCSPR contract.

// // constants key
// pub const PACKAGE_HASH_KEY_NAME: &str = "package_hash";
// pub const CONTRACT_HASH_KEY_NAME: &str = "contract_hash";
// pub const WCSPR_HASH_KEY_NAME: &str = "wcspr_hash";

// /// Name of named-key for `name`.
// pub const NAME_KEY_NAME: &str = "name";
// /// Name of named-key for `symbol`
// pub const SYMBOL_KEY_NAME: &str = "symbol";
// /// Name of named-key for `decimals`
// pub const DECIMALS_KEY_NAME: &str = "decimals";
// /// Name of named-key for `contract`
// pub const ERC20_TOKEN_CONTRACT_KEY_NAME: &str = "erc20_token_contract";
// /// Name of dictionary-key for `balances`
// pub const BALANCES_KEY_NAME: &str = "balances";
// /// Name of dictionary-key for `allowances`
// pub const ALLOWANCES_KEY_NAME: &str = "allowances";
// /// Name of named-key for `total_supply`
// pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";
// /// Name of named-key for 'self_purse'
// pub const SELF_PURSE_KEY_NAME: &str = "self_purse";

// pub const DEPOSIT_TEST_RESULT_KEY_NAME: &str = "deposit_test_result";
// pub const WITHDRAW_TEST_RESULT_KEY_NAME: &str = "withdraw_test_result";
// pub const TRANSFER_TEST_RESULT_KEY_NAME: &str = "transfer_test_result";
// pub const TRANSFER_FROM_TEST_RESULT_KEY_NAME: &str = "transfer_from_test_result";
// pub const ALLOWANCE_KEY_NAME: &str = "allowance";
// pub const BALANCE_OF_KEY_NAME:  &str = "balance_of";

// /// Name of `name` entry point.
// pub const NAME_ENTRY_POINT_NAME: &str = "name";
// /// Name of `symbol` entry point.
// pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
// /// Name of `decimals` entry point.
// pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
// /// Name of `balance_of` entry point.
// pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
// /// Name of `transfer` entry point.
// pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
// /// Name of `approve` entry point.
// pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
// /// Name of `allowance` entry point.
// pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
// /// Name of `transfer_from` entry point.
// pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
// /// Name of `total_supply` entry point.
// pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";
// /// Name of `deposit` entry point.
// pub const DEPOSIT_ENTRY_POINT_NAME: &str = "deposit";
// /// Name of `withdraw` entry point.
// pub const WITHDRAW_ENTRY_POINT_NAME: &str = "withdraw";

// /// Name of `address` runtime argument.
// pub const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
// /// Name of `owner` runtime argument.
// pub const OWNER_RUNTIME_ARG_NAME: &str = "owner";
// /// Name of `spender` runtime argument.
// pub const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
// /// Name of `amount` runtime argument.
// pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
// /// Name of `recipient` runtime argument.
// pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
// /// Name of `name` runtime argument.
// pub const NAME_RUNTIME_ARG_NAME: &str = "name";
// /// Name of `symbol` runtime argument.
// pub const SYMBOL_RUNTIME_ARG_NAME: &str = "symbol";
// /// Name of `decimals` runtime argument.
// pub const DECIMALS_RUNTIME_ARG_NAME: &str = "decimals";
// /// Name of `total_supply` runtime argument.
// pub const TOTAL_SUPPLY_RUNTIME_ARG_NAME: &str = "total_supply";
// /// Name of `purse` runtime argument.
// pub const PURSE_RUNTIME_ARG_NAME: &str = "purse";
// /// Name of `to` runtime argument.
// pub const TO_RUNTIME_ARG_NAME: &str = "to";

// pub const PACKAGE_HASH_RUNTIME_ARG_NAME: &str = "package";
// pub const CONTRACT_HASH_RUNTIME_ARG_NAME: &str = "contract";
// pub const WCSPR_HASH_RUNTIME_ARG_NAME: &str = "wcspr";

// rcontract info
pub const CONTRACT_NAME: &str = "transfer_helper";
pub const CONTRACT_WASM_NAME: &str = "transfer_helper.wasm";
pub const PROXY_CONTRACT_NAME: &str = "proxy_contract";
pub const PROXY_CONTRACT_WASM_NAME: &str = "contract.wasm";

// relating to contracty deployment
pub const PACKAGE_HASH_KEY_NAME: &str = "package_hash";
pub const CONTRACT_HASH_KEY_NAME: &str = "contract_hash";
pub const PACKAGE_HASH_RUNTIME_ARG_NAME: &str = "package_hash";
pub const CONTRACT_HASH_RUNTIME_ARG_NAME: &str = "contract_hash";
pub const SELF_CONTRACT_HASH_RUNTIME_ARG_NAME: &str = "self_contract_hash";
pub const SELF_CONTRACT_HASH_KEY_NAME: &str = "self_contract_hash";

pub const TRANSFER_HELPER_HASH_KEY_NAME: &str = "transfer_helper";
pub const TRANSFER_HELPER_HASH_RUNTIME_ARG_NAME: &str = "transfer_helper";

// relating to contract testing

// key names
pub const TRANSFER_INVOKER_KEY_NAME: &str = "transfer_invoker";

// result key names
pub const FORWARD_FUNDS_RESULT: &str = "forward_funds_result";
pub const GET_TRANSFER_INVOKER_ADDRESS_RESULT: &str = "get_transfer_invoker_address_result";

// runtime args
pub const TRANSFER_INVOKER_RUNTIME_ARG_NAME: &str = "transfer_invoker";
pub const FORWARD_AMOUNT_RUNTIME_ARG_NAME: &str = "forward_amount";
pub const TOKEN_ADDRESS_RUNTIME_ARG_NAME: &str = "token_address";
pub const NAME_RUNTIME_ARG_NAME: &str = "name";
pub const KEY_RUNTIME_ARG_NAME: &str = "key";
// entrypoint names
pub const FORWARD_FUNDS_ENTRYPOINT_NAME: &str = "forward_funds";
pub const GET_TRANSFER_INVOKER_ADDRESS_ENTRYPOINT_NAME: &str = "get_transfer_invoker_address";
pub const SET_KEY_BY_NAME_ENTRYPOINT_NAME: &str = "set_key_by_name";
