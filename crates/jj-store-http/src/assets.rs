//! Preloaded, allowlisted browser files. Request paths never reach the filesystem.
use axum::{
    Router,
    body::{Body, Bytes},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io::Read, path::Path};

#[derive(Clone)]
pub struct BrowserAssets(BTreeMap<String, (Bytes, &'static str)>);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u32,
    files: Vec<String>,
}
fn regular(path: &Path, limit: u64) -> Result<Vec<u8>, &'static str> {
    let metadata = fs::symlink_metadata(path).map_err(|_| "browser asset unavailable")?;
    if !metadata.is_file() || metadata.len() > limit {
        return Err("browser asset type or size");
    }
    let mut bytes = vec![];
    fs::File::open(path)
        .map_err(|_| "browser asset unavailable")?
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "browser asset read")?;
    if bytes.len() as u64 > limit {
        return Err("browser asset size");
    }
    Ok(bytes)
}
impl BrowserAssets {
    pub fn load(root: &Path) -> Result<Self, &'static str> {
        if !fs::symlink_metadata(root)
            .map_err(|_| "browser root unavailable")?
            .is_dir()
        {
            return Err("browser root type");
        }
        let root = root
            .canonicalize()
            .map_err(|_| "browser root unavailable")?;
        let manifest: Manifest =
            serde_json::from_slice(&regular(&root.join("asset-manifest.json"), 16384)?)
                .map_err(|_| "browser manifest")?;
        if manifest.version != 1 || manifest.files.is_empty() || manifest.files.len() > 64 {
            return Err("browser manifest limits");
        }
        let mut files = BTreeMap::new();
        let mut total = 0;
        for name in manifest.files {
            let media = if name == "index.html" {
                "text/html; charset=utf-8"
            } else {
                let file = name.strip_prefix("assets/").ok_or("browser asset path")?;
                if file.is_empty()
                    || !file
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
                    || file.contains("..")
                {
                    return Err("browser asset path");
                }
                if !fs::symlink_metadata(root.join("assets"))
                    .map_err(|_| "browser asset directory")?
                    .is_dir()
                {
                    return Err("browser asset directory");
                }
                if file.ends_with(".js") {
                    "text/javascript; charset=utf-8"
                } else if file.ends_with(".css") {
                    "text/css; charset=utf-8"
                } else if file.ends_with(".woff2") {
                    "font/woff2"
                } else {
                    return Err("browser asset media");
                }
            };
            let bytes = regular(&root.join(&name), 8 * 1024 * 1024)?;
            total += bytes.len();
            if total > 32 * 1024 * 1024 {
                return Err("browser bundle size");
            }
            let route = if name == "index.html" {
                "/".into()
            } else {
                format!("/{name}")
            };
            if files.insert(route, (Bytes::from(bytes), media)).is_some() {
                return Err("duplicate browser asset");
            }
        }
        if !files.contains_key("/") {
            return Err("browser index absent");
        }
        Ok(Self(files))
    }
    pub fn router(&self) -> Router {
        let mut router = Router::new();
        for (path, (bytes, media)) in &self.0 {
            let bytes = bytes.clone();
            let media = *media;
            router = router.route(path, get(move || {
                let bytes = bytes.clone();
                async move {
                    let mut response = (StatusCode::OK, Body::from(bytes)).into_response();
                    let headers = response.headers_mut();
                    headers.insert(header::CONTENT_TYPE, media.parse().unwrap());
                    headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
                    headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
                    headers.insert(header::REFERRER_POLICY, "no-referrer".parse().unwrap());
                    headers.insert(header::CONTENT_SECURITY_POLICY, "default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; font-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'".parse().unwrap());
                    response
                }
            }));
        }
        router
    }
}
