use bevy::prelude::*;
use bevy_channel_message::ChannelMessageSender;
use std::{thread, time::Duration};

#[derive(Message, Clone, Copy)]
struct MyMessage(i32);

fn main() {
    use bevy_channel_message::ChannelMessageApp;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // create channel
    app.add_channel_message::<MyMessage>();
    let sender = app
        .world()
        .get_resource::<ChannelMessageSender<MyMessage>>()
        .unwrap()
        .clone();

    // use sender from anywhere:
    thread::spawn(move || {
        let mut counter = 1;
        loop {
            // send events back to bevy
            sender.send(MyMessage(counter));
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
    });

    app.add_systems(Update, receiver_system);

    app.run();
}

fn receiver_system(mut reader: MessageReader<MyMessage>) {
    for event in reader.read() {
        println!("message with: {}", event.0);
    }
}
