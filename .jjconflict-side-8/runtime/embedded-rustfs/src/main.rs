#![forbid(unsafe_code)]
#![allow(linker_messages)]
use rustfs::embedded::RustFSServerBuilder;
use std::io::{Read as _, Write as _};

fn required_environment(name: &str) -> Result<String, &'static str> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.is_empty() && value.len() <= 4096)
        .ok_or("embedded RustFS credential unavailable")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()?
        .block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let address = args.next().ok_or("missing address")?;
    let volume = args.next().ok_or("missing volume")?;
    let region = args.next().ok_or("missing region")?;
    if args.next().is_some() {
        return Err("unexpected embedded RustFS argument".into());
    }
    let address = address.into_string().map_err(|_| "invalid address")?;
    let volume = volume.into_string().map_err(|_| "invalid volume")?;
    let region = region.into_string().map_err(|_| "invalid region")?;
    let server = RustFSServerBuilder::new()
        .address(address)
        .volume(volume)
        .region(region)
        .access_key(required_environment("JJ_STORE_INTERNAL_RUSTFS_ACCESS_KEY")?)
        .secret_key(required_environment("JJ_STORE_INTERNAL_RUSTFS_SECRET_KEY")?)
        .build()
        .await?;
    println!("READY");
    std::io::stdout().flush()?;
    tokio::task::spawn_blocking(|| {
        let mut input = [0_u8; 16];
        let _ = std::io::stdin().read(&mut input);
    })
    .await?;
    server.shutdown().await;
    Ok(())
}
