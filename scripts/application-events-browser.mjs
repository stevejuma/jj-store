// Cold browser harness for the production SharedWorker adapter. No live APIs.
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { resolve, extname } from "node:path";
import { build } from "vite";
const root = resolve(import.meta.dirname, "..");
const outDir = resolve(root, ".scratch/application-events-browser");
await build({
  configFile: false,
  root,
  worker: { format: "es" },
  build: {
    outDir,
    emptyOutDir: true,
    assetsInlineLimit: 0,
    rollupOptions: {
      input: resolve(
        root,
        "web/tests/fixtures/application-events-browser.html",
      ),
    },
  },
});
const clients = new Set();
let opened = 0,
  closed = 0,
  sequence = 0;
const prefix = `jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:`;
const frame = (value) =>
  `id: ${prefix}${sequence}\nevent: ${value.type}\ndata: ${JSON.stringify(value)}\n\n`;
const server = createServer(async (req, res) => {
  const path = new URL(req.url, "http://127.0.0.1").pathname;
  res.setHeader(
    "Content-Security-Policy",
    "default-src 'self'; script-src 'self'; worker-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'",
  );
  if (path === "/v1/events") {
    opened++;
    clients.add(res);
    res.writeHead(200, {
      "content-type": "text/event-stream",
      "cache-control": "no-store",
    });
    res.write(frame({ version: 1, type: "ready", witnesses: [] }));
    req.on("close", () => {
      closed++;
      clients.delete(res);
    });
    return;
  }
  if (path === "/probe") {
    res.setHeader("content-type", "application/json");
    res.end(JSON.stringify({ opened, closed, active: clients.size }));
    return;
  }
  if (path === "/change" && req.method === "POST") {
    sequence++;
    for (const client of clients)
      client.write(
        frame({
          version: 1,
          type: "changed",
          change: {
            domain: "agent_settings",
            repository: null,
            witness: String(sequence),
          },
        }),
      );
    res.end("ok");
    return;
  }
  const file = resolve(
    outDir,
    path === "/"
      ? "web/tests/fixtures/application-events-browser.html"
      : `.${path}`,
  );
  if (!file.startsWith(outDir + "/")) {
    res.writeHead(404);
    res.end();
    return;
  }
  try {
    await stat(file);
    res.setHeader(
      "content-type",
      extname(file) === ".js" ? "text/javascript" : "text/html",
    );
    res.end(await readFile(file));
  } catch {
    res.writeHead(404);
    res.end();
  }
});
server.listen(17764, "127.0.0.1", () =>
  console.log("Acceptance: http://127.0.0.1:17764/"),
);
const heartbeat = setInterval(() => {
  for (const client of clients) client.write(": heartbeat\n\n");
}, 15000);
function stop() {
  clearInterval(heartbeat);
  for (const client of clients) client.end();
  server.close();
  server.closeAllConnections();
}
process.on("SIGINT", stop);
process.on("SIGTERM", stop);
