use std::sync::mpsc::*;
use std::sync::Arc;

use crate::events::*;

/// Provides a generic, FIFO, MPSC event queue based on [`std::sync::mpsc`].
///
/// # Examples
///
/// To create an `EventQueue`, use [EventQueue::new()].  You must specify the event type you wish
/// to use, or allow Rust to figure it out based on usage.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::<EventType>::new();
/// ```
///
/// Events can be sent directly through [EventQueue::send()] if you have direct access to the
/// `EventQueue`.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::new();
/// event_queue.send(EventType::Event);
/// ```
///
/// The `EventQueue` itself cannot be sent across threads, so if you need to send events across
/// threads, you must create a [Sender] using [EventQueue::sender()].  A [Sender] can also be used
/// to send events from code which does not have direct access to the `EventQueue`.
///
/// ```
/// # use wolf_engine_core::events::*;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::new();
/// let event_sender = event_queue.sender();
///
/// std::thread::spawn(move || {
///     event_sender.send(EventType::Event).unwrap();
/// })
/// # .join()
/// # .unwrap();
/// ```
///
/// Queued events can be accessed by calling [EventQueue::flush()] this will clear all events from
/// the queue and return them in a collection which can be iterated over.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// # let event_queue = EventQueue::<i32>::new();
/// #
/// for event in event_queue.flush() {
///     // Handle events here.
/// }
/// ```
pub struct EventQueue<E> {
    sender: Sender<E>,
    receiver: Receiver<E>,
}

impl<E> EventQueue<E> {
    /// Creates a new event queue.
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }

    /// Send an event to the event queue.
    pub fn send(&self, event: E) {
        self.sender.send(event).unwrap();
    }

    /// Clears all events off the queue and returns them in a collection which can be iterated over.
    pub fn flush(&self) -> Vec<E> {
        self.receiver.try_iter().collect()
    }
}

impl<E: 'static> EventLoop<E> for EventQueue<E> {
    fn next_event(&mut self) -> Option<E> {
        self.receiver.try_recv().ok()
    }

    fn send_event(&self, event: E) {
        self.send(event)
    }
}

impl<E: 'static> HasEventSender<E> for EventQueue<E> {
    fn sender(&self) -> Arc<dyn EventSender<E>> {
        Arc::from(EventQueueSender::from(self.sender.clone()))
    }
}

impl<E> Default for EventQueue<E> {
    fn default() -> Self {
        Self::new()
    }
}

struct EventQueueSender<E> {
    inner: Sender<E>,
}

unsafe impl<E> Send for EventQueueSender<E> {}
unsafe impl<E> Sync for EventQueueSender<E> {}

impl<E> From<Sender<E>> for EventQueueSender<E> {
    fn from(sender: Sender<E>) -> Self {
        Self { inner: sender }
    }
}

impl<E> EventSender<E> for EventQueueSender<E> {
    fn send(&self, event: E) -> Result<(), String> {
        match self.inner.send(event) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod event_queue_tests {
    use std::thread;

    pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let event_queue = EventQueue::new();

        event_queue.send(0);
        let events = event_queue.flush();

        assert_eq!(events.get(0).expect("No event in the queue"), &0);
    }

    #[test]
    pub fn should_send_events_through_a_sender() {
        let event_queue = EventQueue::new();
        let sender = event_queue.sender();

        sender.send(0).unwrap();
        let thread_sender = sender.clone();
        thread::spawn(move || {
            thread_sender.send(1).unwrap();
        })
        .join()
        .unwrap();
        sender.send(2).unwrap();

        let events = event_queue.flush();
        assert_eq!(*events.get(0).unwrap(), 0);
        assert_eq!(*events.get(1).unwrap(), 1);
        assert_eq!(*events.get(2).unwrap(), 2);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let event_queue = EventQueue::<i32>::new();

        assert!(event_queue.flush().is_empty());
    }

    #[test]
    pub fn should_implement_default_trait() {
        let _event_queue = EventQueue::<i32>::default();
    }
}
