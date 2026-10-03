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
  name="Coverage pause resume rescan and import"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("button", { name: "Refresh capture" }),
    ).toBeEnabled();
    await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Pause capture" }));
    await expect(
      c.getByRole("button", { name: "Rescan capture" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Resume capture" }));
    await userEvent.click(c.getByRole("button", { name: "Rescan capture" }));
    await expect(c.getByText(/Rescan requested/)).toBeVisible();
    await expect(
      c.getByRole("button", { name: /Import history for/ }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
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
    await userEvent.click(c.getByRole("button", { name: "Pause capture" }));
    await expect(
      c.getByRole("button", { name: "Pause capture" }),
    ).toBeDisabled();
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
    await expect(
      c.getByText("1 degraded · 0 unavailable observations"),
    ).toBeVisible();
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
    await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
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
    await expect(await c.findByText("Codex CLI source 1")).toBeVisible();
    await expect(
      c.getByRole("heading", { name: "Agents", level: 1 }),
    ).toBeVisible();
    const refresh = c.getByRole("button", { name: "Refresh capture" });
    await expect(refresh.closest(".store-settings__tab-bar")).not.toBeNull();
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
    const sourceAction = c.getByRole("button", { name: /Import history for/ });
    await expect(sourceAction.closest("dl")).toHaveClass(
      "store-settings__rows",
    );
    const request = c.getByRole("textbox", {
      name: "Previous import request ID",
    });
    await expect(request.closest(".store-settings__rows")).not.toBeNull();
  }}
/>
