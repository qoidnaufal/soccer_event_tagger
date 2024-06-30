use crate::app::invoke;
use leptos::*;
use types::{MatchInfo, Payload};
use wasm_bindgen::JsValue;

const BUTTON_STYLE: &str = "border-none rounded-full bg-lime-400 px-4 hover:bg-indigo-600 h-[30px] w-[200px] text-xs text-black hover:text-white";
const MENU_BAR: &str = "block absolute z-20 left-[35px] top-[10px] flex flex-col rounded-xl bg-slate-600/[.85] p-[10px] w-fit space-y-[10px] ease-in-out duration-300";

#[component]
pub fn MenuBar(
    match_info: ReadSignal<MatchInfo>,
    open_video_action: Action<JsValue, JsValue>,
) -> impl IntoView {
    let show_menu = expect_context::<ReadSignal<bool>>();

    // pls refer to this documentation: https://tauri.app/v1/api/js/tauri/#convertfilesrc
    let get_file_path = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        open_video_action.dispatch(JsValue::null());
    };

    view! {
        <Show when=move || show_menu.get()>
            <div
                class=MENU_BAR
            >
                <OpenVideo get_file_path/>
                <RegisterMatchInfo/>
                <ExportData match_info/>
                <ClearDatabse/>
                <ShortcutsInfo/>
                <UserManual/>
            </div>
        </Show>
    }
}

#[component]
fn OpenVideo<F: FnMut(ev::MouseEvent) + 'static>(get_file_path: F) -> impl IntoView {
    view! {
        <button
            on:click=get_file_path
            class=BUTTON_STYLE
        >"Open Video"</button>
    }
}

#[component]
fn RegisterMatchInfo() -> impl IntoView {
    let navigate = move |_: ev::MouseEvent| {
        let nav = leptos_router::use_navigate();
        nav("/team_sheet", Default::default());
    };

    view! {
        <button
            on:click=navigate
            class=BUTTON_STYLE
        >"Register Match Info"</button>
    }
}

#[component]
fn ExportData(match_info: ReadSignal<MatchInfo>) -> impl IntoView {
    let export_data = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let payload = match_info.get_untracked();
            let payload = Payload { payload };
            let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
            invoke("export_tagged_events_from_match_id", payload).await;
        });
    };

    view! {
        <button
            on:click=export_data
            class=BUTTON_STYLE
        >"Export Data [*.csv]"</button>
    }
}

#[component]
fn ClearDatabse() -> impl IntoView {
    let clear_db = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            invoke("clear_db", JsValue::null()).await;
        })
    };

    view! {
        <button
            on:click=clear_db
            class=BUTTON_STYLE
        >"Clear Database"</button>
    }
}

#[component]
fn ShortcutsInfo() -> impl IntoView {
    let navigate = move |_: ev::MouseEvent| {
        let nav = leptos_router::use_navigate();
        nav("shortcuts", Default::default());
    };

    view! {
        <button
            on:click=navigate
            class=BUTTON_STYLE
        >"Shortcuts Info"</button>
    }
}

#[component]
fn UserManual() -> impl IntoView {
    let navigate = move |_: ev::MouseEvent| {
        let nav = leptos_router::use_navigate();
        nav("user_manual", Default::default());
    };

    view! {
        <button
            on:click=navigate
            class=BUTTON_STYLE
        >"User Manual"</button>
    }
}
