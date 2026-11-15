// Adapted from local_stacks/forge.rs: bounded child IO and environment custody.
use super::*;
use std::process::Stdio;
use tokio::{io::AsyncReadExt as _, process::Command};
impl RemoteTransfer {
    pub(super) async fn run(&self, args: &[&str]) -> Result<(Option<i32>, String)> {
        let _transport = binding::transport(&self.binding, false)?;
        self.binding.verify_saved()?;
        let mut command = Command::new("git");
        command.env_clear();
        for key in ["PATH", "SYSTEMROOT", "TMPDIR", "TEMP"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        command
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .args(["--no-pager", "--git-dir"])
            .arg(&self.git)
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "core.fsmonitor=false",
                "-c",
                "gc.auto=0",
                "-c",
                "maintenance.auto=false",
                "-c",
                "credential.helper=",
                "-c",
                "http.followRedirects=false",
                "-c",
                "protocol.ext.allow=never",
                "-c",
                if self.local {
                    "protocol.file.allow=always"
                } else {
                    "protocol.file.allow=never"
                },
            ]);
        if let Some(identity) = self.ssh {
            // Never honor repository/global SSH commands or disable known-host
            // verification. OpenSSH uses the executing user's existing keys.
            let command_line = match identity {
                HostSshIdentity::Agent => {
                    "ssh -F /dev/null -oBatchMode=yes -oStrictHostKeyChecking=yes -oPasswordAuthentication=no -oKbdInteractiveAuthentication=no -oIdentityFile=none"
                }
                HostSshIdentity::Default => {
                    "ssh -F /dev/null -oBatchMode=yes -oStrictHostKeyChecking=yes -oPasswordAuthentication=no -oKbdInteractiveAuthentication=no -oIdentityAgent=none -oIdentitiesOnly=yes"
                }
            };
            command
                .env("GIT_SSH_COMMAND", command_line)
                .env("GIT_SSH_VARIANT", "ssh");
            if identity == HostSshIdentity::Agent
                && let Some(socket) = std::env::var_os("SSH_AUTH_SOCK")
            {
                command.env("SSH_AUTH_SOCK", socket);
            }
        }
        if let Some(credential) = &self.credential {
            let endpoint = Url::parse(&self.endpoint).map_err(|_| Error::Binding)?;
            command
                .env("GIT_CONFIG_COUNT", "1")
                .env(
                    "GIT_CONFIG_KEY_0",
                    format!(
                        "http.{}.extraheader",
                        endpoint.as_str().trim_end_matches('/')
                    ),
                )
                .env("GIT_CONFIG_VALUE_0", credential.0.as_str());
        }
        command
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut child = command.spawn()?;
        let mut stdout = child.stdout.take().ok_or(Error::Binding)?;
        let result = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            let mut data = Vec::new();
            (&mut stdout)
                .take(1024 * 1024 + 1)
                .read_to_end(&mut data)
                .await?;
            if data.len() > 1024 * 1024 {
                return Err(Error::Binding);
            }
            let status = child.wait().await?;
            Ok((status.code(), String::from_utf8(data).map_err(jj)?))
        })
        .await
        .map_err(|_| {
            Error::Jj("Remote backup outcome is unknown; observe before retrying".into())
        })?;
        self.binding.verify_saved()?;
        result
    }
}
