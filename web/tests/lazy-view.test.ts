import test from "node:test";
import assert from "node:assert/strict";
import { lazyView } from "../src/lib/ui/store-application/lazy-view.js";

test("view imports wait for demand and share completion across consumers", async () => {
  let calls = 0;
  let resolve!: (value: object) => void;
  const result = new Promise<object>((done) => {
    resolve = done;
  });
  const load = lazyView(() => {
    calls += 1;
    return result;
  });
  assert.equal(calls, 0);
  const a = load(),
    b = load();
  assert.equal(calls, 1);
  assert.equal(a, b);
  const module = {};
  resolve(module);
  assert.equal(await a, module);
  assert.equal(await load(), module);
  assert.equal(calls, 1);
});

test("a failed view import rejects all consumers and allows a fresh explicit retry", async () => {
  let calls = 0;
  const load = lazyView(async () => {
    if (++calls === 1) throw Error("offline");
    return "loaded";
  });
  const first = load();
  assert.equal(load(), first);
  await assert.rejects(first, /offline/);
  assert.equal(await load(), "loaded");
  assert.equal(calls, 2);
});
