use super::{invoke, menu, pitch, table_data, team_sheet, video, CtxProvider};

use leptos::*;
use wasm_bindgen::prelude::*;

use types::{
    AppError, MatchData, MatchInfo, Payload, PlayerInfo, Point, TaggedEvent, TeamInfoQuery,
};

async fn get_all_players_from_match_id(match_id: String) -> Result<Vec<PlayerInfo>, AppError> {
    let match_id = Payload { payload: match_id };
    let match_id = serde_wasm_bindgen::to_value(&match_id).unwrap();
    let team_info = invoke("get_all_players_from_match_id", match_id).await;
    let team_info =
        serde_wasm_bindgen::from_value::<Vec<PlayerInfo>>(team_info).unwrap_or_default();

    Ok(team_info)
}

async fn get_all_match_info() -> Result<Vec<MatchInfo>, AppError> {
    let match_info = invoke("get_all_match_info", JsValue::null()).await;
    let match_info =
        serde_wasm_bindgen::from_value::<Vec<MatchInfo>>(match_info).unwrap_or_default();

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
    let (match_data, set_match_data) = create_signal(MatchData::default());

    let video_player_node_ref = create_node_ref::<html::Video>();

    let register_event_action = expect_context::<CtxProvider>().register_event_action;

    let register_match_info_action = expect_context::<CtxProvider>().register_match_info_action;

    let team_info_resource = create_resource(
        move || match_data.get().match_id,
        get_all_players_from_match_id,
    );

    let match_info_resource = create_resource(
        move || register_match_info_action.version().get(),
        move |_| get_all_match_info(),
    );

    let team_info_action =
        create_action(|payload: &JsValue| invoke("get_team_info_by_query", payload.clone()));

    let team_end_action =
        create_action(|payload: &JsValue| invoke("get_team_info_by_query", payload.clone()));

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
            "ArrowRight" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time + 0.2);
            }
            "ArrowLeft" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time - 0.2);
            }
            num if ev.ctrl_key() && ("1"..="2").contains(&num) => {
                let rate = num.parse::<f64>().unwrap();
                video_player.set_playback_rate(rate);
            }
            // --- open video
            open if ev.ctrl_key() && open == "o" => {
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
                set_tagged_event.update(|tag| {
                    tag.time_start = video_player.current_time();
                    tag.x_start = coordinate.get_untracked().x;
                    tag.y_start = coordinate.get_untracked().y;
                });
                set_coordinate.set(Point::default());
            }
            "E" => {
                if !video_player.paused() {
                    let _ = video_player.pause();
                }
                set_tagged_event.update(|tag| {
                    tag.time_end = Some(video_player.current_time());
                    tag.x_end = coordinate.get_untracked().x;
                    tag.y_end = coordinate.get_untracked().y;
                });
                set_coordinate.set(Point::default());
            }
            // --- event outcome
            "Enter" => {
                // --- dump the data to the table
                set_tagged_event.update(|tag| {
                    tag.player_name = player_buffer.get_untracked();
                    tag.team_name = team_buffer.get_untracked();
                    tag.match_id = match_data.get_untracked().match_id;
                    tag.match_teams = format!(
                        "{} vs {}",
                        match_data.get_untracked().team_home,
                        match_data.get_untracked().team_away
                    );
                    tag.opponent_team = if tag.team_name == match_data.get_untracked().team_home {
                        match_data.get_untracked().team_away
                    } else {
                        match_data.get_untracked().team_home
                    };
                });
                let tagged_event = tagged_event.get_untracked();
                let payload = Payload {
                    payload: tagged_event.clone(),
                };
                let payload = serde_wasm_bindgen::to_value(&payload).unwrap();

                register_event_action.dispatch(payload);

                set_tagged_event.set(TaggedEvent::default());
                set_event_buffer.set(String::new());
                set_team_buffer.set(String::new());
                set_player_buffer.set(String::new());
            }
            keys if ("0"..="9").contains(&keys) || ("a"..="z").contains(&keys) || keys == "/" => {
                set_event_buffer.update(|e| e.push_str(keys));

                let args = event_buffer.get();

                let number = args
                    .split('/')
                    .take(1)
                    .collect::<String>()
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or_default();

                let team_state = match args
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
                    _ => "".to_string(),
                };

                let payload = Payload {
                    payload: TeamInfoQuery {
                        match_id: match_data.get_untracked().match_id,
                        team_state: team_state.clone(),
                    },
                };
                let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
                team_info_action.dispatch(payload);
                let team_info = team_info_action.value().get().unwrap_or_default();
                let team_info = serde_wasm_bindgen::from_value::<Vec<PlayerInfo>>(team_info)
                    .unwrap_or_default();

                let player_info = team_info
                    .iter()
                    .filter(|p| p.number == number)
                    .collect::<Vec<_>>();

                set_player_buffer.update(|p| {
                    let player_name = if let Some(player) = player_info.first() {
                        player.player_name.clone()
                    } else {
                        number.to_string()
                    };

                    *p = player_name;
                });
                set_team_buffer.update(|t| {
                    let team_name = if let Some(player) = player_info.first() {
                        player.team_name.clone()
                    } else {
                        team_state
                    };
                    *t = team_name;
                });

                let event_args = args
                    .split('/')
                    .skip(1)
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>();

                let (team_end, player_end) = if let Some(evt_arg) = event_args.get(1) {
                    let num = evt_arg
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap_or_default();
                    let t_state = match evt_arg
                        .chars()
                        .filter(|c| !c.is_numeric())
                        .collect::<String>()
                        .as_str()
                    {
                        "h" => "Home".to_string(),
                        "a" => "Away".to_string(),
                        _ => "".to_string(),
                    };
                    let payload = Payload {
                        payload: TeamInfoQuery {
                            match_id: match_data.get_untracked().match_id,
                            team_state: t_state.clone(),
                        },
                    };
                    let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
                    team_end_action.dispatch(payload);
                    let team_outcome = team_end_action.value().get().unwrap_or_default();
                    let team_outcome =
                        serde_wasm_bindgen::from_value::<Vec<PlayerInfo>>(team_outcome)
                            .unwrap_or_default();

                    let player_outcome = team_outcome
                        .iter()
                        .filter(|p| p.number == num)
                        .collect::<Vec<_>>();

                    let player_end = if let Some(p_end) = player_outcome.first() {
                        Some(p_end.player_name.clone())
                    } else {
                        None
                    };
                    let team_end = if let Some(p_end) = player_outcome.first() {
                        p_end.team_name.clone()
                    } else {
                        t_state
                    };
                    (Some(team_end), player_end)
                } else {
                    (None, None)
                };

                let event_args = event_args.first().unwrap_or(&Default::default()).clone();

                set_tagged_event.update(|t| {
                    t.assign_event_from_args(event_args.as_str(), team_end, player_end);
                });
            }
            _unregistered_key => {
                logging::log!("unregistered key: {}", _unregistered_key);
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
                    <Show when=move || !show_menu.get()
                        fallback=move || view! {
                            <button
                                on:click=toggle_menu
                                class="border-none bg-slate-300 hover:bg-slate-500 px-2 size-[30px] rounded-r-lg text-white">
                                <img src="public/close.svg"/>
                            </button>
                        }>
                        <button
                            on:click=toggle_menu
                            class="border-none bg-slate-300 hover:bg-slate-500 px-2 size-[30px] rounded-r-lg">
                            <img src="public/menu.svg" width="30" height="30"/>
                        </button>
                    </Show>
                </div>
                <menu::MenuBar menu_bar_node_ref match_data/>
                <div id="video_container" class="flex flex-col w-fit px-[10px] shrink-0">
                    <div class="relative flex flex-row w-fit h-fit">
                        <pitch::Pitch set_coordinate/>
                        <video::VideoPlayer video_src video_player_node_ref/>
                    </div>
                </div>
                <div id="info" class="flex flex-col mr-[40px] items-center w-full bg-slate-800/[.65] p-4 rounded-lg">
                    <team_sheet::SelectTeamSheet match_info_resource set_match_data/>
                    <div class="flex flex-row">
                        <team_sheet::TeamSheet team_info_resource team_state="Home".to_string()/>
                        <team_sheet::TeamSheet team_info_resource team_state="Away".to_string()/>
                    </div>
                </div>
            </div>
            <div class="flex flex-col px-[40px]">
                <div class="w-full bg-green-50 rounded-lg px-2 py-1 mt-1">
                    <table class="table-fixed w-full">
                        <tbody>
                            <tr>
                                <td class="text-xs w-[200px]">
                                    "time & loc buffer (S / E): "
                                    { move || coordinate.get().x }
                                    " "
                                    { move || coordinate.get().y }
                                </td>
                                <td class="text-xs flex flex-row">
                                    <p>"S: "{ move || format!("{:.3}", tagged_event.get().time_start) }"--"</p>
                                    <p>"x: "{ move || tagged_event.get().x_start }", y: "{ move || tagged_event.get().y_start }"-->"</p>
                                    <p>"E: "{ move || format!("{:.3}", tagged_event.get().time_end.unwrap_or_default()) }"--"</p>
                                    <p>"x: "{ move || tagged_event.get().x_end }", y: "{ move || tagged_event.get().y_end }</p>
                                </td>
                            </tr>
                            <tr>
                                <td class="text-xs w-[200px]">"event args buffer: "{ move || event_buffer.get() }</td>
                                <td class="text-xs flex flex-row">
                                    <p class="text-xs">{ move || team_buffer.get() }"_"</p>
                                    <p class="text-xs">{ move || player_buffer.get() }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().event_name }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().event_type }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().event_source }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().play_position }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().outcome }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().team_end }"_"</p>
                                    <p class="text-xs">{ move || tagged_event.get().player_end }</p>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <table_data::TableData video_player_node_ref register_event_action match_data/>
            </div>
        </div>
    }
}
