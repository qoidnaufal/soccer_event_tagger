use crate::app::{get_all_data, invoke};

use leptos::*;
use wasm_bindgen::JsValue;

#[component]
pub fn DataDashboard() -> impl IntoView {
    let delete_row_action =
        create_action(|payload: &JsValue| invoke("delete_by_id", payload.clone()));

    let delete_all_events_action =
        create_action(|payload: &JsValue| invoke("delete_all_data", payload.clone()));

    let delete_all_match_info_action =
        create_action(|payload: &JsValue| invoke("delete_all_match_info", payload.clone()));

    let data_resource = create_resource(
        move || {
            (
                delete_row_action.version().get(),
                delete_all_events_action.version().get(),
            )
        },
        move |_| get_all_data(),
    );

    let delete_all = move |ev: ev::MouseEvent| {
        ev.prevent_default();

        delete_all_events_action.dispatch(JsValue::null());
        delete_all_match_info_action.dispatch(JsValue::null());
    };

    view! {
        <div
            class="block m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <div class="m-auto right-0 left-0 top-0 bottom-0 p-2 block w-full h-full">
                <div class="text-xs">
                    <Suspense>
                        { move || {
                            let memoized_data = create_memo(move |_| {
                                data_resource.get().unwrap_or(Ok(Vec::new())).unwrap_or_default()
                            });

                            view! {
                                <div class="w-full">
                                    <table class="table-fixed w-full">
                                        <thead class="w-full">
                                            <tr class="w-full">
                                                <th scope="col" class=" text-left w-[10px]">
                                                    <button
                                                        on:click=delete_all
                                                        class="hover:bg-red-600 px-2 py-1 rounded-md"
                                                    >
                                                        <img src="/public/buttons/delete.svg" width="15" height="15"/>
                                                    </button>
                                                </th>
                                                <th scope="col" class="text-left w-[30px]">
                                                    "match date"
                                                </th>
                                                <th scope="col" class="text-left w-[15px]">
                                                    "time start"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "location start"
                                                </th>
                                                <th scope="col" class="text-left w-[15px]">
                                                    "time end"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "location end"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "opponent"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "team name"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "player name"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "position"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "event name"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "event type"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "event source"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "outcome"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "team end"
                                                </th>
                                                <th scope="col" class="text-left w-[20px]">
                                                    "player end"
                                                </th>
                                            </tr>
                                        </thead>
                                    </table>
                                </div>
                                <div
                                    class="w-full grow-0 h-[900px] overflow-y-scroll flex flex-col"
                                >
                                    <table class="table-fixed w-full">
                                        <tbody class="w-full">
                                            <For
                                                each=move || memoized_data.get()
                                                key=|event| event.uuid.clone()
                                                children=move |event| {
                                                    let event = create_rw_signal(event).read_only();

                                                    let delete = move |ev: ev::MouseEvent| {
                                                        ev.stop_immediate_propagation();
                                                        let to_delete = event.get_untracked().uuid;
                                                        let payload = types::Payload {
                                                            payload: to_delete,
                                                        };
                                                        let payload = serde_wasm_bindgen::to_value(&payload)
                                                            .unwrap_or_default();
                                                        delete_row_action.dispatch(payload);
                                                        event.dispose();
                                                    };

                                                    view! {
                                                        <tr
                                                            class="w-full h-fit py-2 odd:bg-slate-200 even:bg-white hover:bg-green-300 hover:cursor-pointer"
                                                        >
                                                            <td class="w-[10px]">
                                                                <button
                                                                    on:click=delete
                                                                    class="bg-blue-300 hover:bg-red-600 hover:text-white rounded-md w-fit px-2 py-1 m-1  z-10"
                                                                >
                                                                    "X"
                                                                </button>
                                                            </td>
                                                            <td class=" w-[30px]">
                                                                {move || event.get().match_date}
                                                            </td>
                                                            <td class=" w-[15px]">
                                                                {move || format!("{:02}", (event.get().time_start / 60.) as u8)} ":"
                                                                {move || format!("{:02}", (event.get().time_start % 60.) as u8)}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                "x: "{move || event.get().x_start.unwrap_or_default()}
                                                                ", y: "{move || event.get().y_start.unwrap_or_default()}
                                                            </td>
                                                            <td class=" w-[15px]">
                                                                {move || format!("{:02}", (event.get().time_end.unwrap_or_default() / 60.) as u8)} ":"
                                                                {move || format!("{:02}", (event.get().time_end.unwrap_or_default() % 60.) as u8)}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                "x: "{move || event.get().x_end.unwrap_or_default()}
                                                                ", y: "{move || event.get().y_end.unwrap_or_default()}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().opponent_team}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().team_name}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().player_name}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().play_position}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().event_name}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().event_type}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().event_source}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().outcome}
                                                            </td>
                                                            <td class=" w-[20px]">
                                                                {move || event.get().team_end}
                                                            </td>
                                                            <td class=" w-[20px]">
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
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
