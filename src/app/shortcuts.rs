use leptos::*;
use leptos_router::A;

// const TEXT_HIGHLIGHT: &str = "text-red-600";
// const SUBTITLE_BAR: &str = "bg-slate-800 pl-1 text-white";
const SOURCE_CODE: &str = include_str!("../../types/src/tagged_event.rs");

#[component]
pub fn Shortcuts() -> impl IntoView {
    let sc = SOURCE_CODE
        .lines()
        .map(|l| {
            let line = l.replace('"', "\"");
            line
        })
        .collect::<Vec<_>>();
    let src = create_rw_signal(sc).read_only();

    view! {
        <div class="text-xs absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row">
            <div>
                <A href="/">
                    <button
                        class="bg-slate-600 rounded-r-lg size-[30px] pl-1"
                    >
                        <img src="public/home.svg" width="20" height="20"/>
                    </button>
                </A>
            </div>
            <div class="flex flex-col px-2 pb-2 w-full">
                <div class="py-2 flex flex-col items-center text-xl bg-indigo-600 text-white">"SHORTCUTS"</div>
                <div class="bg-green-300 w-fit rounded-lg px-2 py-1 mt-1">"GUIDES"</div>
                <div class="text-wrap pl-2 pr-10 mb-1">
                    <p>
                        "Command shortcuts are separated by \"/\".
                        for example: 3h/pi/9a, will be translated into: [PLAYER] player number [3] from team [h]ome / [EVENT] [p]ass [i]ntercepted / [OUTCOME] intercepted by player number [9] from team [a]way.
                        Events which are categorized as pass, need 3 commands to register: the [PLAYER], the [EVENT], and the [OUTCOME]. The [PLAYER] is the guy doing the action. The [EVENT] is the action.
                        The [OUTCOME] is the player at the end of an action, for example a player receiving or intercepting the pass, etc. In case you forget to register the [OUTCOME],
                        it would still be okay and the event would still be registered. It's just that later on you will need to do more effort when analyzing the data. Substitution also requires you
                        to register the [OUTCOME]."
                    </p>
                </div>
                <div class="flex flex-col overflow-scroll w-full">
                    <For
                        each=move || src.get().into_iter().enumerate()
                        key=|(idx, _)| idx.clone()
                        children=|(idx, source)| {
                            if !source.is_empty() {
                                view! {
                                    <pre class="flex flex-row w-full">
                                        <div class="w-[30px]">{ move || idx }</div>
                                        <code>
                                            { move || source.clone() }
                                        </code>
                                    </pre>
                                }.into_view()
                            } else {
                                view! {
                                    <pre class="flex flex-row w-full">
                                        <div class="w-[30px]">{ move || idx }</div>
                                        <br/>
                                    </pre>
                                }.into_view()
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
