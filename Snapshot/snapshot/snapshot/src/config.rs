pub mod Structs {
    use casper_types::U256;
    extern crate serde;
    use serde::{Deserialize, Serialize};

    pub const SNAPSHOT: &str = "snapshot";
    pub const RSNAPSHOT: &str = "rsnapshot";
    pub const LSNAPSHOT: &str = "lsnapshot";

    #[derive(Serialize, Deserialize)]
    pub struct ConstantParameters {
        pub _decimals: u32,
        pub yodas_per_wise: U256,
        pub seconds_in_day: u32,
        pub min_lock_days: u32,
        pub formula_day: u32,
        pub max_lock_days: u32,
        pub max_bonus_days_a: u32,
        pub max_bonus_days_b: u32,
        pub min_referral_days: u32,
        pub min_stake_amount: U256,
        pub referrals_rate: U256, // 1.000% (direct value, can be used right away)
        pub inflation_rate_max: U256, // 3.000% (indirect -> checks through LiquidityGuard)
        pub precision_rate: U256,
        pub threshold_limit: U256, // $10,000 $BUSD
        pub daily_bonus_a: U256,   // 25%:1825 = 0.01369863013 per day;
        pub daily_bonus_b: U256,   // 5%:13505 = 0.00037023324 per day;
    }
    // regular shares
    #[derive(Serialize, Deserialize)]
    pub struct Snapshot {
        pub total_shares: U256,
        pub inflation_amount: U256,
        pub scheduled_to_end: U256,
    }

    // referral shares
    #[derive(Serialize, Deserialize)]
    pub struct RSnapshot {
        pub total_shares: U256,
        pub inflation_amount: U256,
        pub scheduled_to_end: U256,
    }

    // liquidity shares
    #[derive(Serialize, Deserialize)]
    pub struct LSnapShot {
        pub total_shares: U256,
        pub inflation_amount: U256,
    }
}