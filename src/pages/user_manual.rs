use leptos::*;
use pulldown_cmark::{html, Parser};

const USER_MANUAL: &str = include_str!("../../USER_MANUAL.md");

#[component]
pub fn UserManual() -> impl IntoView {
    let mut user_manual = String::new();
    let parser = Parser::new(USER_MANUAL);

    html::push_html(&mut user_manual, parser);

    logging::log!("{}", user_manual);

    let user_manual = move || user_manual.clone();

    view! {
        <div
            class="block m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <div
                inner_html=user_manual
                class="prose mx-auto overflow-y-scroll"
            >
            </div>
        </div>
    }
}
