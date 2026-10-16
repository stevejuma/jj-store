<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/RepositoryDeletionRecoveryFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Repository deletion recovery",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Reload recovery retains original request and requires confirmation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await waitFor(() =>
      expect(
        page.getByText("Repository deletion needs attention"),
      ).toBeVisible(),
    );
    await expect(
      c.getByRole("status", { name: "Recovery requests" }),
    ).toHaveTextContent("0");
    await userEvent.click(
      page.getByRole("button", { name: "Continue deletion" }),
    );
    let dialog = within(
      await page.findByRole("alertdialog", {
        name: "Continue deleting Interrupted repo?",
      }),
    );
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(page.queryByRole("alertdialog")).not.toBeInTheDocument(),
    );
    await expect(
      c.getByRole("status", { name: "Recovery requests" }),
    ).toHaveTextContent("0");
    await userEvent.click(
      page.getByRole("button", { name: "Continue deletion" }),
    );
    dialog = within(await page.findByRole("alertdialog"));
    await userEvent.click(
      dialog.getByRole("button", { name: "Check and continue deletion" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("status", { name: "Recovery destination" }),
      ).toHaveTextContent("repositories"),
    );
    await expect(
      c.getByRole("status", { name: "Recovery requests" }),
    ).toHaveTextContent("1");
  }}
/>
<Story
  name="Recovery failure uses a toast and retries the exact retained request"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await waitFor(() =>
      expect(
        page.getByText("Repository deletion needs attention"),
      ).toBeVisible(),
    );
    await userEvent.click(
      page.getByRole("button", { name: "Continue deletion" }),
    );
    const dialog = within(await page.findByRole("alertdialog"));
    await userEvent.click(
      dialog.getByRole("button", { name: "Check and continue deletion" }),
    );
    await waitFor(() =>
      expect(page.getByText("Repository deletion failed")).toBeVisible(),
    );
    await expect(
      c.getByRole("status", { name: "Recovery requests" }),
    ).toHaveTextContent("1");
    await userEvent.click(
      dialog.getByRole("button", { name: "Check and continue deletion" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("status", { name: "Recovery destination" }),
      ).toHaveTextContent("repositories"),
    );
    await expect(
      c.getByRole("status", { name: "Recovery requests" }),
    ).toHaveTextContent("2");
  }}
/>
