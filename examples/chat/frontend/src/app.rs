use shared::{DownMsg, Message, UpMsg};
use zoon::{eprintln, *};

pub mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn username() -> &'static Mutable<String> {
    Mutable::new("John".to_owned())
}

#[static_ref]
fn messages() -> &'static MutableVec<Message> {
    MutableVec::new()
}

#[static_ref]
fn new_message_text() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        messages().lock_mut().push_cloned(message);
        jump_to_bottom();
    })
}

#[static_ref]
fn received_messages_viewport_y() -> &'static Mutable<i32> {
    Mutable::new(0)
}

// ------ ------
//   Commands
// ------ ------

fn set_username(name: String) {
    username().set(name);
}

fn set_new_message_text(text: String) {
    new_message_text().set(text);
}

fn send_message() {
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                username: username().get_cloned(),
                text: new_message_text().take(),
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

pub async fn send_messages_forever() {
    let mut i = 0;
    loop {
        let mut messages_lock = messages().lock_mut();
        messages_lock.push_cloned(Message { username: username().get_cloned(), text: format!("hello {}", i) });
        jump_to_bottom();
        let l = messages_lock.len();
        let max = 200;
        if l > max {
            let excess = l - max;
            for _ in 0..excess {
                messages_lock.remove(0);
            }
        }
        drop(messages_lock);
        Timer::sleep(10).await;
        i += 1;
    }
}

fn jump_to_bottom() {
    received_messages_viewport_y().set(i32::MAX);
}
