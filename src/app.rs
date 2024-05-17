use leptos::*;
// use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

mod menu;
mod pitch;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (is_playing, set_is_playing) = create_signal(false);
    let (video_src, set_video_src) = create_signal::<Option<String>>(None);
    let (coordinate, set_coordinate) = create_signal(pitch::Point::default());
    let (time, set_time) = create_signal::<f64>(0.0);
    let (show_menu, set_show_menu) = create_signal(false);

    let video_player = create_node_ref::<html::Video>();

    window_event_listener(ev::keydown, move |ev| {
        let video_player = video_player.get().unwrap();
        match ev.key().as_str() {
            " " => {
                set_is_playing.update(|val| *val = !*val);
                if !is_playing.get() {
                    let _ = video_player.pause();
                    let current_time = video_player.current_time();
                    set_time.set(current_time);
                } else {
                    let _ = video_player.play();
                }
            }
            "c" => {
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate + 0.25);
            }
            "C" => {
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate - 0.25);
            }
            num if ev.ctrl_key() && ("1"..="2").contains(&num) => {
                let rate = num.parse::<f64>().unwrap();
                video_player.set_playback_rate(rate);
            }
            n => logging::log!("key: {n}"),
        }
    });

    let toggle_menu = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_show_menu.set(true);
    };

    let menu_bar = create_node_ref::<html::Div>();

    window_event_listener(ev::click, move |ev| {
        if let Some(menu_bar) = menu_bar.get() {
            if menu_bar.is_mounted()
                && !ev
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::Node>()
                    .ok()
                    .as_ref()
                    .unwrap()
                    .contains(Some(&menu_bar))
            {
                set_show_menu.set(false);
            }
        }
    });

    // let elapsed_time = move || {
    //     let t = std::time::Duration::from_secs_f64(time.get());
    //     format!("{:?}", t)
    // };

    provide_context(show_menu);
    provide_context(set_video_src);

    view! {
        <main class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row">
            <div>
                <Show when=move || show_menu.get() == false
                    fallback=|| view! {
                        <button
                            class="rotate-90 border-none bg-slate-600 hover:bg-slate-400 px-2 size-[30px] rounded-t-lg text-white">
                            "X"
                        </button>
                    }>
                    <button
                        on:click=toggle_menu
                        class="rotate-90 border-none bg-slate-600 hover:bg-slate-400 px-2 size-[30px] rounded-t-lg text-white">
                        "|||"
                    </button>
                </Show>
            </div>
            <menu::MenuBar menu_bar/>
            <div id="event-tagger" class="flex flex-col px-[10px]">
                <div class="relative flex flex-row w-fit h-fit">
                    <pitch::Pitch set_coordinate/>
                    <video
                        _ref=video_player
                        src=move || video_src.get().unwrap_or_default()
                        controls
                        width="1020"
                        class="pointer-events-none"/>
                </div>
                <div>"button picker"</div>
            </div>
            <div id="data-view" class="flex flex-col shrink-0">
                <p>"NON CONSOLE APP DEBUGGER"</p>
                <p>"x1: "{ move || coordinate.get().x1 }", y1: "{ move || coordinate.get().y1 }</p>
                <p>"x2: "{ move || coordinate.get().x2 }", y2: "{ move || coordinate.get().y2 }</p>
                <p>"video time: "{ move || (time.get() / 60.0).floor() }":"{ move || (time.get() % 60.0).floor() }</p>
                <p>"elapsed time: "{ move || format!("{:.3}", time.get()) }</p>
            </div>
        </main>
    }

    // view! {
    //     <main class="absolute size-full">
    //         <div class="block absolute m-auto left-0 right-0 top-0 bottom-0 size-full flex flex-col">
    //             <div class="flex flex-row bg-transparent px-[30px] h-[40px] items-center space-x-[10px]">
    //                 <filepicker::FilePicker get_file_path/>
    //                 <button class="border-none rounded-full bg-indigo-800 px-4 text-white hover:bg-indigo-600 h-[30px] w-[130px] text-xs">"Load File"</button>
    //                 <button class="border-none rounded-full bg-indigo-800 px-4 text-white hover:bg-indigo-600 h-[30px] w-[130px] text-xs">"Settings"</button>
    //             </div>
    //             <div class="relative flex flex-row">
    //                 <pitch::Pitch set_coordinate/>
    //                 <video
    //                     _ref=video_player
    //                     src=move || video_src.get().unwrap_or_default()
    //                     controls
    //                     class="absolute h-[600px] w-[937px] top-[10px] left-[30px]"/>
    //                 <p class="absolute top-[620px] left-[30px]">"x1: "{ move || coordinate.get().x1 }", y1: "{ move || coordinate.get().y1 }</p>
    //                 <p class="absolute top-[640px] left-[30px]">"x2: "{ move || coordinate.get().x2 }", y2: "{ move || coordinate.get().y2 }</p>
    //                 <p class="absolute top-[660px] left-[30px]">"current time: "{ move || time.get() }</p>
    //             </div>
    //         </div>
    //     </main>
    // }
}
