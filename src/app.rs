use leptos::*;
use wasm_bindgen::prelude::*;

mod menu;
mod pitch;
mod table_data;
mod video;

use types::{Event, Payload, Point, TaggedEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn convertFileSrc(path: String, protocol: String) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (video_src, set_video_src) = create_signal::<Option<String>>(None);
    let (coordinate, set_coordinate) = create_signal(Point::default());
    let (show_menu, set_show_menu) = create_signal(false);
    let (tagged_event, set_tagged_event) = create_signal(TaggedEvent::default());
    let (event_buffer, set_event_buffer) = create_signal(String::new());
    let (team_buffer, set_team_buffer) = create_signal(String::new());
    let (player_buffer, set_player_buffer) = create_signal(String::new());
    let (action_buffer, set_action_buffer) = create_signal(Event::default());
    // let (data, set_data) = create_signal::<Vec<TaggedEvent>>(Vec::new());

    let video_player_node_ref = create_node_ref::<html::Video>();

    let register_action = create_action(|payload: &JsValue| invoke("insert_data", payload.clone()));

    window_event_listener(ev::keydown, move |ev| {
        let video_player = video_player_node_ref.get().unwrap();
        match ev.key().as_str() {
            // --- playback control
            " " => {
                if !video_player.paused() {
                    let _ = video_player.pause();
                } else {
                    let _ = video_player.play();
                }
            }
            "+" => {
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate + 0.25);
            }
            "-" => {
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate - 0.25);
            }
            num if ev.ctrl_key() && ("1"..="2").contains(&num) => {
                let rate = num.parse::<f64>().unwrap();
                video_player.set_playback_rate(rate);
            }
            // --- buffer management
            "Backspace" => {
                set_event_buffer.update(|e| {
                    if !e.is_empty() {
                        e.pop();
                    }
                });
            }
            // --- start & end time of the event
            "S" => {
                if !video_player.paused() {
                    let _ = video_player.pause();
                }
                set_tagged_event.update(|tag| tag.time_start = video_player.current_time());
                set_tagged_event.update(|tag| tag.loc_start = coordinate.get());
                set_coordinate.set(Point::default());
            }
            "E" => {
                if !video_player.paused() {
                    let _ = video_player.pause();
                }
                set_tagged_event.update(|tag| tag.time_end = video_player.current_time());
                set_tagged_event.update(|tag| tag.loc_end = coordinate.get());
                set_coordinate.set(Point::default());
            }
            // --- event outcome
            "Enter" => {
                // --- dump the data to the table
                set_tagged_event.update(|tag| {
                    tag.player_name = player_buffer.get_untracked();
                    tag.team_name = team_buffer.get_untracked();
                    tag.event = action_buffer.get_untracked();
                });
                let tagged_event = tagged_event.get_untracked();
                let payload = Payload {
                    payload: tagged_event.clone(),
                };
                let payload = serde_wasm_bindgen::to_value(&payload).unwrap();

                register_action.dispatch(payload);

                set_tagged_event.set(TaggedEvent::default());
                set_event_buffer.set(String::new());
            }
            o if ("0"..="9").contains(&o) || ("a"..="z").contains(&o) => {
                set_event_buffer.update(|e| e.push_str(o));
                let args = event_buffer.get();
                let number = args
                    .chars()
                    .into_iter()
                    .filter(|c| c.to_string().parse::<i32>().is_ok())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or_default();
                set_player_buffer.update(|p| *p = number.to_string());
                let team = match args.chars().nth(0).unwrap() {
                    'a' => "Away".to_string(),
                    'h' => "Home".to_string(),
                    _ => "".to_string(),
                };
                set_team_buffer.update(|t| *t = team);
                let event_args = args
                    .chars()
                    .into_iter()
                    .filter(|c| c.to_string().parse::<i32>().is_err())
                    .skip(1)
                    .collect::<String>();
                set_action_buffer.update(|a| *a = Event::from_event_args(event_args.as_str()));
            }
            "ArrowRight" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time + 0.2);
            }
            "ArrowLeft" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time - 0.2);
            }
            rest => {
                logging::log!("unregistered key: {}", rest);
            }
        }
    });

    let toggle_menu = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_show_menu.set(true);
    };

    let menu_bar = create_node_ref::<html::Div>();

    window_event_listener(ev::click, move |ev| {
        if let Some(menu) = menu_bar.get() {
            if menu.is_mounted()
                && !ev
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::Node>()
                    .ok()
                    .as_ref()
                    .unwrap()
                    .contains(Some(&menu))
            {
                set_show_menu.set(false);
            }
        }
    });

    provide_context(show_menu);
    provide_context(set_video_src);

    view! {
        <main class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-col">
            <div class="flex flex-row">
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
                        <video::VideoPlayer video_src video_player_node_ref/>
                    </div>
                </div>
                <div id="info" class="flex flex-col shrink-0">
                    <div class="text-xs">"TODO: team sheet"</div>
                    <div class="text-xs">"TODO: shortcut info"</div>
                </div>
            </div>
            <div class="flex flex-col px-[40px]">
                <table class="max-w-[1000px]">
                    <tbody>
                        <tr>
                            <td class="text-xs w-[200px]">"time & location buffer (S / E) "{ move || coordinate.get().x }" "{ move || coordinate.get().y }</td>
                            <td class="text-xs flex flex-row">
                                <p>{ move || format!("{:.3}", tagged_event.get().time_start) }"-"</p>
                                <p>{ move || tagged_event.get().loc_start.to_string() }"-"</p>
                                <p>{ move || format!("{:.3}", tagged_event.get().time_end) }"-"</p>
                                <p>{ move || tagged_event.get().loc_end.to_string() }</p>
                            </td>
                        </tr>
                        <tr>
                            <td class="text-xs w-[200px]">"event args buffer "{ move || event_buffer.get() }</td>
                            <td class="text-xs flex flex-row">
                                <p class="text-xs">{ move || team_buffer.get() }</p>
                                <p class="text-xs">{ move || player_buffer.get() }</p>
                                <p class="text-xs">{ move || action_buffer.get().to_string() }</p>
                            </td>
                        </tr>
                    </tbody>
                </table>
                <table_data::TableData video_player_node_ref register_action/>
            </div>
        </main>
    }
}
