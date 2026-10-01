<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import SyncSettingsFixture from "../../fixtures/SyncSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Sync",
    component: SyncSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Pause resume and progress"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Pause sync" }));
    await expect(c.getByRole("button", { name: "Retry sync" })).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Resume sync" }));
    await expect(c.getByRole("button", { name: "Retry sync" })).toBeEnabled();
    await userEvent.click(c.getByRole("tab", { name: "Progress" }));
    await expect(c.getByText("Local pending positions")).toBeVisible();
    await expect(c.getByText("Peer pending positions")).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Overview" }));
    await expect(c.getByText(/Required — review/)).toBeVisible();
  }}
/>
<Story
  name="Local only"
  args={{ configured: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByText("No upstream is configured. Repository work remains local."),
    ).toBeVisible();
    await expect(c.getByRole("tab", { name: "Progress" })).toBeDisabled();
    await expect(
      c.queryByRole("button", { name: "Pause sync" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Unsupported"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByText(
        "Sync management is not available for this connection.",
      ),
    ).toBeVisible();
  }}
/>
<Story
  name="Failure requires refresh"
  args={{ failed: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Pause sync" }));
    await expect(c.getByRole("button", { name: "Pause sync" })).toBeDisabled();
    await expect(
      c.getByText(/Refresh sync status before retrying/),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh sync" }));
    await expect(c.getByRole("button", { name: "Pause sync" })).toBeEnabled();
  }}
/>
