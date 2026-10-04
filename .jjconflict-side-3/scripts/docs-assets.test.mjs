import assert from "node:assert/strict";
import { EventEmitter } from "node:events";
import { mkdtemp, mkdir, rm, writeFile, rename } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { documentationPlugin } from "./docs/assets.mjs";

async function fixture(t) {
  const root = await mkdtemp(join(tmpdir(), "jj-store-help-reload-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  return root;
}
async function publish(root, body, files = ["index.html"]) {
  await mkdir(root, { recursive: true });
  await writeFile(join(root, "index.html"), body);
  await writeFile(
    join(root, "manifest.tmp"),
    JSON.stringify({ version: 1, files }),
  );
  await rename(join(root, "manifest.tmp"), join(root, "asset-manifest.json"));
}
async function eventually(check) {
  const deadline = Date.now() + 3000;
  while (true) {
    try {
      return await check();
    } catch (error) {
      if (Date.now() >= deadline) throw error;
      await new Promise((resolve) => setTimeout(resolve, 10));
    }
  }
}
async function harness(root) {
  const watcher = new EventEmitter();
  const warnings = [];
  const httpServer = new EventEmitter();
  const plugin = documentationPlugin(root);
  let middleware;
  await plugin.configureServer({
    watcher,
    httpServer,
    config: { logger: { warn: (message) => warnings.push(message) } },
    middlewares: {
      use: (handler) => {
        middleware = handler;
      },
    },
  });
  return {
    warnings,
    httpServer,
    plugin,
    read(path = "/help/") {
      const response = {
        statusCode: 200,
        headers: {},
        body: undefined,
        setHeader(name, value) {
          this.headers[name] = value;
        },
        writeHead(status, headers) {
          this.statusCode = status;
          Object.assign(this.headers, headers);
        },
        end(body) {
          this.body = body?.toString();
        },
      };
      middleware({ url: path, method: "GET" }, response, () =>
        assert.fail("Help route must be owned"),
      );
      return response;
    },
  };
}

test("Help recovers from a missing initial bundle when its manifest is published", async (t) => {
  const root = join(await fixture(t), "build");
  const server = await harness(root);
  t.after(() => server.plugin.closeBundle?.());
  assert.equal(server.read().statusCode, 404);
  await publish(root, "recovered");
  await eventually(() => assert.equal(server.read().body, "recovered"));
  assert.match(
    server.read().headers["Content-Security-Policy"],
    /frame-ancestors 'self'/u,
  );
});

test("Help retains validated bytes until a safe complete replacement is published", async (t) => {
  const root = await fixture(t);
  await publish(root, "first");
  const server = await harness(root);
  t.after(() => server.plugin.closeBundle?.());
  await rm(join(root, "asset-manifest.json"));
  await writeFile(join(root, "index.html"), "second");
  assert.equal(server.read().body, "first");
  await publish(root, "unsafe", ["index.html", "../secret.html"]);
  await eventually(() =>
    assert.ok(
      server.warnings.some((message) =>
        /Unsafe documentation path/u.test(message),
      ),
    ),
  );
  assert.equal(server.read().body, "first");
  assert.equal(server.read("/help/secret.html").statusCode, 404);
  await publish(root, "third");
  await eventually(() => assert.equal(server.read().body, "third"));
});

test("Help releases its manifest observers when the server closes", async (t) => {
  const root = await fixture(t);
  await publish(root, "first");
  const server = await harness(root);
  server.httpServer.emit("close");
  await publish(root, "after close");
  await new Promise((resolve) => setTimeout(resolve, 50));
  assert.equal(server.read().body, "first");
});

test("a cold Vite server reloads rebuilt Help through real filesystem publication events", async (t) => {
  const { createServer } = await import("vite");
  const root = await fixture(t);
  const web = join(root, "web");
  await mkdir(web);
  const docs = join(root, "docs/build");
  await mkdir(join(root, "docs"));
  const server = await createServer({
    configFile: false,
    root: web,
    publicDir: false,
    plugins: [documentationPlugin(docs)],
    server: { host: "127.0.0.1", port: 0 },
  });
  t.after(() => server.close());
  await server.listen();
  const hmr = [];
  const send = server.environments.client.hot.send.bind(
    server.environments.client.hot,
  );
  server.environments.client.hot.send = (...args) => {
    hmr.push(args[0]);
    return send(...args);
  };
  const origin = `http://127.0.0.1:${server.httpServer.address().port}`;
  assert.equal((await fetch(origin + "/help/")).status, 404);
  await publish(docs, "initial manual");
  await eventually(async () =>
    assert.equal(
      await (await fetch(origin + "/help/")).text(),
      "initial manual",
    ),
  );
  const response = await fetch(origin + "/help/");
  assert.equal(response.status, 200);
  assert.equal(await response.text(), "initial manual");
  await rm(docs, { recursive: true });
  await mkdir(docs, { recursive: true });
  await writeFile(join(docs, "index.html"), "unfinished manual");
  assert.equal(await (await fetch(origin + "/help/")).text(), "initial manual");
  await publish(docs, "rebuilt manual");
  await eventually(async () =>
    assert.equal(
      await (await fetch(origin + "/help/")).text(),
      "rebuilt manual",
    ),
  );
  assert.equal(server.httpServer.address().port, Number(new URL(origin).port));
  assert.deepEqual(
    hmr.filter((message) => message.type === "full-reload"),
    [],
  );
});

test("superseded Help publications settle on the latest validated bundle", async (t) => {
  const root = await fixture(t);
  await publish(root, "first");
  const server = await harness(root);
  t.after(() => server.plugin.closeBundle?.());
  for (const body of ["second", "third", "latest"]) {
    await publish(root, body);
  }
  await eventually(() => assert.equal(server.read().body, "latest"));
});
