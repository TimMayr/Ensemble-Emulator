use crossbeam_channel::{Receiver, Sender};

#[cfg(feature = "frontend")]
use crate::app::imgui_frontend::ImguiFrontend;
use crate::app::{AppToEmuMessages, EmuToAppMessages};

pub enum Frontends {
    #[cfg(feature = "frontend")]
    Imgui(ImguiFrontend),
    None(Sender<AppToEmuMessages>),
}

impl Frontend for Frontends {
    fn run(&mut self) {
        match self {
            #[cfg(feature = "frontend")]
            Frontends::Imgui(frontend) => frontend.run(),
            Frontends::None(s) => s.send(AppToEmuMessages::Quit).unwrap(),
        }
    }

    fn set_message_sender(
        &mut self,
        #[cfg(not(feature = "frontend"))] _: Sender<AppToEmuMessages>,
        #[cfg(feature = "frontend")] sender: Sender<AppToEmuMessages>,
    ) {
        match self {
            #[cfg(feature = "frontend")]
            Frontends::Imgui(frontend) => frontend.set_message_sender(sender),
            Frontends::None(_) => {}
        }
    }

    fn set_message_receiver(
        &mut self,
        #[cfg(not(feature = "frontend"))] _: Receiver<EmuToAppMessages>,
        #[cfg(feature = "frontend")] receiver: Receiver<EmuToAppMessages>,
    ) {
        match self {
            #[cfg(feature = "frontend")]
            Frontends::Imgui(frontend) => frontend.set_message_receiver(receiver),
            Frontends::None(_) => {}
        }
    }
}

pub trait Frontend {
    fn run(&mut self);
    fn set_message_sender(&mut self, sender: Sender<AppToEmuMessages>);
    fn set_message_receiver(&mut self, receiver: Receiver<EmuToAppMessages>);
}
