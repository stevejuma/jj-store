import { expect, test } from "vitest";
import { createInMemoryStoreController } from "./in-memory-store-controller.svelte.js";
import { IDS } from "../fixtures/repository.js";
test("controller_restores_snapshot_history", () => {
  const controller = createInMemoryStoreController();
  controller.actions.openRepository("jj-store");
  controller.actions.selectBookmark("feature/native-objects");
  controller.actions.selectFile("src/lib.rs");
  expect(controller.state.snapshot?.id).toBe(IDS.left);
  expect(controller.state.blob?.contents).toContain("stream_native_objects");
  controller.actions.selectBookmark("main");
  expect(controller.state.route.path).toBeNull();
  controller.actions.navigate(-1);
  expect(controller.state.route.path).toBe("src/lib.rs");
  expect(controller.state.snapshot?.id).toBe(IDS.left);
  controller.actions.navigate(1);
  expect(controller.state.snapshot?.id).toBe(IDS.current);
});
test("working_copy_card_scopes_branch_history_to_its_native_commit", () => {
  const controller = createInMemoryStoreController();
  controller.actions.selectWorkingCopy(IDS.current);
  expect(controller.state.route.bookmark).toBe("");
  expect(controller.state.route.commitId).toBe(IDS.current);
  expect(controller.state.targets).toEqual([IDS.current]);
  expect(controller.state.graph.revisions[0]?.eventId).toBe(IDS.current);
});
test("controller_isolates_instances_and_conflicts", () => {
  const a = createInMemoryStoreController(),
    b = createInMemoryStoreController();
  a.actions.openRepository("jj-store");
  a.actions.selectBookmark("conflicted");
  expect(a.state.snapshot).toBeUndefined();
  expect(a.state.targets).toEqual([IDS.left, IDS.right]);
  a.actions.selectRevision(IDS.right);
  expect(a.state.snapshot?.id).toBe(IDS.right);
  a.actions.signOut();
  expect(a.state.authenticated).toBe(false);
  expect(b.state.authenticated).toBe(true);
  expect(b.state.route.page).toBe("branches");
  a.actions.openRepository("native-demo");
  expect(a.state.route.path).toBeNull();
  expect(a.state.repository.id).toBe("native-demo");
});

test("controller_restores_directory_history", () => {
  const controller = createInMemoryStoreController("files");
  controller.actions.selectDirectory("src");
  controller.actions.selectFile("src/lib.rs");
  controller.actions.navigate(-1);
  expect(controller.state.route.directory).toBe("src");
  expect(controller.state.route.path).toBeNull();
  controller.actions.navigate(1);
  expect(controller.state.route.path).toBe("src/lib.rs");
  controller.actions.selectBookmark("main");
  expect(controller.state.route.directory).toBe("");
  expect(() => controller.actions.selectDirectory("missing")).toThrow(
    "Directory absent",
  );
});
