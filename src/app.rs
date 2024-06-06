use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

mod menu;
mod pitch;
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
    let (coordinate, set_coordinate) = create_signal(Point::new());
    let (show_menu, set_show_menu) = create_signal(false);
    let (tagged_event, set_tagged_event) = create_signal(TaggedEvent::new());
    let (event_buffer, set_event_buffer) = create_signal(String::new());

    let video_player_node_ref = create_node_ref::<html::Video>();

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
                        let _ = e.pop();
                    }
                });
            }
            "R" => {
                if !event_buffer.get().is_empty() {
                    let args = event_buffer.get();
                    let number = args
                        .chars()
                        .into_iter()
                        .filter(|c| c.to_string().parse::<i32>().is_ok())
                        .collect::<String>();
                    // .parse::<i32>()
                    // .unwrap_or_default();
                    let team = match args.chars().next().unwrap() {
                        'a' => "Away".to_string(),
                        'h' => "Home".to_string(),
                        _ => "".to_string(),
                    };
                    let event_args = args
                        .chars()
                        .into_iter()
                        .filter(|c| c.to_string().parse::<i32>().is_err())
                        .skip(1)
                        .collect::<String>();
                    let event = Event::from_event_args(event_args.as_str());

                    set_tagged_event.update(|evt| {
                        evt.player_name = number;
                        evt.team_name = team;
                        evt.event = event;
                    });

                    set_event_buffer.set(String::new());
                }
            }
            // --- start & end time of the event
            "S" => {
                if !video_player.paused() {
                    let _ = video_player.pause();
                }
                set_tagged_event.update(|evt| evt.time_start = video_player.current_time());
                set_tagged_event.update(|evt| evt.loc_start = coordinate.get());
                set_coordinate.set(Point::new());
            }
            "E" => {
                if !video_player.paused() {
                    let _ = video_player.pause();
                }
                set_tagged_event.update(|evt| evt.time_end = video_player.current_time());
                set_tagged_event.update(|evt| evt.loc_end = coordinate.get());
                set_coordinate.set(Point::new());
            }
            // --- event outcome
            "Enter" => {
                logging::log!("tagged event: {:?}", tagged_event.get());
                // --- dump the data to the table
                spawn_local(async move {
                    let tagged_event = tagged_event.get_untracked();
                    let payload = Payload {
                        payload: tagged_event.clone(),
                    };
                    let args = to_value(&payload).unwrap();
                    let _ = invoke("register", args).await;

                    set_tagged_event.set(TaggedEvent::new());
                });
            }
            o if ("0"..="9").contains(&o) || ("a"..="z").contains(&o) => {
                set_event_buffer.update(|e| e.push_str(o));
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
                    <video::VideoPlayer video_src video_player_node_ref/>
                </div>
                <div>
                    <table>
                        <tbody>
                            <tr>
                                <td class="text-xs w-[130px]">"location buffer (S / E)"</td>
                                <td class="text-xs">{ move || coordinate.get().x }" "{ move || coordinate.get().y }</td>
                            </tr>
                            <tr>
                                <td class="text-xs w-[130px]">"event args buffer (R)"</td>
                                <td class="text-xs">{ move || event_buffer.get() }</td>
                            </tr>
                        </tbody>
                    </table>
                    <table>
                        <thead>
                            <tr>
                                <th scope="col" class="text-xs text-left w-[120px]">"time start (S)"</th>
                                <th scope="col" class="text-xs text-left w-[120px]">"location start (S)"</th>
                                <th scope="col" class="text-xs text-left w-[120px]">"time end (E)"</th>
                                <th scope="col" class="text-xs text-left w-[120px]">"location end (E)"</th>
                                <th scope="col" class="text-xs text-left w-[120px]">"team name (R)"</th>
                                <th scope="col" class="text-xs text-left w-[120px]">"player name (R)"</th>
                                <th scope="col" class="text-xs text-left w-[200px]">"event (R)"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="text-xs">{ move || format!("{:.3}", tagged_event.get().time_start) }</td>
                                <td class="text-xs">"x1: "{ move || tagged_event.get().loc_start.x }", y1: "{ move || tagged_event.get().loc_start.y }</td>
                                <td class="text-xs">{ move || format!("{:.3}", tagged_event.get().time_end) }</td>
                                <td class="text-xs">"x2: "{ move || tagged_event.get().loc_end.x }", y2: "{ move || tagged_event.get().loc_end.y }</td>
                                <td class="text-xs">{ move || tagged_event.get().team_name }</td>
                                <td class="text-xs">{ move || tagged_event.get().player_name }</td>
                                <td class="text-xs">{move || tagged_event.get().event.to_string() }</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div>"TODO: team sheet"</div>
                <div>"TODO: shortcut info"</div>
            </div>
            <div id="data-view" class="flex flex-col shrink-0">
                <p class="text-xs">"TAGGED EVENT => TODO: tables"</p>
            </div>
        </main>
    }
}
