use leptos::*;

#[component]
pub fn VideoPlayer(
    video_src: ReadSignal<Option<String>>,
    video_player_node_ref: NodeRef<html::Video>,
) -> impl IntoView {
    let (duration, set_duration) = create_signal::<f64>(0.);

    create_effect(move |_| {
        let vid = video_player_node_ref.get().unwrap();
        if video_src.get().is_some() {
            set_duration.set(vid.duration());
        }
    });

    view! {
        <div class="flex flex-col">
            <video
                src=move || video_src.get().unwrap_or_default()
                _ref=video_player_node_ref
                controls
                width="1020"
                class="pointer-events-none"
            />
            <div class="flex flex-row">
                <progress value=100 min=0></progress>
                <p class="text-xs">"duration: "{move || duration.get()}</p>
            </div>
        </div>
    }
}
