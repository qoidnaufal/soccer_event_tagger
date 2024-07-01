use crate::app::invoke;

use leptos::*;
use types::{AppError, MatchInfo, Payload, PlayerInfo};
use wasm_bindgen::JsValue;

#[component]
pub fn SelectTeamSheet(
    match_info_resource: Resource<(usize, usize), Result<Vec<MatchInfo>, AppError>>,
    match_info: ReadSignal<MatchInfo>,
    set_match_info: WriteSignal<MatchInfo>,
    delete_match_info_action: Action<JsValue, JsValue>,
    delete_all_row_action: Action<JsValue, JsValue>,
) -> impl IntoView {
    let delete_player_info_action = create_action(|payload: &JsValue| {
        invoke("delete_all_players_from_match_id", payload.clone())
    });

    let select_match_info = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        let m_info = serde_json::from_str::<MatchInfo>(value.as_str()).unwrap_or_default();
        set_match_info.set(m_info);
    };

    let delete_match_info = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let payload = Payload {
            payload: match_info.get_untracked().match_id,
        };
        let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();

        delete_match_info_action.dispatch(payload.clone());
        delete_player_info_action.dispatch(payload.clone());
        delete_all_row_action.dispatch(payload);
    };

    on_cleanup(move || set_match_info.set(MatchInfo::default()));

    view! {
        <div class="flex flex-row w-full mb-3 text-xs">
            <select class="w-full" on:change=select_match_info>
                <option value="">"Select team sheet.."</option>
                <Suspense>
                    <For
                        each=move || match_info_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                        key=|m_info| (m_info.match_date.clone(), m_info.match_id.clone())
                        children=move |m_info| {
                            let m_info = create_rw_signal(m_info).read_only();

                            view! {
                                <option value=move || serde_json::to_string(&m_info.get()).unwrap_or_default()>
                                    { move || m_info.get().match_date } ": "
                                    { move || m_info.get().team_home } " vs "
                                    { move || m_info.get().team_away }
                                </option>
                            }
                        }
                    />
                </Suspense>
            </select>
            // <button
            //     type="button"
            //     class="text-xs bg-lime-400 border-none rounded-xl w-[50px] py-1 ml-2 flex flex-row justify-center hover:bg-green-400"
            // >
            //     "edit"
            // </button>
            <button
                on:click=delete_match_info
                type="button"
                class="text-xs bg-lime-400 border-none rounded-xl w-[50px] py-1 ml-2 flex flex-row justify-center hover:bg-red-500"
            >
                <img src="/public/buttons/delete.svg" width="15" height="15"/>
            </button>
        </div>
    }
}

#[component]
pub fn TeamSheet(
    team_info_resource: Resource<(String, usize), Result<Vec<PlayerInfo>, AppError>>,
    team_state: String,
) -> impl IntoView {
    let team_state = create_rw_signal(team_state).read_only();
    let (team_info, set_team_info) = create_signal::<Vec<PlayerInfo>>(Vec::new());

    view! {
        <div class="flex flex-col p-2 text-xs w-[280px] grow-0">
            <Suspense>
                { move || {
                    let t =
                        team_info_resource.and_then(|t| {
                            match team_state.get().as_str() {
                                "Home" => t.iter().filter(|v| v.team_state.as_str() == "Home").cloned().collect::<Vec<_>>(),
                                "Away" => t.iter().filter(|v| v.team_state.as_str() == "Away").cloned().collect::<Vec<_>>(),
                                _ => t.iter().filter(|v| v.team_state.as_str() == "Neutral").cloned().collect::<Vec<_>>()
                            }
                        }).unwrap_or(Ok(Vec::new())).unwrap_or_default();

                    set_team_info.set(t);

                    view! {
                        <p class="text-white mb-1 font-bold">
                            { move || team_info.get().first().unwrap_or(&PlayerInfo::default()).team_name.clone() }
                        </p>
                        <ol class="max-h-[450px] overflow-y-scroll">
                            <For
                                each=move || team_info.get()
                                key=|player_info| player_info.number
                                children=move |player_info| {
                                    if player_info.play {
                                        view! {
                                            <li class="px-2 py-1 even:bg-lime-200 odd:bg-green-300">
                                                { move || player_info.number } ". "
                                                { move || player_info.player_name.clone() }
                                            </li>
                                        }
                                    } else {
                                        view! {
                                            <li class="px-2 py-1 even:bg-slate-200 odd:bg-slate-300">
                                                { move || player_info.number } ". "
                                                { move || player_info.player_name.clone() }
                                            </li>
                                        }
                                    }
                                }
                            />
                        </ol>
                    }}
                }
            </Suspense>
        </div>
    }
}
