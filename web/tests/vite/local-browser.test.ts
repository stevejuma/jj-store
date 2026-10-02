import assert from "node:assert/strict";
import { test } from "vitest";
import { HttpLocalStoreDataSource } from "../../src/lib/controller/http-local-store-data-source.js";
import { LocalFolderManager } from "../../src/lib/controller/local-folder-manager.js";
import { LocalEventParser } from "../../src/lib/api/local-events.js";
import { repositorySource } from "../../src/lib/controller/repository-source.js";
import { validateProfileRoute } from "../../src/lib/controller/repository-profile.js";
import { storeRouteFromHash } from "../../src/lib/controller/store-route-codec.js";
import { followProductionStoreEvents } from "../../src/lib/controller/production-store-events.js";
import { pretty } from "../../src/lib/api/client.js";
import { createProfileRouteRestorer } from "../../src/lib/controller/production-profile-route.js";
const repository = "11".repeat(16),
  operation = "22".repeat(64),
  commit = "33".repeat(20),
  metadata = "44".repeat(20);
const head = {
  version: 1,
  profile: "git_sha1_jj045",
  repository,
  generation: 1,
  heads: [operation],
  metadata_commit: metadata,
  actor: "55".repeat(16),
};
const json = (value: unknown) =>
  new Response(pretty(value), {
    headers: { "content-type": "application/json" },
  });
function storage() {
  const values = new Map<string, string>();
  return {
    getItem: (k: string) => values.get(k) ?? null,
    setItem: (k: string, v: string) => {
      values.set(k, v);
    },
    removeItem: (k: string) => {
      values.delete(k);
    },
  };
}

test("local_transport_preserves_git_ids_and_independent_metadata_pin", async () => {
  const calls: { url: string; body: any }[] = [];
  const source = new HttpLocalStoreDataSource(repository, async (url, init) => {
    assert.equal(new Headers(init?.headers).has("authorization"), false);
    assert.equal(init?.credentials, "omit");
    const body = init?.body ? JSON.parse(String(init.body)) : null;
    calls.push({ url: String(url), body });
    if (String(url).endsWith("snapshot")) return json(head);
    if (String(url).endsWith("work/page"))
      return json({
        snapshot: {
          profile: "local_jj_git",
          repo_id: repository,
          metadata_commit: metadata,
          work_state_version: 1,
        },
        projects: [],
        tasks: [],
        comments: [],
        next_cursor: null,
      });
    return json({
      snapshot: {
        version: 1,
        profile: head.profile,
        repository,
        generation: 1,
        operation,
      },
      data: { kind: "file", data: { text: "hello", bytes: 5 } },
      next_cursor: null,
      previous_cursor: null,
    });
  });
  assert.equal((await source.file(commit, operation)).text, "hello");
  await source.work.page({ kind: "projects", limit: 50 });
  assert.equal(calls[1].body.request.query.file, commit);
  assert.equal(calls[2].body.metadata_commit, metadata);
  await assert.rejects(
    source.search({ query: "x", operation }),
    /unsupported local capability/,
  );
  assert.equal(calls.length, 3);
  source.close();
});

test("local_reads_retry_busy_without_advancing_pins_or_replaying_writes", async () => {
  const queries: any[] = [];
  let snapshots = 0,
    writes = 0;
  const busy = () =>
    new Response(pretty({ version: 1, code: "busy" }), {
      status: 503,
      headers: { "content-type": "application/json" },
    });
  const source = new HttpLocalStoreDataSource(repository, async (url, init) => {
    if (String(url).endsWith("snapshot")) {
      snapshots++;
      return json(head);
    }
    if (String(url).endsWith("work/commands")) {
      writes++;
      return busy();
    }
    queries.push(JSON.parse(String(init?.body)));
    if (queries.length < 3) return busy();
    return json({
      snapshot: {
        version: 1,
        profile: head.profile,
        repository,
        generation: 1,
        operation,
      },
      data: { kind: "file", data: { text: "retained selection", bytes: 18 } },
      next_cursor: null,
      previous_cursor: null,
    });
  });
  try {
    assert.equal(
      (await source.file(commit, operation)).text,
      "retained selection",
    );
    assert.equal(snapshots, 1);
    assert.equal(queries.length, 3);
    assert.deepEqual(queries[1], queries[0]);
    assert.deepEqual(queries[2], queries[0]);
    assert.equal((await source.snapshot()).metadata_commit, metadata);
    await assert.rejects(source.work.apply({} as any), /busy/);
    assert.equal(writes, 1);
  } finally {
    source.close();
  }
});

