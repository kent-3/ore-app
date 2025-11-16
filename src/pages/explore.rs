use dioxus::prelude::*;
use crate::components::{Header, SolanaIcon};
use crate::hooks::{use_treasury, use_v3_board};

#[component]
pub fn Explore() -> Element {
    // Fetch global protocol data
    let treasury = use_treasury();
    let board = use_v3_board();
    
    // Format helper for large numbers
    let format_ore = |amount: u64| -> String {
        let ore = amount as f64 / 10f64.powi(11); // 11 decimals for ORE
        format!("{:.0}", ore) // No decimals for display
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
                        "Explore"
                    }
                    p {
                        class: "text-elements-lowEmphasis",
                        "Review protocol stats and activity."
                    }
                }

                // Protocol Stats Cards
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-4 mb-12",
                    
                    StatCard {
                        label: "Max Supply",
                        value: "⦿ 5,000,000".to_string(),
                        icon: "⦿"
                    }
                    
                    StatCard {
                        label: "Circulating Supply",
                        value: "⦿ 411,308".to_string(),
                        icon: "⦿"
                    }
                    
                    StatCard {
                        label: "Buried (7d)",
                        value: "⦿ 11,177".to_string(),
                        icon: "⦿"
                    }
                    
                    div {
                        class: "bg-gray-900 rounded-lg p-6",
                        div {
                            class: "flex items-center gap-2 mb-2",
                            SolanaIcon { class: "w-5 h-5".to_string() }
                            span {
                                class: "text-2xl font-bold",
                                "28,246"
                            }
                        }
                        p {
                            class: "text-sm text-elements-lowEmphasis",
                            "Protocol Rev (7d)"
                        }
                    }
                }

                // Mining Section
                div {
                    class: "mb-8",
                    h2 {
                        class: "text-2xl font-semibold mb-2",
                        "Mining"
                    }
                    p {
                        class: "text-elements-lowEmphasis mb-6",
                        "Recent mining activity."
                    }
                    
                    // Mining Table
                    div {
                        class: "bg-gray-900 rounded-lg overflow-x-auto",
                        table {
                            class: "w-full",
                            thead {
                                tr {
                                    class: "bg-gray-800 text-sm text-elements-lowEmphasis",
                                    th { class: "px-6 py-4 text-left font-medium", "Round" }
                                    th { class: "px-6 py-4 text-left font-medium", "Block" }
                                    th { class: "px-6 py-4 text-left font-medium", "ORE Winner" }
                                    th { class: "px-6 py-4 text-left font-medium", "Winners" }
                                    th { class: "px-6 py-4 text-left font-medium", "Deployed" }
                                    th { class: "px-6 py-4 text-left font-medium", "Vaulted" }
                                    th { class: "px-6 py-4 text-left font-medium", "Winnings" }
                                    th { class: "px-6 py-4 text-left font-medium", "Motherlode" }
                                    th { class: "px-6 py-4 text-left font-medium", "Time" }
                                }
                            }
                            tbody {
                                // Table Rows - Show data from board
                                match board.read().as_ref() {
                                    Some(Ok(b)) => rsx! {
                                        MiningRow {
                                            round: format!("#{}", b.round_id),
                                            block: format!("#{}", b.start_slot),
                                            winner: "—".to_string(),
                                            winners: "—".to_string(),
                                            deployed: "—".to_string(),
                                            vaulted: "—".to_string(),
                                            winnings: "—".to_string(),
                                            motherlode: "—".to_string(),
                                            time: "0 sec ago".to_string()
                                        }
                                    },
                                    Some(Err(_)) => rsx! {
                                        tr {
                                            td {
                                                colspan: "9",
                                                class: "px-6 py-12 text-center text-elements-lowEmphasis",
                                                "Error loading mining data"
                                            }
                                        }
                                    },
                                    None => rsx! {
                                        tr {
                                            td {
                                                colspan: "9",
                                                class: "px-6 py-12 text-center text-elements-lowEmphasis",
                                                "Loading mining data..."
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String, icon: String) -> Element {
    rsx! {
        div {
            class: "bg-gray-900 rounded-lg p-6",
            span {
                class: "text-2xl font-bold",
                "{value}"
            }
            p {
                class: "text-sm text-elements-lowEmphasis",
                "{label}"
            }
        }
    }
}

#[component]
fn MiningRow(
    round: String,
    block: String,
    winner: String,
    winners: String,
    deployed: String,
    vaulted: String,
    winnings: String,
    motherlode: String,
    time: String,
) -> Element {
    rsx! {
        tr {
            class: "border-t border-gray-800 hover:bg-gray-800/50 transition-colors text-sm",
            td { class: "px-6 py-4 font-medium", "{round}" }
            td { class: "px-6 py-4", "{block}" }
            td { class: "px-6 py-4 font-mono text-xs", "{winner}" }
            td { class: "px-6 py-4", "{winners}" }
            td { 
                class: "px-6 py-4",
                div { class: "flex items-center gap-1",
                    SolanaIcon { class: "w-3 h-3".to_string() }
                    span { "{deployed}" }
                }
            }
            td { 
                class: "px-6 py-4",
                div { class: "flex items-center gap-1",
                    SolanaIcon { class: "w-3 h-3".to_string() }
                    span { "{vaulted}" }
                }
            }
            td { 
                class: "px-6 py-4",
                div { class: "flex items-center gap-1",
                    SolanaIcon { class: "w-3 h-3".to_string() }
                    span { "{winnings}" }
                }
            }
            td { class: "px-6 py-4", "{motherlode}" }
            td { class: "px-6 py-4 text-elements-lowEmphasis", "{time}" }
        }
    }
}
