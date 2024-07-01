use crate::app::invoke;
use types::{MatchInfo, Payload};

use leptos::*;
use leptos_router::A;
use wasm_bindgen::JsValue;

const BUTTON_STYLE: &str = "border-none rounded-full bg-lime-400 px-4 hover:bg-indigo-600 h-[30px] w-[200px] text-xs text-black hover:text-white";
const MENU_BAR: &str = "block absolute z-20 left-[35px] top-[10px] flex flex-col rounded-xl bg-slate-600/[.85] p-[10px] w-fit space-y-[10px] ease-in-out duration-300";

#[component]
pub fn MenuButton(show_menu: ReadSignal<bool>, set_show_menu: WriteSignal<bool>) -> impl IntoView {
    let toggle_menu = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_show_menu.update(|v| *v = !*v);
    };

    view! {
        <div>
            <button
                on:click=toggle_menu
                class="border-none bg-slate-300 hover:bg-lime-400 px-2 size-[30px] rounded-r-lg">
                <Show
                    when=move || !show_menu.get()
                    fallback=move || view! { <img src="public/buttons/close.svg"/> }
                >
                    <img src="public/buttons/menu.svg"/>
                </Show>
            </button>
        </div>
    }
}

#[component]
pub fn MenuBar(
    match_info: ReadSignal<MatchInfo>,
    open_video_action: Action<JsValue, JsValue>,
    show_menu: ReadSignal<bool>,
) -> impl IntoView {
    let get_file_path = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        open_video_action.dispatch(JsValue::null());
    };

    view! {
        <Show when=move || show_menu.get()>
            <div
                class=MENU_BAR
            >
                <Home/>
                <OpenVideo get_file_path/>
                <RegisterMatchInfo/>
                <ExportData match_info/>
                <DataDashboard/>
                <ShortcutsInfo/>
                <UserManual/>
            </div>
        </Show>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <A href="/">
            <button
                class=BUTTON_STYLE
            >"Home"</button>
        </A>
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
    view! {
        <A href="/team_sheet">
            <button
                class=BUTTON_STYLE
            >"Register Match Info"</button>
        </A>
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
fn DataDashboard() -> impl IntoView {
    view! {
        <A href="/dashboard">
            <button
                class=BUTTON_STYLE
            >"Data Dashboard"</button>
        </A>
    }
}

#[component]
fn ShortcutsInfo() -> impl IntoView {
    view! {
        <A href="/shortcuts">
            <button
                class=BUTTON_STYLE
            >"Shortcuts Info"</button>
        </A>
    }
}

#[component]
fn UserManual() -> impl IntoView {
    view! {
        <A href="/user_manual">
            <button
                class=BUTTON_STYLE
            >"User Manual"</button>
        </A>
    }
}
