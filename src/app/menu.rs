use super::invoke;
use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

const STYLE: &str = "border-none rounded-full bg-lime-400 px-4 hover:bg-indigo-600 h-[30px] w-[200px] text-xs text-black hover:text-white";
const MENU_BAR: &str = "block absolute z-20 left-[35px] top-[10px] flex flex-col rounded-xl bg-slate-800/[.85] p-[10px] w-fit space-y-[10px] ease-in-out duration-300";

#[component]
pub fn MenuBar(menu_bar_node_ref: NodeRef<html::Div>) -> impl IntoView {
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
                <SaveFile/>
                <Settings/>
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
pub fn SaveFile() -> impl IntoView {
    view! {
        <button
            class=STYLE
        >"Save File"</button>
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <button
            class=STYLE
        >"Settings"</button>
    }
}
