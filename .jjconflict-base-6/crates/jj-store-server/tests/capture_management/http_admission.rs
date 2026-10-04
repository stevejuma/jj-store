use super::*;
use reqwest::{Client, Method, StatusCode};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn installation_management_enforces_transport_admin_and_bounded_input() {
    let f = Fixture::new(true).await;
    let bytes = f.bytes();
    let client = Client::new();
    let admin = jj_store_http::Access::LocalTrusted {
        principal: f.actor,
        permission: Permission::Admin,
    };
    for access in [
        jj_store_http::Access::Authenticated,
        jj_store_http::Access::LocalTrusted {
            principal: f.actor,
            permission: Permission::Read,
        },
        jj_store_http::Access::LocalTrusted {
            principal: f.actor,
            permission: Permission::Write,
        },
        admin,
    ] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let admitted = matches!(
            access,
            jj_store_http::Access::LocalTrusted {
                permission: Permission::Admin,
                ..
            }
        );
        let api = jj_store_http::HttpApi::new(
            f.source.host.clone(),
            f.source.root.path().join("admission-reads"),
            jj_store_http::Config {
                origin: base.clone(),
                access,
                local_capture: Some(Arc::new(f.manager.clone())),
                ..Default::default()
            },
        )
        .unwrap();
        let (stop, stopped) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, api.router())
                .with_graceful_shutdown(async {
                    let _ = stopped.await;
                })
                .await
                .unwrap();
        });
        for (method, path) in [
            (Method::GET, "/v1/local/capture"),
            (Method::GET, "/v1/local/capture/sources"),
            (Method::POST, "/v1/local/capture/control"),
            (Method::POST, "/v1/local/capture/rescan"),
        ] {
            if !admitted {
                let mut request = client
                    .request(method.clone(), format!("{base}{path}"))
                    .header("x-jj-store-protocol", "1");
                if method == Method::POST {
                    request = request.header("content-type", "application/json").body(
                        serde_json::to_vec(&f.command(1, 0, CaptureControlAction::Pause)).unwrap(),
                    );
                }
                assert_eq!(
                    request.send().await.unwrap().status(),
                    StatusCode::FORBIDDEN
                );
            } else {
                for (name, value) in [
                    ("x-jj-store-protocol", None),
                    ("host", Some("foreign.example")),
                    ("authorization", Some("Bearer cannot-lend-admin")),
                    ("sec-fetch-site", Some("cross-site")),
                    ("x-http-method-override", Some("POST")),
                ] {
                    let mut request = client.request(method.clone(), format!("{base}{path}"));
                    if name != "x-jj-store-protocol" {
                        request = request.header("x-jj-store-protocol", "1");
                    }
                    if let Some(value) = value {
                        request = request.header(name, value);
                    }
                    if method == Method::POST {
                        request = request.header("content-type", "application/json").body(
                            serde_json::to_vec(&f.command(1, 0, CaptureControlAction::Pause))
                                .unwrap(),
                        );
                    }
                    assert_eq!(
                        request.send().await.unwrap().status(),
                        StatusCode::FORBIDDEN,
                        "{method} {name}"
                    );
                }
            }
        }
        if admitted {
            let post = || {
                client
                    .post(format!("{base}/v1/local/capture/control"))
                    .header("x-jj-store-protocol", "1")
                    .header("content-type", "application/json")
            };
            let mut body =
                serde_json::to_vec(&f.command(1, 0, CaptureControlAction::Pause)).unwrap();
            body.resize(4097, b' ');
            assert_eq!(
                post().body(body).send().await.unwrap().status(),
                StatusCode::PAYLOAD_TOO_LARGE
            );
            for field in ["version", "request_id", "generation"] {
                let mut invalid =
                    serde_json::to_value(f.command(1, 0, CaptureControlAction::Pause)).unwrap();
                invalid.as_object_mut().unwrap().remove(field);
                assert_eq!(
                    post()
                        .body(serde_json::to_vec(&invalid).unwrap())
                        .send()
                        .await
                        .unwrap()
                        .status(),
                    StatusCode::BAD_REQUEST
                );
            }
            let mut foreign = f.command(1, 0, CaptureControlAction::Pause);
            foreign.generation = "ff".repeat(16);
            assert_eq!(
                post()
                    .body(serde_json::to_vec(&foreign).unwrap())
                    .send()
                    .await
                    .unwrap()
                    .status(),
                StatusCode::CONFLICT
            );
        }
        let _ = stop.send(());
        task.await.unwrap();
        assert_eq!(f.bytes(), bytes);
        assert!(f.source.spool.pending().unwrap().is_empty());
    }
}
