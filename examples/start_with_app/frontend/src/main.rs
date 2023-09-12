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

fn root() -> impl IntoElementIterator {
    element_vec![
        Button::new().label("-").on_press(decrement),
        Text::with_signal(counter().signal()),
        Button::new().label("+").on_press(increment),
        Row::new()
        .item_signal(
            always(true)
            .map_true(|| {
                // Task::start(async{});
                El::new()
                .s(Height::fill())
                .s(Width::exact_signal(always(1)))
            })
        )
    ]
}

async fn startup() {
    async {
        window().open_with_url("https://google.com");
    }
    .await;
}

fn main() {
    start_app("app", root);
    Task::start(startup());
}
