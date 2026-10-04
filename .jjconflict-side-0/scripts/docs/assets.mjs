import { lstat, readFile } from "node:fs/promises";
import { watch } from "node:fs";
import { dirname, join, resolve } from "node:path";
export const helpCsp =
  "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; worker-src 'self'; frame-ancestors 'self'; base-uri 'none'; form-action 'none'";
/** @type {Record<string, string>} */
const media = {
  html: "text/html; charset=utf-8",
  js: "text/javascript; charset=utf-8",
  css: "text/css; charset=utf-8",
  json: "application/json",
  svg: "image/svg+xml",
  png: "image/png",
  ico: "image/x-icon",
  woff2: "font/woff2",
  woff: "font/woff",
  ttf: "font/ttf",
  xml: "application/xml",
  gz: "application/gzip",
};
/** @param {string} root */
export async function helpAssets(root) {
  if (!(await lstat(root)).isDirectory())
    throw Error("Unsafe documentation root");
  const manifestPath = join(root, "asset-manifest.json");
  const meta = await lstat(manifestPath);
  if (!meta.isFile() || meta.size > 128 * 1024)
    throw Error("Invalid documentation manifest");
  const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
  if (
    manifest.version !== 1 ||
    !Array.isArray(manifest.files) ||
    !manifest.files.includes("index.html") ||
    manifest.files.length > 512 ||
    new Set(manifest.files).size !== manifest.files.length
  )
    throw Error("Invalid documentation manifest");
  let total = 0;
  const result = new Map();
  for (const name of manifest.files) {
    if (
      typeof name !== "string" ||
      name === "asset-manifest.json" ||
      name.split("/").length > 8 ||
      name
        .split("/")
        .some(
          (part) =>
            !/^[a-zA-Z0-9_.-]+$/.test(part) || part === "." || part === "..",
        )
    )
      throw Error("Unsafe documentation path");
    let path = root;
    for (const part of name.split("/")) {
      path = join(path, part);
      if ((await lstat(path)).isSymbolicLink())
        throw Error("Documentation symlink");
    }
    const stat = await lstat(path),
      type = media[name.split(".").pop() ?? ""];
    total += stat.size;
    if (
      !stat.isFile() ||
      !type ||
      stat.size > 8 * 1024 * 1024 ||
      total > 32 * 1024 * 1024
    )
      throw Error("Documentation asset limits");
    result.set(name, { bytes: await readFile(path), type });
  }
  return result;
}
/** @param {string} root @returns {import("vite").Plugin} */
export function documentationPlugin(root) {
  let dispose = () => {};
  /** @param {import("vite").ViteDevServer | import("vite").PreviewServer} server */
  const configure = async (server) => {
    /** @type {Awaited<ReturnType<typeof helpAssets>> | undefined} */
    let assets;
    try {
      assets = await helpAssets(root);
    } catch (error) {
      if (!(
        error instanceof Error &&
        "code" in error &&
        error.code === "ENOENT"
      ))
        throw error;
      server.config.logger.warn(
        "Help is unavailable. Run pnpm docs:build before starting Vite.",
      );
    }
    if ("watcher" in server) {
      const directory = resolve(root);
      const parent = dirname(directory);
      const manifestPath = join(directory, "asset-manifest.json");
      let generation = 0;
      let loading = false;
      let closed = false;
      const reload = async () => {
        if (loading) return;
        loading = true;
        try {
          while (!closed) {
            const selected = generation;
            try {
              const next = await helpAssets(root);
              if (!closed && selected === generation) assets = next;
            } catch (error) {
              if (
                !closed &&
                selected === generation &&
                !(
                  error instanceof Error &&
                  "code" in error &&
                  error.code === "ENOENT"
                )
              )
                server.config.logger.warn(
                  `Help rebuild rejected: ${error instanceof Error ? error.message : String(error)}. Keeping the last validated bundle.`,
                );
            }
            if (selected === generation) break;
          }
        } finally {
          loading = false;
        }
      };
      const observer = watch(
        parent,
        { recursive: true },
        (_event, filename) => {
          if (filename !== null) {
            const path = resolve(parent, filename.toString());
            if (path !== manifestPath && path !== directory) return;
          }
          generation += 1;
          void reload();
        },
      );
      observer.on("error", (error) => {
        dispose();
        server.config.logger.warn(
          `Help observer stopped: ${error.message}. Restart Vite after rebuilding documentation.`,
        );
      });
      dispose = () => {
        closed = true;
        observer.close();
        server.httpServer?.off("close", dispose);
      };
      server.httpServer?.once("close", dispose);
      // Close the startup read/subscribe gap without periodic filesystem reads.
      generation += 1;
      await reload();
    }
    server.middlewares.use((req, res, next) => {
      const path = new URL(req.url ?? "/", "http://localhost").pathname;
      if (path !== "/help" && !path.startsWith("/help/")) return next();
      if (path === "/help") {
        res.writeHead(308, { Location: "/help/" });
        return res.end();
      }
      const name = path.slice(6) + (path.endsWith("/") ? "index.html" : "");
      const asset = assets?.get(name);
      res.setHeader("Content-Security-Policy", helpCsp);
      res.setHeader("X-Content-Type-Options", "nosniff");
      res.setHeader("Referrer-Policy", "no-referrer");
      res.setHeader("Cache-Control", "no-store");
      if (!asset) {
        res.statusCode = 404;
        return res.end("Documentation unavailable");
      }
      if (!["GET", "HEAD"].includes(req.method ?? "")) {
        res.statusCode = 405;
        return res.end();
      }
      res.setHeader("Content-Type", asset.type);
      res.end(req.method === "HEAD" ? undefined : asset.bytes);
    });
  };
  return {
    name: "bundled-documentation",
    configureServer: configure,
    configurePreviewServer: configure,
    closeBundle: () => dispose(),
  };
}
