use dioxus::prelude::*;

use crate::pages::*;

#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[route("/")]
    Hello {},

    #[route("/explore")]
    Explore {},

    #[route("/deploy")]
    Deploy {},

    #[route("/stake")]
    Stake {},

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
