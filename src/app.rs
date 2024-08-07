use crate::components::{MenuBar, MenuButton};
use crate::pages::{DataDashboard, EventTagger, RegisterMatchInfo, Shortcuts, UserManual};
use types::{AppError, MatchInfo, Payload, PlayerInfo, TaggedEvent};

use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct CtxProvider {
    pub register_match_info_action: Action<JsValue, JsValue>,
    pub register_player_info_action: Action<JsValue, JsValue>,
    pub register_event_action: Action<JsValue, JsValue>,
    pub open_video_action: Action<JsValue, JsValue>,
    pub match_info: ReadSignal<MatchInfo>,
    pub set_match_info: WriteSignal<MatchInfo>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn convertFileSrc(path: String, protocol: String) -> JsValue;
}

pub async fn get_players_by_match_id(match_id: String) -> Result<Vec<PlayerInfo>, AppError> {
    let match_id = Payload { payload: match_id };
    let match_id = serde_wasm_bindgen::to_value(&match_id).unwrap();
    let team_info = invoke("get_all_players_from_match_id", match_id).await;
    let team_info =
        serde_wasm_bindgen::from_value::<Vec<PlayerInfo>>(team_info).unwrap_or_default();

    Ok(team_info)
}

pub async fn get_all_match() -> Result<Vec<MatchInfo>, AppError> {
    let match_info = invoke("get_all_match_info", JsValue::null()).await;
    let match_info =
        serde_wasm_bindgen::from_value::<Vec<MatchInfo>>(match_info).unwrap_or_default();

    Ok(match_info)
}

pub async fn get_table_data(match_id: String) -> Result<Vec<TaggedEvent>, AppError> {
    let payload = Payload { payload: match_id };
    let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
    let data = invoke("get_events_by_match_id", payload).await;
    let vec_data = serde_wasm_bindgen::from_value::<Vec<TaggedEvent>>(data).unwrap_or_default();

    Ok(vec_data)
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

    let (show_menu, set_show_menu) = create_signal(false);
    let (match_info, set_match_info) = create_signal(MatchInfo::default());

    provide_context(CtxProvider {
        register_match_info_action,
        register_player_info_action,
        register_event_action,
        open_video_action,
        match_info,
        set_match_info,
    });

    view! {
        <Router fallback=|| view! { <p>"Error"</p> }.into_view()>
            <main class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row">
                <MenuButton show_menu set_show_menu/>
                <MenuBar open_video_action show_menu/>
                <Routes>
                    <Route path="/" view=EventTagger/>
                    <Route path="/team_sheet" view=RegisterMatchInfo/>
                    <Route path="/shortcuts" view=Shortcuts/>
                    <Route path="/user_manual" view=UserManual/>
                    <Route path="/dashboard" view=DataDashboard/>
                </Routes>
            </main>
        </Router>
    }
}
