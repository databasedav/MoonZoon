use std::{collections::VecDeque, iter::FromIterator};
use zoon::{
    strum::{EnumIter, IntoEnumIterator, IntoStaticStr},
    *,
    RawEl,
    println,
};
use rust_hsluv;
use rand::{Rng, thread_rng};

// ------ ------
//    Types
// ------ ------

use LightState::*;

#[derive(Clone, Copy, IntoStaticStr, EnumIter)]
#[strum(crate = "strum")]
enum LightState {
    Stop,
    Ready,
    Go,
}

// ------ ------
//   States
// ------ ------

#[static_ref]
fn light_state() -> &'static Mutable<VecDeque<LightState>> {
    Mutable::new(VecDeque::from_iter(LightState::iter()))
}

// ------ ------
//   Signals
// ------ ------

fn color_attr_signal() -> impl Signal<Item = &'static str> {
    light_state().signal_ref(|light_state| match light_state[0] {
        Stop => "red",
        Ready => "yellow",
        Go => "green",
    })
}

fn cy_attr_signal() -> impl Signal<Item = &'static str> {
    light_state().signal_ref(|light_state| match light_state[0] {
        Stop => "50",
        Ready => "150",
        Go => "250",
    })
}

// ------ ------
//   Commands
// ------ ------

fn next_light_state() {
    light_state().lock_mut().rotate_left(1);
}

// ------ ------
//     View
// ------ ------

fn triangle_height(side: f32) -> f32 {
    3.0_f32.sqrt() / 2.0 * side
}

fn triangle_side(height: f32) -> f32 {
    2.0 * height / 3.0_f32.sqrt()
}

fn triangle(side: f32, color: HSLuv) -> impl Element + Styleable<'static> {
    let height = triangle_height(side).to_string();
    El::new()
    // .s(Height::growable())
    // .s(Width::growable())
    // .s(Height::exact(triangle_height(side) as u32))
    // .s(Width::exact(side as u32))
    .child(
        RawSvgEl::new("svg")
        // .after_insert(|el| {
        //     Task::start(async move {
        //         for _ in 0..10 { Task::next_macro_tick().await; }
        //         let bbox = el.get_bounding_client_rect();
        //         println!("bbox: {} {} {} {}", bbox.x(), bbox.y(), bbox.width(), bbox.height());
        //         // el.set_attribute("viewBox", &format!("{} {} {} {}", bbox.x(), bbox.y(), bbox.width(), bbox.height()));
        //         el.set_attribute("viewBox", &format!("0 0 {} {}", bbox.width(), bbox.height()));
        //     })
        // })
        .attr("width", &side.to_string())
        .attr("height", &height)
        .child(
            RawSvgEl::new("polygon")
            // .attr("viewBox", &format!("0 0 {} {}", side, &height))
            // .after_insert(|el| {
            //     let bbox = el.get_bounding_client_rect();
            //     el.set_attribute("viewBox", &format!("0 0 {} {}", bbox.width(), bbox.height()));
            // })
            .attr(
                "points",
                &format!("{} 0, 0 {}, {} {}", side / 2.0, &height, side, &height),
            )
            .style("fill", &color.to_string())
        )
    )
}

fn hsluv_from_hex(hex: &str) -> HSLuv {
    // TODO: this conversion should be fallible
    let (h, s, l) = rust_hsluv::hex_to_hsluv(&hex);
    HSLuv::hsl(h, s, l)
}

fn breathify(el: RawSvgEl<web_sys::SvgElement>) -> RawSvgEl<web_sys::SvgElement> {
    let oscillator = Oscillator::new(Duration::milliseconds(500));
    oscillator.jump_to(js_sys::Math::random());
    oscillator.cycle();
    RawSvgEl::new("g")
    .attr("transform-origin", "center")
    .attr_signal("transform", oscillator.signal().map(|oscillation| format!("scale({})", 1. + (oscillation * 0.035))))
    .child(el)
}

fn beatify(el: RawSvgEl<web_sys::SvgElement>) -> RawSvgEl<web_sys::SvgElement> {
    let oscillator = Oscillator::new(Duration::milliseconds(1000));
    oscillator.cycle();
    let scale = Mutable::new(false);
    let task = Task::start_droppable(clone!((scale) async move {
        oscillator.signal().map(|oscillation| oscillation > 0.9).dedupe()
        .for_each_sync(|beating| {
            if beating {
                scale.set_neq(true);
                Task::start(clone!((scale) async move {
                    Timer::sleep(150).await;
                    scale.set_neq(false);
                    Timer::sleep(150).await;
                    scale.set_neq(true);
                    Timer::sleep(150).await;
                    scale.set_neq(false);
                }));
            }
        })
        .await;
    }));
    RawSvgEl::new("g")
    .attr("height", "150")
    .attr("width", "100")
    .after_remove(move |_| drop(task))
    .attr_signal("transform", scale.signal().map_true(|| format!("scale(1.05)")))
    .attr("transform-origin", "center")
    .child(el)
}

fn root() -> impl Element {
    let side = 40.;
    let height = triangle_height(side);

    El::new()
    .s(Align::center())
    .child(
        RawSvgEl::new("svg")
        .attr("height", "150")
        .attr("width", "100")
        .child(
            RawSvgEl::new("g")
            .child(
                RawSvgEl::new("polygon")
                .attr("points", &format!("0,0, {},0, {},{}", height * 2.0, height, side / 2.0))
                .attr("fill", "#e6e6e6")
                .apply(breathify)
            )
            .child(
                RawSvgEl::new("polygon")
                .attr("points", &format!("0,0, {},{}, {},100, 0,80", height, side / 2.0, height))
                .attr("fill", "#cccccc")
                .apply(breathify)
            )
            .child(
                RawSvgEl::new("g")
                .attr("transform", &format!("translate({})", height))
                .child(
                    RawSvgEl::new("g")
                    .attr("fill", "#cccccc")
                    .child(
                        RawSvgEl::new("polygon")
                        .attr("id", "triangle")
                        .attr("transform", &format!("rotate(-90 {} {})", side / 2.0, side / 2.0))
                        .attr("points", &format!("{} 0, 0 {}, {} {}", side / 2.0, &height, side, &height))
                    )
                    .apply(breathify)
                )
                .child(
                    RawSvgEl::new("use")
                    .attr("href", "#triangle")
                    .attr("transform", &format!("rotate(180 {} {}) translate(0, -20)", height / 2.0, side / 2.0))
                    .attr("fill", "#999999")
                    .apply(breathify)
                )
                .child(
                    RawSvgEl::new("polygon")
                    .attr("transform", "translate(0, 60)")
                    .attr("points", &format!("0,0, {},-20, {},40, 0,40", height, height))
                    .attr("fill", "#666666")
                    .apply(breathify)
                )
            )
            .apply(beatify)
        )
    )
}

// ------ ------
//    Start
// ------ ------

fn main() {
    start_app("app", root);
}

