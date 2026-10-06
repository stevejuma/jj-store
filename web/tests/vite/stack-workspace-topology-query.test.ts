import { expect, test, vi } from "vitest";
import { StackTopologyQuery } from "../../src/lib/controller/stack-workspace/topology-query.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";

test("pre-setup topology is pinned, cached per target/operation and materializes unloaded branch identities", async () => {
  const snapshot = workspaceFixture();
  snapshot.stacks = [];
  const topology = {
    complete: true,
    integrated: [],
    lanes: [],
    inventory: [
      {
        applied: [],
        unapply: [],
        heads: [
          {
            names: ["upper"],
            commit: "tip",
            base_commit: "lower",
            shared: false,
          },
          {
            names: ["lower", "alias"],
            commit: "lower",
            base_commit: "fork",
            shared: false,
          },
        ],
      },
    ],
  };
  const request = vi.fn(async (args) => ({
    version: 1,
    operation: args.operation,
    target: args.target,
    target_commit: args.target_commit,
    topology,
  }));
  const context = {
    source: { stacks: { topology: request, execute: vi.fn() } },
    binding: { generation: 7n },
    pin: () => ({ operation: "a".repeat(128) }),
    signal: new AbortController().signal,
  };
  const query = new StackTopologyQuery();
  await query.project(snapshot, context);
  expect(request).toHaveBeenCalledWith({
    version: 1,
    operation: "a".repeat(128),
    binding_generation: 7n,
    target: snapshot.target.name,
    target_commit: snapshot.target.commit,
  });
  expect(context.source.stacks.execute).not.toHaveBeenCalled();
  expect(snapshot.stacks.map((s) => s.name)).toEqual([
    "upper",
    "lower",
    "alias",
  ]);
  expect(snapshot.stacks.every((s) => !s.commits.length)).toBe(true);
  snapshot.topology = undefined;
  await query.project(snapshot, context);
  expect(request).toHaveBeenCalledTimes(1);
  snapshot.topology = undefined;
  snapshot.target = { ...snapshot.target, commit: "new-target" };
  await query.project(snapshot, context);
  expect(request).toHaveBeenCalledTimes(2);
});

test("mismatched operation and cancelled queries cannot install discovery", async () => {
  const snapshot = workspaceFixture(),
    query = new StackTopologyQuery();
  const owner = new AbortController();
  const context = {
    source: {
      stacks: {
        execute: vi.fn(),
        topology: vi.fn(async () => ({
          version: 1,
          operation: "wrong",
          target: snapshot.target.name,
          target_commit: snapshot.target.commit,
          topology: {
            complete: true,
            inventory: [],
            lanes: [],
            integrated: [],
          },
        })),
      },
    },
    binding: { generation: 1 },
    pin: () => ({ operation: "a".repeat(128) }),
    signal: owner.signal,
  };
  await expect(query.project(snapshot, context)).rejects.toThrow();
  expect(snapshot.topology).toBeUndefined();
  owner.abort();
  await expect(query.project(snapshot, context)).rejects.toThrow();
  expect(snapshot.topology).toBeUndefined();
});
