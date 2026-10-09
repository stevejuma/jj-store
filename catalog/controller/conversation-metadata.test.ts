import { describe, it, expect } from "vitest";
import {
  ConversationMetadata,
  type MetadataView,
} from "../../web/src/lib/controller/conversation-metadata.svelte.js";
import { LocalClient } from "../../web/src/lib/api/local-client.js";
const query = { session: "44".repeat(16) };
const view = (): MetadataView => ({
  version: 1,
  repository: "11".repeat(16),
  target: { type: "conversation", id: query.session },
  thread: null,
  revision: null,
  value: { custom_title: null, generated: null, agent_titles: {} },
  sources: [],
  title: "Original",
  fresh: false,
  writable: true,
  unavailable: null,
  details: [],
  job: null,
});
const response = (value: unknown) =>
  new Response(JSON.stringify(value), {
    headers: { "content-type": "application/json" },
  });
describe("conversation metadata controller", () => {
  it("prevents duplicate submissions and preserves a later rename draft", async () => {
    const value = view(),
      posts: unknown[] = [];
    let release!: () => void;
    const gate = new Promise<void>((resolve) => (release = resolve));
    const manager = new ConversationMetadata(
      new LocalClient("", async (_input, init) => {
        if (init?.body) {
          const command = JSON.parse(String(init.body));
          posts.push(command);
          await gate;
          value.job = {
            request_id: command.request_id,
            state: "completed",
            error: null,
            result: null,
          };
          return response(value.job);
        }
        return response(String(_input).includes("/job") ? value.job : value);
      }),
      () => {},
    );
    await manager.load(query);
    const entry = manager.entry(query);
    entry.rename = "First";
    const first = manager.command(query, { action: "rename", title: "First" });
    await manager.command(query, { action: "rename", title: "Duplicate" });
    entry.rename = "Later draft";
    release();
    await first;
    expect(posts).toHaveLength(1);
    expect(entry.rename).toBe("Later draft");
    expect(entry.pending).toBeNull();
    manager.destroy();
  });
  it("retains and retries the exact request when the response is lost", async () => {
    const value = view(),
      posts: unknown[] = [];
    const manager = new ConversationMetadata(
      new LocalClient("", async (input, init) => {
        if (init?.body) {
          const command = JSON.parse(String(init.body));
          posts.push(command);
          value.job = {
            request_id: command.request_id,
            state: "completed",
            error: null,
            result: null,
          };
          if (posts.length === 1) throw Error("lost");
          return response(value.job);
        }
        return response(String(input).includes("/job") ? value.job : value);
      }),
      () => {},
    );
    await manager.load(query);
    await manager.command(query, { action: "refresh" });
    expect(manager.entry(query).pending).not.toBeNull();
    manager.leave(query);
    await manager.load(query);
    await manager.retry(query);
    expect(posts).toHaveLength(2);
    expect(posts[0]).toEqual(posts[1]);
    expect(manager.entry(query).pending).toBeNull();
    manager.destroy();
  });
  it("ignores superseded reads without erasing session drafts", async () => {
    let reads = 0,
      release!: () => void;
    const gate = new Promise<void>((resolve) => (release = resolve));
    const manager = new ConversationMetadata(
      new LocalClient("", async () => {
        const n = ++reads;
        if (n === 1) await gate;
        return response({ ...view(), title: String(n) });
      }),
      () => {},
    );
    manager.entry(query).rename = "Retained";
    const first = manager.load(query);
    await manager.load(query);
    release();
    await first;
    expect(manager.entry(query).value?.title).toBe("2");
    expect(manager.entry(query).rename).toBe("Retained");
    expect(manager.entry(query).loading).toBe(false);
    manager.destroy();
  });
  it("GET restores unknown request recovery without starting inference", async () => {
    const value = view(),
      posts: unknown[] = [];
    const original = {
      version: 1,
      request_id: "a1".repeat(16),
      query: { ...query, repository: null },
      expected: null,
      action: { action: "refresh" as const },
    };
    value.job = {
      request_id: original.request_id,
      state: "outcome_unknown",
      error: "interrupted",
      result: null,
      original_command: original,
    };
    const manager = new ConversationMetadata(
      new LocalClient("", async (input, init) => {
        if (init?.body) {
          posts.push(JSON.parse(String(init.body)));
          value.job = { ...value.job!, state: "completed", error: null };
          return response(value.job);
        }
        return response(String(input).includes("/job") ? value.job : value);
      }),
      () => {},
    );
    await manager.load(query);
    expect(posts).toHaveLength(0);
    expect(manager.entry(query).pending).toEqual(original);
    await manager.retry(query);
    expect(posts).toEqual([original]);
    manager.destroy();
  });
  it("retains unknown requests until recovery and clears definitive failed submissions", async () => {
    const value = view(),
      posts: unknown[] = [];
    const manager = new ConversationMetadata(
      new LocalClient("", async (input, init) => {
        if (init?.body) {
          const command = JSON.parse(String(init.body));
          posts.push(command);
          value.job = {
            request_id: command.request_id,
            state: "failed",
            error: "metadata_changed",
            result: null,
          };
          return response(value.job);
        }
        return response(String(input).includes("/job") ? value.job : value);
      }),
      () => {},
    );
    await manager.load(query);
    const entry = manager.entry(query);
    entry.pending = {
      version: 1,
      request_id: "aa".repeat(16),
      query,
      expected: null,
      action: { action: "refresh" },
    };
    await manager.command(query, {
      action: "rename",
      title: "Do not replace unknown intent",
    });
    expect(posts).toHaveLength(0);
    const retained = entry.pending;
    await manager.retry(query);
    expect(posts).toEqual([retained]);
    expect(entry.pending).toBeNull();
    expect(entry.error).toBe("metadata_changed");
    await manager.command(query, {
      action: "rename",
      title: "Allowed after refusal",
    });
    expect(posts).toHaveLength(2);
    manager.destroy();
  });
});
