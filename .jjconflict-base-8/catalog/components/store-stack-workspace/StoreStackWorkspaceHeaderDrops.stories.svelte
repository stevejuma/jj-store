<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    waitFor,
    within,
  } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import { WORKSPACE_DRAG_TYPE } from "../../../web/src/lib/controller/stack-workspace/drag.js";
  import {
    applyPreview,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Header drops",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  function header(element: HTMLElement, name: string) {
    return workspaceCanvas(element)
      .getByRole("button", { name: `Show all changes in ${name}` })
      .closest<HTMLElement>('[data-ui-part="card-header"]')!;
  }
  function row(element: HTMLElement, name: string) {
    return workspaceCanvas(element)
      .getByRole("button", { name })
      .closest<HTMLElement>("[data-revision-id]")!;
  }
  async function start(source: HTMLElement, target: HTMLElement) {
    const dataTransfer = new DataTransfer();
    await fireEvent(
      source,
      new DragEvent("dragstart", { bubbles: true, dataTransfer }),
    );
    await fireEvent(
      target,
      new DragEvent("dragover", {
        bubbles: true,
        cancelable: true,
        dataTransfer,
      }),
    );
    return dataTransfer;
  }
  async function drop(target: HTMLElement, dataTransfer: DataTransfer) {
    await fireEvent(
      target,
      new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer }),
    );
  }
  function readableGuide(target: HTMLElement) {
    const guide = within(target).getByRole("status", { name: "Drop guide" });
    const style = getComputedStyle(guide);
    expect(style.backgroundColor).not.toBe("rgba(0, 0, 0, 0)");
    expect(style.backgroundColor).not.toMatch(/rgba\(.*,[\s]*0\.[\d]+\)/);
    expect(Number(style.zIndex)).toBeGreaterThan(
      Number(getComputedStyle(target, "::after").zIndex) || 0,
    );
    return guide;
  }
</script>

<Story
  name="Staged controls align to the header edge"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Create commit in feature/api",
      }),
    );
    const staged = canvas.getByRole("group", {
      name: "Staged files for feature/api",
    });
    const toggle = within(staged).getByRole("group", {
      name: "File listing view mode",
    });
    const heading = staged.querySelector<HTMLElement>(
      '[data-ui-part="card-header"]',
    )!;
    expect(
      Math.abs(
        toggle.getBoundingClientRect().right -
          (heading.getBoundingClientRect().right -
            parseFloat(getComputedStyle(heading).paddingRight)),
      ),
    ).toBeLessThan(1);
    expect(toggle.getBoundingClientRect().left).toBeGreaterThan(
      within(staged)
        .getByText("Staged", { exact: true })
        .getBoundingClientRect().right,
    );
  }}
/>

<Story
  name="Inline files and selected rows use inset spacing and left markers"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const branch = header(canvasElement, "feature/api");
    await userEvent.click(branch);
    const files = canvas.getByRole("region", {
      name: "Changed files in branch feature/api",
    });
    const card = branch.closest<HTMLElement>(
      ".stack-workspace__bookmark-card",
    )!;
    expect(
      files.getBoundingClientRect().left - card.getBoundingClientRect().left,
    ).toBeGreaterThanOrEqual(8);
    expect(
      files.getBoundingClientRect().top - branch.getBoundingClientRect().bottom,
    ).toBeGreaterThanOrEqual(8);
    expect(
      card.getBoundingClientRect().right - files.getBoundingClientRect().right,
    ).toBeGreaterThanOrEqual(8);
    expect(getComputedStyle(branch).borderBottomWidth).toBe("0px");
    expect(
      Math.abs(
        files.getBoundingClientRect().left -
          within(branch)
            .getByRole("button", { name: "Show all changes in feature/api" })
            .getBoundingClientRect().left,
      ),
    ).toBeLessThan(1);
    const style = getComputedStyle(branch, "::before");
    const marker = {
      content: style.content,
      width: style.width,
      height: style.height,
      borderTopRightRadius: style.borderTopRightRadius,
      backgroundColor: style.backgroundColor,
    };
    expect(marker.content).not.toBe("none");
    expect(marker.width).toBe("4px");
    expect(marker.height).toBe("48px");
    expect(marker.borderTopRightRadius).not.toBe("0px");
    await userEvent.click(
      canvas.getByRole("button", { name: "Expose repository API" }),
    );
    const selected = row(canvasElement, "Expose repository API");
    const rowMarker = getComputedStyle(selected, "::before");
    expect(rowMarker.content).not.toBe("none");
    expect(rowMarker.width).toBe(marker.width);
    expect(rowMarker.height).toBe(marker.height);
    expect(rowMarker.backgroundColor).toBe(marker.backgroundColor);
    expect(getComputedStyle(branch, "::before").content).toBe("none");
    expect(
      within(selected).queryByText("Selected", { exact: true }),
    ).toBeNull();
  }}
/>

<Story
  name="Drop guides have an opaque foreground background"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const file = await canvas.findByRole("button", { name: "README.md" });
    const stage = canvas.getByRole("group", {
      name: "Commit composer for feature/api",
    });
    const transfer = await start(file, stage);
    expect(stage).toHaveAttribute("data-drop-target-state", "valid");
    expect(readableGuide(stage)).toHaveTextContent(
      "Stage changes for a new commit",
    );
    await fireEvent(
      file,
      new DragEvent("dragend", { bubbles: true, dataTransfer: transfer }),
    );
    expect(stage).toHaveAttribute("data-drop-target-state", "idle");
  }}
