use dioxus::prelude::*;

use crate::pages::*;

#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[route("/")]
    Hello {},

    #[route("/deploy")]
    Deploy {},

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
