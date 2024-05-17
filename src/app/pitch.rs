use leptos::*;

#[derive(Debug, Clone, Default)]
pub struct Point {
    pub x1: Option<i32>,
    pub y1: Option<i32>,
    pub x2: Option<i32>,
    pub y2: Option<i32>,
}

impl Point {
    fn reset(&mut self) {
        self.x1 = None;
        self.y1 = None;
        self.x2 = None;
        self.y2 = None;
    }
}

#[component]
pub fn Pitch(set_coordinate: WriteSignal<Point>) -> impl IntoView {
    let image_node = create_node_ref::<html::Img>();
    let handle_pitch_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let image = image_node.get().unwrap();
        logging::log!("image width: {}", image.width() as i32);
        logging::log!("image height: {}", image.height() as i32);
        set_coordinate.update(|point| match ev.type_().as_str() {
            "mousedown" => {
                point.reset();
                point.x1 = Some(ev.offset_x() * 100 / image.width() as i32);
                point.y1 = Some(ev.offset_y() * 60 / image.height() as i32);
            }
            "mouseup" => {
                let x2 = Some(ev.offset_x() * 100 / image.width() as i32);
                let y2 = Some(ev.offset_y() * 60 / image.height() as i32);

                if x2 != point.x1 || y2 != point.y1 {
                    point.x2 = x2;
                    point.y2 = y2;
                }
            }
            _ => (),
        });
    };

    view! {
        <img
            _ref=image_node
            on:mousedown=handle_pitch_click
            on:mouseup=handle_pitch_click
            src="public/pitch.svg"
            draggable="false"
            width="500"
            class="absolute cursor-copy z-10 top-0 bottom-0 left-0 right-0 m-auto"/>
    }
}
