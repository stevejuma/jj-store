//! Read-only host facts, cached independently of model lifecycle.
use super::*;
pub(super) fn facts() -> Value {
    static FACTS: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    FACTS.get_or_init(|| {
        #[cfg(target_os="macos")]
        let memory=std::process::Command::new("/usr/sbin/sysctl").args(["-n","hw.memsize"]).output().ok().and_then(|v|String::from_utf8(v.stdout).ok()).and_then(|v|v.trim().parse::<u64>().ok());
        #[cfg(target_os="linux")]
        let memory=fs::read_to_string("/proc/meminfo").ok().and_then(|v|v.lines().find(|l|l.starts_with("MemTotal:")).and_then(|l|l.split_whitespace().nth(1)).and_then(|n|n.parse::<u64>().ok()).map(|n|n*1024));
        #[cfg(not(any(target_os="macos",target_os="linux")))]
        let memory:Option<u64>=None;
        json!({"os":std::env::consts::OS,"architecture":std::env::consts::ARCH,"cpu_threads":std::thread::available_parallelism().map_or(1,|n|n.get()),"memory_bytes":memory})
    }).clone()
}
