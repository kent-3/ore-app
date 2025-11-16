use dioxus::prelude::*;
use crate::components::Header;

#[component]
pub fn Deploy() -> Element {
    
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
                        "Deploy"
                    }
                    p {
                        class: "text-elements-lowEmphasis",
                        "Deploy SOL to the mining grid to earn ORE rewards"
                    }
                }

                // Grid Container
                div {
                    class: "max-w-3xl mx-auto",
                    
                    // 5x5 Mining Grid
                    div {
                        class: "bg-gray-900 rounded-lg p-6 mb-6",
                        h2 {
                            class: "text-xl font-semibold mb-4",
                            "Mining Grid"
                        }
                        div {
                            class: "grid grid-cols-5 gap-2",
                            for square in 0..25 {
                                GridSquare {
                                    index: square,
                                    deployed: 0.0
                                }
                            }
                        }
                    }

                    // Deploy Controls
                    div {
                        class: "bg-gray-900 rounded-lg p-6",
                        h3 {
                            class: "text-lg font-semibold mb-4",
                            "Deploy SOL"
                        }
                        div {
                            class: "space-y-4",
                            div {
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "Amount (SOL)"
                                }
                                input {
                                    r#type: "number",
                                    class: "w-full px-4 py-2 bg-gray-800 rounded-lg border border-gray-700 focus:border-blue-500 focus:outline-none",
                                    placeholder: "0.1",
                                    step: "0.01",
                                    min: "0"
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "Select Squares"
                                }
                                p {
                                    class: "text-sm text-elements-lowEmphasis",
                                    "Click on grid squares above to select deployment positions"
                                }
                            }
                            button {
                                class: "w-full py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors",
                                "Deploy"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String) -> Element {
    rsx! {
        div {
            class: "bg-gray-900 rounded-lg p-4",
            p {
                class: "text-sm text-elements-lowEmphasis mb-1",
                "{label}"
            }
            p {
                class: "text-xl font-bold",
                "{value}"
            }
        }
    }
}

#[component]
fn GridSquare(index: i32, deployed: f64) -> Element {
    let mut is_selected = use_signal(|| false);
    
    let bg_color = if *is_selected.read() {
        "bg-blue-600"
    } else if deployed > 0.0 {
        "bg-green-700"
    } else {
        "bg-gray-800"
    };

    rsx! {
        button {
            class: "aspect-square {bg_color} hover:brightness-110 rounded transition-all border-2 border-gray-700 hover:border-gray-500 flex flex-col items-center justify-center",
            onclick: move |_| {
                is_selected.set(!is_selected());
            },
            span {
                class: "text-xs text-elements-lowEmphasis",
                "{index}"
            }
            if deployed > 0.0 {
                span {
                    class: "text-xs font-semibold mt-1",
                    "{deployed:.2}"
                }
            }
        }
    }
}
