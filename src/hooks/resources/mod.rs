mod use_board;
// mod use_boost;
mod use_boost_apy;
// mod use_boost_proof;
mod use_boost_config_wss;
// mod use_boost_proof_wss;
mod use_boost_tvl;
mod use_boost_wss;
mod use_liquidity_pair;
// mod use_member; // v1 pools - disabled for v3
mod use_ore_holders;
mod use_ore_price;
mod use_reserve_wss;
// mod use_stake;
mod use_stake_wss;
mod use_token_balance;
mod use_token_balance_wss;
mod use_token_price;
mod use_wss;
mod use_wss_sub;
// v3 protocol hooks
mod use_treasury;
mod use_v3_board;
mod use_v3_config;
mod use_v3_miner;

pub use use_board::*;
// pub use use_boost::*;
pub use use_boost_apy::*;
// pub use use_boost_proof::*;
pub use use_boost_config_wss::*;
// pub use use_boost_proof_wss::*;
pub use use_boost_tvl::*;
pub use use_boost_wss::*;
pub use use_liquidity_pair::*;
// pub use use_member::*; // v1 pools - disabled for v3
pub use use_ore_holders::*;
pub use use_ore_price::*;
pub use use_reserve_wss::*;
// pub use use_stake::*;
pub use use_stake_wss::*;
pub use use_token_balance::*;
pub use use_token_balance_wss::*;
pub use use_token_price::*;
pub use use_wss::*;
pub use use_wss_sub::*;
// v3 protocol hooks
pub use use_treasury::*;
pub use use_v3_board::*;
pub use use_v3_config::*;
pub use use_v3_miner::*;

pub fn use_cache_provider() {
    // V1 boost/pool related - DISABLED for v3 to avoid error spam
    // These providers look for accounts that don't exist in v3
    // use_boosts_wss_provider();
    // use_boost_config_wss_provider();
    // use_boost_proof_wss_provider();
    // use_reserve_balance_wss_provider();
    // use_liquidity_pairs_provider();
    // use_stakes_wss_provider();
    // use_members_provider();
    // use_boost_yield_provider();
    
    // Keep these for basic token functionality
    use_token_balance_provider();
    use_token_balance_wss_provider();
    // Token price provider disabled - requires USDC for quotes
    // use_token_price_provider();
}
