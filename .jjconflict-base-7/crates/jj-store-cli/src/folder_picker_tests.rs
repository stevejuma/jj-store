use super::*;

fn fixture(script: &str) -> Command {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", script]);
    command
}

#[tokio::test]
async fn selection_cancellation_and_unavailable_preserve_path_bytes() {
    let picker = NativePicker::default();
    let selected = picker
        .run(
            fixture("printf '/a folder/with trailing space \\n'"),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
    assert_eq!(
        selected.path.as_deref(),
        Some("/a folder/with trailing space ")
    );
    assert!(
        picker
            .run(fixture("printf '\\n'"), Duration::from_secs(2))
            .await
            .unwrap()
            .path
            .is_none()
    );
    assert_eq!(
        picker
            .run(fixture("exit 1"), Duration::from_secs(2))
            .await
            .unwrap_err(),
        PickerError::Unavailable
    );
    assert_eq!(
        picker
            .run(
                Command::new("/nonexistent-jj-store-picker"),
                Duration::from_secs(2)
            )
            .await
            .unwrap_err(),
        PickerError::Unavailable
    );
    for bytes in [b"relative\n".as_slice(), b"/nul\0\n", b"/bad\xff\n"] {
        assert_eq!(selection(bytes).unwrap_err(), PickerError::Invalid);
    }
    assert_eq!(
        selection(format!("/{}\n", "x".repeat(MAX_PATH_BYTES)).as_bytes()).unwrap_err(),
        PickerError::Invalid
    );
}

#[tokio::test]
async fn busy_timeout_and_cancellation_hold_ownership_until_reaped() {
    let picker = Arc::new(NativePicker::default());
    let owner = picker.run(fixture("exec /bin/sleep 30"), Duration::from_millis(50));
    tokio::pin!(owner);
    // Start the request without waiting for the helper to close.
    tokio::select! { biased; result = &mut owner => panic!("finished early: {result:?}"), _ = tokio::task::yield_now() => {} }
    assert_eq!(
        picker
            .run(fixture("exit 0"), Duration::from_secs(1))
            .await
            .unwrap_err(),
        PickerError::Busy
    );
    assert_eq!(owner.await.unwrap_err(), PickerError::Timeout);
    tokio::time::timeout(Duration::from_secs(2), async {
        let guard = picker.active.lock().await;
        drop(guard);
    })
    .await
    .unwrap();

    let directory = tempfile::tempdir().unwrap();
    let pid_file = directory.path().join("pid");
    let mut command = fixture("echo $$ > \"$1\"; exec /bin/sleep 30");
    command.args(["fixture", pid_file.to_str().unwrap()]);
    let copy = picker.clone();
    let request = tokio::spawn(async move { copy.run(command, Duration::from_secs(25)).await });
    let pid = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if let Ok(pid) = std::fs::read_to_string(&pid_file)
                && !pid.trim().is_empty()
            {
                break pid.trim().to_owned();
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    request.abort();
    let _ = request.await;
    let guard = tokio::time::timeout(Duration::from_secs(2), picker.active.lock())
        .await
        .unwrap();
    assert!(
        !Command::new("/bin/kill")
            .args(["-0", &pid])
            .stderr(Stdio::null())
            .status()
            .await
            .unwrap()
            .success()
    );
    drop(guard);
    assert!(
        picker
            .run(fixture("printf '/next\\n'"), Duration::from_secs(2))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn output_is_bounded_before_process_exit() {
    let picker = NativePicker::default();
    for script in [
        "while :; do printf 'xxxxxxxxxxxxxxxx'; done",
        "while :; do printf 'xxxxxxxxxxxxxxxx' >&2; done",
    ] {
        assert_eq!(
            picker
                .run(fixture(script), Duration::from_secs(2))
                .await
                .unwrap_err(),
            PickerError::Invalid
        );
    }
    assert!(
        picker
            .run(fixture("printf '/next\\n'"), Duration::from_secs(2))
            .await
            .is_ok()
    );
}
