use zoon::*;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().update(|counter| counter + 1)
}

fn decrement() {
    counter().update(|counter| counter - 1)
}

const LAYER_INDEX_MASTER_ORDERING: &[&str] = &[
    "dropdown",
    "attachment",
];

fn get_layer_index(name: &str) -> i32 {
    LayerIndex::MAX_VALUE - LAYER_INDEX_MASTER_ORDERING.iter().position(|&n| n == name).map(|i| i as i32).unwrap_or(0)
}

#[static_ref]
fn show() -> &'static Mutable<bool> {
    Mutable::new(false)
}

fn root() -> impl IntoElementIterator {
    element_vec![
        {
            Button::new().label("+").on_press(clone!((show) move || show().set_neq(!show().get())))
            .s(Width::fill())
            .element_below_signal(
                show().signal()
                .map_true(||
                    El::new()
                    .s(LayerIndex::new(get_layer_index("dropdown")))
                    .update_raw_el(|raw_el| raw_el.style("transform", "translateX(100%)"))
                    .s(Width::percent(50))
                    .s(Height::exact(10))
                    .s(Background::new().color(named_color::PURPLE_5))
                )
            )
            .update_raw_el(|raw_el| raw_el.child(element_below_container_with_layer_index(get_layer_index("dropdown")).child_signal(
                show().signal()
                .map_true(||
                    El::new()
                    .s(Width::percent(50))
                    .s(Height::exact(10))
                    .s(Background::new().color(named_color::RED_5))
                )
            )))
        },
        {
            El::new()
            .s(Align::new().center_x())
            .s(Width::percent(50))
            .s(Height::exact(20))
            .s(Background::new().color(named_color::BLUE_5))
            .element_on_right_signal(
                show().signal()
                .map_true(||
                    El::new()
                    .s(LayerIndex::new(get_layer_index("attachment")))
                    .s(Align::center())
                    .s(Width::exact(20))
                    .s(Height::exact(20))
                    .s(Background::new().color(named_color::PINK_5))
                )
            )
            .update_raw_el(|raw_el| raw_el.child(element_on_left_container_with_layer_index(get_layer_index("attachment")).child_signal(
                show().signal()
                .map_true(||
                    El::new()
                    .s(Align::center())
                    .s(Width::exact(20))
                    .s(Height::exact(20))
                    .s(Background::new().color(named_color::YELLOW_5))
                )
            )))
        },
    ]
}

fn element_on_left_container_with_layer_index(index: i32) -> RawHtmlEl<web_sys::HtmlElement> {
    RawHtmlEl::new("div")
        .class("nearby_element_container")
        .class("on_left_custom")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("position", "absolute")
        .style("right", "100%")
        .style("top", "0")
        .style("height", "100%")
        .style("pointer-events", "none")
        .style("z-index", &index.to_string())
}

fn element_below_container_with_layer_index(index: i32) -> RawHtmlEl<web_sys::HtmlElement> {
    RawHtmlEl::new("div")
        .class("nearby_element_container")
        .class("below_custom")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("position", "absolute")
        .style("top", "100%")
        .style("left", "0")
        .style("width", "100%")
        .style("pointer-events", "none")
        .style("z-index", &index.to_string())
}

fn main() {
    start_app("app", root);
}
