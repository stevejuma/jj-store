import { expect, test, vi } from "vitest";
import { createProductionStoreController } from "../../src/lib/controller/production-store-controller.svelte.js";
import { threadInboxRows } from "../../src/lib/ui/store-threads/thread-inbox.js";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";

test("Codebase publication refreshes the cached metadata-home directory before displaying repository aliases", async () => {
  const fixture = createThreadTransport(),
    other = "99".repeat(16);
  const initial = fixture.threads.get(ids.thread)!;
  fixture.threads.set(ids.thread, {
    ...initial,
    value: { ...initial.value, codebase: [ids.repo] },
  });
  let homeReads = 0;
  const fetcher: typeof fetch = async (input) => {
    const url = new URL(String(input), "http://localhost"),
      path = url.pathname;
    const json = (value: unknown) => Response.json(value);
    if (path === "/v1/bootstrap")
      return json({
        version: 1,
        access_mode: "local_trusted",
        storage: { ready: true },
        backups: [],
        capabilities: {},
      });
    if (path === "/v1/repositories")
      return json({
        version: 1,
        generation: 1,
        repositories: [
          {
            id: ids.repo,
            name: "Home",
            profile: "native_v1",
            permission: "admin",
            principal: ids.actor,
          },
          {
            id: other,
            name: "Other",
            profile: "native_v1",
            permission: "admin",
            principal: ids.actor,
          },
        ],
      });
    if (path === "/v1/local/threads") {
      const row = fixture.threads.get(ids.thread)!;
      fixture.threads.set(ids.thread, {
        ...row,
        value: { ...row.value, codebase: [ids.repo, other] },
      });
      return json({
        home: ids.repo,
        thread: ids.thread,
        revision: row.reference,
      });
    }
    if (path.endsWith("/capabilities"))
      return json({
        typed_mutations: [],
        conflict_resolution_actions: [],
        conflict_upload_bytes: 0,
        search: false,
      });
    if (path.endsWith("/snapshot"))
      return json({
        version: 1,
        repo_id: ids.repo,
        epoch: "11".repeat(16),
        published_seq: 1,
        manifest_revision: 1,
        code_state_version: 1,
        heads: [],
      });
    if (path.endsWith("/threads")) {
      const page = await fixture.transport.list();
      if (path.includes(ids.repo)) {
        homeReads++;
        return json(page);
      }
      return json({
        ...page,
        snapshot: { ...page.snapshot, repo_id: other },
        items: { type: "threads", threads: [] },
      });
    }
    if (path.endsWith(`/threads/${ids.thread}`))
      return json(await fixture.transport.thread(ids.thread));
    if (path.endsWith("/timeline"))
      return json(await fixture.transport.events(ids.thread));
    if (path.endsWith("/reaction-emojis"))
      return json(await fixture.transport.emojis());
    if (path.endsWith("/reactions/query"))
      return json({
        version: 1,
        snapshot: fixture.snapshot(),
        items: { type: "reactions", reactions: [] },
        next_cursor: null,
      });
    throw new Error(`Unexpected request ${path}`);
  };
  const controller = createProductionStoreController({
    fetcher,
    browser: {
      hash: () => "#/threads",
      push() {},
      replace() {},
      go() {},
      listen: () => () => {},
    },
  });
  try {
    await vi.waitFor(() =>
      expect(controller.state.repositories).toHaveLength(2),
    );
    await controller.threadNavigation!.load();
    expect(threadInboxRows(controller)).toHaveLength(1);
    expect(homeReads).toBe(1);
    await controller.threadWorkspaces!.submit({
      type: "codebase",
      home: ids.repo,
      thread: ids.thread,
      expected: initial.reference,
      repositories: [ids.repo, other],
    });
    expect(
      threadInboxRows(controller)
        .map((r) => r.repository?.id)
        .sort(),
    ).toEqual([ids.repo, other].sort());
    expect(homeReads).toBe(2);
    await controller.threadNavigation!.load();
    expect(homeReads).toBe(2);
  } finally {
    controller.actions.destroy();
  }
});
