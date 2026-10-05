use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, Response},
};
use jj_store_server::opencode_read_api::OpenCodeReadApi;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
pub struct Data {
    pub directory: String,
    pub messages: Vec<Value>,
    pub busy: bool,
    pub retry: bool,
}
pub struct Api {
    // Separate fixture consumers select either configured supervision or explicit acquisition.
    #[allow(dead_code)]
    pub endpoint: String,
    #[allow(dead_code)]
    pub api: OpenCodeReadApi,
    pub data: Arc<Mutex<Data>>,
    task: tokio::task::JoinHandle<()>,
}
impl Api {
    pub async fn new(directory: String, messages: Vec<Value>, busy: bool) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let api = OpenCodeReadApi::new(&endpoint).unwrap();
        let data = Arc::new(Mutex::new(Data {
            directory,
            messages,
            busy,
            retry: false,
        }));
        let app = Router::new().fallback(handle).with_state(data.clone());
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        Self {
            endpoint,
            api,
            data,
            task,
        }
    }
}
impl Drop for Api {
    fn drop(&mut self) {
        self.task.abort()
    }
}
async fn handle(State(data): State<Arc<Mutex<Data>>>, request: Request<Body>) -> Response<Body> {
    assert_eq!(request.method(), "GET");
    for field in ["authorization", "proxy-authorization", "cookie", "referer"] {
        assert!(!request.headers().contains_key(field));
    }
    let fields: std::collections::BTreeMap<_, _> =
        url::form_urlencoded::parse(request.uri().query().unwrap_or_default().as_bytes())
            .into_owned()
            .collect();
    let data = data.lock().unwrap();
    let mut response = Response::builder().header("content-type", "application/json");
    let value = match request.uri().path() {
        "/global/health" => json!({"healthy":true,"version":"1.18.30"}),
        "/project" => json!([{"id":"global","worktree":data.directory,"sandboxes":[]}]),
        "/session" => json!([{"directory":data.directory}]),
        "/session/ses_first" => {
            json!({"id":"ses_first","version":"1.18.30","directory":data.directory})
        }
        "/session/status" => {
            if data.retry {
                json!({"ses_first":{"type":"retry"}})
            } else if data.busy {
                json!({"ses_first":{"type":"busy"}})
            } else {
                json!({})
            }
        }
        "/session/ses_first/message" => {
            assert_eq!(fields.get("directory"), Some(&data.directory));
            assert_eq!(fields.get("limit").map(String::as_str), Some("32"));
            let page = fields
                .get("before")
                .map_or(0, |s| s.parse::<usize>().unwrap());
            let start = page * 2;
            let end = (start + 2).min(data.messages.len());
            if end < data.messages.len() {
                response = response.header("x-next-cursor", (page + 1).to_string());
            }
            json!(&data.messages[start.min(end)..end])
        }
        _ => panic!("unexpected native API request"),
    };
    response
        .body(Body::from(serde_json::to_vec(&value).unwrap()))
        .unwrap()
}
pub fn message(id: &str, directory: Option<&str>, text: &str) -> Value {
    json!({"info":{"id":id,"sessionID":"ses_first","role":"assistant","path":{"cwd":directory}},"parts":[{"id":format!("prt_{id}"),"sessionID":"ses_first","messageID":id,"type":"text","text":text}]})
}
