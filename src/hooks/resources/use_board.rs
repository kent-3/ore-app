use dioxus::prelude::*;
use ore_api::state::{board_pda, Board};

use crate::{
    gateway::{ore::OreGateway, GatewayError, GatewayResult},
    hooks::use_gateway,
};

pub fn use_board() -> Resource<GatewayResult<Board>> {
    use_resource(move || async move {
        let board_address = board_pda().0;
        use_gateway()
            .get_board(board_address)
            .await
            .map_err(GatewayError::from)
    })
}
