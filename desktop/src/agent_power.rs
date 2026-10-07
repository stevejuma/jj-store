//! Bounded macOS idle-sleep inhibition owned by the desktop process.
use std::{
    process::{Child, Command, Stdio},
    sync::Mutex,
    time::{Duration, Instant},
};
#[derive(Default)]
pub struct AgentPower(Mutex<State>);
#[derive(Default)]
struct State {
    active: bool,
    activity: u64,
    child: Option<Child>,
    deadline: Option<Instant>,
}
impl Drop for State {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
impl AgentPower {
    pub fn release(&self) {
        if let Ok(mut state) = self.0.lock() {
            state.active = false;
            if let Some(mut child) = state.child.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}
#[tauri::command]
pub fn agent_power(
    state: tauri::State<'_, AgentPower>,
    active: bool,
    activity: u64,
) -> Result<(), String> {
    let mut state = state.0.lock().map_err(|_| "power state unavailable")?;
    let changed = activity > state.activity || (active && state.deadline.is_none());
    if active && changed {
        state.deadline = Some(Instant::now() + Duration::from_secs(3600));
    }
    state.activity = state.activity.max(activity);
    state.active = active;
    if !active || changed {
        if let Some(mut child) = state.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
    #[cfg(target_os = "macos")]
    if active
        && (changed || state.child.is_none())
        && let Some(remaining) = state
            .deadline
            .and_then(|deadline| deadline.checked_duration_since(Instant::now()))
    {
        let seconds = remaining.as_secs().max(1).to_string();
        state.child = Some(
            Command::new("/usr/bin/caffeinate")
                .args(["-i", "-t", &seconds])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|_| "sleep inhibition unavailable")?,
        );
    }
    Ok(())
}
