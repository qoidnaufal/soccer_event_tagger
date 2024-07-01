use leptos::*;
use types::Point;
use wasm_bindgen::{JsCast, JsValue};

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

#[component]
pub fn Bullet(latest_start: ReadSignal<Point>, latest_end: ReadSignal<Point>) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    create_effect(move |_| {
        let canvas = canvas_ref.get().unwrap();

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        let ctx = canvas.get_context("2d").unwrap().unwrap();
        let ctx = ctx.unchecked_into::<web_sys::CanvasRenderingContext2d>();

        ctx.reset();

        if latest_start.get().x.is_some() {
            let x_start = latest_start.get().x.unwrap_or_default() as f64 * width / 100.;
            let y_start = latest_start.get().y.unwrap_or_default() as f64 * height / 100.;

            ctx.begin_path();
            ctx.arc(x_start, y_start, 5., 0., 2. * std::f64::consts::PI)
                .unwrap();
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill();
            ctx.stroke();

            if latest_end.get().x.is_some() {
                let x_end = latest_end.get().x.unwrap_or_default() as f64 * width / 100.;
                let y_end = latest_end.get().y.unwrap_or_default() as f64 * height / 100.;

                ctx.begin_path();
                ctx.arc(x_end, y_end, 5., 0., 2. * std::f64::consts::PI)
                    .unwrap();
                ctx.set_fill_style(&JsValue::from_str("red"));
                ctx.fill();
                ctx.stroke();
            }
        }
    });

    view! {
        <canvas
            _ref=canvas_ref
            width="500"
            height="332"
            class="absolute z-5 top-0 bottom-0 left-0 right-0 m-auto bg-slate-200/[.40]"
        >
        </canvas>
    }
}
