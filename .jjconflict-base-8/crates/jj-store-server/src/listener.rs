//! Shared, bounded socket admission. TLS terminates here for HTTP/1.1 + h2 ALPN.
use crate::{Error, Result};
use futures::{Stream, StreamExt as _};
use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::{TcpListener, TcpStream},
    sync::{OwnedSemaphorePermit, Semaphore},
    time::{Instant, Sleep},
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self,
        pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
    },
};

pub(crate) fn tls(certificate: &[u8], key: &[u8]) -> Result<TlsAcceptor> {
    let certificates = CertificateDer::pem_slice_iter(certificate)
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| Error::Config("TLS certificate"))?;
    let key = PrivateKeyDer::from_pem_slice(key).map_err(|_| Error::Config("TLS private key"))?;
    let mut config = rustls::ServerConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|_| Error::Config("TLS protocols"))?
    .with_no_client_auth()
    .with_single_cert(certificates, key)
    .map_err(|_| Error::Config("TLS certificate or private key"))?;
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    Ok(TlsAcceptor::from(Arc::new(config)))
}
trait Socket: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Socket for T {}
pub(crate) struct Connection {
    socket: Box<dyn Socket>,
    _permit: OwnedSemaphorePermit,
    idle: Duration,
    timer: Pin<Box<Sleep>>,
}
impl Connection {
    fn new(socket: impl Socket + 'static, permit: OwnedSemaphorePermit, idle: Duration) -> Self {
        Self {
            socket: Box::new(socket),
            _permit: permit,
            idle,
            timer: Box::pin(tokio::time::sleep(idle)),
        }
    }
    fn deadline(&mut self, cx: &mut Context<'_>) -> io::Result<()> {
        use std::future::Future as _;
        if self.timer.as_mut().poll(cx).is_ready() {
            Err(io::Error::new(io::ErrorKind::TimedOut, "idle connection"))
        } else {
            Ok(())
        }
    }
    fn activity(&mut self) {
        self.timer.as_mut().reset(Instant::now() + self.idle);
    }
}
impl tonic::transport::server::Connected for Connection {
    type ConnectInfo = ();
    fn connect_info(&self) {}
}
impl AsyncRead for Connection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.deadline(cx)?;
        let before = buf.filled().len();
        let result = Pin::new(&mut self.socket).poll_read(cx, buf);
        if matches!(result, Poll::Ready(Ok(()))) && buf.filled().len() > before {
            self.activity();
        }
        result
    }
}
impl AsyncWrite for Connection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.deadline(cx)?;
        let result = Pin::new(&mut self.socket).poll_write(cx, bytes);
        if matches!(result, Poll::Ready(Ok(n)) if n > 0) {
            self.activity();
        }
        result
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.deadline(cx)?;
        Pin::new(&mut self.socket).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.socket).poll_shutdown(cx)
    }
}
async fn accept(
    socket: TcpStream,
    tls: Option<TlsAcceptor>,
    permit: OwnedSemaphorePermit,
    idle: Duration,
) -> io::Result<Connection> {
    socket.set_nodelay(true)?;
    if let Some(tls) = tls {
        let socket = tokio::time::timeout(Duration::from_secs(5), tls.accept(socket))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "TLS handshake"))??;
        Ok(Connection::new(socket, permit, idle))
    } else {
        Ok(Connection::new(socket, permit, idle))
    }
}
pub(crate) fn incoming(
    listener: TcpListener,
    tls: Option<TlsAcceptor>,
    capacity: usize,
    idle: Duration,
) -> impl Stream<Item = io::Result<Connection>> + Send + 'static {
    let slots = Arc::new(Semaphore::new(capacity));
    tokio_stream::wrappers::TcpListenerStream::new(listener)
        .filter_map(move |socket| {
            let admitted = socket.ok().zip(slots.clone().try_acquire_owned().ok());
            futures::future::ready(admitted)
        })
        .map(move |(socket, permit)| accept(socket, tls.clone(), permit, idle))
        .buffer_unordered(capacity.min(32))
        .filter_map(|result| futures::future::ready(result.ok().map(Ok)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

    #[tokio::test(start_paused = true)]
    async fn listener_idle_deadline_releases_owned_connection_slot() {
        let slots = Arc::new(Semaphore::new(1));
        let (socket, mut peer) = tokio::io::duplex(16);
        let mut connection = Connection::new(
            socket,
            slots.clone().try_acquire_owned().unwrap(),
            Duration::from_secs(10),
        );
        assert_eq!(slots.available_permits(), 0);
        tokio::time::advance(Duration::from_secs(9)).await;
        peer.write_all(b"x").await.unwrap();
        assert_eq!(connection.read_u8().await.unwrap(), b'x');
        tokio::time::advance(Duration::from_secs(9)).await;
        connection.write_all(b"y").await.unwrap();
        assert_eq!(peer.read_u8().await.unwrap(), b'y');
        tokio::time::advance(Duration::from_secs(11)).await;
        assert_eq!(
            connection.read_u8().await.unwrap_err().kind(),
            io::ErrorKind::TimedOut
        );
        drop(connection);
        assert_eq!(slots.available_permits(), 1);
    }

    #[tokio::test]
    async fn listener_connection_capacity_is_retained_until_drop() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let incoming = incoming(listener, None, 1, Duration::from_secs(30));
        futures::pin_mut!(incoming);
        let _first_client = TcpStream::connect(address).await.unwrap();
        let first = incoming.next().await.unwrap().unwrap();
        let mut excess = TcpStream::connect(address).await.unwrap();
        tokio::select! {
            _ = incoming.next() => panic!("admitted excess connection"),
            result = tokio::time::timeout(Duration::from_secs(2), excess.read_u8()) => {
                assert_eq!(result.unwrap().unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
            }
        }
        drop(first);
        let _next_client = TcpStream::connect(address).await.unwrap();
        assert!(
            tokio::time::timeout(Duration::from_secs(2), incoming.next())
                .await
                .unwrap()
                .unwrap()
                .is_ok()
        );
    }
}
