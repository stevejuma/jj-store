use super::*;
#[tokio::test]
async fn stop_signal_is_delivered_when_the_command_queue_is_full() {
    let (sender, _commands) = mpsc::channel(1);
    sender.try_send(json!({"type":"prompt"})).unwrap();
    assert_eq!(sender.capacity(), 0);
    let (stop, mut stopping) = tokio::sync::watch::channel(false);
    let mut worker = Worker {
        configuration: String::new(),
        inference_host: None,
        sender,
        stop,
        state: "idle".into(),
        generation: 1,
        error: None,
        last_used: tokio::time::Instant::now(),
    };
    worker.request_stop();
    tokio::time::timeout(Duration::from_millis(100), stopping.changed())
        .await
        .unwrap()
        .unwrap();
    assert!(*stopping.borrow());
    assert_eq!(worker.state, "stopping");
    assert_eq!(worker.sender.capacity(), 0);
}
