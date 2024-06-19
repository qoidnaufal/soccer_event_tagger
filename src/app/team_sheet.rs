use leptos::*;
use types::{AppError, MatchData, MatchInfo, TeamInfo};

#[component]
pub fn SelectTeamSheet(
    match_info_resource: Resource<usize, Result<Vec<MatchInfo>, AppError>>,
    set_match_data: WriteSignal<MatchData>,
) -> impl IntoView {
    let handle_change = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        let match_data = MatchData::from_str(value);
        set_match_data.set(match_data);
    };

    view! {
        <select class="text-xs w-full mb-1" on:change=handle_change>
            <option value="">"Select team sheet.."</option>
            <Transition>
                {move || {
                    let match_info = create_memo(move |_| {
                        match_info_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                    });

                    view! {
                        <For
                            each=move || match_info.get()
                            key=|match_info| match_info.match_id.clone()
                            children=move |match_info| {
                                let match_info = create_rw_signal(match_info).read_only();

                                view! {
                                    <option value=move || match_info.get().to_string()>
                                        { move || match_info.get().match_date } ": "
                                        { move || match_info.get().team_home.team_name } " vs "
                                        { move || match_info.get().team_away.team_name }
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
    team_info_resource: Resource<String, Result<MatchInfo, AppError>>,
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
                    team_info_resource.and_then(|m| {
                        match team_state.get().as_str() {
                            "Home" => m.team_home.clone(),
                            "Away" => m.team_away.clone(),
                            _ => TeamInfo::default()
                        }
                    }).unwrap_or(Ok(TeamInfo::default())).unwrap_or_default()
                });

                view! {
                    <p class="text-white mb-1 font-bold">{ move || team_info.get().team_name }</p>
                    <ol class="max-h-[410px] overflow-y-scroll">
                        <For
                            each=move || team_info.get().players
                            key=|player_info| player_info.number.clone()
                            children=move |player_info| {
                                view! {
                                    <li class="px-2 py-1 even:bg-slate-200 odd:bg-white">
                                        {move || player_info.number.clone() }". "
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
