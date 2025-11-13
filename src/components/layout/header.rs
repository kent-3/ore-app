use dioxus::prelude::*;
use crate::route::Route;
use crate::components::WalletAdapter;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 z-50 bg-base-bg/80 backdrop-blur-sm border-b border-gray-800",
            div {
                class: "max-w-[1920px] mx-auto px-6 lg:px-8",
                div {
                    class: "flex items-center justify-between h-20",
                    
                    // Left: Logo and Navigation
                    div {
                        class: "flex items-center gap-12",
                        
                        // Logo
                        Link {
                            to: Route::Hello {},
                            class: "flex items-center",
                            span {
                                class: "text-elements-highEmphasis text-2xl font-bold font-wide tracking-wider",
                                "ORE"
                            }
                        }
                        
                        // Navigation Links
                        nav {
                            class: "hidden md:flex items-center gap-8",
                            Link {
                                to: Route::Deploy {},
                                class: "text-elements-lowEmphasis hover:text-elements-highEmphasis transition-colors font-medium",
                                "Deploy"
                            }
                            NavLink { label: "Stake" }
                            NavLink { label: "About" }
                        }
                    }
                    
                    // Right: Price and Connect Button
                    div {
                        class: "flex items-center gap-6",
                        
                        // ORE Price Display
                        div {
                            class: "hidden sm:flex items-center gap-2 text-elements-highEmphasis",
                            span {
                                class: "text-lg",
                                "⛏"
                            }
                            span {
                                class: "font-semibold",
                                "ORE"
                            }
                            span {
                                class: "text-elements-lowEmphasis",
                                "$292.72"
                            }
                        }
                        
                        // Wallet Adapter (Connect Button)
                        WalletAdapter {}
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(label: String) -> Element {
    rsx! {
        button {
            class: "text-elements-lowEmphasis hover:text-elements-highEmphasis transition-colors font-medium",
            "{label}"
        }
    }
}
