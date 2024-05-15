use leptos::*;

#[component]
pub fn FilePicker<F: FnMut(ev::MouseEvent) + 'static>(get_file_path: F) -> impl IntoView {
    view! {
        <button
            on:click=get_file_path
            class="border-none rounded-full bg-indigo-800 px-4 text-white hover:bg-indigo-600 h-[30px] w-[130px] text-xs"
        >"Open Video"</button>
    }
}
