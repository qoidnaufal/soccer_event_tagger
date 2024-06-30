use crate::pages::{EventTagger, RegisterMatchInfo, Shortcuts, UserManual};

use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn convertFileSrc(path: String, protocol: String) -> JsValue;
}

#[derive(Clone)]
pub struct CtxProvider {
    pub register_match_info_action: Action<JsValue, JsValue>,
    pub register_player_info_action: Action<JsValue, JsValue>,
    pub register_event_action: Action<JsValue, JsValue>,
    pub open_video_action: Action<JsValue, JsValue>,
}

#[component]
pub fn App() -> impl IntoView {
    let register_match_info_action =
        create_action(|payload: &JsValue| invoke("register_match_info", payload.clone()));
    let register_player_info_action =
        create_action(|payload: &JsValue| invoke("register_player_info", payload.clone()));
    let register_event_action =
        create_action(|payload: &JsValue| invoke("insert_data", payload.clone()));
    let open_video_action = create_action(|payload: &JsValue| invoke("open", payload.clone()));

    provide_context(CtxProvider {
        register_match_info_action,
        register_player_info_action,
        register_event_action,
        open_video_action,
    });

    view! {
        <Router fallback=|| view! { <p>"Error"</p> }.into_view()>
            <main>
                <Routes>
                    <Route path="/" view=EventTagger/>
                    <Route path="/team_sheet" view=RegisterMatchInfo/>
                    <Route path="/shortcuts" view=Shortcuts/>
                    <Route path="/user_manual" view=UserManual/>
                </Routes>
            </main>
        </Router>
    }
}
