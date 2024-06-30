use leptos::*;
use leptos_router::A;

#[component]
pub fn HomeButton() -> impl IntoView {
    view! {
        <div>
            <A href="/">
                <button
                    class="bg-slate-600 rounded-r-lg size-[30px] pl-1"
                >
                    <img src="public/buttons/home.svg" width="20" height="20"/>
                </button>
            </A>
        </div>
    }
}
