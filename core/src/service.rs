use crate::arc_any::AsArcAny;
use crate::core::Core;
use std::{sync::Arc, thread::JoinHandle};

pub trait Service: AsArcAny {
    fn ident(self: Arc<Self>) -> &'static str;
    fn start(self: Arc<Self>, core: Arc<Core>) -> Vec<JoinHandle<()>>;
    fn stop(self: Arc<Self>);
}