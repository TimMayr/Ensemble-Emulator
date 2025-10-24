use crossbeam_channel::{Receiver, Sender};

#[cfg(feature = "frontend")]
use crate::app::imgui_frontend::ImguiFrontend;
use crate::app::{AppToEmuMessages, EmuToAppMessages};

#[cfg(feature = "frontend")]
pub enum Frontends {
    Imgui(ImguiFrontend),
    None(Sender<AppToEmuMessages>),
}

#[cfg(not(feature = "frontend"))]
pub enum Frontends {
    None(Sender<AppToEmuMessages>),
}

#[cfg(feature = "frontend")]
impl Frontend for Frontends {
    fn run(&mut self) {
        match self {
            Frontends::Imgui(frontend) => frontend.run(),
            Frontends::None(sender) => sender.send(AppToEmuMessages::Quit).unwrap(),
        }
    }

    fn set_message_sender(&mut self, sender: Sender<AppToEmuMessages>) {
        match self {
            Frontends::Imgui(frontend) => frontend.set_message_sender(sender),
            Frontends::None(_) => {}
        }
    }

    fn set_message_receiver(&mut self, receiver: Receiver<EmuToAppMessages>) {
        match self {
            Frontends::Imgui(frontend) => frontend.set_message_receiver(receiver),
            Frontends::None(_) => {}
        }
    }
}

#[cfg(not(feature = "frontend"))]
impl Frontend for Frontends {
    fn run(&mut self) {
        match self {
            Frontends::None(sender) => sender.send(AppToEmuMessages::Quit).unwrap(),
        }
    }

    fn set_message_sender(&mut self, _sender: Sender<AppToEmuMessages>) {}

    fn set_message_receiver(&mut self, _receiver: Receiver<EmuToAppMessages>) {}
}

pub trait Frontend {
    fn run(&mut self);
    fn set_message_sender(&mut self, sender: Sender<AppToEmuMessages>);
    fn set_message_receiver(&mut self, receiver: Receiver<EmuToAppMessages>);
}