test("concurrent_local_snapshot_reads_share_capture_and_refresh_advances_metadata_explicitly", async () => {
  let calls = 0;
  const source = new HttpLocalStoreDataSource(repository, async () => {
    calls++;
    await Promise.resolve();
    return json({
      ...head,
      metadata_commit: calls === 1 ? metadata : "66".repeat(20),
    });
  });
  const [first, second] = await Promise.all([
    source.snapshot(),
    source.snapshot(),
  ]);
  assert.deepEqual(first, second);
  assert.equal(calls, 1);
  assert.equal((await source.snapshot()).metadata_commit, metadata);
  assert.equal((await source.snapshot(true)).metadata_commit, "66".repeat(20));
  assert.equal(calls, 2);
  source.close();
});

test("local_refresh_does_not_reuse_an_older_in_flight_capture", async () => {
  const pending: ((response: Response) => void)[] = [];
  const source = new HttpLocalStoreDataSource(
    repository,
    () => new Promise<Response>((resolve) => pending.push(resolve)),
  );
  const original = source.snapshot().then(
    () => "accepted",
    () => "superseded",
  );
  const refresh = source.snapshot(true);
  void refresh.catch(() => {});
  try {
    assert.equal(pending.length, 2);
    pending[1](json({ ...head, metadata_commit: "66".repeat(20) }));
    assert.equal((await refresh).metadata_commit, "66".repeat(20));
    pending[0](json(head));
    assert.equal(await original, "superseded");
    assert.equal((await source.snapshot()).metadata_commit, "66".repeat(20));
  } finally {
    source.close();
  }
});

for (const newerFirst of [false, true]) {
  test(`superseded_local_fresh_refresh_uses_latest_capture_${newerFirst}`, async () => {
    const pending: ((response: Response) => void)[] = [];
    const source = new HttpLocalStoreDataSource(
      repository,
      () => new Promise<Response>((resolve) => pending.push(resolve)),
    );
    const original = source.snapshot(true);
    void original.catch(() => {});
    const replacement = source.snapshot(true);
    const latest = { ...head, metadata_commit: "66".repeat(20) };
    try {
      assert.equal(pending.length, 2);
      if (newerFirst) {
        pending[1](json(latest));
        await replacement;
        pending[0](json(head));
      } else {
        pending[0](json(head));
        pending[1](json(latest));
      }
      const [first, second] = await Promise.all([original, replacement]);
      assert.deepEqual(first, second);
      assert.equal(first.metadata_commit, latest.metadata_commit);
      assert.equal(
        (await source.snapshot()).metadata_commit,
        latest.metadata_commit,
      );
    } finally {
      source.close();
    }
  });
}

test("history_restoration_reconnects_repository_before_validating_profile_specific_ids", async () => {
  const parsed = storeRouteFromHash(
    `#/repos/${repository}/files?at_op=${operation}&commit=${commit}`,
  );
  assert.equal(parsed.kind, "route");
  if (parsed.kind !== "route") return;
  const calls: string[] = [];
  const restore = createProfileRouteRestorer({
    data: {} as any,
    route: () => parsed.route,
    source: () => ({ repository: "aa".repeat(16), profile: "native" }) as any,
    projects: {
      restore: async () => {
        calls.push("projects");
      },
    },
    connect: async (id, token) => {
      assert.equal(token, null);
      calls.push(id);
    },
    token: () => null,
    invalidate: () => {
      calls.push("invalidate");
    },
    restore: async () => {
      calls.push("restore");
    },
  });
  await restore();
  assert.deepEqual(calls, ["invalidate", repository]);
});

test("local_transport_rejects_identity_generation_and_operation_mismatch", async () => {
  for (const override of [
    { repository: "aa".repeat(16) },
    { generation: 2 },
    { operation: "bb".repeat(64) },
  ]) {
    const source = new HttpLocalStoreDataSource(repository, async (url) =>
      String(url).endsWith("snapshot")
        ? json(head)
        : json({
            snapshot: {
              version: 1,
              profile: head.profile,
              repository,
              generation: 1,
              operation,
              ...override,
            },
            data: { kind: "file", data: { text: "x", bytes: 1 } },
          }),
    );
    await assert.rejects(source.file(commit, operation), /snapshot mismatch/);
    source.close();
  }
});

