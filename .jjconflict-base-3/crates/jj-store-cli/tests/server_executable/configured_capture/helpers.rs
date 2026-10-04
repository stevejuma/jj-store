use super::*;
use std::io::Write as _;
pub(super) async fn get(client: &reqwest::Client, origin: &str, path: &str) -> Value {
    request(client, origin, path, None).await
}
async fn request(client: &reqwest::Client, origin: &str, path: &str, body: Option<Value>) -> Value {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let url = format!("{origin}{path}");
            let request = if let Some(body) = &body {
                client.post(url).json(body)
            } else {
                client.get(url)
            };
            let response = request.send().await.unwrap();
            let status = response.status();
            let value: Value = response.json().await.unwrap();
            if status == reqwest::StatusCode::CONFLICT && value["code"] == "capture_control_busy" {
                tokio::time::sleep(Duration::from_millis(20)).await;
                continue;
            }
            assert_eq!(status, reqwest::StatusCode::OK, "{path}: {value}");
            break value;
        }
    })
    .await
    .unwrap()
}
pub(super) fn append(path: &Path, row: Value) {
    writeln!(
        fs::OpenOptions::new().append(true).open(path).unwrap(),
        "{row}"
    )
    .unwrap();
}
pub(super) async fn control(
    client: &reqwest::Client,
    origin: &str,
    generation: &str,
    revision: u64,
    paused: bool,
) {
    use jj_store_service::capture_management::{CaptureControlAction, CaptureControlCommand};
    let command = CaptureControlCommand {
        version: 1,
        generation: generation.into(),
        expected_revision: revision,
        request_id: jj_store_model::RequestId::new(if paused { [1; 16] } else { [2; 16] }),
        action: if paused {
            CaptureControlAction::Pause
        } else {
            CaptureControlAction::Resume
        },
    };
    let value = request(
        client,
        origin,
        "/v1/local/capture/control",
        Some(serde_json::to_value(command).unwrap()),
    )
    .await;
    assert_eq!(value["runtime_requested"], true);
}
pub(super) async fn wait_status(client: &reqwest::Client, origin: &str, field: &str) -> Value {
    tokio::time::timeout(Duration::from_secs(180), async {
        loop {
            let value = get(client, origin, "/v1/local/capture").await;
            if value["runtime"][field] == 1 {
                break value;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap()
}
