use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError};

#[derive(Resource, Clone, Debug)]
pub struct ChannelMessageSender<T: Message>(Sender<T>);

impl<T: Message> ChannelMessageSender<T> {
    pub fn send(&self, event: impl Into<T>) {
        let event = event.into();
        if let Err(err) = self.0.try_send(event) {
            match err {
                // we have an unbounded channel, so this would only happen if we're out of memory
                TrySendError::Full(_) => panic!("unable to send event, channel full"),
                // This should only happen if callbacks happen as the app is shutting down, so we ignore it
                TrySendError::Disconnected(_) => {}
            }
        };
    }
}

#[derive(Resource)]
struct ChannelMessageReceiver<T: Message>(Receiver<T>);

pub trait ChannelMessageApp {
    fn add_channel_message<T: Message>(&mut self) -> &mut Self;
}

impl ChannelMessageApp for App {
    fn add_channel_message<T: Message>(&mut self) -> &mut Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        self.insert_resource(ChannelMessageSender::<T>(sender));
        self.insert_resource(ChannelMessageReceiver::<T>(receiver));
        self.add_message::<T>();
        self.add_systems(PreUpdate, process_channel_messages::<T>);
        self
    }
}

fn process_channel_messages<T: Message>(
    receiver: Res<ChannelMessageReceiver<T>>,
    mut events: MessageWriter<T>,
) {
    loop {
        match receiver.0.try_recv() {
            Ok(msg) => {
                events.write(msg);
            }
            Err(TryRecvError::Disconnected) => {
                //TODO: handle this better
                panic!("sender resource dropped")
            }
            Err(TryRecvError::Empty) => {
                break;
            }
        }
    }
}
