use crate::handler::msg::Message;
use once_cell::sync::OnceCell;
use std::process;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use tracing::error;

type ArcSender = Arc<Mutex<Sender<Message>>>;
type ArcReceiver = Arc<Mutex<Receiver<Message>>>;
static CHANNEL: OnceCell<(ArcSender, ArcReceiver)> = OnceCell::new();

pub fn get_sender() -> ArcSender {
    CHANNEL.get().expect("cannot get sender").0.clone()
}

pub fn get_receiver() -> ArcReceiver {
    CHANNEL.get().expect("cannot get receiver.").1.clone()
}

pub fn init_channels() {
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    if let Err(e) = CHANNEL.set((Arc::new(Mutex::new(sender)), Arc::new(Mutex::new(receiver)))) {
        error!("init channels failed: {:?}", e);
        process::exit(1);
    }
}
