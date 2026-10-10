//! Only acknowledged adapter turns establish a resumable persisted conversation.
use super::*;

pub(super) fn load_session(saved: &SavedSession) -> Option<String> {
    if saved.view.capabilities["initialize"]["loadSession"] != true {
        return None;
    }
    let wanted = saved.view.adapter_session_id.as_deref()?;
    let mut opened = None;
    for event in &saved.events {
        let payload = &event.payload;
        match payload["type"].as_str() {
            Some("opened") => opened = payload["adapterSessionId"].as_str(),
            Some("started") => {
                // Older fixtures/captures omit the redundant adapter identity.
                let adapter = payload["adapterSessionId"].as_str().or(opened);
                if adapter == Some(wanted) {
                    return Some(wanted.to_owned());
                }
            }
            _ => {}
        }
    }
    None
}
