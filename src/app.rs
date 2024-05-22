use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

mod menu;
mod pitch;

use types::{Event, Payload, Point, TaggedEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (video_src, set_video_src) = create_signal::<Option<String>>(None);
    let (coordinate, set_coordinate) = create_signal(Point::new());
    let (show_menu, set_show_menu) = create_signal(false);
    let (tagged_event, set_tagged_event) = create_signal(TaggedEvent::new());
    let (event_buffer, set_event_buffer) = create_signal(String::new());

    let video_player = create_node_ref::<html::Video>();

    window_event_listener(ev::keydown, move |ev| {
        let video_player = video_player.get().unwrap();
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
            "_" => {
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
                    let event = match event_args.as_str() {
                        // --- kick off
                        "kos" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Kick Off".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "koo" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Kick Off".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        "koi" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Kick Off".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        // --- pass
                        "ps" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Open Play".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "pi" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Open Play".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "pb" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Open Play".to_string(),
                            outcome: "Blocked".to_string(),
                        },
                        "po" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Open Play".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        // --- goal kick
                        "gks" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Goalkick".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "gki" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Goalkick".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "gko" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Goalkick".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        // --- shot
                        "son" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Open Play".to_string(),
                            outcome: "On target".to_string(),
                        },
                        "sof" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Open Play".to_string(),
                            outcome: "Off target".to_string(),
                        },
                        "sb" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Open Play".to_string(),
                            outcome: "Blocked".to_string(),
                        },
                        "sg" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Open Play".to_string(),
                            outcome: "Goal".to_string(),
                        },
                        // --- penalty
                        "pkon" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Penalty".to_string(),
                            outcome: "On target".to_string(),
                        },
                        "pkof" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Penalty".to_string(),
                            outcome: "Off target".to_string(),
                        },
                        "pkg" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Penalty".to_string(),
                            outcome: "Goal".to_string(),
                        },
                        // --- carry
                        "dp" => Event::Dribble {
                            name: "Dribble".to_string(),
                            outcome: "Pass".to_string(),
                        },
                        "ds" => Event::Dribble {
                            name: "Dribble".to_string(),
                            outcome: "Shot".to_string(),
                        },
                        "dt" => Event::Dribble {
                            name: "Dribble".to_string(),
                            outcome: "Tackled".to_string(),
                        },
                        "dl" => Event::Dribble {
                            name: "Dribble".to_string(),
                            outcome: "Lost ball".to_string(),
                        },
                        // --- crossing
                        "crs" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Cross".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "cri" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Cross".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "crc" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Cross".to_string(),
                            outcome: "Claimed".to_string(),
                        },
                        "crb" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Cross".to_string(),
                            outcome: "Blocked".to_string(),
                        },
                        // --- throw
                        "tis" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Throw In".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "tii" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Throw In".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "tio" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Throw In".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        // --- Direct cornerkick
                        "ckds" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Direct Cornerkick".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "ckdi" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Direct Cornerkick".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "ckdc" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Direct Cornerkick".to_string(),
                            outcome: "Claimed".to_string(),
                        },
                        "ckdo" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Direct Cornerkick".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        "cksg" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Direct Cornerkick".to_string(),
                            outcome: "Goal".to_string(),
                        },
                        "ckson" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Direct Cornerkick".to_string(),
                            outcome: "On Target".to_string(),
                        },
                        // --- Short cornerkick
                        "ckps" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Short Cornerkick".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "ckpi" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Short Cornerkick".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "ckpo" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Short Cornerkick".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        // --- Direct freekick
                        "fkon" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Freekick".to_string(),
                            outcome: "On Target".to_string(),
                        },
                        "fkof" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Freekick".to_string(),
                            outcome: "Off Target".to_string(),
                        },
                        "fkb" => Event::Shot {
                            name: "Shot".to_string(),
                            shot_type: "Freekick".to_string(),
                            outcome: "Blocked".to_string(),
                        },
                        // --- Indirect freekick
                        "fkps" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Freekick".to_string(),
                            outcome: "Success".to_string(),
                        },
                        "fkpi" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Freekick".to_string(),
                            outcome: "Intercepted".to_string(),
                        },
                        "fkpo" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Freekick".to_string(),
                            outcome: "Out of Play".to_string(),
                        },
                        "fkpc" => Event::Pass {
                            name: "Pass".to_string(),
                            pass_type: "Freekick".to_string(),
                            outcome: "Claimed".to_string(),
                        },
                        // --- tackle
                        "tw" => Event::Tackle {
                            name: "Tackle".to_string(),
                            outcome: "Won".to_string(),
                        },
                        "tl" => Event::Tackle {
                            name: "Tackle".to_string(),
                            outcome: "Lost".to_string(),
                        },
                        // --- intercept
                        "iop" => Event::Intercept {
                            name: "Intercept".to_string(),
                            event_source: "Open Play".to_string(),
                        },
                        "ifk" => Event::Intercept {
                            name: "Intercept".to_string(),
                            event_source: "Freekick".to_string(),
                        },
                        "ick" => Event::Intercept {
                            name: "Intercept".to_string(),
                            event_source: "Cornerkick".to_string(),
                        },
                        "iti" => Event::Intercept {
                            name: "Intercept".to_string(),
                            event_source: "Throw In".to_string(),
                        },
                        // --- clearance
                        "clrop" => Event::Clearance {
                            name: "Clearance".to_string(),
                            event_source: "Open Play".to_string(),
                        },
                        "clrfk" => Event::Clearance {
                            name: "Clearance".to_string(),
                            event_source: "Freekick".to_string(),
                        },
                        "clrck" => Event::Clearance {
                            name: "Clearance".to_string(),
                            event_source: "Cornerkick".to_string(),
                        },
                        "clrti" => Event::Clearance {
                            name: "Clearance".to_string(),
                            event_source: "Throw In".to_string(),
                        },
                        // --- block
                        "bs" => Event::Block {
                            name: "Block".to_string(),
                            event_source: "Shot".to_string(),
                        },
                        "bp" => Event::Block {
                            name: "Block".to_string(),
                            event_source: "Pass".to_string(),
                        },
                        "bcr" => Event::Block {
                            name: "Block".to_string(),
                            event_source: "Crossing".to_string(),
                        },
                        // --- pressure
                        "prop" => Event::Pressure {
                            name: "Pressure".to_string(),
                            outcome: "Opponent Pass".to_string(),
                        },
                        "prod" => Event::Pressure {
                            name: "Pressure".to_string(),
                            outcome: "Opponent Dribble".to_string(),
                        },
                        "prtw" => Event::Pressure {
                            name: "Pressure".to_string(),
                            outcome: "Tackle Won".to_string(),
                        },
                        "prtl" => Event::Pressure {
                            name: "Pressure".to_string(),
                            outcome: "Tackle Lost".to_string(),
                        },
                        "probl" => Event::Pressure {
                            name: "Pressure".to_string(),
                            outcome: "Opponent Ball Lost".to_string(),
                        },
                        // --- goalkeeper save
                        "svs" => Event::Save {
                            name: "Save".to_string(),
                            event_source: "Shot".to_string(),
                        },
                        "svp" => Event::Save {
                            name: "Save".to_string(),
                            event_source: "Penalty".to_string(),
                        },
                        "svfk" => Event::Save {
                            name: "Save".to_string(),
                            event_source: "Freekick".to_string(),
                        },
                        "svck" => Event::Save {
                            name: "Save".to_string(),
                            event_source: "Cornerkick".to_string(),
                        },
                        "svti" => Event::Save {
                            name: "Save".to_string(),
                            event_source: "Throw In".to_string(),
                        },
                        // --- goalkeeper claim
                        "ctcr" => Event::Catch {
                            name: "Catch".to_string(),
                            event_source: "Crossing".to_string(),
                        },
                        "ctp" => Event::Catch {
                            name: "Catch".to_string(),
                            event_source: "Pass".to_string(),
                        },
                        "ctfk" => Event::Catch {
                            name: "Catch".to_string(),
                            event_source: "Freekick".to_string(),
                        },
                        "ctck" => Event::Catch {
                            name: "Catch".to_string(),
                            event_source: "Cornerkick".to_string(),
                        },
                        "ctti" => Event::Catch {
                            name: "Catch".to_string(),
                            event_source: "Throw In".to_string(),
                        },
                        // --- other events
                        "f" => Event::Other {
                            name: "Foul".to_string(),
                        },
                        "yc" => Event::Other {
                            name: "Yellow Card".to_string(),
                        },
                        "rc" => Event::Other {
                            name: "Red Card".to_string(),
                        },
                        "eofh" => Event::Other {
                            name: "End of First Half".to_string(),
                        },
                        "eosh" => Event::Other {
                            name: "End of Second Half".to_string(),
                        },
                        "eom" => Event::Other {
                            name: "End of Match".to_string(),
                        },
                        _ => Event::Other {
                            name: "Unregistered".to_string(),
                        },
                    };

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
                    <video
                        src=move || video_src.get().unwrap_or_default()
                        _ref=video_player
                        controls
                        width="1020"
                        class="pointer-events-none"
                    />
                    //     <source src=move || video_src.get().unwrap_or_default() type="video/mp4"/>
                    // </video>
                </div>
                <div>"TODO: team sheet"</div>
                <div>"TODO: shortcut info"</div>
            </div>
            <div id="data-view" class="flex flex-col shrink-0">
                <p class="text-xs">"TAGGED EVENT"</p>
                // <p class="text-xs">"active buffer(press \"p\" / \"e\" to toggle): "{ move || format!("{:?}", buffer.get()) }</p>
                <p class="text-xs">"coordinate buffer (press \"S\" / \"E\" to register): "{ move || coordinate.get().x }" "{ move || coordinate.get().y }</p>
                <p class="text-xs">"event buffer (press \"R\" to register): "{ move || event_buffer.get() }</p>
                <p class="text-xs">"time start (press \"S\" to register): "{ move || format!("{:.3}", tagged_event.get().time_start) }</p>
                <p class="text-xs">"player name: "{ move || tagged_event.get().player_name }</p>
                <p class="text-xs">"team name: "{ move || tagged_event.get().team_name }</p>
                <p class="text-xs">"x1: "{ move || tagged_event.get().loc_start.x }", y1: "{ move || tagged_event.get().loc_start.y }</p>
                <p class="text-xs">"event name: "{move || tagged_event.get().event.to_string() }</p>
                <p class="text-xs">"time end (press \"E\" to register): "{ move || format!("{:.3}", tagged_event.get().time_end) }</p>
                <p class="text-xs">"x2: "{ move || tagged_event.get().loc_end.x }", y2: "{ move || tagged_event.get().loc_end.y }</p>
                // <p>"video time: "{ move || (time.get() / 60.0).floor() }":"{ move || (time.get() % 60.0).floor() }</p>
            </div>
        </main>
    }
}
