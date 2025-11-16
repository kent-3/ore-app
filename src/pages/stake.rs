use dioxus::prelude::*;
use crate::components::Header;
use crate::hooks::{use_treasury, use_wallet, Wallet};

#[component]
pub fn Stake() -> Element {
    let treasury = use_treasury();
    let wallet = use_wallet();
    
    // Format helpers
    let format_ore = |amount: u64| -> String {
        let ore = amount as f64 / 10f64.powi(11);
        format!("{:.4}", ore)
    };
    
    // Calculate APR (placeholder - needs real calculation)
    let _calculate_apr = || -> String {
        "23.41%".to_string()
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-base-bg text-elements-highEmphasis",
            Header {}
            div {
                class: "container mx-auto px-6 pt-24 pb-12",
                
                // Page Header
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-4xl font-bold mb-2",
                        "Stake"
                    }
                    p {
                        class: "text-elements-lowEmphasis",
                        "Earn a share of protocol revenue."
                    }
                }

                // Stake Form
                div {
                    class: "max-w-2xl mx-auto mb-8",
                    div {
                        class: "bg-gray-900 rounded-lg p-6",
                        
                        // Deposit/Withdraw Tabs
                        div {
                            class: "flex gap-2 mb-6",
                            button {
                                class: "flex-1 py-3 px-6 bg-blue-600 rounded-lg font-semibold",
                                "Deposit"
                            }
                            button {
                                class: "flex-1 py-3 px-6 bg-gray-800 rounded-lg font-semibold text-elements-lowEmphasis hover:bg-gray-700",
                                "Withdraw"
                            }
                        }
                        
                        // Amount Input
                        div {
                            class: "mb-6",
                            div {
                                class: "flex justify-between items-center mb-2",
                                label {
                                    class: "text-sm text-elements-lowEmphasis",
                                    "0 ORE"
                                }
                                div {
                                    class: "flex gap-2",
                                    button {
                                        class: "px-3 py-1 text-sm bg-gray-800 rounded hover:bg-gray-700",
                                        "HALF"
                                    }
                                    button {
                                        class: "px-3 py-1 text-sm bg-gray-800 rounded hover:bg-gray-700",
                                        "ALL"
                                    }
                                }
                            }
                            div {
                                class: "flex items-center gap-3 bg-gray-800 rounded-lg px-4 py-3",
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 bg-orange-500 rounded-full flex items-center justify-center",
                                        "⦿"
                                    }
                                    span { "ORE" }
                                }
                                input {
                                    r#type: "text",
                                    class: "flex-1 bg-transparent text-right text-2xl outline-none",
                                    placeholder: "1.0",
                                }
                            }
                        }
                        
                        // Deposit Button
                        match *wallet.read() {
                            Wallet::Disconnected => rsx! {
                                button {
                                    class: "w-full py-3 bg-gray-700 rounded-lg font-semibold cursor-not-allowed",
                                    disabled: true,
                                    "Connect Wallet"
                                }
                            },
                            Wallet::Connected(_) => rsx! {
                                button {
                                    class: "w-full py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors",
                                    "Deposit"
                                }
                            }
                        }
                    }
                }

                // Summary Section
                div {
                    class: "max-w-2xl mx-auto",
                    h2 {
                        class: "text-2xl font-semibold mb-4",
                        "Summary"
                    }
                    div {
                        class: "bg-gray-900 rounded-lg p-6 space-y-4",
                        
                        SummaryRow {
                            label: "Total deposits",
                            value: match treasury.read().as_ref() {
                                Some(Ok(t)) => format!("⦿ {}", format_ore(t.total_staked)),
                                Some(Err(_)) => "Error".to_string(),
                                None => "Loading...".to_string()
                            }
                        }
                        
                        SummaryRow {
                            label: "APR",
                            value: "23.41%".to_string()
                        }
                        
                        SummaryRow {
                            label: "TVL",
                            value: "$80,844,731".to_string()
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SummaryRow(label: String, value: String) -> Element {
    rsx! {
        div {
            class: "flex justify-between items-center",
            div {
                class: "flex items-center gap-2",
                span {
                    class: "text-elements-lowEmphasis",
                    "{label}"
                }
                span {
                    class: "w-4 h-4 rounded-full bg-gray-700 flex items-center justify-center text-xs",
                    "i"
                }
            }
            span {
                class: "font-semibold",
                "{value}"
            }
        }
    }
}
