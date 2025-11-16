use dioxus::prelude::*;

use crate::hooks::{use_board, use_ore_price, use_wallet, Wallet};

#[component]
pub fn Grid() -> Element {
    // Fetch blockchain state
    let _board = use_board();
    let wallet = use_wallet();
    
    // Get ORE price  
    let ore_price = use_ore_price();
    let ore_price_val = ore_price.read().unwrap_or(0.0);

    // UI state
    let mut sol_amount = use_signal(|| 0.0);
    let mut is_manual = use_signal(|| true);
    let mut selected_cell = use_signal(|| None::<usize>);
    
    // Mock grid data for now - will be replaced with real round data
    let grid_cells: Vec<(usize, f64, u64)> = (0..25)
        .map(|i| (i, 0.7 + (i as f64 * 0.01), 550 + i as u64))
        .collect();
    let motherlode = 55.6;
    let total_deployed = 19.7788;
    
    // Get wallet status
    let is_connected = matches!(*wallet.read(), Wallet::Connected(_));

    rsx! {
        div {
            class: "min-h-screen bg-[#0a0a0a] text-white",

            // Top Navigation
            nav {
                class: "flex items-center justify-between px-6 py-4 border-b border-gray-800",
                
                div {
                    class: "flex items-center gap-8",
                    // Logo
                    div {
                        class: "text-2xl font-bold",
                        "ORE"
                    }
                    // Nav links
                    div {
                        class: "flex gap-6 text-gray-400",
                        a { class: "hover:text-white transition-colors", href: "/", "About" }
                        a { class: "hover:text-white transition-colors", "Explore" }
                        a { class: "hover:text-white transition-colors", "Stake" }
                    }
                }

                div {
                    class: "flex items-center gap-4",
                    // Price indicator
                    div {
                        class: "flex items-center gap-2 text-sm",
                        span { class: "text-gray-400", "⚪ ORE" }
                        span { "${ore_price_val:.2}" }
                    }
                    // Connect button
                    button {
                        class: "bg-white text-black px-6 py-2 rounded-full font-semibold hover:bg-gray-200 transition-colors",
                        if is_connected {
                            "Connected"
                        } else {
                            "Connect"
                        }
                    }
                }
            }

            // Main content
            div {
                class: "flex gap-6 p-6",

                // Left side - Grid
                div {
                    class: "flex-1",
                    
                    div {
                        class: "grid grid-cols-5 gap-3",
                        
                        for (cell_index, multiplier, participants) in grid_cells.iter() {
                            {
                                let is_selected = selected_cell() == Some(*cell_index);
                                let cell_id = *cell_index;
                                
                                rsx! {
                                    div {
                                        key: "{cell_id}",
                                        class: "bg-gray-900/50 border border-gray-700 rounded-lg p-4 hover:border-gray-600 transition-all cursor-pointer",
                                        class: if is_selected { "border-yellow-500" } else { "" },
                                        onclick: move |_| {
                                            selected_cell.set(Some(cell_id));
                                        },
                                        
                                        // Header with cell number and participants
                                        div {
                                            class: "flex justify-between items-center mb-6 text-gray-400 text-sm",
                                            span { "#{cell_id + 1}" }
                                            span {
                                                class: "flex items-center gap-1",
                                                "{participants} 👤"
                                            }
                                        }
                                        
                                        // Multiplier
                                        div {
                                            class: "text-white text-xl font-mono text-center",
                                            "{multiplier:.4}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right side - Control Panel
                div {
                    class: "w-96 space-y-4",

                    // Motherload indicator
                    div {
                        class: "bg-gray-900/50 border-2 border-yellow-500 rounded-lg p-4",
                        div {
                            class: "flex items-center justify-center gap-2 text-2xl font-bold",
                            span { "⚪ {motherlode:.1}" }
                        }
                        div {
                            class: "text-center text-gray-400 text-sm mt-1",
                            "Motherload"
                        }
                    }

                    // Timer
                    div {
                        class: "bg-gray-900/50 border border-gray-700 rounded-lg p-4 text-center",
                        div {
                            class: "text-3xl font-mono mb-1",
                            "00:18"
                        }
                        div {
                            class: "text-gray-400 text-sm",
                            "Time remaining"
                        }
                    }

                    // Stats
                    div {
                        class: "grid grid-cols-2 gap-4",
                        
                        div {
                            class: "bg-gray-900/50 border border-gray-700 rounded-lg p-4",
                            div {
                                class: "flex items-center gap-2 text-xl mb-1",
                                span { "◎" }
                                span { "{total_deployed:.4}" }
                            }
                            div {
                                class: "text-gray-400 text-sm",
                                "Total deployed"
                            }
                        }

                        div {
                            class: "bg-gray-900/50 border border-gray-700 rounded-lg p-4",
                            div {
                                class: "flex items-center gap-2 text-xl mb-1",
                                span { "◎" }
                                span { "0.0000" }
                            }
                            div {
                                class: "text-gray-400 text-sm",
                                "You deployed"
                            }
                        }
                    }

                    // Manual/Auto toggle
                    div {
                        class: "bg-gray-900/50 border border-gray-700 rounded-lg p-1 flex",
                        
                        button {
                            class: if is_manual() { "flex-1 py-2 bg-gray-800 rounded font-semibold" } else { "flex-1 py-2 text-gray-400 hover:text-white transition-colors" },
                            onclick: move |_| is_manual.set(true),
                            "Manual"
                        }
                        button {
                            class: if !is_manual() { "flex-1 py-2 bg-gray-800 rounded font-semibold" } else { "flex-1 py-2 text-gray-400 hover:text-white transition-colors" },
                            onclick: move |_| is_manual.set(false),
                            "Auto"
                        }
                    }

                    // SOL input section
                    div {
                        class: "bg-gray-900/50 border border-gray-700 rounded-lg p-4 space-y-3",
                        
                        div {
                            class: "text-gray-400 text-sm",
                            "0 SOL"
                        }

                        div {
                            class: "flex gap-2",
                            button {
                                class: "px-4 py-1 bg-gray-800 rounded hover:bg-gray-700 transition-colors text-sm",
                                onclick: move |_| sol_amount.set(sol_amount() + 1.0),
                                "+1"
                            }
                            button {
                                class: "px-4 py-1 bg-gray-800 rounded hover:bg-gray-700 transition-colors text-sm",
                                onclick: move |_| sol_amount.set(sol_amount() + 0.1),
                                "+0.1"
                            }
                            button {
                                class: "px-4 py-1 bg-gray-800 rounded hover:bg-gray-700 transition-colors text-sm",
                                onclick: move |_| sol_amount.set(sol_amount() + 0.01),
                                "+0.01"
                            }
                        }

                        div {
                            class: "flex items-center justify-between",
                            span {
                                class: "flex items-center gap-2",
                                span { "◎" }
                                span { "SOL" }
                            }
                            span {
                                class: "text-3xl font-mono text-gray-400",
                                "{sol_amount():.1}"
                            }
                        }

                        div {
                            class: "flex justify-between text-sm",
                            span { 
                                class: "text-gray-400",
                                "Blocks"
                            }
                            span { "x0" }
                        }

                        div {
                            class: "flex justify-between font-semibold",
                            span { "Total" }
                            span { "{sol_amount():.1} SOL" }
                        }
                    }

                    // Deploy button
                    button {
                        class: "w-full bg-gray-800 hover:bg-gray-700 text-white font-bold py-4 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: selected_cell().is_none(),
                        "Deploy"
                    }
                }
            }
        }
    }
}
