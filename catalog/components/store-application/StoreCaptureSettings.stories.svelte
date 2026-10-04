<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import CaptureSettingsFixture from "../../fixtures/CaptureSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Capture",
    component: CaptureSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Operational errors use dismissed scoped notifications"
  args={{ feedbackControls: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await expect(
      await c.findByText("/Users/capture/.codex/sessions"),
    ).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Previous import request ID" }),
      "ef".repeat(16),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Fail capture observations" }),
    );
    await expect(
      await page.findByRole("button", {
        name: "Dismiss Capture request unavailable",
      }),
    ).toBeVisible();
    await expect(
      canvasElement.querySelector("[data-notification-toasts]"),
    ).toBeNull();
    await expect(page.getAllByRole("alert")).toHaveLength(1);
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await expect(page.getAllByRole("alert")).toHaveLength(1);
    await userEvent.click(
      page.getByRole("button", { name: "Dismiss Capture request unavailable" }),
    );
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Diagnostics" }));
    await userEvent.click(
      c.getByRole("button", { name: "Toggle capture page" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle capture page" }),
    );
    await expect(page.queryByRole("alert")).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Recover capture observations" }),
    );
    await expect(
      c.getByRole("button", { name: "Pause capture" }),
    ).toBeEnabled();
    await userEvent.click(
      c.getByRole("button", { name: "Fail capture observations" }),
    );
    await expect(
      await page.findByRole("button", {
        name: "Dismiss Capture request unavailable",
      }),
    ).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Recover capture observations" }),
    );
    await expect(page.queryByRole("alert")).toBeNull();
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    await expect(
      c.getByRole("textbox", { name: "Previous import request ID" }),
    ).toHaveValue("ef".repeat(16));
  }}
/>

<Story
  name="Coverage pause resume rescan and import"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("button", { name: "Refresh capture" }),
    ).toBeEnabled();
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await userEvent.click(c.getByRole("button", { name: "Pause capture" }));
    await expect(
      c.getByRole("button", { name: "Rescan capture" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Resume capture" }));
    await userEvent.click(c.getByRole("button", { name: "Rescan capture" }));
    await expect(
      within(canvasElement.ownerDocument.body).getByText(/Rescan requested/),
    ).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await expect(
      c.getByRole("button", { name: /Import history for/ }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await userEvent.click(
      c.getByRole("button", { name: /Import history for/ }),
    );
    await expect(c.getByText("completed · Boundary available")).toBeVisible();
    await expect(c.getByText("2,048 / 1,024 bytes")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Refresh import status" }),
    );
    await expect(c.getByText("History acceptance")).toBeVisible();
  }}
/>
<Story
  name="Unknown control keeps original request"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await userEvent.click(c.getByRole("button", { name: "Pause capture" }));
    await expect(
      c.getByRole("button", { name: "Pause capture" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await expect(c.getByText("Capture outcome unknown")).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Retry original capture request" }),
    ).toBeEnabled();
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await userEvent.click(
      c.getByRole("button", { name: "Retry original capture request" }),
    );
    await expect(c.getByText("Capture outcome unknown")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Retry original capture request" }),
    );
    await expect(
      c.getByRole("button", { name: "Resume capture" }),
    ).toBeEnabled();
    await expect(
      c.queryByText("Capture outcome unknown"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Unknown import receipt inspection"
  args={{ importUnknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await userEvent.click(
      c.getByRole("button", { name: /Import history for/ }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Check original import receipt" }),
    );
    await expect(
      c.queryByText("Capture outcome unknown"),
    ).not.toBeInTheDocument();
    await expect(c.getByText("completed · Boundary available")).toBeVisible();
  }}
/>
<Story
  name="Disabled"
  args={{ enabled: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await expect(
      c.getByText("Transcript capture is disabled in server configuration."),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Pause capture" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Unsupported"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByText("Capture management is not available for this connection."),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Refresh capture" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Degraded capacity and missing receipt"
  args={{ degraded: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Diagnostics" }));
    await expect(
      c.getByText("1 degraded · 0 unavailable observations"),
    ).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Previous import request ID" }),
      "ef".repeat(16),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Inspect import request" }),
    );
    await expect(c.getByText("No receipt available")).toBeVisible();
    await expect(
      c.getByText(/This does not prove rejection or completion/),
    ).toBeVisible();
  }}
/>

<Story
  name="Automatic load and retained request draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Previous import request ID" }),
      "ef".repeat(16),
    );
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await expect(
      c.getByRole("textbox", { name: "Previous import request ID" }),
    ).toHaveValue("ef".repeat(16));
  }}
/>

<Story
  name="Shared settings rows and action columns"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
    await expect(
      c.getByRole("heading", { name: "Agents", level: 1 }),
    ).toBeVisible();
    const refresh = c.getByRole("button", { name: "Refresh capture" });
    await expect(refresh.closest(".store-settings__tab-bar")).not.toBeNull();
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    const pause = c.getByRole("button", { name: "Pause capture" });
    const rescan = c.getByRole("button", { name: "Rescan capture" });
    const actions = pause.closest(".store-settings__actions")!;
    await expect(actions.parentElement?.parentElement).toHaveClass(
      "store-settings__rows",
    );
    await expect(getComputedStyle(actions).flexDirection).toBe("column");
    await expect(pause.getBoundingClientRect().width).toBe(
      rescan.getBoundingClientRect().width,
    );
    await expect(rescan.getBoundingClientRect().top).toBeGreaterThan(
      pause.getBoundingClientRect().bottom,
    );
    await userEvent.click(c.getByRole("tab", { name: "Sources" }));
    const sourceAction = c.getByRole("button", { name: /Import history for/ });
    await expect(sourceAction.closest("dl")).toHaveClass(
      "store-settings__rows",
    );
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    const request = c.getByRole("textbox", {
      name: "Previous import request ID",
    });
    await expect(request.closest(".store-settings__rows")).not.toBeNull();
  }}
/>

<Story
  name="Configured CLI root directories"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByText("/Users/capture/.codex/sessions", { exact: true }),
    ).toBeVisible();
    const paths = c.getByText("/Users/capture/.codex/sessions", {
      exact: true,
    });
    await expect(paths.closest("dl")).toHaveClass("store-settings__rows");
    await expect(paths.closest("code")).not.toBeNull();
  }}
