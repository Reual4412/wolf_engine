use std::sync::Arc;

use super::EventSenderProxy;

/// Provides the main event-loop functions used by the [`Engine`](crate::Engine).
pub trait EventLoop<E>: HasEventSenderProxy<E> {
    /// Returns the next event in the loop.
    fn next_event(&mut self) -> Option<E>;
}

/// A type from which an [EventSender] can be created.
pub trait HasEventSenderProxy<E> {
    /// Create a new [`EventSender`] from this `EventLoop`.
    fn sender(&self) -> Arc<dyn EventSenderProxy<E>>;
}
