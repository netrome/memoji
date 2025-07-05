use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct CopyArgs {
    text: String,
}

#[derive(Clone, Debug)]
struct Emoji {
    symbol: &'static str,
    name: &'static str,
    keywords: &'static [&'static str],
}

// Hardcoded emoji list for MVP
const EMOJIS: &[Emoji] = &[
    Emoji {
        symbol: "😀",
        name: "grinning face",
        keywords: &["happy", "smile", "grin"],
    },
    Emoji {
        symbol: "😃",
        name: "grinning face with big eyes",
        keywords: &["happy", "smile", "joy"],
    },
    Emoji {
        symbol: "😄",
        name: "grinning face with smiling eyes",
        keywords: &["happy", "smile", "joy"],
    },
    Emoji {
        symbol: "😁",
        name: "beaming face with smiling eyes",
        keywords: &["happy", "smile", "grin"],
    },
    Emoji {
        symbol: "😆",
        name: "grinning squinting face",
        keywords: &["happy", "laugh", "funny"],
    },
    Emoji {
        symbol: "😅",
        name: "grinning face with sweat",
        keywords: &["happy", "sweat", "relief"],
    },
    Emoji {
        symbol: "😂",
        name: "face with tears of joy",
        keywords: &["laugh", "funny", "joy"],
    },
    Emoji {
        symbol: "🤣",
        name: "rolling on the floor laughing",
        keywords: &["laugh", "funny", "rofl"],
    },
    Emoji {
        symbol: "😊",
        name: "smiling face with smiling eyes",
        keywords: &["happy", "smile", "blush"],
    },
    Emoji {
        symbol: "😇",
        name: "smiling face with halo",
        keywords: &["angel", "innocent", "halo"],
    },
    Emoji {
        symbol: "😋",
        name: "face savoring food",
        keywords: &["yum", "delicious", "food"],
    },
    Emoji {
        symbol: "😎",
        name: "smiling face with sunglasses",
        keywords: &["cool", "sunglasses", "awesome"],
    },
    Emoji {
        symbol: "😍",
        name: "smiling face with heart-eyes",
        keywords: &["love", "heart", "crush"],
    },
    Emoji {
        symbol: "😘",
        name: "face blowing a kiss",
        keywords: &["kiss", "love", "flirt"],
    },
    Emoji {
        symbol: "😗",
        name: "kissing face",
        keywords: &["kiss", "love", "smooch"],
    },
    Emoji {
        symbol: "😙",
        name: "kissing face with smiling eyes",
        keywords: &["kiss", "love", "happy"],
    },
    Emoji {
        symbol: "😚",
        name: "kissing face with closed eyes",
        keywords: &["kiss", "love", "peace"],
    },
    Emoji {
        symbol: "🙂",
        name: "slightly smiling face",
        keywords: &["smile", "happy", "subtle"],
    },
    Emoji {
        symbol: "🤗",
        name: "hugging face",
        keywords: &["hug", "embrace", "comfort"],
    },
    Emoji {
        symbol: "🤔",
        name: "thinking face",
        keywords: &["think", "hmm", "wonder"],
    },
];

fn search_emojis(query: &str) -> Vec<Emoji> {
    if query.is_empty() {
        return EMOJIS.iter().cloned().collect();
    }

    let query_lower = query.to_lowercase();
    let mut results: Vec<Emoji> = EMOJIS
        .iter()
        .filter(|emoji| {
            emoji.name.contains(&query_lower)
                || emoji
                    .keywords
                    .iter()
                    .any(|keyword| keyword.contains(&query_lower))
        })
        .cloned()
        .collect();

    // Limit results to prevent UI overflow
    results.truncate(10);
    results
}

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    let filtered_emojis = move || {
        let query = search_query.get();
        search_emojis(&query)
    };

    let update_search = move |ev| {
        let value = event_target_value(&ev);
        set_search_query.set(value);
    };

    let copy_emoji = move |emoji: String| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&CopyArgs { text: emoji }).unwrap();
            let _ = invoke("copy_to_clipboard", args).await;
        });
    };

    view! {
        <main class="emoji-picker">
            <div class="search-container">
                <input
                    type="text"
                    placeholder="Search emojis..."
                    class="search-input"
                    on:input=update_search
                    prop:value=search_query
                />
            </div>

            <div class="emoji-grid">
                <For
                    each=filtered_emojis
                    key=|emoji| emoji.symbol
                    children=move |emoji| {
                        let emoji_symbol = emoji.symbol.to_string();
                        view! {
                            <button
                                class="emoji-button"
                                on:click=move |_| copy_emoji(emoji_symbol.clone())
                                title=emoji.name
                            >
                                {emoji.symbol}
                            </button>
                        }
                    }
                />
            </div>
        </main>
    }
}
