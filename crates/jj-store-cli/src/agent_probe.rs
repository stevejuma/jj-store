use crate::agent_integrations::{HarnessSupportV1, IntegrationStatusV1};
use jj_agent::IntegrationId;
use jj_store_jj::operation_stores::AuthorityResult;
use std::{
    io::{Read as _, Seek as _, SeekFrom},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const PROBE_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_VERSION_BYTES: u64 = 256;

pub(crate) fn probe(
    integration: IntegrationId,
    search: &[PathBuf],
) -> AuthorityResult<IntegrationStatusV1> {
    let expected = expected_version(integration).ok_or("unsupported integration probe")?;
    let Some(executable) = find_executable(integration, search) else {
        return Ok(IntegrationStatusV1::detected(
            integration,
            expected,
            None,
            None,
            HarnessSupportV1::Absent,
        ));
    };
    let Some(version) = run_version(&executable)? else {
        return Ok(IntegrationStatusV1::detected(
            integration,
            expected,
            Some(executable),
            None,
            HarnessSupportV1::ProbeFailed,
        ));
    };
    let expected = if integration == IntegrationId::Cursor && version == "2026.09.26-dd393fe" {
        "2026.09.26-dd393fe"
    } else {
        expected
    };
    let support = if version == expected {
        HarnessSupportV1::Supported
    } else {
        HarnessSupportV1::UnsupportedVersion
    };
    Ok(IntegrationStatusV1::detected(
        integration,
        expected,
        Some(executable),
        Some(version),
        support,
    ))
}

pub(crate) fn environment_search_paths() -> Vec<PathBuf> {
    std::env::var_os("PATH")
        .map(|value| std::env::split_paths(&value).collect())
        .unwrap_or_default()
}

fn expected_version(integration: IntegrationId) -> Option<&'static str> {
    match integration {
        IntegrationId::Codex => Some("0.156.1"),
        IntegrationId::Cursor => Some("3.22.7"),
        IntegrationId::OpenCode => Some("1.18.30"),
        IntegrationId::OwnedController => None,
    }
}

fn executable_name(integration: IntegrationId) -> Option<&'static str> {
    match integration {
        IntegrationId::Codex => Some("codex"),
        IntegrationId::Cursor => Some("cursor-agent"),
        IntegrationId::OpenCode => Some("opencode"),
        IntegrationId::OwnedController => None,
    }
}

fn find_executable(integration: IntegrationId, search: &[PathBuf]) -> Option<PathBuf> {
    let name = executable_name(integration)?;
    search.iter().find_map(|directory| {
        let candidate = directory.join(name);
        candidate
            .is_file()
            .then(|| candidate.canonicalize().ok())
            .flatten()
    })
}

fn run_version(executable: &Path) -> AuthorityResult<Option<String>> {
    let mut stdout = tempfile::tempfile()?;
    let stderr = tempfile::tempfile()?;
    let mut child = Command::new(executable)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout.try_clone()?))
        .stderr(Stdio::from(stderr))
        .spawn()?;
    let deadline = Instant::now() + PROBE_TIMEOUT;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            break None;
        }
        thread::sleep(Duration::from_millis(10));
    };
    if !status.is_some_and(|value| value.success()) {
        return Ok(None);
    }
    stdout.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    stdout.take(MAX_VERSION_BYTES + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_VERSION_BYTES {
        return Ok(None);
    }
    let Some(version) = std::str::from_utf8(&bytes)
        .ok()
        .and_then(|value| value.split_whitespace().last())
        .filter(|value| {
            !value.is_empty()
                && value.len() <= 128
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || b".-_+".contains(&byte))
        })
    else {
        return Ok(None);
    };
    Ok(Some(version.to_owned()))
}
