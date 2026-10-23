<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within, userEvent, waitFor } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const repository = "42".repeat(16),
    home = "41".repeat(16),
    run = "51".repeat(16),
    base = `#/repos/${repository}/actions`;
  const { Story } = defineMeta({
    title: "JJ Store/Repository Actions",
    component: WorkflowsFixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    args: { shell: true, historyExamples: true, initialHash: base },
  });
</script>

<Story
  name="Repository Actions scopes the shared split view and restores pinned details"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await expect(
      await canvas.findByRole("tab", { name: "Actions" }),
    ).toHaveAttribute("aria-selected", "true");
    const nav = within(
      await canvas.findByRole("navigation", {
        name: "Workflows by repository",
      }),
    );
    await expect(
      getComputedStyle(
        canvasElement.querySelector(
          '.store-application__body-regions[data-page="actions"]',
        )!,
      ).borderTopWidth,
    ).toBe("1px");
    await expect(
      getComputedStyle(canvasElement.querySelector(".workflow-history-layout")!)
        .borderTopWidth,
    ).toBe("0px");
    await expect(
      nav.getByRole("button", { name: "All workflows" }),
    ).toHaveAttribute("aria-current", "page");
    await expect(
      nav.getByRole("group", { name: "Parser workflows" }),
    ).toBeVisible();
    await expect(
      nav.queryByRole("group", { name: "Automation home workflows" }),
    ).toBeNull();
    await expect(
      canvas.queryByRole("button", { name: /Open run Publish preview/ }),
    ).toBeNull();
    await userEvent.click(nav.getByRole("button", { name: "Build parser" }));
    await expect(
      await canvas.findByRole("button", {
        name: /Open run Build parser 51515151/,
      }),
    ).toBeVisible();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      base + "?scope=" + home,
    );
    await userEvent.click(nav.getByRole("button", { name: "All workflows" }));
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run author filter" }),
    );
    await userEvent.click(
      await body.findByRole("option", { name: /^Server author/ }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: /Open run Build parser 51515151/ }),
    );
    await expect(
      await canvas.findByRole("article", { name: "Run details" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("complementary", { name: "Workflow navigation" }),
    ).toBeNull();
    await expect(canvas.queryByRole("tab", { name: "Actions" })).toBeNull();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      `home=${home}&run=${run}&at=1`,
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to history" }),
    );
    await expect(
      await canvas.findByRole("tab", { name: "Actions" }),
    ).toBeVisible();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "author=" + "22".repeat(16),
    );
    await storyControl(document.body, "Fixture back");
    await expect(
      await canvas.findByRole("article", { name: "Run details" }),
    ).toBeVisible();
    await storyControl(document.body, "Fixture forward");
    await expect(
      await canvas.findByRole("navigation", {
        name: "Workflows by repository",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Saved repository Actions link restores a run and refuses foreign repository output"
  args={{ initialHash: base + `?home=${home}&run=${"55".repeat(16)}&at=1` }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByText("Run unavailable in this repository"),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("region", { name: "Step console" }),
    ).toBeNull();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "run=" + "55".repeat(16),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to history" }),
    );
    await expect(
      await canvas.findByRole("navigation", {
        name: "Workflows by repository",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Repository Actions is a native tab reachable from repository overview"
  args={{ initialHash: `#/repos/${repository}/files` }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(await canvas.findByRole("tab", { name: "Actions" }));
    await expect(
      await canvas.findByRole("button", {
        name: /Open run Build parser 51515151/,
      }),
    ).toBeVisible();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      base,
    );
    await userEvent.click(canvas.getByRole("button", { name: "Search runs" }));
    await userEvent.type(
      canvas.getByRole("textbox", { name: "Search runs" }),
      "linux-builder{Enter}",
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
        "q=linux-builder",
      ),
    );
  }}
/>
