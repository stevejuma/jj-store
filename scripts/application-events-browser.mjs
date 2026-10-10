// Cold browser harness for the production SharedWorker adapter. No live APIs.
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { resolve, extname } from "node:path";
import { build } from "vite";
import { WebSocketServer } from "ws";
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
  JSON.stringify({ id: `${prefix}${sequence}`, event: value });
const server = createServer(async (req, res) => {
  const path = new URL(req.url, "http://127.0.0.1").pathname;
  res.setHeader(
    "Content-Security-Policy",
    "default-src 'self'; script-src 'self'; worker-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'",
  );
  if (path === "/v1/events") {
    res.writeHead(500);
    res.end("SSE must not be used");
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
      client.send(
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
const sockets = new WebSocketServer({ noServer: true, maxPayload: 8192 });
server.on("upgrade", (req, socket, head) => {
  if (
    req.url !== "/v1/events/ws" ||
    req.headers["sec-websocket-protocol"] !== "jj-store-events-v1"
  )
    return socket.destroy();
  sockets.handleUpgrade(req, socket, head, (client) => {
    opened++;
    clients.add(client);
    client.once("message", (data) => {
      const hello = JSON.parse(data.toString());
      if (hello.version !== 1 || hello.token !== null)
        return client.close(4403);
      client.send(frame({ version: 1, type: "ready", witnesses: [] }));
    });
    client.on("close", () => {
      closed++;
      clients.delete(client);
    });
  });
});
server.listen(17764, "127.0.0.1", () =>
  console.log("Acceptance: http://127.0.0.1:17764/"),
);
const heartbeat = setInterval(() => {
  for (const client of clients)
    client.send(JSON.stringify({ version: 1, type: "heartbeat" }));
}, 15000);
function stop() {
  clearInterval(heartbeat);
  for (const client of clients) client.close();
  server.close();
  server.closeAllConnections();
}
process.on("SIGINT", stop);
process.on("SIGTERM", stop);
