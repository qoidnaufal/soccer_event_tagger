use leptos::*;
use types::{AppError, MatchInfo, TeamInfo};
use wasm_bindgen::JsValue;

#[component]
pub fn TeamSheet(
    match_info_resource: Resource<JsValue, Result<MatchInfo, AppError>>,
    team_state: String,
) -> impl IntoView {
    let team_state = create_rw_signal(team_state).read_only();
    view! {
        <div class="flex flex-col p-2 text-xs w-[170px]">
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
            { move || {
                let team_info = create_memo(move |_| {
                    match_info_resource.and_then(|m| {
                        match team_state.get().as_str() {
                            "Home" => m.team_home.clone(),
                            "Away" => m.team_away.clone(),
                            _ => TeamInfo::default()
                        }
                    }).unwrap_or(Ok(TeamInfo::default())).unwrap_or_default()
                });

                view! {
                    <p class="text-white mb-1 font-bold">{ move || team_info.get().team_name }</p>
                    <ol>
                        <For
                            each=move || team_info.get().players
                            key=|player_info| player_info.number.clone()
                            children=move |player_info| {
                                view! {
                                    <li class="even:bg-slate-200 odd:bg-white">{move || player_info.number.clone() }". "{ move || player_info.player_name.clone() }</li>
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