test("local_routes_validate_full_ids_against_discovered_profile", () => {
  const parsed = storeRouteFromHash(
    `#/repos/${repository}/files?at_op=${operation}&commit=${commit}`,
  );
  assert.equal(parsed.kind, "route");
  if (parsed.kind !== "route") return;
  validateProfileRoute(parsed.route, "git_sha1_jj045");
  assert.throws(
    () => validateProfileRoute(parsed.route, "native"),
    /invalid full id/,
  );
  assert.equal(
    storeRouteFromHash(`#/repos/${repository}/files?at_op=${commit}`).kind,
    "invalid",
  );
  assert.throws(
    () =>
      repositorySource(
        {
          id: repository,
          name: "moved",
          permission: "admin",
          profile: head.profile as "git_sha1_jj045",
          availability: "unavailable",
        },
        null,
        "session",
        () => {},
      ),
    /local folder unavailable/,
  );
});

test("folder_add_survives_lost_response_reload_and_receipt_recovery", async () => {
  const saved = storage();
  let accepted: any;
  const registrations: any[] = [];
  const fetcher: typeof fetch = async (url, init) => {
    if (String(url).endsWith("repositories") && !init?.body)
      return json({ version: 1, revision: 7, bindings: [] });
    if (String(url).includes("requests/"))
      return json({
        status: "committed",
        request_id: accepted.request_id,
        binding: {
          repository,
          path: accepted.path,
          name: accepted.name,
          generation: 1,
          availability: "ready",
        },
        revision: 8,
      });
    accepted = JSON.parse(String(init?.body));
    registrations.push(accepted);
    throw new Error("lost response");
  };
  const first = new LocalFolderManager(fetcher, saved);
  await first.list();
  await assert.rejects(first.add("/test/repo", "Local"), /lost response/);
  first.close();
  const next = new LocalFolderManager(fetcher, saved);
  assert.deepEqual(next.pending, accepted);
  assert.equal((await next.status())?.status, "committed");
  assert.equal(next.pending, null);
  assert.equal(registrations.length, 1);
  next.close();
});

test("folder_unknown_retry_retains_exact_request_and_inspection_does_not_register", async () => {
  const bodies: any[] = [];
  const manager = new LocalFolderManager(async (url, init) => {
    if (String(url).endsWith("inspect"))
      return json({
        path: "/folder",
        metadata: "/folder/.jj/repo",
        repository: null,
      });
    if (!init?.body) return json({ version: 1, revision: 1, bindings: [] });
    const body = JSON.parse(String(init.body));
    bodies.push(body);
    return json({ status: "outcome_unknown", request_id: body.request_id });
  }, storage());
  await manager.list();
  await manager.inspect("/folder");
  assert.equal(bodies.length, 0);
  await manager.add("/folder", "Local");
  await manager.add("/another", "Different");
  assert.deepEqual(bodies[0], bodies[1]);
  manager.close();
});

test("local_sse_reconnect_announces_freshness_without_advancing_display", async () => {
  const events: any[] = [];
  const parser = new LocalEventParser(repository, (e) => events.push(e));
  const changed = { ...head, metadata_commit: "66".repeat(20) };
  parser.push(
    `event: ready\ndata: ${JSON.stringify({ type: "ready", snapshot: changed })}\n\n`,
  );
  assert.throws(
    () => parser.push(`id: 1\nevent: ready\ndata: {}\n\n`),
    /invalid local event/,
  );
  const stop = new AbortController();
  const data: any = {
    head,
    heads: [operation],
    status: { refreshAvailable: false },
  };
  const source = new HttpLocalStoreDataSource(repository);
  source.snapshot = async () => head as any;
  source.events = async (_after, _signal, deliver) => {
    deliver(events[0]);
    stop.abort();
  };
  await followProductionStoreEvents(source, () => true, data, stop.signal);
  assert.equal(data.status.refreshAvailable, true);
  assert.deepEqual(data.head, head);
  assert.deepEqual(data.heads, [operation]);
  source.close();
});

