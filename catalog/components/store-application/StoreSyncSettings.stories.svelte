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
    const actionRow = canvasElement.querySelector(
      ".store-settings__action-row",
    )!;
    await expect(actionRow.firstElementChild).toHaveTextContent(
      "Sync controls",
    );
    await expect(
      actionRow.querySelector("p")!.getBoundingClientRect().right,
    ).toBeLessThanOrEqual(
      actionRow
        .querySelector(".store-settings__actions")!
        .getBoundingClientRect().left,
    );
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
    const empty = c
      .getByText("WorkStore sync is not supported.")
      .closest('[data-slot="empty"]')!;
    await expect(
      empty.querySelector('[data-ui-part="empty-media"] svg'),
    ).toBeVisible();
    await expect(
      c
        .getByText("No upstream is configured. Repository work remains local.")
        .closest(".store-settings__rows"),
    ).not.toBeNull();
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
    await expect(
      canvasElement.querySelector('[data-ui-part="empty-media"] svg'),
    ).toBeVisible();
    await expect(
      within(canvasElement).queryByRole("button", { name: "Refresh sync" }),
    ).toBeNull();
  }}
/>
<Story
  name="Failure retains status recovery"
  args={{ failed: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Pause sync" }));
    await expect(c.getByRole("button", { name: "Pause sync" })).toBeDisabled();
    await expect(
      c.getByText(/Sync status will be retried automatically/),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh sync" }));
    await expect(c.getByRole("button", { name: "Pause sync" })).toBeEnabled();
  }}
/>
