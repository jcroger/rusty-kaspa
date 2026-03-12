use std::{any::Any, sync::Arc};

pub trait AsArcAny: Any + Send + Sync + 'static {
    fn arc_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: Any + Send + Sync + 'static> AsArcAny for T {
    fn arc_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}