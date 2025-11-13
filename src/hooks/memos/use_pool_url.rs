use dioxus::prelude::*;

// v1 pools - disabled for v3
// use crate::hooks::use_pool;

pub fn use_pool_url() -> Memo<Option<String>> {
    // Pools no longer exist in v3
    use_memo(move || None)
}
