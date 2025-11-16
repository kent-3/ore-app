use dioxus::prelude::*;
use ore_api::state::Config;
use steel::AccountDeserialize;

use crate::gateway::{Gateway, GatewayError, GatewayResult, Rpc};
use crate::hooks::use_gateway;

/// Hook to fetch the Config account for ORE v3
pub fn use_v3_config() -> Resource<GatewayResult<Config>> {
    use_resource(move || async move {
        let gateway = use_gateway();
        let (config_address, _bump) = ore_api::state::config_pda();
        
        match gateway.rpc.get_account_data(&config_address).await {
            Ok(data) => {
                if data.is_empty() {
                    return Err(GatewayError::AccountNotFound);
                }
                
                Config::try_from_bytes(&data)
                    .map(|c| *c)
                    .map_err(|e| anyhow::anyhow!("Failed to deserialize Config: {:?}", e).into())
            }
            Err(e) => Err(e),
        }
    })
}