test("local_reviews_pin_companion_history_and_confirm_actions_without_advancing_code", async () => {
  const nextMetadata = "66".repeat(20),
    nextOperation = "77".repeat(64),
    thread = "88".repeat(16),
    request = "99".repeat(16);
  const snapshot = (pin: string) => ({
    profile: "local_jj_git",
    repo_id: repository,
    metadata_commit: pin,
    review_root: "aa".repeat(32),
    review_state_version: 1,
  });
  const calls: { url: string; body: any }[] = [];
  let confirmed = false;
  const outcome = {
    status: "committed",
    receipt: {
      repository,
      request_id: request,
      command: { thread_id: thread },
    },
  };
  const source = new HttpLocalStoreDataSource(repository, async (url, init) => {
    const body = init?.body ? JSON.parse(String(init.body)) : null;
    calls.push({ url: String(url), body });
    if (String(url).endsWith("snapshot"))
      return json(
        confirmed
          ? { ...head, heads: [nextOperation], metadata_commit: nextMetadata }
          : head,
      );
    if (String(url).endsWith("reviews/page"))
      return json({
        data: {
          version: 1,
          snapshot: snapshot(body.metadata_commit),
          threads: [],
          next_after: null,
        },
        next_cursor: "retained",
      });
    if (String(url).endsWith("reviews/detail"))
      return json({
        snapshot: snapshot(body.metadata_commit),
        thread: { thread_id: thread },
      });
    if (String(url).endsWith("reviews/commands")) {
      confirmed = true;
      return json(outcome);
    }
    return json(outcome);
  });
  await source.reviewThreads();
  assert.equal(calls[1].body.metadata_commit, metadata);
  await source.reviewThread(thread);
  assert.equal(calls[2].body.metadata_commit, metadata);
  await source.applyReviewCommand({
    version: 1,
    request_id: request,
    command: {
      command: "resolve_thread",
      thread_id: thread,
      expected_thread_version: "aa".repeat(32),
      reason: null,
    },
  });
  assert.deepEqual((await source.snapshot()).heads, [operation]);
  await source.reviewThreads();
  assert.equal(calls.at(-1)!.body.metadata_commit, nextMetadata);
  assert.equal(
    calls.filter((c) => c.url.endsWith("reviews/commands")).length,
    1,
  );
  source.close();
});

test("local_review_reads_reject_foreign_companion_snapshots", async () => {
  const source = new HttpLocalStoreDataSource(repository, async (url) =>
    String(url).endsWith("snapshot")
      ? json(head)
      : json({
          data: {
            snapshot: {
              profile: "local_jj_git",
              repo_id: "ff".repeat(16),
              metadata_commit: metadata,
            },
            threads: [],
          },
          next_cursor: null,
        }),
  );
  await assert.rejects(source.reviewThreads(), /snapshot mismatch/);
  source.close();
});

test("physical_save_preserves_selection_and_never_retries_or_advances_code", async () => {
  const selection = {
    version: 1,
    repository,
    generation: 1,
    workspace: "66".repeat(32),
    path: "README.md",
    fingerprint: "77".repeat(32),
  };
  const calls: { url: string; body: any }[] = [];
  let physical = "unsnapshotted physical text";
  const source = new HttpLocalStoreDataSource(repository, async (url, init) => {
    const body = init?.body ? JSON.parse(String(init.body)) : null;
    calls.push({ url: String(url), body });
    if (String(url).endsWith("snapshot")) return json(head);
    if (String(url).endsWith("working-copy/save")) {
      physical = body.contents;
      throw new TypeError("lost response after physical replacement");
    }
    return json({ selection, contents: physical });
  });
  await source.snapshot();
  const file = await source.workingCopy.read("README.md");
  assert.equal(file.contents, physical);
  await assert.rejects(
    source.workingCopy.save(file.selection, "saved draft"),
    /lost response/,
  );
  assert.equal(
    calls.filter((c) => c.url.endsWith("working-copy/save")).length,
    1,
  );
  assert.deepEqual(calls[2].body, { selection, contents: "saved draft" });
  assert.equal(
    (await source.workingCopy.read("README.md")).contents,
    "saved draft",
  );
  assert.deepEqual(await source.snapshot(), head);
  assert.equal(calls.filter((c) => c.url.endsWith("snapshot")).length, 1);
  source.close();
});
