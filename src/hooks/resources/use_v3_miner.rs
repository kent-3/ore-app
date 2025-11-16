use dioxus::prelude::*;
use ore_api::state::{Miner, miner_pda};
use steel::AccountDeserialize;

use crate::gateway::{Gateway, GatewayError, GatewayResult, Rpc};
use crate::hooks::{use_gateway, use_wallet, Wallet};

/// Hook to fetch the user's Miner account for ORE v3
pub fn use_v3_miner() -> Resource<GatewayResult<Miner>> {
    let wallet = use_wallet();
    
    use_resource(move || async move {
        let gateway = use_gateway();
        // Only fetch if wallet is connected
        let pubkey = match *wallet.read() {
            Wallet::Connected(pk) => pk,
            _ => return Err(GatewayError::AccountNotFound),
        };
        
        let (miner_address, _bump) = miner_pda(pubkey);
        
        match gateway.rpc.get_account_data(&miner_address).await {
            Ok(data) => {
                if data.is_empty() {
                    return Err(GatewayError::AccountNotFound);
                }
                
                Miner::try_from_bytes(&data)
                    .map(|m| *m)
                    .map_err(|e| anyhow::anyhow!("Failed to deserialize Miner: {:?}", e).into())
            }
            Err(e) => Err(e),
        }
    })
}
