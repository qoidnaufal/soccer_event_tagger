use crate::app::{get_all_match, invoke};

use leptos::*;
use types::Payload;
use wasm_bindgen::JsValue;

#[component]
pub fn DataDashboard() -> impl IntoView {
    let delete_all_events_action =
        create_action(|payload: &JsValue| invoke("delete_all_data", payload.clone()));

    let delete_all_row_action = create_action(|payload: &JsValue| {
        invoke("delete_all_records_by_match_id", payload.clone())
    });

    let delete_all_match_info_action =
        create_action(|payload: &JsValue| invoke("delete_all_match_info", payload.clone()));

    let delete_match_info_action =
        create_action(|payload: &JsValue| invoke("delete_match_info_by_id", payload.clone()));

    let export_all_data_action =
        create_action(|payload: &JsValue| invoke("export_all_tagged_events", payload.clone()));

    let data_resource = create_resource(
        move || {
            (
                delete_all_match_info_action.version().get(),
                delete_all_events_action.version().get(),
                delete_all_row_action.version().get(),
                delete_match_info_action.version().get(),
            )
        },
        move |_| get_all_match(),
    );

    let delete_all = move |ev: ev::MouseEvent| {
        ev.prevent_default();

        delete_all_events_action.dispatch(JsValue::null());
        delete_all_match_info_action.dispatch(JsValue::null());
    };

    let export_all = move |ev: ev::MouseEvent| {
        ev.prevent_default();

        export_all_data_action.dispatch(JsValue::null());
    };

    view! {
        <div
            class="block m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <div class="bg-slate-600 p-4 rounded-lg block m-auto right-0 left-0 top-0 bottom-0 w-[800px] h-[500px] text-xs">
                <div class="flex flex-row">
                    <button
                        on:click=export_all
                        type="button"
                        class="bg-lime-300 rounded-full px-2 py-1 hover:bg-indigo-500 hover:text-white"
                    >
                        "Export All"
                    </button>
                    <button
                        on:click=delete_all
                        type="button"
                        class="bg-red-500 rounded-full px-2 py-1 hover:bg-red-600 ml-2"
                    >
                        "Delete All"
                    </button>
                </div>
                <div class="my-2 h-[480px] overflow-y-scroll">
                    <Suspense>
                        <ul>
                            <For
                                each=move || data_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                                key=|m| m.match_id.clone()
                                children=move |m| {
                                    let match_info = create_rw_signal(m);

                                    let export_data = move |ev: ev::MouseEvent| {
                                        ev.prevent_default();
                                        spawn_local(async move {
                                            let payload = Payload {
                                                payload: match_info.get_untracked(),
                                            };
                                            let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
                                            invoke("export_tagged_events_from_match_id", payload).await;
                                        });
                                    };

                                    let delete_match_info = move |ev: ev::MouseEvent| {
                                        ev.prevent_default();
                                        let payload = Payload {
                                            payload: match_info.get_untracked().match_id,
                                        };
                                        let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();

                                        delete_match_info_action.dispatch(payload.clone());
                                        delete_all_row_action.dispatch(payload);
                                    };

                                    view! {
                                        <li class="px-2 py-1 mb-1 even:bg-white odd:bg-slate-200 hover:bg-green-300 flex flex-row items-center">
                                            <div class="w-full">
                                                { move || match_info.get().match_date } ": "
                                                { move || match_info.get().team_home } " vs "
                                                { move || match_info.get().team_away }
                                            </div>
                                            <button
                                                on:click=export_data
                                                type="button"
                                                class="bg-blue-300 hover:bg-blue-400 rounded-md px-2 py-1"
                                            >
                                                "export"
                                            </button>
                                            <button
                                                on:click=delete_match_info
                                                type="button"
                                                class="bg-red-500 hover:bg-red-600 rounded-md px-2 py-1 ml-2"
                                            >
                                                "delete"
                                            </button>
                                        </li>

                                    }
                                }
                            />
                        </ul>
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
