//! Independently bounded, preloaded documentation. URLs never select disk paths.
use super::regular;
use axum::{
    Router,
    body::{Body, Bytes},
    http::{StatusCode, header},
    response::{IntoResponse, Redirect},
    routing::get,
};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};

const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; worker-src 'self'; frame-ancestors 'self'; base-uri 'none'; form-action 'none'";
#[derive(Clone)]
pub(super) struct HelpAssets(BTreeMap<String, (Bytes, &'static str)>);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u32,
    files: Vec<String>,
}

impl HelpAssets {
    pub(super) fn load(root: &Path) -> Result<Option<Self>, &'static str> {
        let root = root.join("help");
        match fs::symlink_metadata(&root) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Ok(meta) if meta.is_dir() => (),
            _ => return Err("documentation root type"),
        }
        let manifest: Manifest =
            serde_json::from_slice(&regular(&root.join("asset-manifest.json"), 128 * 1024)?)
                .map_err(|_| "documentation manifest")?;
        if manifest.version != 1 || manifest.files.is_empty() || manifest.files.len() > 512 {
            return Err("documentation manifest limits");
        }
        let mut files = BTreeMap::new();
        let mut total = 0;
        for name in manifest.files {
            let parts = name.split('/').collect::<Vec<_>>();
            if parts.len() > 8
                || parts.iter().any(|part| {
                    part.is_empty()
                        || *part == "."
                        || *part == ".."
                        || !part
                            .bytes()
                            .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
                })
            {
                return Err("documentation asset path");
            }
            let mut directory = root.clone();
            for part in &parts[..parts.len() - 1] {
                directory.push(part);
                if !fs::symlink_metadata(&directory)
                    .map_err(|_| "documentation directory")?
                    .is_dir()
                {
                    return Err("documentation directory");
                }
            }
            let media = match Path::new(&name).extension().and_then(|ext| ext.to_str()) {
                Some("html") => "text/html; charset=utf-8",
                Some("js") => "text/javascript; charset=utf-8",
                Some("css") => "text/css; charset=utf-8",
                Some("json") => "application/json",
                Some("svg") => "image/svg+xml",
                Some("png") => "image/png",
                Some("ico") => "image/x-icon",
                Some("woff2") => "font/woff2",
                Some("woff") => "font/woff",
                Some("ttf") => "font/ttf",
                Some("xml") => "application/xml",
                Some("gz") => "application/gzip",
                _ => return Err("documentation asset media"),
            };
            if name == "asset-manifest.json" {
                return Err("documentation manifest is private");
            }
            let bytes = regular(&root.join(&name), 8 * 1024 * 1024)?;
            total += bytes.len();
            if total > 32 * 1024 * 1024 {
                return Err("documentation bundle size");
            }
            let route = format!("/help/{name}");
            if files.insert(route, (Bytes::from(bytes), media)).is_some() {
                return Err("duplicate documentation asset");
            }
        }
        if !files.contains_key("/help/index.html") {
            return Err("documentation index absent");
        }
        Ok(Some(Self(files)))
    }
    pub(super) fn router(&self) -> Router {
        let mut router =
            Router::new().route("/help", get(|| async { Redirect::permanent("/help/") }));
        for (path, (bytes, media)) in &self.0 {
            let bytes = bytes.clone();
            let media = *media;
            let serve = get(move || {
                let bytes = bytes.clone();
                async move {
                    let mut response = (StatusCode::OK, Body::from(bytes)).into_response();
                    let headers = response.headers_mut();
                    headers.insert(header::CONTENT_TYPE, media.parse().unwrap());
                    headers.insert(header::CONTENT_SECURITY_POLICY, CSP.parse().unwrap());
                    headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
                    headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
                    headers.insert(header::REFERRER_POLICY, "no-referrer".parse().unwrap());
                    response
                }
            });
            router = router.route(path, serve.clone());
            if let Some(directory) = path.strip_suffix("index.html") {
                router = router.route(directory, serve);
            }
        }
        router
    }
}
