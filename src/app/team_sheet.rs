use leptos::*;
use types::{AppError, MatchInfo, PlayerInfo};

#[component]
pub fn SelectTeamSheet(
    match_info_resource: Resource<usize, Result<Vec<MatchInfo>, AppError>>,
    set_match_info: WriteSignal<MatchInfo>,
) -> impl IntoView {
    let handle_change = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        let match_info = serde_json::from_str::<MatchInfo>(value.as_str()).unwrap_or_default();
        set_match_info.set(match_info);
    };

    view! {
        <select class="text-xs w-full mb-1" on:change=handle_change>
            <option value="">"Select team sheet.."</option>
            <Transition>
                {move || {
                    let match_info_memo = create_memo(move |_| {
                        match_info_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                    });

                    view! {
                        <For
                            each=move || match_info_memo.get()
                            key=|match_info| (match_info.match_date.clone(), match_info.match_id.clone())
                            children=move |match_info| {
                                let match_info = create_rw_signal(match_info).read_only();

                                view! {
                                    <option value=move || serde_json::to_string(&match_info.get()).unwrap_or_default()>
                                        { move || match_info.get().match_date } ": "
                                        { move || match_info.get().team_home } " vs "
                                        { move || match_info.get().team_away }
                                    </option>
                                }
                            }
                        />
                    }
                }}
            </Transition>
        </select>
    }
}

#[component]
pub fn TeamSheet(
    team_info_resource: Resource<String, Result<Vec<PlayerInfo>, AppError>>,
    team_state: String,
) -> impl IntoView {
    let team_state = create_rw_signal(team_state).read_only();

    view! {
        <div class="flex flex-col p-2 text-xs w-[280px]">
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
            { move || {
                let team_info = create_memo(move |_| {
                    team_info_resource.and_then(|t| {
                        let mut vec = match team_state.get().as_str() {
                            "Home" => t.iter().filter(|v| v.team_state.as_str() == "Home").cloned().collect::<Vec<_>>(),
                            "Away" => t.iter().filter(|v| v.team_state.as_str() == "Away").cloned().collect::<Vec<_>>(),
                            _ => t.iter().filter(|v| v.team_state.as_str() == "Neutral").cloned().collect::<Vec<_>>()
                        };
                        vec.sort_by_key(|p| p.number);
                        vec
                    }).unwrap_or(Ok(Vec::new())).unwrap_or_default()
                });

                view! {
                    <p class="text-white mb-1 font-bold">{ move || team_info.get().first().unwrap_or(&PlayerInfo::default()).team_name.clone() }</p>
                    <ol class="max-h-[410px] overflow-y-scroll">
                        <For
                            each=move || team_info.get()
                            key=|player_info| player_info.number
                            children=move |player_info| {
                                view! {
                                    <li class="px-2 py-1 even:bg-slate-200 odd:bg-white">
                                        {move || player_info.number }". "
                                        { move || player_info.player_name.clone() }
                                    </li>
                                }
                            }
                        />
                    </ol>
                }}
            }
            </Transition>
        </div>
    }
}
