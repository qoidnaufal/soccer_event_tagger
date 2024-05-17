use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn convertFileSrc(path: &str) -> JsValue;
}

const STYLE: &str = "border-none rounded-full bg-lime-400 px-4 hover:bg-indigo-600 h-[30px] w-[130px] text-xs text-black hover:text-white";

#[component]
pub fn MenuBar(menu_bar: NodeRef<html::Div>) -> impl IntoView {
    let set_video_src = expect_context::<WriteSignal<Option<String>>>();
    let show_menu = expect_context::<ReadSignal<bool>>();

    // pls refer to this documentation: https://tauri.app/v1/api/js/tauri/#convertfilesrc
    let get_file_path = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let res = invoke("open").await.as_string().unwrap();
            if let Some(res) = res.strip_prefix("\"") {
                let res = res.strip_suffix("\"").unwrap();
                let resolved_path = convertFileSrc(res).as_string().unwrap();
                logging::log!("resolved path: {:?}", resolved_path);
                set_video_src.set(Some(resolved_path));
            }
        });
    };

    view! {
        <Show when=move || show_menu.get()>
            <div
                _ref=menu_bar
                class="block absolute z-20 left-[35px] top-[10px] flex flex-col rounded-xl bg-slate-800/[.85] p-[10px] w-fit space-y-[10px] ease-in-out duration-300">
                <OpenVideo get_file_path/>
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
