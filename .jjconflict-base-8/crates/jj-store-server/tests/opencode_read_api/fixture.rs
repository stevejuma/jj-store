use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, Response},
};
use jj_store_server::opencode_read_api::OpenCodeReadApi;
use serde_json::json;
use std::sync::{Arc, Mutex};
#[derive(Clone, Copy)]
pub enum Mode {
    Healthy,
    WrongVersion,
    Redirect,
    Oversized,
    Rebound,
    Cycle,
}
struct Data {
    mode: Mode,
    requests: Mutex<Vec<String>>,
}
pub struct Fixture {
    pub api: OpenCodeReadApi,
    data: Arc<Data>,
    task: tokio::task::JoinHandle<()>,
}
impl Fixture {
    pub async fn start(mode: Mode) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let data = Arc::new(Data {
            mode,
            requests: Mutex::new(Vec::new()),
        });
        let app = Router::new().fallback(handle).with_state(data.clone());
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            api: OpenCodeReadApi::new(&endpoint).unwrap(),
            data,
            task,
        }
    }
    pub fn requests(&self) -> Vec<String> {
        self.data.requests.lock().unwrap().clone()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}
async fn handle(State(data): State<Arc<Data>>, request: Request<Body>) -> Response<Body> {
    assert_eq!(request.method(), "GET");
    for field in ["authorization", "proxy-authorization", "cookie", "referer"] {
        assert!(!request.headers().contains_key(field));
    }
    let path = request.uri().path().to_owned();
    let query = request.uri().query().unwrap_or_default().to_owned();
    let requests = {
        let mut rows = data.requests.lock().unwrap();
        rows.push(request.uri().to_string());
        rows.clone()
    };
    assert!(
        axum::body::to_bytes(request.into_body(), 1)
            .await
            .unwrap()
            .is_empty()
    );
    let mut response = Response::builder().header("content-type", "application/json");
    let body = match path.as_str() {
        "/global/health" => {
            if matches!(data.mode, Mode::Redirect) {
                return response
                    .status(302)
                    .header("location", "/forbidden")
                    .body(Body::empty())
                    .unwrap();
            }
            if matches!(data.mode, Mode::Oversized) {
                let chunks = vec![
                    Ok::<_, std::convert::Infallible>(vec![b'x'; 512 * 1024]),
                    Ok(vec![b'x'; 512 * 1024 + 1]),
                ];
                return response
                    .body(Body::from_stream(futures::stream::iter(chunks)))
                    .unwrap();
            }
            json!({"healthy":true,"version":if matches!(data.mode, Mode::WrongVersion) { "1.18.31" } else { "1.18.30" }})
        }
        "/session/ses_one" => {
            let count = requests
                .iter()
                .filter(|r| r.starts_with("/session/ses_one") && !r.contains("/message"))
                .count();
            json!({"id":"ses_one","version":"1.18.30","directory":if matches!(data.mode,Mode::Rebound) && count>1 { "/workspace/rebound" } else { "/workspace/one" }})
        }
        "/session/status" => {
            json!({"ses_one":{"type":"busy"},"ses_idle":{"type":"idle"},"ses_future":{"type":"future"},"invalid":{"type":"busy"}})
        }
        "/session/ses_one/message" => {
            let fields: std::collections::BTreeMap<_, _> =
                url::form_urlencoded::parse(query.as_bytes())
                    .into_owned()
                    .collect();
            assert_eq!(fields.get("limit").map(String::as_str), Some("32"));
            assert_eq!(
                fields.get("directory").map(String::as_str),
                Some("/workspace/one")
            );
            let old = fields.contains_key("before");
            if !old || matches!(data.mode, Mode::Cycle) {
                response = response
                    .header("x-next-cursor", "opaque:next/with?query=value")
                    .header("link", "<http://192.0.2.1/forbidden>; rel=next");
            }
            let id = if old { "msg_old" } else { "msg_new" };
            json!([{"info":{"id":id,"sessionID":"ses_one","role":"assistant","path":{"cwd":"/workspace/one","root":"/workspace/one"}},"parts":[{"id":"prt_one","sessionID":"ses_one","messageID":id,"type":"text","text":"Recorded native text"}]}])
        }
        _ => panic!("Unexpected route in controlled fixture"),
    };
    response
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap()
}
