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
        // JJ also polls these futures synchronously through pollster. That
        // executor cannot replenish Tokio's task-local cooperative budget.
        // The owned storage/query boundaries supply work/deadline/cancel limits.
        let mut future = std::pin::pin!(tokio::task::unconstrained(future));
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
            std::pin::pin!(tokio::task::unconstrained(stream.next())).poll(cx)
        })
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::noop_waker_ref;
    use std::task::{Context, Poll};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn native_runtime_future_progress_does_not_require_foreign_executor_cooperative_reset() {
        let runtime = RuntimeContext::current().unwrap();
        let mut future = Box::pin(runtime.run(async {
            for _ in 0..512 {
                tokio::task::consume_budget().await;
            }
            42
        }));
        let mut context = Context::from_waker(noop_waker_ref());
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(42));
        let (tx, rx) = tokio::sync::oneshot::channel();
        let mut pending = Box::pin(runtime.run(rx));
        assert!(pending.as_mut().poll(&mut context).is_pending());
        tx.send(17).unwrap();
        assert_eq!(pending.as_mut().poll(&mut context), Poll::Ready(Ok(17)));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn native_runtime_stream_progress_does_not_require_foreign_executor_cooperative_reset() {
        let runtime = RuntimeContext::current().unwrap();
        let source = futures::stream::unfold(0, |i| async move {
            tokio::task::consume_budget().await;
            Some((i, i + 1))
        })
        .boxed();
        let mut stream = runtime.stream(source);
        let mut context = Context::from_waker(noop_waker_ref());
        for i in 0..512 {
            assert_eq!(
                stream.as_mut().poll_next(&mut context),
                Poll::Ready(Some(i))
            );
        }
    }
}
