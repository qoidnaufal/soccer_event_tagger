use leptos::*;
use leptos_router::A;
use types::{MatchInfo, Payload, PlayerInfo};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

#[component]
pub fn RegisterMatchInfo() -> impl IntoView {
    let (match_info, set_match_info) = create_signal(MatchInfo::default());
    let input_home_ref = create_node_ref::<html::Div>();
    let input_away_ref = create_node_ref::<html::Div>();

    let register_match_info_action = expect_context::<Action<JsValue, JsValue>>();

    let register = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let player_list_home = input_home_ref
            .get()
            .unwrap()
            .inner_text()
            .trim()
            .to_string();
        let player_list_home = player_list_home
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut token = s.split(')');
                let number = token.next().unwrap().trim().to_string();
                let player_name = token.next().unwrap().to_string();
                PlayerInfo {
                    team_name: match_info.get_untracked().team_home.team_name,
                    number,
                    player_name,
                }
            })
            .collect::<Vec<_>>();
        let player_list_away = input_away_ref
            .get()
            .unwrap()
            .inner_text()
            .trim()
            .to_string();
        let player_list_away = player_list_away
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut token = s.split(')');
                let number = token.next().unwrap().trim().to_string();
                let player_name = token.next().unwrap().to_string();
                PlayerInfo {
                    team_name: match_info.get_untracked().team_away.team_name,
                    number,
                    player_name,
                }
            })
            .collect::<Vec<_>>();
        set_match_info.update(|m| {
            m.team_home.players = player_list_home;
            m.team_away.players = player_list_away;
        });

        let payload = match_info.get_untracked();
        let payload = Payload {
            payload: payload.clone(),
        };
        let payload = serde_wasm_bindgen::to_value(&payload).unwrap_throw();
        register_match_info_action.dispatch(payload);

        set_match_info.set(MatchInfo::default());
    };

    let handle_focus = move |ev: ev::FocusEvent| {
        ev.prevent_default();
        let input = input_home_ref.get().unwrap();
        if input.inner_text().as_str() == "Use \")\" after the number, and \",\" to separate each players. Example: 1) Toldo, 2) D. Alves, ..." {
            input_home_ref.get().unwrap().set_inner_text("");
        }
    };

    let submit_button_ref = create_node_ref::<html::Button>();

    create_effect(move |_| {
        submit_button_ref.on_load(|b| {
            if b.inner_text() != "Submit" {
                b.set_inner_text("Submit");
            }
        });
    });

    view! {
        <div
            class="block m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <div>
                <A href="/">
                    <button
                        class="bg-slate-600 rounded-r-lg size-[30px] pl-1"
                    >
                        <img src="public/icon-home-blue.svg"/>
                    </button>
                </A>
            </div>
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
                        on:change=move |ev| set_match_info.update(|m| m.match_date = event_target_value(&ev))
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
                                on:change=move |ev| {
                                    ev.prevent_default();
                                    set_match_info.update(|m| {
                                        m.team_home.team_name = event_target_value(&ev);
                                        m.team_home.team_state = "Home".to_string();
                                    });
                                }
                            />
                            <div
                                required
                                on:focus=handle_focus
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
                                "Use \")\" after the number, and \",\" to separate each players. Example: 1) Toldo, 2) D. Alves, ..."
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
                                on:change=move |ev| {
                                    ev.prevent_default();
                                    set_match_info.update(|m| {
                                        m.team_away.team_name = event_target_value(&ev);
                                        m.team_away.team_state = "Away".to_string();
                                    });
                                }
                            />
                            <div
                                required
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
                            </div>
                        </div>
                    </div>
                    <button
                        _ref=submit_button_ref
                        class="bg-lime-400 hover:bg-indigo-600 rounded-full hover:cursor-pointer text-xs text-black hover:text-white w-[150px] h-[30px] self-center"
                        type="submit"
                    >
                        { move || if register_match_info_action.value().get().is_some() {
                            "Success"
                        } else { "Submit" } }
                    </button>
                </form>
            </div>
        </div>
    }
}
