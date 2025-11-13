use dioxus::prelude::*;

#[component]
pub fn WalletDrawer(
    on_close: EventHandler<MouseEvent>,
    wallet_remount: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-full w-full bg-gray-900 text-white",
            "Wallet Drawer (Temporarily Disabled)"
        }
    }
}
