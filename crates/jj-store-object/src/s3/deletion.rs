//! Explicit installation administration only; ObjectStore retains no LIST/DELETE port.
use super::*;
use jj_store_model::RepositoryId;
use serde::Deserialize;
use tokio::sync::OwnedRwLockReadGuard;

type Admission = OwnedRwLockReadGuard<()>;
impl S3Store {
    pub fn repository_has_unknown_writes(&self, repository: RepositoryId) -> bool {
        self.uncertain
            .read()
            .expect("storage uncertainty lock")
            .contains(&repository)
    }
    pub(super) async fn admit_repository(&self, key: &Key) -> Result<Admission> {
        let admission = self.lifecycle.clone().read_owned().await;
        if let Some(id) = key
            .as_str()
            .strip_prefix("repos/")
            .and_then(|s| s.split('/').next())
        {
            let id: RepositoryId = id
                .parse()
                .map_err(|_| Error::Invalid("repository namespace"))?;
            if self
                .deleted
                .read()
                .expect("repository deletion lock")
                .contains(&id)
            {
                return Err(Error::NotFound);
            }
        }
        Ok(admission)
    }
    /// Restore durable installation tombstones before binding any repository host.
    pub async fn fence_repository(&self, repository: RepositoryId) {
        self.deleted
            .write()
            .expect("repository deletion lock")
            .insert(repository);
        let _drained = self.lifecycle.write().await;
    }
    /// Versioned buckets need a separate version-erasure profile. Refuse before admission.
    pub async fn check_repository_deletion(&self) -> Result<()> {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("/{}", self.config.bucket));
        url.set_query(Some("versioning="));
        let response = self
            .signed_request(Method::GET, url, Sha256::digest([]).into(), BTreeMap::new())
            .send()
            .await
            .map_err(|_| Error::Transport)?;
        read_status(&response, StatusCode::OK)?;
        #[derive(Deserialize)]
        struct Versioning {
            #[serde(rename = "Status")]
            status: Option<String>,
        }
        let value: Versioning = xml(response).await?;
        if value.status.is_some() {
            return Err(Error::Invalid("versioned bucket deletion is unsupported"));
        }
        Ok(())
    }
    /// Erase at most sixteen keys under one typed repository namespace. The
    /// caller durably fences first and retries this idempotent batch after uncertainty.
    pub async fn delete_repository_batch(&self, repository: RepositoryId) -> Result<bool> {
        let _admission = self.lifecycle.write().await;
        if self.repository_has_unknown_writes(repository) {
            return Err(Error::Invalid(
                "uncertain repository writes require managed storage restart",
            ));
        }
        if !self
            .deleted
            .read()
            .expect("repository deletion lock")
            .contains(&repository)
        {
            return Err(Error::Invalid("repository is not fenced"));
        }
        let prefix = format!("repos/{repository}/");
        let mut url = self.endpoint.clone();
        url.set_path(&format!("/{}", self.config.bucket));
        url.set_query(Some(&format!(
            "list-type=2&max-keys=16&prefix=repos%2F{repository}%2F"
        )));
        let response = self
            .signed_request(Method::GET, url, Sha256::digest([]).into(), BTreeMap::new())
            .send()
            .await
            .map_err(|_| Error::Transport)?;
        read_status(&response, StatusCode::OK)?;
        let page: Listing = xml(response).await?;
        if page.contents.len() > 16 || page.contents.iter().any(|v| !v.key.starts_with(&prefix)) {
            return Err(Error::Response("foreign deletion key"));
        }
        if page.contents.is_empty() {
            return if page.truncated {
                Err(Error::Response("empty truncated deletion page"))
            } else {
                Ok(true)
            };
        }
        for object in page.contents {
            let key = Key::new(object.key)?;
            let response = self
                .request(
                    Method::DELETE,
                    &key,
                    Sha256::digest([]).into(),
                    BTreeMap::new(),
                )?
                .send()
                .await
                .map_err(|_| Error::Transport)?;
            if !matches!(
                response.status(),
                StatusCode::OK | StatusCode::NO_CONTENT | StatusCode::NOT_FOUND
            ) {
                return Err(Error::Rejected(response.status().as_u16()));
            }
        }
        // Require an independently observed empty page, including after the last batch.
        Ok(false)
    }
}
#[derive(Deserialize)]
struct Listing {
    #[serde(rename = "IsTruncated")]
    truncated: bool,
    #[serde(rename = "Contents", default)]
    contents: Vec<ListedObject>,
}
#[derive(Deserialize)]
struct ListedObject {
    #[serde(rename = "Key")]
    key: String,
}
async fn xml<T: serde::de::DeserializeOwned>(response: Response) -> Result<T> {
    let mut stream = response.bytes_stream();
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.try_next().await.map_err(|_| Error::Transport)? {
        if bytes.len().saturating_add(chunk.len()) > 128 * 1024 {
            return Err(Error::Limit);
        }
        bytes.extend_from_slice(&chunk);
    }
    quick_xml::de::from_reader(bytes.as_slice())
        .map_err(|_| Error::Response("invalid maintenance XML"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read as _, Write as _},
        net::TcpListener,
        sync::Arc,
    };
    fn store(address: std::net::SocketAddr) -> Arc<S3Store> {
        Arc::new(
            S3Store::new(Config {
                endpoint: format!("http://{address}"),
                bucket: "isolated-deletion-tests".into(),
                region: "us-east-1".into(),
                credentials: Credentials {
                    access_key: "synthetic".into(),
                    secret_key: "synthetic-secret".into(),
                    session_token: None,
                },
                allow_http_loopback: true,
                max_object_bytes: 1024,
                timeout: Duration::from_secs(5),
            })
            .unwrap(),
        )
    }
    #[tokio::test]
    async fn cancelling_a_put_cannot_release_its_fence_before_storage_acknowledges() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let storage = store(listener.local_addr().unwrap());
        let (entered_tx, entered_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let server = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = Vec::new();
            let mut buffer = [0; 4096];
            loop {
                let n = socket.read(&mut buffer).unwrap();
                assert_ne!(n, 0);
                request.extend_from_slice(&buffer[..n]);
                if let Some(end) = request.windows(4).position(|v| v == b"\r\n\r\n") {
                    if request.len() >= end + 4 + 3 {
                        break;
                    }
                }
            }
            entered_tx.send(()).unwrap();
            release_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            socket.write_all(b"HTTP/1.1 200 OK\r\nETag: \"acknowledged\"\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
        });
        let repository = RepositoryId::new([0xaa; 16]);
        let key = Key::new(format!("repos/{repository}/objects/test")).unwrap();
        let task = {
            let storage = storage.clone();
            let key = key.clone();
            tokio::spawn(async move {
                let upload = StagedUpload::from_bytes(b"abc", 1024).await.unwrap();
                storage.put(&key, Condition::Absent, &upload).await
            })
        };
        entered_rx.await.unwrap();
        task.abort();
        let _ = task.await;
        let mut fence = {
            let storage = storage.clone();
            tokio::spawn(async move {
                storage.fence_repository(repository).await;
            })
        };
        assert!(
            tokio::time::timeout(Duration::from_millis(30), &mut fence)
                .await
                .is_err()
        );
        release_tx.send(()).unwrap();
        fence.await.unwrap();
        server.join().unwrap();
        assert!(matches!(storage.head(&key).await, Err(Error::NotFound)));
        assert!(
            storage
                .admit_repository(
                    &Key::new(format!("repos/{}/keep", RepositoryId::new([0xab; 16]))).unwrap()
                )
                .await
                .is_ok()
        );
    }
    #[tokio::test]
    async fn a_foreign_listed_key_is_rejected_before_any_delete() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let storage = store(listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut buffer = [0; 4096];
            let n = socket.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            assert!(request.starts_with(
                "GET /isolated-deletion-tests?list-type=2&max-keys=16&prefix=repos%2F"
            ));
            let body = "<ListBucketResult><IsTruncated>false</IsTruncated><Contents><Key>repos/foreign/objects/data</Key></Contents></ListBucketResult>";
            write!(
                socket,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        });
        let repository = RepositoryId::new([0xaa; 16]);
        storage.fence_repository(repository).await;
        assert!(matches!(
            storage.delete_repository_batch(repository).await,
            Err(Error::Response("foreign deletion key"))
        ));
        server.join().unwrap();
    }
}