/>

<Story
  name="Tabs retain drafts and separate diagnostics"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByRole("tab", { name: "Capture" })).toHaveAttribute(
      "data-state",
      "active",
    );
    await expect(c.queryByText("Publication backlog")).toBeNull();
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Previous import request ID" }),
      "ef".repeat(16),
    );
    await userEvent.click(c.getByRole("tab", { name: "Diagnostics" }));
    await expect(c.getByText("Publication backlog")).toBeVisible();
    await expect(
      c.queryByRole("textbox", { name: "Previous import request ID" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("tab", { name: "Imports" }));
    await expect(
      c.getByRole("textbox", { name: "Previous import request ID" }),
    ).toHaveValue("ef".repeat(16));
  }}
/>

<Story
  name="Edit disabled local capture configuration"
  args={{ enabled: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
    const enabled = await c.findByRole("switch", {
      name: "Enable automatic capture",
    });
    await userEvent.click(enabled);
    const roots = c.getByRole("textbox", { name: "Codex CLI folders" });
    await userEvent.type(roots, "/tmp/agent-settings/sessions");
    await userEvent.click(c.getByRole("tab", { name: "Diagnostics" }));
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
    await expect(
      c.getByRole("textbox", { name: "Codex CLI folders" }),
    ).toHaveValue("/tmp/agent-settings/sessions");
    await userEvent.click(
      c.getByRole("button", { name: "Save configuration" }),
    );
    await expect(c.getByText("Disabled · Restart required")).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Save configuration" }),
    ).toBeDisabled();
    const actions = c
      .getByRole("button", { name: "Cancel changes" })
      .closest(".store-settings__actions")!;
    await expect(getComputedStyle(actions).flexDirection).toBe("column");
  }}
/>
<Story
  name="Stale configuration keeps the draft"
  args={{ configurationFailure: "stale" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
    await userEvent.click(
      await c.findByRole("switch", { name: "Enable automatic capture" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Save configuration" }),
    );
    await expect(
      await page.findByRole("button", {
        name: "Dismiss Agent configuration needs attention",
      }),
    ).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Save configuration" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("switch", { name: "Enable automatic capture" }),
    ).not.toBeChecked();
  }}
/>
<Story
  name="Unknown save resolves after observation"
  args={{ configurationFailure: "unknown" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
    await userEvent.click(
      await c.findByRole("switch", { name: "Enable automatic capture" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Save configuration" }),
    );
    await expect(c.getByText(/Save outcome unknown\. Refresh/)).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await expect(c.queryByText(/Save outcome unknown\. Refresh/)).toBeNull();
    await expect(c.getByText("Enabled · Restart required")).toBeVisible();
  }}
/>
