use super::invoke;
use leptos::*;
use types::{MatchData, Payload};
use wasm_bindgen::UnwrapThrowExt;

const STYLE: &str = "border-none rounded-full bg-lime-400 px-4 hover:bg-indigo-600 h-[30px] w-[200px] text-xs text-black hover:text-white";
const MENU_BAR: &str = "block absolute z-20 left-[35px] top-[10px] flex flex-col rounded-xl bg-slate-800/[.85] p-[10px] w-fit space-y-[10px] ease-in-out duration-300";

#[component]
pub fn MenuBar(
    menu_bar_node_ref: NodeRef<html::Div>,
    match_data: ReadSignal<MatchData>,
) -> impl IntoView {
    let set_video_src = expect_context::<WriteSignal<Option<String>>>();
    let show_menu = expect_context::<ReadSignal<bool>>();

    // pls refer to this documentation: https://tauri.app/v1/api/js/tauri/#convertfilesrc
    let get_file_path = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let path_protocol = invoke("open", wasm_bindgen::JsValue::null()).await;
            let (path, protocol): (String, String) =
                serde_wasm_bindgen::from_value(path_protocol).unwrap_throw();
            let resolved_path = format!("{}://localhost/{}", protocol, path);

            logging::log!("resolved path: {:?}", resolved_path);

            set_video_src.set(Some(resolved_path));
        });
    };

    view! {
        <Show when=move || show_menu.get() == true>
            <div
                _ref=menu_bar_node_ref
                class=MENU_BAR
            >
                <OpenVideo get_file_path/>
                <RegisterMatchInfo/>
                <ExportData match_data/>
                <ShortcutsInfo/>
            </div>
        </Show>
    }
}

#[component]
pub fn OpenVideo<F: FnMut(ev::MouseEvent) + 'static>(get_file_path: F) -> impl IntoView {
    view! {
        <button
            on:click=get_file_path
            class=STYLE
        >"Open Video"</button>
    }
}

#[component]
pub fn RegisterMatchInfo() -> impl IntoView {
    let navigate = move |_: ev::MouseEvent| {
        let nav = leptos_router::use_navigate();
        nav("/team_sheet", Default::default());
    };

    view! {
        <button
            on:click=navigate
            class=STYLE
        >"Register Match Info"</button>
    }
}

#[component]
pub fn ExportData(match_data: ReadSignal<MatchData>) -> impl IntoView {
    let export_data = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let match_id = match_data.get_untracked().match_id;
            let payload = Payload { payload: match_id };
            let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
            invoke("export_tagged_events_from_match_id", payload).await;
        });
    };

    view! {
        <button
            on:click=export_data
            class=STYLE
        >"Export Data"</button>
    }
}

#[component]
pub fn ShortcutsInfo() -> impl IntoView {
    let navigate = move |_: ev::MouseEvent| {
        let nav = leptos_router::use_navigate();
        nav("shortcuts", Default::default());
    };

    view! {
        <button
            on:click=navigate
            class=STYLE
        >"Shortcuts Info"</button>
    }
}
