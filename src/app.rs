use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

pub mod event_tagger;
pub mod menu;
pub mod pitch;
pub mod register_match_info;
pub mod shortcuts;
pub mod table_data;
pub mod team_sheet;
pub mod video;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn convertFileSrc(path: String, protocol: String) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let register_action =
        create_action(|payload: &JsValue| invoke("register_match_info", payload.clone()));

    provide_context(register_action);

    view! {
        <Router fallback=|| view! { <p>"Error"</p> }.into_view()>
            <main class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-col">
                <Routes>
                    <Route path="/" view=event_tagger::EventTagger/>
                    <Route path="/team_sheet" view=register_match_info::RegisterMatchInfo/>
                    <Route path="/shortcuts" view=shortcuts::Shortcuts/>
                </Routes>
            </main>
        </Router>
    }
}
