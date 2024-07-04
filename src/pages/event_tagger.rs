use crate::app::{convertFileSrc, get_all_match, get_players_by_match_id, invoke, CtxProvider};
use crate::components::{Pitch, SelectTeamSheet, TableData, TeamSheet, VideoPlayer};

use types::{Payload, PlayerInfo, Point, TaggedEvent, TeamInfoQuery};

use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn EventTagger() -> impl IntoView {
    let (coordinate, set_coordinate) = create_signal(Point::default());
    let (tagged_event, set_tagged_event) = create_signal(TaggedEvent::default());
    let (event_buffer, set_event_buffer) = create_signal(String::new());
    let (team_buffer, set_team_buffer) = create_signal(String::new());
    let (player_buffer, set_player_buffer) = create_signal(PlayerInfo::default());
    let (latest_start, set_latest_start) = create_signal(Point::default());
    let (latest_end, set_latest_end) = create_signal(Point::default());

    let match_info = expect_context::<CtxProvider>().match_info;
    let set_match_info = expect_context::<CtxProvider>().set_match_info;

    let video_player_node_ref = create_node_ref::<html::Video>();

    let register_event_action = expect_context::<CtxProvider>().register_event_action;
    let register_match_info_action = expect_context::<CtxProvider>().register_match_info_action;
    let open_video_action = expect_context::<CtxProvider>().open_video_action;

    let delete_match_info_action =
        create_action(|payload: &JsValue| invoke("delete_match_info_by_id", payload.clone()));
    let team_info_action =
        create_action(|payload: &JsValue| invoke("query_team_info", payload.clone()));
    let team_end_action =
        create_action(|payload: &JsValue| invoke("query_team_info", payload.clone()));
    let delete_all_row_action = create_action(|payload: &JsValue| {
        invoke("delete_all_records_by_match_id", payload.clone())
    });

    let team_info_resource = create_resource(
        move || {
            (
                match_info.get().match_id,
                delete_match_info_action.version().get(),
                register_event_action.version().get(),
            )
        },
        move |(match_id, _, _)| get_players_by_match_id(match_id),
    );

    let match_info_resource = create_resource(
        move || {
            (
                register_match_info_action.version().get(),
                delete_match_info_action.version().get(),
            )
        },
        move |_| get_all_match(),
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
            speed_up if ev.ctrl_key() && speed_up == "=" => {
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate + 0.25);
            }
            slow_down if ev.ctrl_key() && slow_down == "-" => {
                let current_playback_rate = video_player.playback_rate();
                video_player.set_playback_rate(current_playback_rate - 0.25);
            }
            num if ev.ctrl_key() && ("1"..="3").contains(&num) => {
                let rate = num.parse::<f64>().unwrap();
                video_player.set_playback_rate(rate);
            }
            "ArrowRight" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time + 1.);
            }
            "ArrowLeft" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time - 1.);
            }
            "ArrowUp" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time + 0.05);
            }
            "ArrowDown" => {
                let current_time = video_player.current_time();
                let _ = video_player.fast_seek(current_time - 0.1);
            }
            // --- open video
            open if ev.ctrl_key() && open == "o" => {
                open_video_action.dispatch(JsValue::null());
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
                    logging::log!("{:?}", player_buffer.get_untracked());
                    tag.player_name = player_buffer.get_untracked().player_name;
                    if tag.event_name != "Change Position" {
                        tag.play_position =
                            player_buffer.get_untracked().play_position.last().cloned();
                    }
                    tag.team_name = team_buffer.get_untracked();
                    tag.match_id = match_info.get_untracked().match_id;
                    tag.match_date = match_info.get_untracked().match_date;
                    tag.match_teams = format!(
                        "{} vs {}",
                        match_info.get_untracked().team_home,
                        match_info.get_untracked().team_away
                    );
                    tag.opponent_team = if tag.team_name == match_info.get_untracked().team_home {
                        match_info.get_untracked().team_away
                    } else {
                        match_info.get_untracked().team_home
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
                set_player_buffer.set(PlayerInfo::default());
            }
            // --- buffer management
            "Backspace" => {
                set_event_buffer.update(|e| {
                    if !e.is_empty() {
                        e.pop();
                    }
                });
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
                        match_id: match_info.get_untracked().match_id,
                        team_state: team_state.clone(),
                    },
                };
                let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
                team_info_action.dispatch(payload);
                let team_info = team_info_action.value().get().unwrap_or_default();
                let team_info = serde_wasm_bindgen::from_value::<Vec<PlayerInfo>>(team_info)
                    .unwrap_or_default();

                let player_info = team_info.iter().find(|p| p.number == number).cloned();

                set_player_buffer.update(|p| {
                    if let Some(player) = player_info.clone() {
                        *p = player;
                    } else {
                        p.player_name = number.to_string();
                    };
                });
                set_team_buffer.update(|t| {
                    let team_name = if let Some(player) = player_info.clone() {
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
                            match_id: match_info.get_untracked().match_id,
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

                    let player_end = player_outcome.first().map(|p| p.player_name.clone());
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
                    t.player_id = player_info.unwrap_or_default().player_id;
                    t.assign_event_from_args(event_args.as_str(), team_end, player_end);
                });
            }
            _unregistered_key => (),
        }
    });

    let video_src = create_memo(move |_| {
        open_video_action.value().get().map(|path| {
            let path = serde_wasm_bindgen::from_value::<String>(path).unwrap_throw();
            let resolved_path = convertFileSrc(path, "stream".to_string());

            serde_wasm_bindgen::from_value::<String>(resolved_path).unwrap_or_default()
        })
    });

    on_cleanup(move || {
        shortcuts.remove();
    });

    view! {
        <div class="block m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-col pr-[40px]">
            <div class="flex flex-row">
                <div id="video_container" class="flex flex-col w-fit px-[10px] shrink-0">
                    <div class="relative flex flex-row w-fit h-fit">
                        <Pitch
                            set_coordinate
                            latest_start
                            latest_end
                        />
                        <VideoPlayer video_src video_player_node_ref/>
                    </div>
                </div>
                <div id="info" class="flex flex-col items-center w-full grow-0 bg-slate-800/[.65] p-4 rounded-lg">
                    <SelectTeamSheet
                        match_info_resource
                        match_info
                        set_match_info
                        delete_match_info_action
                        delete_all_row_action
                    />
                    <div class="flex flex-row">
                        <TeamSheet team_info_resource team_state="Home".to_string()/>
                        <TeamSheet team_info_resource team_state="Away".to_string()/>
                    </div>
                </div>
            </div>
            <div class="flex flex-col">
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
                                    <p class="text-xs">{ move || player_buffer.get().player_name }"_"</p>
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
                <TableData
                    video_player_node_ref
                    register_event_action
                    delete_match_info_action
                    delete_all_row_action
                    match_info
                    set_latest_start
                    set_latest_end
                />
            </div>
        </div>
    }
}