#[cfg(test)]
mod uncertainty_tests {
    use super::*;
    use std::{
        io::{Read as _, Write as _},
        net::TcpListener,
    };
    #[tokio::test]
    async fn unknown_put_outcomes_block_erasure_before_list_or_delete() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut bytes = Vec::new();
            let mut buf = [0; 4096];
            loop {
                let n = socket.read(&mut buf).unwrap();
                assert_ne!(n, 0);
                bytes.extend_from_slice(&buf[..n]);
                if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
                    if bytes.len() >= end + 7 {
                        break;
                    }
                }
            }
            socket.write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
        });
        let store = S3Store::new(Config {
            endpoint: format!("http://{address}"),
            bucket: "uncertain-deletion-test".into(),
            region: "us-east-1".into(),
            credentials: Credentials {
                access_key: "synthetic".into(),
                secret_key: "synthetic-secret".into(),
                session_token: None,
            },
            allow_http_loopback: true,
            max_object_bytes: 1024,
            timeout: Duration::from_secs(5),
        })
        .unwrap();
        let repository = RepositoryId::new([0xaa; 16]);
        let key = Key::new(format!("repos/{repository}/late-write")).unwrap();
        let upload = StagedUpload::from_bytes(b"abc", 1024).await.unwrap();
        assert_eq!(
            store.put(&key, Condition::Absent, &upload).await.unwrap(),
            WriteOutcome::Unknown
        );
        server.join().unwrap();
        store.fence_repository(repository).await;
        assert!(store.repository_has_unknown_writes(repository));
        assert!(matches!(
            store.delete_repository_batch(repository).await,
            Err(Error::Invalid(
                "uncertain repository writes require managed storage restart"
            ))
        ));
    }
}
