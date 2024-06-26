use leptos::*;

#[component]
pub fn VideoPlayer(
    video_src: Memo<Option<String>>,
    video_player_node_ref: NodeRef<html::Video>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <video
                src=move || video_src.get().unwrap_or_default()
                _ref=video_player_node_ref
                controls
                width="1000"
                height="563"
            />
        </div>
    }
}
