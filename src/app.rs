pub mod emoji;

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

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let filtered_emojis = move || {
        let query = search_query.get();
        emoji::search(&query)
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

    // Auto-focus the search input when component mounts
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    view! {
        <main class="emoji-picker">
            <div class="search-container">
                <input
                    node_ref=input_ref
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
                    key=|emoji| emoji.as_str()
                    children=move |emoji| {
                        view! {
                            <button
                                class="emoji-button"
                                on:click=move |_| copy_emoji(emoji.to_string())
                                title=emoji.name()
                            >
                                {emoji.as_str()}
                            </button>
                        }
                    }
                />
            </div>
        </main>
    }
}
