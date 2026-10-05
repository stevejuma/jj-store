import { describe, expect, it } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import {
  workspaceSourceFiles,
  workspaceHunkSelection,
} from "../../src/lib/controller/stack-workspace/files.js";

const source = (name: string) => `branch:${JSON.stringify(["api", name])}`;
describe("inline branch segment changes", () => {
  it("selects only a branch's own files and preserves its exact diff through refresh", async () => {
    const controller = new StackWorkspaceController(
      new MockStackWorkspaceAdapter(workspaceFixture()),
    );
    await controller.refresh();
    (controller.selectStackFiles as (id: string, branch: string) => void)(
      "api",
      "feature/model",
    );
    expect(controller.stackSelections.api).toBe(source("feature/model"));
    expect(
      workspaceSourceFiles(controller.snapshot!, source("feature/model")).map(
        (file) => file.path,
      ),
    ).toEqual(["src/model.ts"]);
    expect(
      workspaceSourceFiles(controller.snapshot!, source("feature/api")).map(
        (file) => file.path,
      ),
    ).toEqual(["src/api.ts"]);
    controller.selectFile(source("feature/model"), "src/model.ts");
    expect(controller.selection).toBeNull();
    expect(controller.diffs[0]).toMatchObject({
      origin: "stack-api",
      source: source("feature/model"),
    });
    await controller.refresh();
    expect(controller.diffs[0]).toMatchObject({
      origin: "stack-api",
      source: source("feature/model"),
    });
    expect(
      workspaceHunkSelection(
        controller.snapshot!,
        source("feature/model"),
        "src/model.ts",
        "src/model.ts:repository model",
      )?.source,
    ).toBe("api1");
  });
});