/>

<Story
  name="Commit drops into the selected branch segment"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const source = row(canvasElement, "Add workspace toolbar"),
      destination = header(canvasElement, "feature/model");
    const original = source.closest(".stack-workspace__bookmark-card");
    const transfer = await start(source, destination);
    expect(destination).toHaveAttribute("data-drop-target-state", "valid");
    expect(readableGuide(destination)).toHaveTextContent("feature/model");
    expect(
      canvas.queryAllByRole("status", { name: "Drop guide" }),
    ).toHaveLength(1);
    const title = within(destination).getByRole("button", {
      name: "Show all changes in feature/model",
    });
    await fireEvent(
      destination,
      new DragEvent("dragleave", {
        bubbles: true,
        relatedTarget: title,
        dataTransfer: transfer,
      }),
    );
    expect(destination).toHaveAttribute("data-drop-target-state", "valid");
    await drop(destination, transfer);
    expect(destination).toHaveAttribute("data-drop-target-state", "idle");
    expect((await portal.findByRole("dialog")).textContent).toContain(
      "feature/model",
    );
    expect(
      row(canvasElement, "Add workspace toolbar").closest(
        ".stack-workspace__bookmark-card",
      ),
    ).toBe(original);
    await applyPreview(canvasElement);
    expect(
      row(canvasElement, "Add workspace toolbar").closest(
        ".stack-workspace__bookmark-card",
      ),
    ).toBe(destination.closest(".stack-workspace__bookmark-card"));
    expect(
      row(canvasElement, "Expose repository API").closest(
        ".stack-workspace__bookmark-card",
      ),
    ).not.toBe(destination.closest(".stack-workspace__bookmark-card"));
  }}
/>

<Story
  name="Branch headers reject immutable and self drops"
  args={{ immutable: true }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const source = row(canvasElement, "Add workspace toolbar"),
      destination = header(canvasElement, "feature/model");
    const immutable = row(canvasElement, "Prepare add repository model");
    expect(immutable).toHaveAttribute("draggable", "false");
    const transfer = new DataTransfer();
    transfer.setData(
      WORKSPACE_DRAG_TYPE,
      JSON.stringify({ kind: "commit", commit: "api1-setup" }),
    );
    await fireEvent(
      destination,
      new DragEvent("dragover", {
        bubbles: true,
        cancelable: true,
        dataTransfer: transfer,
      }),
    );
    expect(destination).toHaveAttribute("data-drop-target-state", "invalid");
    expect(readableGuide(destination)).toHaveTextContent("immutable");
    expect(transfer.dropEffect).toBe("none");
    await drop(destination, transfer);
    expect(portal.queryByRole("dialog")).toBeNull();
    const same = header(canvasElement, "feature/workspace");
    const next = await start(source, same);
    expect(same).toHaveAttribute("data-drop-target-state", "invalid");
    expect(readableGuide(same)).toHaveTextContent("another branch");
    await drop(same, next);
    expect(portal.queryByRole("dialog")).toBeNull();
  }}
/>

<Story
  name="Commit row thresholds show sortable insertion borders"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    const source = row(canvasElement, "Prepare add repository model"),
      target = row(canvasElement, "Expose repository API");
    const before = target.getBoundingClientRect();
    const transfer = await start(source, target);
    const over = async (ratio: number) =>
      fireEvent(
        target,
        new DragEvent("dragover", {
          bubbles: true,
          cancelable: true,
          dataTransfer: transfer,
          clientY: before.top + before.height * ratio,
        }),
      );
    await over(0.2);
    expect(target).toHaveAttribute("data-drop-placement", "above");
    expect(getComputedStyle(target, "::after").borderTopWidth).toBe("2px");
    expect(readableGuide(target)).toHaveTextContent("above");
    await over(0.5);
    expect(target).not.toHaveAttribute("data-drop-placement");
    expect(readableGuide(target)).toHaveTextContent("Squash");
    await over(0.8);
    expect(target).toHaveAttribute("data-drop-placement", "below");
    expect(getComputedStyle(target, "::after").borderBottomWidth).toBe("2px");
    await over(0.2);
    await fireEvent(
      target,
      new DragEvent("drop", {
        bubbles: true,
        cancelable: true,
        dataTransfer: transfer,
        clientY: before.top + before.height * 0.2,
      }),
    );
    expect(target).not.toHaveAttribute("data-drop-placement");
    expect((await portal.findByRole("dialog")).textContent).toContain("above");
    await applyPreview(canvasElement);
    const graph = canvas.getByRole("list", { name: "Commits in feature/api" });
    expect(
      Array.from(graph.querySelectorAll<HTMLElement>("[data-revision-id]")).map(
        (row) => row.dataset.revisionId,
      ),
    ).toEqual(["api1-setup", "api2", "api2-setup"]);
    expect(
      row(canvasElement, "Prepare add repository model").getBoundingClientRect()
        .top,
    ).toBeLessThan(
      row(canvasElement, "Expose repository API").getBoundingClientRect().top,
    );
  }}
/>
