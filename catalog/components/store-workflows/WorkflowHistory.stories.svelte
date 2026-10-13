<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within, userEvent, waitFor } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow history",
    component: WorkflowsFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  const run = "51".repeat(16),
    home = "41".repeat(16);
  const selected = `#/workflows?tab=history&home=${home}&run=${run}&at=1`;
  async function checkFailure() {
    const body = within(document.body),
      toast = await body.findByRole("alert");
    await expect(toast).toHaveTextContent("Workflow history lookup failed");
    await expect(toast.closest('[data-ui-part="toast"]')).not.toBeNull();
    await userEvent.click(within(toast).getByRole("button", { name: /Retry/ }));
    await waitFor(() => expect(body.queryByRole("alert")).toBeNull());
  }
</script>

<Story
  name="History filters and detail preserve routes and browser navigation"
  args={{ initialHash: "#/workflows?tab=history" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await expect(
      await canvas.findByRole("button", { name: /^Mine/ }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: /^Other runs/ }),
    ).toBeVisible();
    await expect(
      await canvas.findByRole("button", {
        name: "View Server author's profile",
      }),
    ).toBeVisible();
    const row = canvas
      .getByRole("button", { name: /Open run Build parser 51515151/ })
      .closest(".workflow-history-row")!;
    await expect(getComputedStyle(row).boxShadow).toBe("none");
    await expect(parseFloat(getComputedStyle(row).paddingLeft)).toBeGreaterThan(
      8,
    );
    await userEvent.click(
      canvas.getByRole("button", { name: /Open run Build parser 51515151/ }),
    );
    await expect(
      await canvas.findByRole("region", { name: "Step console" }),
    ).toBeVisible();
    for (const name of ["Attempts", "Steps", "Outputs"])
      await expect(canvas.queryByRole("button", { name })).toBeNull();
    await userEvent.click(canvas.getByRole("button", { name: "Run output" }));
    await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
      "12 tests passed",
    );
    await expect(
      canvas.queryByRole("dialog", { name: /Run history/ }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to history" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Filters for history" }),
    );
    const filters = within(
      await body.findByRole("dialog", { name: "History filters" }),
    );
    await userEvent.click(
      filters.getByRole("combobox", { name: "Run status filter" }),
    );
    await userEvent.click(await body.findByRole("option", { name: /^failed/ }));
    await userEvent.click(
      filters.getByRole("button", { name: "Close filters" }),
    );
    await expect(
      await canvas.findByRole("button", {
        name: /Open run Build parser 53535353/,
      }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: /Open run Build parser 51515151/ }),
    ).toBeNull();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "state=failed",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Fixture back" }));
    await expect(
      await canvas.findByRole("button", {
        name: /Open run Build parser 51515151/,
      }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Fixture forward" }),
    );
    await expect(
      await canvas.findByRole("button", {
        name: /Open run Build parser 53535353/,
      }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: /Open run Build parser 51515151/ }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Filters for history" }),
    );
    await userEvent.click(
      await body.findByRole("button", { name: "Clear filters" }),
    );
    await userEvent.click(body.getByRole("button", { name: "Close filters" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Search history" }),
    );
    await userEvent.type(
      canvas.getByRole("textbox", { name: "Search history" }),
      "linux-builder{Enter}",
    );
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "q=linux-builder",
    );
    await expect(
      await canvas.findByRole("button", {
        name: /Open run Build parser 51515151/,
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="A saved run link restores pinned details and attempt selection"
  args={{ initialHash: selected }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await expect(
      await canvas.findByRole("region", { name: "Step console" }),
    ).toBeVisible();
    for (const name of ["Attempts", "Steps", "Outputs"])
      await expect(canvas.queryByRole("button", { name })).toBeNull();
    await userEvent.click(canvas.getByRole("button", { name: "Run output" }));
    await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
      "12 tests passed",
    );
    await expect(
      canvas.getByRole("button", { name: "Follow latest" }),
    ).toHaveAttribute("aria-pressed", "true");
    const details = canvas.getByRole("article", { name: "Run details" });
    await expect(
      details.querySelector(".community-projects__task-meta"),
    ).not.toBeNull();
    await userEvent.hover(
      canvas.getByRole("button", { name: "Follow latest" }),
    );
    await expect(await body.findByRole("tooltip")).toHaveTextContent(
      "Follow latest",
    );
    await userEvent.unhover(
      canvas.getByRole("button", { name: "Follow latest" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Run output" }));
    await expect(
      canvas.getByRole("button", { name: "Run output" }),
    ).toHaveAttribute("aria-expanded", "false");
    await waitFor(() =>
      expect(canvas.getByLabelText("stdout log")).not.toBeVisible(),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Run output" }));
    await expect(await canvas.findByLabelText("stdout log")).toBeVisible();
    await expect(
      canvas.getByRole("combobox", { name: "Run attempt" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run attempt" }),
    );
    await userEvent.click(
      await body.findByRole("option", { name: /Run attempt #1/ }),
    );
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "attempt=" + "52".repeat(16),
    );
    await userEvent.click(
      await canvas.findByRole("button", { name: "Run output" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Follow latest" }),
    );
    await waitFor(() =>
      expect(
        canvas.getByLabelText("Workflow location").textContent,
      ).not.toContain("at=1"),
    );
  }}
/>
<Story
  name="History read failure uses a retry toast"
  args={{ initialHash: "#/workflows?tab=history", historyFailure: "list" }}
  play={async ({ canvasElement }) => {
    await checkFailure();
    await expect(
      await within(canvasElement).findByRole("button", {
        name: /Open run Build parser 51515151/,
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Run detail read failure retains the route and retries through toast"
  args={{ initialHash: selected, historyFailure: "detail" }}
  play={async ({ canvasElement }) => {
    await checkFailure();
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Run output" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toBeVisible();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "run=" + run,
    );
  }}
/>
<Story
  name="Unavailable saved run does not select another run"
  args={{
    initialHash: `#/workflows?tab=history&home=${home}&run=${"99".repeat(16)}&at=1`,
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByText("Run unavailable at this publication"),
    ).toBeVisible();
    await expect(canvas.queryByLabelText("stdout log")).toBeNull();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "99".repeat(16),
    );
  }}
/>

<Story
  name="Pending run has header tooltips and collapsed empty sections"
  args={{ initialHash: selected, pendingRun: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const cancel = await canvas.findByRole("button", { name: "Cancel run" });
    await expect(cancel.closest("header")).not.toBeNull();
    await expect(cancel.textContent).toBe("");
    await userEvent.hover(cancel);
    await expect(await body.findByRole("tooltip")).toHaveTextContent(
      "Cancel run",
    );
    await userEvent.unhover(cancel);
    const step = canvas.getByRole("button", { name: "Step build · pending" });
    await expect(step).toHaveAttribute("aria-expanded", "false");
    await expect(
      canvas.queryByRole("button", { name: "Run output" }),
    ).toBeNull();
    await expect(canvas.queryByRole("button", { name: "Outputs" })).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Follow latest" }),
    );
    await userEvent.click(
      await canvas.findByRole("button", { name: "Pin this history" }),
    );
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "at=1",
    );
  }}
/>
<Story
  name="Oversized history search uses a toast and retains the current route"
  args={{ initialHash: "#/workflows?tab=history" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Search history" }),
    );
    await userEvent.type(
      canvas.getByRole("textbox", { name: "Search history" }),
      "界".repeat(172) + "{Enter}",
    );
    const toast = await body.findByRole("alert");
    await expect(toast).toHaveTextContent(
      "Search is limited to 512 UTF-8 bytes",
    );
    await expect(toast.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(
      canvas.getByLabelText("Workflow location"),
    ).not.toHaveTextContent("q=");
  }}
/>

<Story
  name="Repository discovery shows history rows instead of a metadata-home empty"
  args={{ initialHash: "#/workflows?tab=history", discovering: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(await canvas.findByLabelText("Loading history")).toBeVisible();
    await expect(
      canvas.queryByText("Choose a native metadata home"),
    ).toBeNull();
    await expect(canvas.getByRole("tab", { name: "History" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      canvasElement.querySelectorAll(".workflow-history-row"),
    ).toHaveLength(3);
  }}
/>
<Story
  name="Run reload skeleton follows detail header metadata and sections"
  args={{ initialHash: selected, discovering: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByLabelText("Loading run details"),
    ).toBeVisible();
    await expect(
      canvas.queryByText("Choose a native metadata home"),
    ).toBeNull();
    await expect(
      canvasElement.querySelector(".community-projects__task-detail-header"),
    ).not.toBeNull();
    await expect(
      canvasElement.querySelectorAll(".workflow-page-skeleton__step"),
    ).toHaveLength(3);
    await expect(
      canvasElement.querySelector(".workflow-page-skeleton__console"),
    ).not.toBeNull();
    await expect(
      canvasElement.querySelectorAll(".community-projects__task-meta > div"),
    ).toHaveLength(7);
  }}
/>
