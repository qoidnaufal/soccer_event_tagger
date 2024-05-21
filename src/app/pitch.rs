use leptos::*;
use types::Point;

#[component]
pub fn Pitch(set_coordinate: WriteSignal<Point>) -> impl IntoView {
    let image_node = create_node_ref::<html::Img>();

    let handle_pitch_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let image = image_node.get().unwrap();
        set_coordinate.update(|point| {
            point.reset();
            point.x = Some(ev.offset_x() * 100 / image.width() as i32);
            point.y = Some(ev.offset_y() * 100 / image.height() as i32);
        });
    };

    view! {
        <img
            _ref=image_node
            on:mousedown=handle_pitch_click
            src="public/pitch.svg"
            draggable="false"
            width="500"
            class="absolute cursor-copy z-10 top-0 bottom-0 left-0 right-0 m-auto"/>
    }
}
