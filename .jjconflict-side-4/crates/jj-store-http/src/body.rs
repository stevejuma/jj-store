//! Keep response admission through delivery, including an unpolled body deadline.
use axum::body::{Body, Bytes};
use http_body::{Body as HttpBody, Frame, SizeHint};
use std::{
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
use tokio::{sync::OwnedSemaphorePermit, time::Instant};
struct State {
    body: Option<Body>,
    permit: Option<OwnedSemaphorePermit>,
    expired: bool,
    waker: Option<Waker>,
}
pub(crate) struct Guard {
    state: Arc<Mutex<State>>,
    timer: tokio::task::JoinHandle<()>,
}
impl Guard {
    pub fn new(body: Body, permit: OwnedSemaphorePermit, deadline: Instant) -> Self {
        let state = Arc::new(Mutex::new(State {
            body: Some(body),
            permit: Some(permit),
            expired: false,
            waker: None,
        }));
        let weak = Arc::downgrade(&state);
        let timer = tokio::spawn(async move {
            tokio::time::sleep_until(deadline).await;
            if let Some(state) = weak.upgrade() {
                let mut state = state.lock().expect("HTTP body");
                state.body.take();
                state.permit.take();
                state.expired = true;
                let waker = state.waker.take();
                drop(state);
                if let Some(waker) = waker {
                    waker.wake();
                }
            }
        });
        Self { state, timer }
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        self.timer.abort();
    }
}
impl HttpBody for Guard {
    type Data = Bytes;
    type Error = io::Error;
    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, io::Error>>> {
        let mut state = self.state.lock().expect("HTTP body");
        if state.expired {
            state.expired = false;
            return Poll::Ready(Some(Err(io::Error::other("HTTP response deadline"))));
        }
        state.waker = Some(cx.waker().clone());
        let Some(body) = state.body.as_mut() else {
            return Poll::Ready(None);
        };
        match Pin::new(body).poll_frame(cx) {
            Poll::Ready(None) => {
                state.body.take();
                state.permit.take();
                self.timer.abort();
                Poll::Ready(None)
            }
            Poll::Ready(Some(Err(_))) => {
                state.body.take();
                state.permit.take();
                self.timer.abort();
                Poll::Ready(Some(Err(io::Error::other("HTTP response failed"))))
            }
            Poll::Ready(Some(Ok(frame))) => Poll::Ready(Some(Ok(frame))),
            Poll::Pending => Poll::Pending,
        }
    }
    fn size_hint(&self) -> SizeHint {
        self.state
            .lock()
            .expect("HTTP body")
            .body
            .as_ref()
            .map_or_else(SizeHint::default, HttpBody::size_hint)
    }
}
