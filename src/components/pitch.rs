use leptos::*;
use types::Point;
use wasm_bindgen::{JsCast, JsValue};

const WIDTH: &str = "500";
const HEIGHT: &str = "332";

#[component]
pub fn Pitch(
    set_coordinate: WriteSignal<Point>,
    latest_start: ReadSignal<Point>,
    latest_end: ReadSignal<Point>,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    let handle_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let canvas = canvas_ref.get().unwrap();

        set_coordinate.update(|point| {
            point.reset();
            point.x = Some(ev.offset_x() * 100 / canvas.width() as i32);
            point.y = Some(ev.offset_y() * 100 / canvas.height() as i32);
        });
    };

    create_effect(move |_| {
        latest_start.track();
        latest_end.track();

        let canvas = canvas_ref.get().unwrap();

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        let ctx = canvas.get_context("2d").unwrap().unwrap();
        let ctx = ctx.unchecked_into::<web_sys::CanvasRenderingContext2d>();

        ctx.clear_rect(0., 0., width, height);

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
            on:mousedown=handle_click
            width=WIDTH
            height=HEIGHT
            class="absolute z-10 top-0 bottom-0 left-0 right-0 m-auto cursor-pointer"
        ></canvas>
        <img
            src="public/pitch.svg"
            draggable="false"
            width=WIDTH
            class="absolute z-5 top-0 bottom-0 left-0 right-0 m-auto"/>
    }
}
