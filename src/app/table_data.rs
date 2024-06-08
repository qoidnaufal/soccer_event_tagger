use super::invoke;
use leptos::*;
use types::TaggedEvent;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

async fn get_data() -> Vec<TaggedEvent> {
    let data = invoke("get_all", JsValue::null()).await;
    serde_wasm_bindgen::from_value::<Vec<TaggedEvent>>(data).unwrap_or(Vec::new())
}

#[component]
pub fn TableData(
    video_player_node_ref: NodeRef<html::Video>,
    register_action: Action<JsValue, JsValue>,
) -> impl IntoView {
    let delete_action = create_action(|payload: &JsValue| invoke("delete", payload.clone()));
    let data_resource = create_resource(
        move || {
            (
                register_action.version().get(),
                delete_action.version().get(),
            )
        },
        |_| get_data(),
    );

    view! {
        <table>
            <thead>
                <tr>
                    <th scope="col" class="text-xs text-left w-[15px]">"delete"</th>
                    <th scope="col" class="text-xs text-left w-[50px]">"time start"</th>
                    <th scope="col" class="text-xs text-left w-[50px]">"location start"</th>
                    <th scope="col" class="text-xs text-left w-[50px]">"time end"</th>
                    <th scope="col" class="text-xs text-left w-[50px]">"location end"</th>
                    <th scope="col" class="text-xs text-left w-[80px]">"team name"</th>
                    <th scope="col" class="text-xs text-left w-[80px]">"player name"</th>
                    <th scope="col" class="text-xs text-left w-[160px]">"event"</th>
                </tr>
            </thead>
            <tbody class="overflow-auto">
                <Transition fallback=move || { view! {<p>"loading..."</p>} }>
                    <For
                        { move || data_resource.track() }
                        each=move || data_resource.get().unwrap_or(Vec::new()).into_iter().enumerate()
                        key=|(_, event)| (event.uuid.clone(), event.player_name.clone())
                        children=move |(idx, _)| {
                            let event = create_memo(move |_| {
                                data_resource.get().unwrap_or(Vec::new()).get(idx).unwrap_or(&TaggedEvent::default()).clone()
                            });

                            let video = video_player_node_ref.get().unwrap();

                            let seek = move |_| {
                                let _ = video.fast_seek(event.get().time_start);
                            };

                            let delete = move |_| {
                                let to_delete = event.get_untracked();
                                let payload = types::Payload { payload: to_delete.clone() };
                                let payload = serde_wasm_bindgen::to_value(&payload).unwrap_throw();
                                delete_action.dispatch(payload);
                            };

                            view! {
                                <tr
                                    on:click=seek
                                    class=move || if idx % 2 == 0 {
                                        "bg-slate-200 hover:bg-green-300"
                                    } else { "bg-white hover:bg-green-300" }>
                                    <td class="text-xs">
                                        <button
                                            on:click=delete
                                            class="bg-blue-300 hover:bg-blue-600 rounded-xl w-fit p-2">
                                            "del"
                                        </button>
                                    </td>
                                    <td class="text-xs">{ move || format!("{:.3}", event.get().time_start) }</td>
                                    <td class="text-xs">{ move || event.get().loc_start.to_string() }</td>
                                    <td class="text-xs">{ move || format!("{:.3}", event.get().time_end) }</td>
                                    <td class="text-xs">{ move || event.get().loc_end.to_string() }</td>
                                    <td class="text-xs">{ move || event.get().team_name }</td>
                                    <td class="text-xs">{ move || event.get().player_name }</td>
                                    <td class="text-xs">{ move || event.get().event.to_string() }</td>
                                </tr>
                            }
                        }
                    />
                </Transition>
            </tbody>
        </table>
    }
}
