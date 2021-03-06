use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// Implemented by event structures that support resize event.
pub trait ResizeEvent {
    /// Creates a resize event.
    fn from_width_height(w: u32, h: u32) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize<U>(&self, f: |u32, u32| -> U) -> Option<U>;
}

impl<T: GenericEvent> ResizeEvent for T {
    #[inline(always)]
    fn from_width_height(w: u32, h: u32) -> Option<T> {
        let id = TypeId::of::<Box<ResizeEvent>>();
        Ptr::with_ref::<(u32, u32), Option<T>>(&(w, h), |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn resize<U>(&self, f: |u32, u32| -> U) -> Option<U> {
        let id = TypeId::of::<Box<ResizeEvent>>();
        self.with_event(id, |ptr| {
            let &(w, h) = ptr.expect::<(u32, u32)>();
            f(w, h)
        })
    }
}
