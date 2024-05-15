use leptos::*;
// use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

mod filepicker;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn convertFileSrc(path: &str) -> JsValue;
}

#[derive(Debug, Clone, Default)]
struct Point {
    x1: Option<i32>,
    y1: Option<i32>,
    x2: Option<i32>,
    y2: Option<i32>,
}

impl Point {
    fn reset(&mut self) {
        self.x1 = None;
        self.y1 = None;
        self.x2 = None;
        self.y2 = None;
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (is_playing, set_is_playing) = create_signal(false);
    let (video_src, set_video_src) = create_signal::<Option<String>>(None);
    let (coordinate, set_coordinate) = create_signal(Point::default());
    let (time, set_time) = create_signal::<f64>(0.0);

    let video_player = create_node_ref::<html::Video>();

    window_event_listener(ev::keydown, move |ev| {
        logging::log!("event type: {}", ev.type_());
        match ev.key().as_str() {
            " " => {
                set_is_playing.update(|val| *val = !*val);
                if !is_playing.get() {
                    let _ = video_player.get().unwrap().pause();
                    let current_time = video_player.get().unwrap().current_time();
                    set_time.set(current_time);
                } else {
                    let _ = video_player.get().unwrap().play();
                }
            }
            "c" => {
                let video_player = video_player.get().unwrap();
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate + 0.25);
            }
            num if ev.ctrl_key() && ("1"..="2").contains(&num) => {
                let video_player = video_player.get().unwrap();
                let rate = num.parse::<f64>().unwrap();
                video_player.set_playback_rate(rate);
            }
            n => logging::log!("key: {n}"),
        }
    });

    // pls refer to this documentation: https://tauri.app/v1/api/js/tauri/#convertfilesrc
    let get_file_path = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let res = invoke("open").await.as_string().unwrap();
            let res = res.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap();
            let resolved_path = convertFileSrc(res).as_string().unwrap();
            logging::log!("resolved path: {:?}", resolved_path);
            set_video_src.set(Some(resolved_path));
        });
    };

    let handle_pitch_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_coordinate.update(|point| match ev.type_().as_str() {
            "mousedown" => {
                point.reset();
                point.x1 = Some(ev.offset_x());
                point.y1 = Some(ev.offset_y());
            }
            "mouseup" => {
                let x2 = Some(ev.offset_x());
                let y2 = Some(ev.offset_y());

                if x2 != point.x1 || y2 != point.y1 {
                    point.x2 = x2;
                    point.y2 = y2;
                }
            }
            _ => (),
        });
    };

    view! {
        <main class="absolute size-full">
            <div class="block absolute m-auto left-0 right-0 top-0 bottom-0 size-full flex flex-col">
                <div class="flex flex-row bg-transparent px-[30px] h-[40px] items-center space-x-[10px]">
                    <filepicker::FilePicker get_file_path/>
                    <button class="border-none rounded-full bg-indigo-800 px-4 text-white hover:bg-indigo-600 h-[30px] w-[130px] text-xs">"Load File"</button>
                    <button class="border-none rounded-full bg-indigo-800 px-4 text-white hover:bg-indigo-600 h-[30px] w-[130px] text-xs">"Settings"</button>
                </div>
                <div class="relative flex flex-row">
                    <img
                        on:mousedown=handle_pitch_click
                        on:mouseup=handle_pitch_click
                        src="public/pitch.svg"
                        draggable="false"
                        class="absolute z-10 h-[500px] top-[60px] left-[125px]"/>
                    <video
                        _ref=video_player
                        src=move || video_src.get().unwrap_or_default()
                        controls
                        class="absolute h-[600px] w-[937px] top-[10px] left-[30px]"/>
                    <p class="absolute top-[620px] left-[30px]">"x1: "{ move || coordinate.get().x1 }", y1: "{ move || coordinate.get().y1 }</p>
                    <p class="absolute top-[640px] left-[30px]">"x2: "{ move || coordinate.get().x2 }", y2: "{ move || coordinate.get().y2 }</p>
                    <p class="absolute top-[660px] left-[30px]">"current time: "{ move || time.get() }</p>
                </div>
            </div>
        </main>
    }
}
