import { afterEach, expect, test, vi } from "vitest";
import { recoverAdministration } from "../../src/lib/controller/server-agent-administration-recovery.js";
import { connectionBlocked } from "../../src/lib/connections/management.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";
import { ServerAgentAdministration } from "../../src/lib/controller/server-agent-administration.svelte.js";
import {
  newAgentDefinition,
  type ServerAgentReport,
} from "../../src/lib/controller/server-agent-administration-types.js";
const managers: ServerAgentAdministration[] = [];
afterEach(() => {
  for (const manager of managers.splice(0)) manager.destroy();
  vi.useRealTimers();
});
const principal = "11".repeat(16),
  repository = "22".repeat(16);
function fixture() {
  const agent = newAgentDefinition();
  agent.setup.name = "Builder";
  agent.principals = [principal];
  let report: ServerAgentReport = {
    revision: "aa".repeat(32),
    desired: {
      acp_executable: "/opt/agent",
      jj_executable: "/opt/jj",
      agents: [agent],
    },
    effective: null,
    restart_required: true,
    principals: [principal],
  };
  let lost = false,
    available = true,
    receipt: unknown = null;
  const calls: { path: string; body: Record<string, unknown> | undefined }[] =
    [];
  const fetcher: typeof fetch = vi.fn(async (input, init) => {
    const path = String(input),
      body = init?.body ? JSON.parse(String(init.body)) : undefined;
    calls.push({ path, body });
    const json = (value: unknown) =>
      new Response(JSON.stringify(value), {
        headers: { "content-type": "application/json" },
      });
    if (path === "/v1/operator/access")
      return json({ agents: true, identities: true });
    if (path === "/v1/operator/agents") {
      if (body) {
        report = {
          ...report,
          desired: body.configuration,
          revision: "bb".repeat(32),
        };
        if (lost) {
          lost = false;
          throw Error("lost response");
        }
      }
      return json(report);
    }
    if (path.startsWith("/v1/directory/profiles"))
      return json([{ principal, display_name: "Maya" }]);
    if (path === "/v1/directory/identities" && !body) return json([]);
    if (path === "/v1/directory/identities") {
      receipt = { request_id: body.request_id };
      if (lost) {
        lost = false;
        throw Error("lost response");
      }
      return json(receipt);
    }
    if (path.startsWith("/v1/directory/commands/")) return json(receipt);
    if (path.endsWith("agent-access") && !body)
      return json({
        epoch: "33".repeat(16),
        position: 0,
        heads: [],
        code_state_version: 0,
      });
    if (path.endsWith("agent-access/status"))
      return json({ request_id: body.request_id, published: true });
    if (path.endsWith("agent-access"))
      return json({ request_id: body.request_id, published: false });
    throw Error(`unexpected test path ${path}`);
  });
  const manager = new ServerAgentAdministration(
    () => available,
    () => "session-token",
    fetcher,
  );
  managers.push(manager);
  return {
    manager,
    calls,
    fetcher,
    set lost(value: boolean) {
      lost = value;
    },
    set available(value: boolean) {
      available = value;
    },
    update() {
      report = { ...report, revision: "cc".repeat(32) };
    },
  };
}
test("sharing drafts retain their base revision across refresh; rebasing is explicit", async () => {
  const f = fixture();
  await f.manager.refresh();
  expect(f.manager.profiles[principal]).toBe("Maya");
  f.manager.draft!.agents[0]!.setup.name = "Edited builder";
  f.update();
  await f.manager.refresh();
  expect(f.manager.draft!.agents[0]!.setup.name).toBe("Edited builder");
  expect(f.manager.baseRevision).toBe("aa".repeat(32));
  expect(f.manager.conflict).toBe(true);
  await f.manager.save();
  expect(f.calls.filter((c) => c.body)).toHaveLength(0);
  f.manager.useLatestRevision();
  await f.manager.save();
  expect(
    f.calls.find((c) => c.path === "/v1/operator/agents" && c.body)?.body,
  ).toMatchObject({ expected_revision: "cc".repeat(32) });
  expect(f.manager.pending).toBeNull();
  expect(f.manager.dirty).toBe(false);
});
test("lost configuration response retains original intent and recovery sends no new save", async () => {
  const f = fixture();
  await f.manager.refresh();
  f.manager.draft!.agents[0]!.principals = ["44".repeat(16)];
  f.lost = true;
  await f.manager.save();
  expect(f.manager.pending).toMatchObject({
    kind: "configuration",
    body: { expected_revision: "aa".repeat(32) },
  });
  f.manager.reset();
  expect(f.manager.pending).not.toBeNull();
  await f.manager.checkOutcome();
  expect(f.manager.pending).toBeNull();
  expect(
    f.calls.filter((c) => c.path === "/v1/operator/agents" && c.body),
  ).toHaveLength(1);
});
test("lost enrollment acknowledgement recovers the same request without replay", async () => {
  const f = fixture();
  await f.manager.refresh();
  f.lost = true;
  await f.manager.enroll(principal, "user-credential");
  const original = f.manager.pending;
  expect(original).toMatchObject({
    kind: "identity",
    body: { principal, credential: "user-credential" },
  });
  await f.manager.checkOutcome();
  expect(f.manager.pending).toBeNull();
  expect(
    f.calls.filter((c) => c.path === "/v1/directory/identities" && c.body),
  ).toHaveLength(1);
});
test("unknown repository grant status uses its exact snapshot request and credential", async () => {
  const f = fixture();
  await f.manager.refresh();
  await f.manager.grant(repository, principal, "user-credential");
  const submitted = f.calls.find(
    (c) => c.path === `/v1/repos/${repository}/agent-access` && c.body,
  )!;
  expect(f.manager.pending).toMatchObject({ kind: "grant" });
  await f.manager.checkOutcome();
  expect(
    f.calls.find((c) => c.path.endsWith("/agent-access/status"))!.body,
  ).toEqual(submitted.body);
  expect(f.manager.pending).toBeNull();
});
test("inactive administration pauses periodic reads; connection owners retain independent drafts", async () => {
  vi.useFakeTimers();
  const a = fixture(),
    b = fixture();
  await a.manager.refresh();
  await b.manager.refresh();
  a.manager.draft!.agents[0]!.setup.name = "Only A";
  expect(b.manager.draft!.agents[0]!.setup.name).toBe("Builder");
  a.manager.open = true;
  a.available = false;
  const before = a.calls.length;
  await vi.advanceTimersByTimeAsync(11000);
  expect(a.calls).toHaveLength(before);
});
test("late capability discovery cannot restore a destroyed owner", async () => {
  let resolve!: (value: Response) => void;
  const manager = new ServerAgentAdministration(
    () => true,
    () => "token",
    () => new Promise((r) => (resolve = r)),
  );
  managers.push(manager);
  const pending = manager.discover();
  manager.destroy();
  resolve(
    new Response(JSON.stringify({ agents: true, identities: true }), {
      headers: { "content-type": "application/json" },
    }),
  );
  await pending;
  expect(manager.access).toBeNull();
  expect(manager.report).toBeNull();
});

