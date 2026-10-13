//! Protect the latch-before-wake/register-before-consume edge protocol.
use super::*;
use core::task::Waker;

#[test]
fn edge_before_or_between_polls_is_consumed_once() {
    // This channel is private to this host regression; no MMIO is accessed.
    let channel = MAX_CHANNEL;
    take_edge(channel);
    note_edge(channel);
    let mut cx = Context::from_waker(Waker::noop());
    {
        let mut future = core::pin::pin!(WaitEdge { pin: channel });
        assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(()));
    }
    assert!(!take_edge(channel));
    let mut future = core::pin::pin!(WaitEdge { pin: channel });
    assert_eq!(future.as_mut().poll(&mut cx), Poll::Pending);
    note_edge(channel);
    assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(()));
    assert!(!take_edge(channel));
}
