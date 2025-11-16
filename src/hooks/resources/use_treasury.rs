use dioxus::prelude::*;
use ore_api::state::Treasury;
use steel::AccountDeserialize;

use crate::gateway::{Gateway, GatewayError, GatewayResult, Rpc};
use crate::hooks::use_gateway;

pub fn use_treasury() -> Resource<GatewayResult<Treasury>> {
    use_resource(move || async move {
        let gateway = use_gateway();
        let treasury_address = ore_api::consts::TREASURY_ADDRESS;
        
        match gateway.rpc.get_account_data(&treasury_address).await {
            Ok(data) => {
                if data.is_empty() {
                    return Err(GatewayError::AccountNotFound);
                }
                
                Treasury::try_from_bytes(&data)
                    .map(|t| *t)
                    .map_err(|e| anyhow::anyhow!("Failed to deserialize Treasury: {:?}", e).into())
            }
            Err(e) => Err(e),
        }
    })
}
