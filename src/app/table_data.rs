use super::invoke;
use leptos::*;
use types::TaggedEvent;
use wasm_bindgen::UnwrapThrowExt;

#[component]
pub fn TableData(
    data: ReadSignal<Vec<TaggedEvent>>,
    set_data: WriteSignal<Vec<TaggedEvent>>,
    video_player_node_ref: NodeRef<html::Video>,
) -> impl IntoView {
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
            <tbody>
                <For
                    each=move || data.get().into_iter().enumerate()
                    key=|(_, event)| event.uuid.clone()
                    children=move |(idx, _)| {
                        let event = create_memo(move |_| {
                            data.get().get(idx).unwrap_or(&TaggedEvent::default()).clone()
                        });

                        let video = video_player_node_ref.get().unwrap();

                        let seek = move |_| {
                            let _ = video.fast_seek(event.get().time_start);
                        };

                        let delete = move |_| {
                            spawn_local(async move {
                                let to_delete = event.get_untracked();
                                let payload = types::Payload { payload: to_delete.clone() };
                                let payload = serde_wasm_bindgen::to_value(&payload).unwrap_throw();
                                let result = invoke("delete", payload).await;
                                let result = serde_wasm_bindgen::from_value::<Vec<types::TaggedEvent>>(result).unwrap();
                                set_data.update(|tag| *tag = result);
                            });
                        };

                        view! {
                            <tr on:click=seek class="hover:bg-green-300">
                                <td class="text-xs">
                                    <button
                                        on:click=delete
                                        class="bg-blue-300 rounded-md w-fit p-2">
                                        "del"
                                    </button>
                                </td>
                                <td class="text-xs">{ move || format!("{:.3}", event.get().time_start) }</td>
                                <td class="text-xs">"x1: "{ move || event.get().loc_start.x }", y1: "{ move || event.get().loc_start.y }</td>
                                <td class="text-xs">{ move || format!("{:.3}", event.get().time_end) }</td>
                                <td class="text-xs">"x2: "{ move || event.get().loc_end.x }", y2: "{ move || event.get().loc_end.y }</td>
                                <td class="text-xs">{ move || event.get().team_name }</td>
                                <td class="text-xs">{ move || event.get().player_name }</td>
                                <td class="text-xs">{move || event.get().event.to_string() }</td>
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    }
}
