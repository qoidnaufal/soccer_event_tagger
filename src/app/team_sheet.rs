use super::invoke;
use leptos::*;
use types::{AppError, Payload, TeamInfo};

async fn get_player_list(team_state: String) -> Result<TeamInfo, AppError> {
    let payload = Payload {
        payload: team_state.clone(),
    };
    let payload = serde_wasm_bindgen::to_value(&payload).unwrap();
    let team_info = invoke("get_all_player_by_team_name", payload).await;
    let team_info = serde_wasm_bindgen::from_value::<Option<TeamInfo>>(team_info)
        .unwrap_or(None)
        .unwrap_or_default();

    Ok(team_info)
}

#[component]
pub fn TeamSheet(team_state: String) -> impl IntoView {
    let team_resource = create_resource(|| (), move |_| get_player_list(team_state.clone()));
    view! {
        <div class="flex flex-col p-2 text-xs w-[150px]">
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
            { move || {
                view! {
                    <p>{ move || team_resource.get().unwrap_or(Ok(TeamInfo::default())).unwrap_or_default().team_name }</p>
                    <ol>
                        <For
                            each=move || team_resource.get().unwrap_or(Ok(TeamInfo::default())).unwrap_or_default().players
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
