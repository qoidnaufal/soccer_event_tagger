use crate::app::invoke;
use leptos::*;
use types::{AppError, MatchInfo, Payload, TaggedEvent};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

async fn get_data(match_id: String) -> Result<Vec<TaggedEvent>, AppError> {
    let payload = Payload { payload: match_id };
    let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
    let data = invoke("get_match_events_from_match_id", payload).await;
    let vec_data = serde_wasm_bindgen::from_value::<Vec<TaggedEvent>>(data).unwrap_or_default();

    Ok(vec_data)
}

#[component]
pub fn TableData(
    video_player_node_ref: NodeRef<html::Video>,
    register_event_action: Action<JsValue, JsValue>,
    match_info: ReadSignal<MatchInfo>,
) -> impl IntoView {
    let delete_action = create_action(|payload: &JsValue| invoke("delete_by_id", payload.clone()));
    let delete_all_action = create_action(|payload: &JsValue| {
        invoke("delete_all_records_by_match_id", payload.clone())
    });
    let data_resource = create_resource(
        move || {
            (
                register_event_action.version().get(),
                delete_action.version().get(),
                delete_all_action.version().get(),
                match_info.get(),
            )
        },
        move |_| get_data(match_info.get_untracked().match_id),
    );

    let delete_all = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let payload = Payload {
            payload: match_info.get_untracked().match_id,
        };
        let payload = serde_wasm_bindgen::to_value(&payload).unwrap_or_default();
        delete_all_action.dispatch(payload);
    };

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
                                        <th scope="col" class="text-xs text-left w-[15px]">
                                            <button
                                                on:click=delete_all
                                                class="hover:bg-red-600 px-2 py-1 rounded-md text-xs"
                                            >
                                                <img src="/public/buttons/delete.svg" width="15" height="15"/>
                                            </button>
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[20px]">
                                            "time start"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "location start"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[20px]">
                                            "time end"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "location end"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "team name"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "player name"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[20px]">
                                            "position"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "event name"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "event type"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "event source"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "outcome"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "team end"
                                        </th>
                                        <th scope="col" class="text-xs text-left w-[30px]">
                                            "player end"
                                        </th>
                                    </tr>
                                </thead>
                            </table>
                        </div>
                        <div
                            class="w-full grow-0 h-[300px] overflow-y-scroll flex flex-col-reverse"
                        >
                            <table class="table-fixed w-full">
                                <tbody class="w-full">
                                    <For
                                        each=move || data_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                                        key=|event| ((event.time_start * 10000.) as usize, event.uuid.clone())
                                        children=move |event| {
                                            let event = create_rw_signal(event).read_only();
                                            // let video = video_player_node_ref.get().unwrap();
                                            let seek_start = move |ev: ev::MouseEvent| {
                                                ev.prevent_default();
                                                let _ = video_player_node_ref.get().unwrap().fast_seek(event.get().time_start);
                                            };
                                            let seek_end = move |ev: ev::MouseEvent| {
                                                ev.stop_immediate_propagation();
                                                let _ = video_player_node_ref.get().unwrap().fast_seek(event.get().time_end.unwrap_or_default());
                                            };
                                            let delete = move |ev: ev::MouseEvent| {
                                                ev.stop_immediate_propagation();
                                                let to_delete = event.get_untracked().uuid;
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
                                                    on:click=seek_start
                                                    class="w-full h-fit py-2 odd:bg-slate-200 even:bg-white hover:bg-green-300 hover:cursor-pointer"
                                                >
                                                    <td class="w-[15px]">
                                                        <button
                                                            on:click=delete
                                                            class="bg-blue-300 hover:bg-red-600 hover:text-white rounded-md w-fit px-2 py-1 m-1 text-xs z-10"
                                                        >
                                                            "X"
                                                        </button>
                                                    </td>
                                                    <td class="text-xs w-[20px]">
                                                        {move || format!("{:02}", (event.get().time_start / 60.) as u8)} ":"
                                                        {move || format!("{:02}", (event.get().time_start % 60.) as u8)}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        "x: "{move || event.get().x_start.unwrap_or_default()}
                                                        ", y: "{move || event.get().y_start.unwrap_or_default()}
                                                    </td>
                                                    <td class="text-xs w-[20px] hover:text-lg" on:click=seek_end>
                                                        {move || format!("{:02}", (event.get().time_end.unwrap_or_default() / 60.) as u8)} ":"
                                                        {move || format!("{:02}", (event.get().time_end.unwrap_or_default() % 60.) as u8)}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        "x: "{move || event.get().x_end.unwrap_or_default()}
                                                        ", y: "{move || event.get().y_end.unwrap_or_default()}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().team_name}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().player_name}
                                                    </td>
                                                    <td class="text-xs w-[20px]">
                                                        {move || event.get().play_position}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().event_name}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().event_type}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().event_source}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().outcome}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().team_end}
                                                    </td>
                                                    <td class="text-xs w-[30px]">
                                                        {move || event.get().player_end}
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
