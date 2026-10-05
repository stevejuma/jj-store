import { expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { CurrentWorkingTreeReads } from "../../src/lib/controller/stack-workspace/current-working-tree.js";
import { ProductionStackWorkspaceAdapter } from "../../src/lib/controller/stack-workspace/production-adapter.js";
import { WorkspacePhysicalObservation } from "../../src/lib/controller/stack-workspace/physical-observation.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type { StackWorkspaceAdapter } from "../../src/lib/controller/stack-workspace/types.js";
import { validateWorkingTreeObservation } from "../../src/lib/controller/working-tree-validation.js";
import { readFileSync } from "node:fs";

const repository = "b".repeat(32),
  operation = "a".repeat(128);
const binding = {
  repository,
  name: "Local",
  path: "/registered/folder",
  generation: 2,
  availability: "ready" as const,
};
function observation() {
  return {
    snapshot: {
      version: 1,
      repository,
      operation,
      binding_generation: 2,
      view: "c".repeat(128),
      workspace: "default",
      working_copy: "1".repeat(40),
      trees: ["2".repeat(40)],
      tree_labels: [],
      physical_workspace: "3".repeat(64),
      metadata_fingerprint: "4".repeat(64),
      physical_fingerprint: "5".repeat(64),
    },
    changes: [],
    obstacle_count: 0,
  };
}
function fixture() {
  const snapshot = {
    version: 1,
    repository,
    profile: "git_sha1_jj045" as const,
    generation: 2,
    heads: [operation],
    metadata_commit: null,
    actor: null,
  };
  const source = {
    repository,
    profile: "git_sha1_jj045",
    snapshot: vi.fn(async () => snapshot),
    workingTree: { observe: vi.fn(async () => observation()) },
    close: vi.fn(),
  } as unknown as StoreDataSource;
  const reads = new CurrentWorkingTreeReads(source, binding);
  return { source, snapshot, reads };
}
test("current checkout observation uses a fresh single head and revalidates it without changing history", async () => {
  const { source, reads } = fixture();
  const value = await reads.observe();
  expect(source.snapshot).toHaveBeenNthCalledWith(1, true);
  expect(source.snapshot).toHaveBeenNthCalledWith(2, true);
  expect(source.workingTree?.observe).toHaveBeenCalledWith(
    { bindingGeneration: 2, operation },
    expect.any(AbortSignal),
  );
  expect(value.snapshot.working_copy).toBe("1".repeat(40));
  reads.close();
  expect(source.close).not.toHaveBeenCalled();
});
test("production adapter physical reads preserve a distinct historical operation selection", async () => {
  const { source } = fixture();
  const historical = "e".repeat(128);
  const adapter = new ProductionStackWorkspaceAdapter(
    source,
    binding,
    historical,
  );
  expect((await adapter.observeWorkingTree()).snapshot.operation).toBe(
    operation,
  );
  expect(adapter.operation).toBe(historical);
  adapter.close();
  expect(source.close).not.toHaveBeenCalled();
});
test.each([
  "missing",
  "native",
  "generation",
  "repository",
  "no-head",
  "multi-head",
])("%s cannot select current physical authority", async (reason) => {
  const { source, snapshot, reads } = fixture();
  const observe = source.workingTree!.observe;
  if (reason === "missing") Object.assign(source, { workingTree: undefined });
  if (reason === "native") Object.assign(source, { profile: "native" });
  if (reason === "generation") snapshot.generation = 3;
  if (reason === "repository") snapshot.repository = "e".repeat(32);
  if (reason === "no-head") snapshot.heads = [];
  if (reason === "multi-head") snapshot.heads.push("f".repeat(128));
  await expect(reads.observe()).rejects.toBeInstanceOf(ApiError);
  expect(observe).not.toHaveBeenCalled();
  reads.close();
});
test("changed operation after observation is stale and never promoted", async () => {
  const { source, snapshot, reads } = fixture();
  source.workingTree!.observe = async () => {
    snapshot.heads = ["f".repeat(128)];
    return observation();
  };
  await expect(reads.observe()).rejects.toMatchObject({
    code: "stale_working_tree",
  });
  reads.close();
});
test("close/cancel immediately release owned waits and reject later shared snapshot or scan completion", async () => {
  for (const stage of ["snapshot", "observation"]) {
    const { source, reads } = fixture();
    let finish!: (value: any) => void;
    const held = new Promise<any>((resolve) => (finish = resolve));
    if (stage === "snapshot") source.snapshot = () => held;
    else source.workingTree!.observe = vi.fn(() => held);
    const abort = new AbortController();
    const pending = reads.observe(abort.signal);
    if (stage === "observation")
      await vi.waitFor(() =>
        expect(source.workingTree!.observe).toHaveBeenCalledTimes(1),
      );
    abort.abort();
    await expect(pending).rejects.toMatchObject({ name: "AbortError" });
    reads.close();
    finish(observation());
    await expect(reads.observe()).rejects.toMatchObject({ name: "AbortError" });
    expect(source.close).not.toHaveBeenCalled();
  }
});
test("observation owner rejects superseded and post-disposal results and clears failed evidence", async () => {
  const pending: { signal: AbortSignal; finish(value: any): void }[] = [];
  const adapter = {
    observeWorkingTree: (signal: AbortSignal) =>
      new Promise((finish) => pending.push({ signal, finish })),
  } as unknown as StackWorkspaceAdapter;
  const owner = new WorkspacePhysicalObservation(adapter);
  const older = owner.refresh();
  const newer = owner.refresh();
  expect(pending[0]!.signal.aborted).toBe(true);
  const value = observation();
  value.snapshot.physical_fingerprint = "f".repeat(64);
  pending[1]!.finish(value);
  await newer;
  pending[0]!.finish(observation());
  await older;
  expect(owner.observation?.snapshot.physical_fingerprint).toBe("f".repeat(64));
  adapter.observeWorkingTree = async () => {
    throw new ApiError("stale_working_tree");
  };
  await owner.refresh();
  expect(owner.observation).toBeNull();
  expect(owner.error).toBe("stale working tree");
  adapter.observeWorkingTree = (signal) =>
    new Promise((finish) => pending.push({ signal: signal!, finish }));
  const late = owner.refresh();
  owner.close();
  expect(pending[2]!.signal.aborted).toBe(true);
  pending[2]!.finish(observation());
  await late;
  expect(owner.observation).toBeNull();
  expect(owner.loading).toBe(false);
});
test.skipIf(!process.env.JJ_STORE_WORKING_OBSERVATION_CAPTURE)(
  "validator accepts actual cold server observation evidence",
  () => {
    const path = process.env.JJ_STORE_WORKING_OBSERVATION_CAPTURE;
    if (!path) throw new Error("Cold observation capture required");
    const responses = JSON.parse(readFileSync(path, "utf8"));
    const observed = responses.filter(
      (response: any) =>
        response.path.endsWith("/working-tree/observe") &&
        response.status === 200,
    );
    expect(observed.length).toBeGreaterThan(0);
    for (const response of observed) {
      const snapshot = response.data.snapshot;
      const value = validateWorkingTreeObservation(
        response.data,
        snapshot.repository,
        {
          operation: snapshot.operation,
          bindingGeneration: snapshot.binding_generation,
        },
      );
      expect(value.changes.length).toBeGreaterThan(0);
    }
  },
);
