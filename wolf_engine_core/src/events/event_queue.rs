use std::sync::mpsc::*;
use std::sync::Arc;

use crate::events::*;

/// Provides a generic, FIFO, MPSC event queue based on [`std::sync::mpsc`].
///
/// # Examples
///
/// To create an `EventQueue`, use [`EventQueue::new()`]. 
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::<EventType>::new();
/// ```
///
/// The `EventQueue` implements [`EventSender`], so you can send events with
/// [`EventSender::send_event()`] if you have direct access to the `EventQueue`.
///
/// ```
/// # use wolf_engine_core::events::*;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::new();
/// event_queue.send_event(EventType::Event);
/// ```
///
/// The `EventQueue` itself cannot be sent across threads, so you you must create a 
/// [`EventSenderProxy`] using [`HasEventSenderProxy::event_sender()`] to listen to events on other
/// threads.  An [`EventSenderProxy`] can also be used to send events from code which does not 
/// have direct access to the `EventQueue`.
///
/// ```
/// # use wolf_engine_core::events::*;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::new();
/// let event_sender = event_queue.event_sender();
/// std::thread::spawn(move || {
///     event_sender.send_event(EventType::Event).unwrap();
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
}

impl<E: 'static> EventLoop<E> for EventQueue<E> {
    fn next_event(&mut self) -> Option<E> {
        self.receiver.try_recv().ok()
    }
}

impl<E: 'static> EventSender<E> for EventQueue<E> {
    fn send_event(&self, event: E) -> Result<(), String> {
        match self.sender.send(event) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

impl<E: 'static> HasEventSenderProxy<E> for EventQueue<E> {
    fn event_sender(&self) -> Arc<dyn EventSenderProxy<E>> {
        Arc::from(EventQueueSenderProxy::from(self.sender.clone()))
    }
}

impl<E> Default for EventQueue<E> {
    fn default() -> Self {
        Self::new()
    }
}

struct EventQueueSenderProxy<E> {
    inner: Sender<E>,
}

unsafe impl<E> Send for EventQueueSenderProxy<E> {}
unsafe impl<E> Sync for EventQueueSenderProxy<E> {}

impl<E> From<Sender<E>> for EventQueueSenderProxy<E> {
    fn from(sender: Sender<E>) -> Self {
        Self { inner: sender }
    }
}

impl<E> EventSenderProxy<E> for EventQueueSenderProxy<E> {}
impl<E> EventSender<E> for EventQueueSenderProxy<E> {
    fn send_event(&self, event: E) -> Result<(), String> {
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
        let mut event_queue = EventQueue::new();

        event_queue.send_event(0).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue"), 0);
    }

    #[test]
    pub fn should_send_events_through_a_sender() {
        let mut event_queue = EventQueue::new();
        let sender = event_queue.event_sender();

        sender.send_event(0).unwrap();
        let thread_sender = sender.clone();
        thread::spawn(move || {
            thread_sender.send_event(1).unwrap();
        })
        .join()
        .unwrap();
        sender.send_event(2).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue."), 0);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 1);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 2);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let mut event_queue = EventQueue::<i32>::new();

        assert!(event_queue.next_event().is_none());
    }

    #[test]
    pub fn should_implement_default_trait() {
        let _event_queue = EventQueue::<i32>::default();
    }
}
