<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import SyncReconciliationFixture from "../../fixtures/SyncReconciliationFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-settings/StoreSyncReconciliation.svelte",
      },
    },
    title: "JJ Store/Settings/Sync reconciliation",
    component: SyncReconciliationFixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story
  name="Unknown outcome survives close"
  args={{ outcome: "unknown" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      c.getByRole("button", { name: "Reconcile operations" }),
    );
    const dialog = await page.findByRole("dialog", {
      name: "Reconcile operations",
    });
    await waitFor(async () => {
      await expect(
        within(dialog).getByRole("button", { name: "Base operation" }),
      ).toBeVisible();
    });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview reconciliation" }),
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply command" }),
    );
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Publication outcome unknown"),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    await userEvent.click(
      c.getByRole("button", { name: "Recover reconciliation" }),
    );
    const recovered = await page.findByRole("dialog", {
      name: "Reconcile operations",
    });
    await waitFor(() => expect(recovered).toBeVisible());
    await userEvent.click(
      within(recovered).getByRole("button", { name: "Check status" }),
    );
    await waitFor(() =>
      expect(within(recovered).getByText("Repository updated")).toBeVisible(),
    );
    await expect(c.getByTestId("captures")).toHaveTextContent("1");
  }}
/>
