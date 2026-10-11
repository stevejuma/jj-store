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
    await expect(
      c.queryByRole("tab", { name: "Progress" }),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByText("WorkStore sync is not supported."),
    ).not.toBeInTheDocument();
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
      within(canvasElement.ownerDocument.body).getByText(
        /Sync status will be retried automatically/,
      ),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh sync" }));
    await expect(c.getByRole("button", { name: "Pause sync" })).toBeEnabled();
    await expect(
      within(canvasElement.ownerDocument.body).queryByText(
        /Sync status will be retried automatically/,
      ),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Captured target remains fixed"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Progress" }));
    await userEvent.click(
      c.getByRole("button", { name: "Wait for current sync" }),
    );
    await expect(c.getByText("Captured target is still pending")).toBeVisible();
    await expect(
      within(
        c.getByText("Push target through").closest(".store-readonly-row")!,
      ).getByText("9", { exact: true }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Copy full Push target through: 9" }),
    ).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("tab", { name: "Overview" }));
    await userEvent.click(c.getByRole("tab", { name: "Progress" }));
    await userEvent.click(c.getByRole("button", { name: "Continue waiting" }));
    await expect(c.getByText("Captured target reached")).toBeVisible();
    await expect(
      within(
        c.getByText("Push target through").closest(".store-readonly-row")!,
      ).getByText("9", { exact: true }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Copy full Push target through: 9" }),
    ).not.toBeInTheDocument();
  }}
/>
