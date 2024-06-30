use crate::app::{get_all_match, get_players_by_match_id, CtxProvider};
use crate::components::{HomeButton, TableData};
use leptos::*;

#[component]
pub fn DataDashboard() -> impl IntoView {
    view! {
        <div
            class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <HomeButton/>
            <div>
                // <TableData/>
            </div>
        </div>
    }
}