test("set ordering cannot leave a committed configuration save unresolved", async () => {
  const definition = newAgentDefinition();
  definition.principals = ["22".repeat(16), "11".repeat(16)];
  const configuration = {
    acp_executable: "/opt/acp",
    jj_executable: "/opt/jj",
    agents: [definition],
  };
  const report: ServerAgentReport = {
    revision: "bb".repeat(32),
    desired: structuredClone(configuration),
    effective: null,
    restart_required: true,
    principals: [],
  };
  report.desired!.agents[0]!.principals.sort();
  const result = await recoverAdministration(async <T>() => report as T, {
    kind: "configuration",
    body: { expected_revision: "aa".repeat(32), configuration },
  });
  expect(result.error).toBeUndefined();
});
test("dirty and pending administration prevents connection changes until resolved", async () => {
  const f = fixture();
  await f.manager.refresh();
  const controller = { agentAdministration: f.manager } as StoreController;
  expect(connectionBlocked(controller)).toBe(false);
  f.manager.updateDefinition({
    ...f.manager.draft!.agents[0]!,
    setup: { ...f.manager.draft!.agents[0]!.setup, name: "Retained draft" },
  });
  expect(connectionBlocked(controller)).toBe(true);
  f.lost = true;
  await f.manager.save();
  expect(connectionBlocked(controller)).toBe(true);
  await f.manager.checkOutcome();
  expect(connectionBlocked(controller)).toBe(false);
});

test("visible agent directory refreshes only operator capabilities; hidden views pause", async () => {
  vi.useFakeTimers();
  const f = fixture();
  f.manager.directoryOpen = true;
  await f.manager.discover();
  await vi.advanceTimersByTimeAsync(11000);
  expect(f.calls.filter((c) => c.path === "/v1/operator/access")).toHaveLength(
    3,
  );
  expect(f.calls.filter((c) => c.path === "/v1/operator/agents")).toHaveLength(
    0,
  );
  f.manager.directoryOpen = false;
  await vi.advanceTimersByTimeAsync(11000);
  expect(f.calls).toHaveLength(3);
});
