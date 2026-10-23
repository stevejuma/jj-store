<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import NotificationsFixture from "../../fixtures/NotificationsFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-notifications/StoreNotificationHost.svelte",
      },
    },
    title: "JJ Store/Notifications",
    component: NotificationsFixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story
  name="Dismissal recovery and navigation"
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(page.getByRole("button", { name: "Fail request" }));
    await expect(page.getByRole("alert")).toHaveTextContent(
      "Your draft is retained",
    );
    await userEvent.click(page.getByRole("button", { name: "Fail request" }));
    await expect(page.getAllByRole("alert")).toHaveLength(1);
    await userEvent.click(
      page.getByRole("button", { name: "Dismiss Request failed" }),
    );
    await userEvent.click(page.getByRole("button", { name: "Toggle page" }));
    await userEvent.click(page.getByRole("button", { name: "Toggle page" }));
    await expect(page.queryByRole("alert")).not.toBeInTheDocument();
    await userEvent.click(page.getByRole("button", { name: "Recover" }));
    await userEvent.click(page.getByRole("button", { name: "Fail request" }));
    await expect(page.getByRole("alert")).toBeVisible();
    await userEvent.click(page.getByRole("button", { name: "Recover" }));
    await expect(page.queryByRole("alert")).not.toBeInTheDocument();
    await userEvent.click(
      page.getByRole("button", { name: "Show sticky warning" }),
    );
    await userEvent.click(page.getByRole("button", { name: "Toggle page" }));
    await expect(page.getByText("Attention required")).toBeVisible();
    await userEvent.click(
      page.getByRole("button", { name: "Dismiss Attention required" }),
    );
  }}
/>

<Story
  name="Modal feedback preserves drafts"
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(page.getByRole("button", { name: "Edit" }));
    const dialog = await page.findByRole("dialog", { name: "Edit details" });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Save draft" }),
    );
    await waitFor(() =>
      expect(within(dialog).getByRole("alert")).toBeVisible(),
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Dismiss Request failed" }),
    );
    await expect(
      within(dialog).getByRole("textbox", { name: "Draft" }),
    ).toHaveValue("Keep this draft");
    await expect(dialog).toBeVisible();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Close editor" }),
    );
  }}
/>
