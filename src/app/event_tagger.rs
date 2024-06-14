use super::{invoke, menu, pitch, table_data, team_sheet, video};

use leptos::*;
use wasm_bindgen::prelude::*;

use types::{AppError, Event, MatchInfo, Payload, Point, TaggedEvent};

async fn get_match_info(match_id: JsValue) -> Result<MatchInfo, AppError> {
    logging::log!("match id: {:?}", match_id);
    let match_id = serde_wasm_bindgen::from_value::<String>(match_id).unwrap_or_default();
    let match_id = Payload { payload: match_id };
    let match_id = serde_wasm_bindgen::to_value(&match_id).unwrap();
    let match_info = invoke("get_match_info", match_id).await;
    let match_info = serde_wasm_bindgen::from_value::<MatchInfo>(match_info).unwrap_or_default();

    Ok(match_info)
}

#[component]
pub fn EventTagger() -> impl IntoView {
    let (video_src, set_video_src) = create_signal::<Option<String>>(None);
    let (coordinate, set_coordinate) = create_signal(Point::default());
    let (show_menu, set_show_menu) = create_signal(false);
    let (tagged_event, set_tagged_event) = create_signal(TaggedEvent::default());
    let (event_buffer, set_event_buffer) = create_signal(String::new());
    let (team_buffer, set_team_buffer) = create_signal(String::new());
    let (player_buffer, set_player_buffer) = create_signal(String::new());
    let (action_buffer, set_action_buffer) = create_signal(Event::default());

    let video_player_node_ref = create_node_ref::<html::Video>();

    let register_event_action =
        create_action(|payload: &JsValue| invoke("insert_data", payload.clone()));
    let register_match_info_action = expect_context::<Action<JsValue, JsValue>>();

    let match_info_resource = create_resource(
        move || register_match_info_action.value().get().unwrap_or_default(),
        get_match_info,
    );

    let shortcuts = window_event_listener(ev::keydown, move |ev| {
        ev.prevent_default();
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
            // --- open video
            o if ev.ctrl_key() && o == "o" => {
                spawn_local(async move {
                    let path_protocol = invoke("open", wasm_bindgen::JsValue::null()).await;
                    let (path, protocol): (String, String) =
                        serde_wasm_bindgen::from_value(path_protocol).unwrap_throw();
                    let resolved_path = format!("{}://localhost/{}", protocol, path);

                    logging::log!("resolved path: {:?}", resolved_path);

                    set_video_src.set(Some(resolved_path));
                });
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

                register_event_action.dispatch(payload);

                set_tagged_event.set(TaggedEvent::default());
                set_event_buffer.set(String::new());
            }
            b if ("0"..="9").contains(&b) || ("a"..="z").contains(&b) || b == "/" => {
                // --- these stuffs aren't that expensive, right? right? right?
                set_event_buffer.update(|e| e.push_str(b));

                let args = event_buffer.get();

                let number = args
                    .split('/')
                    .take(1)
                    .collect::<String>()
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>();
                // .parse::<i32>()
                // .unwrap_or_default();

                // --- later use number to query the player name
                set_player_buffer.update(|p| *p = number);

                let team = match args
                    .split('/')
                    .take(1)
                    .collect::<String>()
                    .chars()
                    .filter(|c| !c.is_numeric())
                    .collect::<String>()
                    .as_str()
                {
                    "a" => "Away".to_string(),
                    "h" => "Home".to_string(),
                    _ => "Unregistered".to_string(),
                };
                set_team_buffer.update(|t| *t = team);

                let event_args = args.split('/').skip(1).collect::<String>();

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
        set_show_menu.update(|v| *v = !*v);
    };

    let menu_bar_node_ref = create_node_ref::<html::Div>();

    provide_context(show_menu);
    provide_context(set_video_src);

    on_cleanup(move || {
        shortcuts.remove();
    });

    view! {
        <div class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-col">
            <div class="flex flex-row">
                <div>
                    <Show when=move || show_menu.get() == false
                        fallback=move || view! {
                            <button
                                on:click=toggle_menu
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
                <menu::MenuBar menu_bar_node_ref/>
                <div id="video_container" class="flex flex-col w-fit px-[10px] shrink-0">
                    <div class="relative flex flex-row w-fit h-fit">
                        <pitch::Pitch set_coordinate/>
                        <video::VideoPlayer video_src video_player_node_ref/>
                    </div>
                </div>
                <div id="info" class="flex flex-col shrink-0 items-center w-[350px] bg-slate-800/[.65] p-2 rounded-lg">
                    <div
                        on:click=move |_| {
                            let navigate = leptos_router::use_navigate();
                            navigate("/team_sheet", Default::default());
                        }
                        class="text-xs rounded-full bg-lime-400 w-full flex flex-row justify-center h-fit px-2 py-1 hover:cursor-pointer hover:bg-indigo-600 hover:text-white"
                    >
                        "Teamsheet"
                    </div>
                    <div class="flex flex-row">
                        <team_sheet::TeamSheet match_info_resource team_state="Home".to_string()/>
                        <team_sheet::TeamSheet match_info_resource team_state="Away".to_string()/>
                    </div>
                </div>
            </div>
            <div class="flex flex-col px-[40px]">
                <div class="w-full">
                    <table class="table-fixed w-full">
                        <tbody>
                            <tr>
                                <td class="text-xs w-[200px]">"time & loc buffer (S / E): "{ move || coordinate.get().x }" "{ move || coordinate.get().y }</td>
                                <td class="text-xs flex flex-row">
                                    <p>"S: "{ move || format!("{:.3}", tagged_event.get().time_start) }"--"</p>
                                    <p>{ move || tagged_event.get().loc_start.to_string() }"-->"</p>
                                    <p>"E: "{ move || format!("{:.3}", tagged_event.get().time_end) }"--"</p>
                                    <p>{ move || tagged_event.get().loc_end.to_string() }</p>
                                </td>
                            </tr>
                            <tr>
                                <td class="text-xs w-[200px]">"event args buffer: "{ move || event_buffer.get() }</td>
                                <td class="text-xs flex flex-row">
                                    <p class="text-xs">{ move || team_buffer.get() }</p>
                                    <p class="text-xs">{ move || player_buffer.get() }</p>
                                    <p class="text-xs">{ move || action_buffer.get().to_string() }</p>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <table_data::TableData video_player_node_ref register_event_action/>
            </div>
        </div>
    }
}
