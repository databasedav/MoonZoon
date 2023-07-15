use zoon::*;

mod app;
mod markup;

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", app::view::root);
    Task::start(app::send_messages_forever());
    app::connection();
}
