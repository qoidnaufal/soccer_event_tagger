use crate::app::{invoke, CtxProvider};
use types::{AppError, MatchInfo, Payload, PlayerInfo, TaggedEvent};

use leptos::*;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

const GUIDE_HOME: &str = "Check the guide -------->";
const GUIDE_AWAY: &str = "Use \")\" after the number, \"/position\" after starting 11 player's name, and \",\" to separate each players. Example: 1) Toldo /gk, 2) D. Alves /rfb, 3) Zlatan Ibrahimovic, ... Toldo & D. Alves will be registered as starting xi while Zlatan Ibrahimovic isn't";

async fn get_latest_registered() -> Result<MatchInfo, AppError> {
    let response = invoke("get_all_match_info", JsValue::null()).await;
    let response = serde_wasm_bindgen::from_value::<Vec<MatchInfo>>(response).unwrap_or_default();
    let response = response.last().cloned().unwrap_or_default();

    Ok(response)
}

#[component]
pub fn RegisterMatchInfo() -> impl IntoView {
    let (match_info, set_match_info) = create_signal(MatchInfo::default());
    let input_home_ref = create_node_ref::<html::Div>();
    let input_away_ref = create_node_ref::<html::Div>();
    let input_date_ref = create_node_ref::<html::Input>();
    let team_home_name_ref = create_node_ref::<html::Input>();
    let team_away_name_ref = create_node_ref::<html::Input>();
    let submit_button_ref = create_node_ref::<html::Button>();

    let register_match_info_action = expect_context::<CtxProvider>().register_match_info_action;
    let register_player_info_action = expect_context::<CtxProvider>().register_player_info_action;
    let register_event_action = expect_context::<CtxProvider>().register_event_action;

    let register = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let player_list_home = input_home_ref
            .get()
            .unwrap()
            .inner_text()
            .trim()
            .to_string();

        let player_list_away = input_away_ref
            .get()
            .unwrap()
            .inner_text()
            .trim()
            .to_string();

        let match_date = input_date_ref.get().unwrap().value();
        let team_home_name = team_home_name_ref.get().unwrap().value();
        let team_away_name = team_away_name_ref.get().unwrap().value();

        set_match_info.update(|m| {
            m.match_date = match_date;
            m.team_home = team_home_name;
            m.team_away = team_away_name;
            m.assign_id();
        });
        let payload = match_info.get_untracked();
        let payload = Payload { payload };
        let payload = serde_wasm_bindgen::to_value(&payload).unwrap_throw();
        register_match_info_action.dispatch(payload);

        player_list_home
            .split(',')
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                let mut token = s.split(')');
                let number = token
                    .next()
                    .unwrap()
                    .trim()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let p_info = token.next().unwrap().to_string();

                let mut player_info = PlayerInfo::default();

                if p_info.contains('/') {
                    let mut p_token = p_info.split('/').map(|p| p.trim().to_string());
                    let p_name = p_token.next().unwrap();
                    let p_position = p_token.next().unwrap().to_uppercase();

                    player_info.player_name = p_name;
                    player_info.play_position = vec![p_position];
                    player_info.start = true;
                    player_info.play = true;
                    player_info.number = number;
                    player_info.team_state = "Home".to_string();
                    player_info.team_name = match_info.get_untracked().team_home;
                    player_info.assign_id();
                    player_info
                        .match_id
                        .clone_from(&match_info.get_untracked().match_id);
                } else {
                    player_info.number = number;
                    player_info.player_name = p_info;
                    player_info.start = false;
                    player_info.play = false;
                    player_info.team_state = "Home".to_string();
                    player_info.team_name = match_info.get_untracked().team_home;
                    player_info.assign_id();
                    player_info
                        .match_id
                        .clone_from(&match_info.get_untracked().match_id);
                };

                let payload = Payload {
                    payload: player_info.clone(),
                };
                let payload = serde_wasm_bindgen::to_value(&payload).unwrap();
                register_player_info_action.dispatch(payload);

                if player_info.start {
                    let tagged_event = TaggedEvent {
                        match_id: player_info.match_id,
                        player_id: player_info.player_id,
                        match_date: match_info.get_untracked().match_date,
                        match_teams: format!(
                            "{} vs {}",
                            match_info.get_untracked().team_home,
                            match_info.get_untracked().team_away
                        ),
                        opponent_team: match_info.get_untracked().team_away,
                        time_start: 0.,
                        play_position: player_info.play_position.first().cloned(),
                        player_name: player_info.player_name,
                        team_name: match_info.get_untracked().team_home,
                        event_name: "Start".to_string(),
                        event_type: Some("Play".to_string()),
                        ..Default::default()
                    };

                    let payload = Payload {
                        payload: tagged_event,
                    };
                    let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
                    register_event_action.dispatch(payload);
                }
            });

        player_list_away
            .split(',')
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                let mut token = s.split(')');
                let number = token
                    .next()
                    .unwrap()
                    .trim()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let p_info = token.next().unwrap().to_string();

                let mut player_info = PlayerInfo::default();

                if p_info.contains('/') {
                    let mut p_token = p_info.split('/').map(|p| p.trim().to_string());
                    let p_name = p_token.next().unwrap();
                    let p_position = p_token.next().unwrap().to_uppercase();

                    player_info.player_name = p_name;
                    player_info.play_position = vec![p_position];
                    player_info.start = true;
                    player_info.play = true;
                    player_info.number = number;
                    player_info.team_state = "Away".to_string();
                    player_info.team_name = match_info.get_untracked().team_away;
                    player_info.assign_id();
                    player_info
                        .match_id
                        .clone_from(&match_info.get_untracked().match_id);
                } else {
                    player_info.number = number;
                    player_info.player_name = p_info;
                    player_info.start = false;
                    player_info.play = false;
                    player_info.team_state = "Away".to_string();
                    player_info.team_name = match_info.get_untracked().team_away;
                    player_info.assign_id();
                    player_info
                        .match_id
                        .clone_from(&match_info.get_untracked().match_id);
                };

                let payload = Payload {
                    payload: player_info.clone(),
                };
                let payload = serde_wasm_bindgen::to_value(&payload).unwrap();
                register_player_info_action.dispatch(payload);

                if player_info.start {
                    let tagged_event = TaggedEvent {
                        match_id: player_info.match_id,
                        player_id: player_info.player_id,
                        match_date: match_info.get_untracked().match_date,
                        match_teams: format!(
                            "{} vs {}",
                            match_info.get_untracked().team_home,
                            match_info.get_untracked().team_away
                        ),
                        opponent_team: match_info.get_untracked().team_home,
                        time_start: 0.,
                        play_position: player_info.play_position.first().cloned(),
                        player_name: player_info.player_name,
                        team_name: match_info.get_untracked().team_away,
                        event_name: "Start".to_string(),
                        event_type: Some("Play".to_string()),
                        ..Default::default()
                    };

                    let payload = Payload {
                        payload: tagged_event,
                    };
                    let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
                    register_event_action.dispatch(payload);
                }
            });

        set_match_info.set(MatchInfo::default());
    };

    let handle_focus_in_home = move |ev: ev::FocusEvent| {
        ev.prevent_default();
        let input = input_home_ref.get().unwrap();
        if input.inner_text().as_str() == GUIDE_HOME {
            input.set_inner_text("");
        }
    };

    let handle_focus_out_home = move |ev: ev::FocusEvent| {
        ev.prevent_default();
        let input = input_home_ref.get().unwrap();
        if input.inner_text().is_empty() {
            input.set_inner_text(GUIDE_HOME);
        }
    };

    let handle_focus_in_away = move |ev: ev::FocusEvent| {
        ev.prevent_default();
        let input = input_away_ref.get().unwrap();
        if input.inner_text().as_str() == GUIDE_AWAY {
            input.set_inner_text("");
        }
    };

    let handle_focus_out_away = move |ev: ev::FocusEvent| {
        ev.prevent_default();
        let input = input_away_ref.get().unwrap();
        if input.inner_text().is_empty() {
            input.set_inner_text(GUIDE_AWAY);
        }
    };

    let response = create_resource(
        move || register_match_info_action.version().get(),
        move |_| get_latest_registered(),
    );

    view! {
        <div
            class="block m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <div class="bg-slate-600 p-4 rounded-lg block m-auto right-0 left-0 top-0 bottom-0 w-[800px] h-[500px]">
                <form
                    on:submit=register
                    class="flex flex-col content-normal"
                    autocomplete="off"
                >
                    <div class="text-xs self-center text-white mb-1">
                        "Pick match date & time: "
                    </div>
                    <input
                        required
                        _ref=input_date_ref
                        type="datetime-local"
                        class="rounded-full h-[30px] w-[150px] mb-2 text-xs justify-center self-center"
                    />
                    <div class="flex flex-row mb-4 justify-evenly text-xs">
                        <div class="flex flex-col w-[330px] my-1">
                            <p class="mb-2 text-white">"Home Team"</p>
                            <input
                                required
                                class="border focus:outline-none pl-1 mb-2 h-[30px] rounded-md"
                                type="text"
                                autocomplete="off"
                                autocorrect="off"
                                autocapitalize="off"
                                spellcheck="false"
                                placeholder="Team name..."
                                _ref=team_home_name_ref
                            />
                            <div
                                required
                                on:focusin=handle_focus_in_home
                                on:focusout=handle_focus_out_home
                                role="textbox"
                                aria-multiline="true"
                                contenteditable="true"
                                autocomplete="off"
                                autocorrect="off"
                                autocapitalize="off"
                                spellcheck="false"
                                _ref=input_home_ref
                                class="grow border h-[270px] overflow-y-scroll p-2 focus:outline-none bg-white rounded-md"
                            >
                                {move || GUIDE_HOME}
                            </div>
                        </div>
                        <div class="flex flex-col w-[330px] my-1">
                            <p class="mb-2 text-white">"Away Team"</p>
                            <input
                                required
                                class="border focus:outline-none pl-1 mb-2 h-[30px] rounded-md"
                                type="text"
                                autocomplete="off"
                                autocorrect="off"
                                autocapitalize="off"
                                spellcheck="false"
                                placeholder="Team name..."
                                _ref=team_away_name_ref
                            />
                            <div
                                required
                                on:focusin=handle_focus_in_away
                                on:focusout=handle_focus_out_away
                                role="textbox"
                                aria-multiline="true"
                                contenteditable="true"
                                autocomplete="off"
                                autocorrect="off"
                                autocapitalize="off"
                                spellcheck="false"
                                _ref=input_away_ref
                                class="grow border h-[270px] overflow-y-scroll p-2 focus:outline-none bg-white rounded-md"
                            >
                                {move || GUIDE_AWAY}
                            </div>
                        </div>
                    </div>
                    <button
                        _ref=submit_button_ref
                        class="bg-lime-400 hover:bg-indigo-600 rounded-full hover:cursor-pointer text-xs text-black hover:text-white w-[150px] h-[30px] mb-2 self-center"
                        type="submit"
                    >
                        "Submit"
                    </button>
                    <Suspense>
                        <p class="text-xs text-white self-center">
                            { move || {
                                let match_info = response.get().unwrap_or(Ok(MatchInfo::default())).unwrap_or_default();
                                if !match_info.match_date.is_empty() {
                                    format!("Latest registered: {}: {} vs {}",
                                        match_info.match_date,
                                        match_info.team_home,
                                        match_info.team_away
                                    )
                                } else { "".to_string() }
                            }}
                        </p>
                    </Suspense>
                </form>
            </div>
        </div>
    }
}
