use super::invoke;
use leptos::*;
use types::{AppError, Payload, TaggedEvent};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

async fn get_data(match_id: String) -> Result<Vec<TaggedEvent>, AppError> {
    let payload = Payload { payload: match_id };
    let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
    let data = invoke("get_match_events_from_match_id", payload).await;
    let vec_data = serde_wasm_bindgen::from_value::<Vec<TaggedEvent>>(data).unwrap_or(Vec::new());

    Ok(vec_data)
}

#[component]
pub fn TableData(
    video_player_node_ref: NodeRef<html::Video>,
    register_event_action: Action<JsValue, JsValue>,
    match_id: ReadSignal<String>,
) -> impl IntoView {
    let delete_action = create_action(|payload: &JsValue| invoke("delete_by_id", payload.clone()));
    let data_resource = create_resource(
        move || {
            (
                register_event_action.version().get(),
                delete_action.version().get(),
                match_id.get(),
            )
        },
        move |_| get_data(match_id.get_untracked()),
    );

    view! {
        <div class="w-full mt-1">
            <Transition fallback=move || {
                view! { <p>"loading..."</p> }
            }>
                {move || {
                    view! {
                        <div class="w-full">
                            <table class="table-fixed w-full">
                                <thead class="w-full">
                                    <tr class="w-full">
                                        <th scope="col" class="text-xs text-left w-[20px]">
                                            "delete"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "time start"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "location start"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "time end"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "location end"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[50px]">
                                            "team name"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[50px]">
                                            "player name"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[180px]">
                                            "event"
                                        </th>
                                    </tr>
                                </thead>
                            </table>
                        </div>
                        <div
                            class="w-full max-h-[180px] overflow-y-scroll flex flex-col-reverse"
                        >
                            <table class="table-fixed w-full">
                                <tbody class="w-full">
                                    <For
                                        each=move || data_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                                        key=|event| event.uuid.clone()
                                        children=move |event| {
                                            let event = create_rw_signal(event).read_only();
                                            let video = video_player_node_ref.get().unwrap();
                                            let seek = move |ev: ev::MouseEvent| {
                                                ev.prevent_default();
                                                let _ = video.fast_seek(event.get().time_start);
                                            };
                                            let delete = move |ev: ev::MouseEvent| {
                                                ev.stop_immediate_propagation();
                                                let to_delete = event.get_untracked();
                                                let payload = types::Payload {
                                                    payload: to_delete,
                                                };
                                                let payload = serde_wasm_bindgen::to_value(&payload)
                                                    .unwrap_throw();
                                                delete_action.dispatch(payload);
                                                event.dispose();
                                            };
                                            view! {
                                                <tr
                                                    on:click=seek
                                                    class="w-full h-fit py-2 odd:bg-slate-200 even:bg-white hover:bg-green-300"
                                                >
                                                    <td class="w-[20px]">
                                                        <button
                                                            on:click=delete
                                                            class="bg-blue-300 hover:bg-red-600 rounded-md w-fit p-2 text-xs z-10"
                                                        >
                                                            "del"
                                                        </button>
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || (event.get().time_start / 60.) as u8} ":"
                                                        {move || (event.get().time_start % 60.) as u8}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().loc_start.to_string()}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || (event.get().time_end / 60.) as u8} ":"
                                                        {move || (event.get().time_end % 60.) as u8}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().loc_end.to_string()}
                                                    </td>
                                                    <td class="text-xs w-[50px]">
                                                        {move || event.get().team_name}
                                                    </td>
                                                    <td class="text-xs w-[50px]">
                                                        {move || event.get().player_name}
                                                    </td>
                                                    <td class="text-xs w-[180px]">
                                                        {move || event.get().event.to_string()}
                                                    </td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>
                        </div>
                    }
                }}
            </Transition>
        </div>
    }
}
