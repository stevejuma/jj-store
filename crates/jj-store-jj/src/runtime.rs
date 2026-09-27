//! Enter the caller-owned I/O runtime for each poll, including JJ worker threads.
//! An enter guard must never survive an await or migrate between worker threads.
use futures::{StreamExt as _, stream::BoxStream};
use std::future::Future;
#[derive(Clone)]
pub(crate) struct RuntimeContext(tokio::runtime::Handle);
impl RuntimeContext {
    pub fn current() -> Result<Self, tokio::runtime::TryCurrentError> {
        Ok(Self(tokio::runtime::Handle::try_current()?))
    }
    pub fn enter(&self) -> tokio::runtime::EnterGuard<'_> {
        self.0.enter()
    }
    pub async fn run<F: Future>(&self, future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        std::future::poll_fn(|cx| {
            let _guard = self.enter();
            future.as_mut().poll(cx)
        })
        .await
    }
    pub fn stream<'a, T: Send + 'a>(&self, mut stream: BoxStream<'a, T>) -> BoxStream<'a, T> {
        let runtime = self.clone();
        futures::stream::poll_fn(move |cx| {
            let _guard = runtime.enter();
            stream.as_mut().poll_next(cx)
        })
        .boxed()
    }
}
