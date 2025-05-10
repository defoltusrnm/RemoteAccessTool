use std::sync::mpsc::{Receiver, TryRecvError};

use futures::Stream;

pub trait IntoStream<T: Send + 'static> {
    fn into_stream(self) -> impl Stream<Item = T>;
}

impl<T: Send + 'static> IntoStream<T> for Receiver<T> {
    fn into_stream(self) -> impl Stream<Item = T> {
        futures::stream::poll_fn(move |cx| match self.try_recv() {
            Ok(item) => std::task::Poll::Ready(Some(item)),
            Err(TryRecvError::Empty) => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(TryRecvError::Disconnected) => std::task::Poll::Ready(None),
        })
    }
}
