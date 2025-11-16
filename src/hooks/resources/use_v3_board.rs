use dioxus::prelude::*;
use ore_api::state::Board;
use steel::AccountDeserialize;

use crate::gateway::{Gateway, GatewayError, GatewayResult, Rpc};
use crate::hooks::use_gateway;

pub fn use_v3_board() -> Resource<GatewayResult<Board>> {
    use_resource(move || async move {
        let gateway = use_gateway();
        let (board_address, _bump) = ore_api::state::board_pda();
        
        match gateway.rpc.get_account_data(&board_address).await {
            Ok(data) => {
                if data.is_empty() {
                    return Err(GatewayError::AccountNotFound);
                }
                
                Board::try_from_bytes(&data)
                    .map(|b| *b)
                    .map_err(|e| anyhow::anyhow!("Failed to deserialize Board: {:?}", e).into())
            }
            Err(e) => Err(e),
        }
    })
}
