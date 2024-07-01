use leptos::*;

const SOURCE_CODE: &str = include_str!("../../types/src/tagged_event.rs");
const GUIDE: &str = "Command shortcuts are separated by \"/\".
                    for example: 3h/pi/9a, will be translated into: [PLAYER] player number [3] from team [h]ome / [EVENT] [p]ass [i]ntercepted / [OUTCOME] intercepted by player number [9] from team [a]way.
                    Events which are categorized as pass, need 3 commands to register: the [PLAYER], the [EVENT], and the [OUTCOME]. The [PLAYER] is the guy doing the action. The [EVENT] is the action.
                    The [OUTCOME] is the player at the end of an action, for example a player receiving or intercepting the pass, etc. In case you forget to register the [OUTCOME],
                    it would still be okay and the event would still be registered. It's just that later on you will need to do more effort when analyzing the data. Substitution also requires you
                    to register the [OUTCOME].";

async fn source_code(keyword: String, sc: Memo<Vec<(usize, String)>>) -> Vec<(usize, String)> {
    let keyword = keyword.split_whitespace().collect::<Vec<_>>();
    sc.get_untracked()
        .iter()
        .filter(|s| keyword.iter().all(|ss| s.1.to_lowercase().contains(ss)))
        .cloned()
        .collect::<Vec<_>>()
}

#[component]
pub fn Shortcuts() -> impl IntoView {
    let sc = SOURCE_CODE
        .lines()
        .skip(60)
        .filter(|s| !s.contains("//"))
        .collect::<String>();

    let sc = create_memo(move |_| {
        sc.split_inclusive("} ")
            .map(|s| {
                s.replace("self.", "")
                    .replace(';', ";\n")
                    .replace("{ ", "{\n ")
            })
            .take(142)
            .enumerate()
            .collect::<Vec<_>>()
    });

    let (filter, set_filter) = create_signal(String::new());

    let resource = create_local_resource_with_initial_value(
        move || filter.get(),
        move |keyword| source_code(keyword, sc),
        Some(sc.get_untracked()),
    );

    let search = move |ev: ev::Event| {
        ev.prevent_default();
        let keyword = event_target_value(&ev);
        set_filter.set(keyword)
    };

    view! {
        <div class="block m-auto right-0 left-0 top-0 bottom-0 size-full text-xs flex flex-row">
            <div class="flex flex-col px-2 pb-2 w-full">
                <div class="py-2 flex flex-col items-center text-xl bg-indigo-600 text-white">"SHORTCUTS"</div>
                <div class="bg-green-300 w-fit rounded-lg px-2 py-1 mt-1">"GUIDES"</div>
                <div class="text-wrap pl-2 pr-10 mb-1">
                    <p>
                        { move || GUIDE }
                    </p>
                </div>
                <input
                    class="border rounded-lg py-1 px-2 focus:outline-none ml-2 mb-2 w-full"
                    type="search"
                    placeholder="Search... eg: 'free kick intercepted' or 'foul'"
                    on:input=search
                    autocomplete="off"
                    autocorrect="off"
                    autocapitalize="off"
                    spellcheck="false"
                />
                <div class="flex flex-col overflow-scroll w-full pl-2">
                    <Suspense
                        fallback=move || view! { <p>"Loading..."</p> }
                    >
                        <For
                            {move || resource.track()}
                            each=move || resource.get().unwrap_or(Vec::new())
                            key=|(idx, _)| *idx
                            children=|(idx, source)| {
                                view! {
                                    <div class="flex flex-row w-full hover:bg-green-300">
                                        <div class="w-[40px] text-right mr-4 bg-slate-200 px-1">{ move || idx }</div>
                                        <pre>
                                            { move || source.clone() }
                                        </pre>
                                    </div>
                                }
                            }
                        />
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
