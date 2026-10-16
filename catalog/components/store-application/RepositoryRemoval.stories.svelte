<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/RepositoryRemovalFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Repository removal",
    component: Fixture,
    tags: ["visual-pending"],
  });
  const dialogFor = async (canvasElement: HTMLElement) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Remove repository" }));
    return within(
      await within(canvasElement.ownerDocument.body).findByRole("alertdialog"),
    );
  };
</script>

<Story
  name="Confirmation returns to refreshed repository discovery"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      dialog = await dialogFor(canvasElement);
    await expect(
      c.getByRole("status", { name: "Removal requests" }),
    ).toHaveTextContent("0 reads; 0 writes");
    await userEvent.click(
      dialog.getByRole("button", { name: "Remove repository" }),
    );
    await waitFor(
      () =>
        expect(
          c.getByRole("status", { name: "Removal destination" }),
        ).toHaveTextContent("disconnected:repositories:refreshed"),
      { timeout: 3000 },
    );
  }}
/>
<Story
  name="Failure toast retains confirmation and original revision for retry"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body),
      dialog = await dialogFor(canvasElement);
    await userEvent.click(
      dialog.getByRole("button", { name: "Remove repository" }),
    );
    await waitFor(() =>
      expect(page.getByText("Repository removal failed")).toBeVisible(),
    );
    await waitFor(() =>
      expect(
        page.getByText("The repository request could not be completed."),
      ).toBeVisible(),
    );
    await expect(
      page.getByRole("alertdialog").querySelector("p[role=alert]"),
    ).toBeNull();
    await expect(
      c.getByRole("status", { name: "Removal destination" }),
    ).toHaveTextContent("connected::retained");
    await userEvent.click(
      dialog.getByRole("button", { name: "Check and retry removal" }),
    );
    await waitFor(
      () =>
        expect(
          c.getByRole("status", { name: "Removal destination" }),
        ).toHaveTextContent("disconnected:repositories:refreshed"),
      { timeout: 3000 },
    );
    await waitFor(() =>
      expect(
        page.queryByText("Repository removal failed"),
      ).not.toBeInTheDocument(),
    );
  }}
/>
<Story
  name="Stale registration closes the old confirmation with a toast"
  args={{ stale: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body),
      dialog = await dialogFor(canvasElement);
    await userEvent.click(
      dialog.getByRole("button", { name: "Remove repository" }),
    );
    await waitFor(() =>
      expect(page.getByText("Repository removal failed")).toBeVisible(),
    );
    await waitFor(() =>
      expect(page.queryByRole("alertdialog")).not.toBeInTheDocument(),
    );
    await expect(
      c.getByRole("status", { name: "Removal destination" }),
    ).toHaveTextContent("connected::retained");
  }}
/>
<Story
  name="Busy confirmation prevents duplicate submissions and cancellation"
  args={{ slow: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      dialog = await dialogFor(canvasElement);
    await userEvent.click(
      dialog.getByRole("button", { name: "Remove repository" }),
    );
    await expect(
      dialog.getByRole("button", { name: "Working…" }),
    ).toBeDisabled();
    await expect(dialog.getByRole("button", { name: "Cancel" })).toBeDisabled();
    await userEvent.keyboard("{Escape}");
    await expect(
      c.getByRole("status", { name: "Removal requests" }),
    ).toHaveTextContent("1 reads; 1 writes");

    await waitFor(
      () =>
        expect(
          c.getByRole("status", { name: "Removal destination" }),
        ).toHaveTextContent("disconnected:repositories:refreshed"),
      { timeout: 3000 },
    );
  }}
/>
<Story
  name="Readers have no removal action"
  args={{ reader: true }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).queryByRole("button", {
        name: "Remove repository",
      }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Server repositories do not invoke folder removal"
  args={{ native: true }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).queryByRole("button", {
        name: "Remove repository",
      }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Resolve existing settings drafts before removal"
  args={{ dirty: true }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByRole("button", { name: "Remove repository" }),
    ).toBeDisabled();
  }}
/>

<Story
  name="Server deletion confirms permanent object erasure and refreshes discovery"
  args={{ native: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const button = c.getByRole("button", { name: "Delete repository" });
    await expect(button).toHaveAttribute("data-variant", "destructive");
    await userEvent.click(button);
    const dialog = within(
      await page.findByRole("alertdialog", { name: "Delete My local repo?" }),
    );
    await waitFor(() =>
      expect(
        dialog.getByText(/permanently deletes all server objects/),
      ).toBeVisible(),
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Delete repository" }),
    );
    await waitFor(
      () =>
        expect(
          c.getByRole("status", { name: "Removal destination" }),
        ).toHaveTextContent("disconnected:repositories:refreshed"),
      { timeout: 3000 },
    );
    await expect(
      c.getByRole("status", { name: "Removal requests" }),
    ).toHaveTextContent("0 reads; 2 writes");
  }}
/>

<Story
  name="Confirmed server deletion survives repository settings teardown"
  args={{ native: true, unmount: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(c.getByRole("button", { name: "Delete repository" }));
    const dialog = within(await page.findByRole("alertdialog"));
    await userEvent.click(
      dialog.getByRole("button", { name: "Delete repository" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("status", { name: "Removal destination" }),
      ).toHaveTextContent("disconnected:repositories:refreshed"),
    );
    await expect(
      c.getByRole("status", { name: "Removal requests" }),
    ).toHaveTextContent("0 reads; 2 writes");
  }}
/>

<Story
  name="Unknown storage writes pause deletion with an exact-request recovery toast"
  args={{ native: true, uncertain: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(c.getByRole("button", { name: "Delete repository" }));
    const dialog = within(await page.findByRole("alertdialog"));
    await userEvent.click(
      dialog.getByRole("button", { name: "Delete repository" }),
    );
    await waitFor(() =>
      expect(
        page.getByText(
          "A storage write has an unknown outcome. Restart this local service, then continue the original deletion request.",
        ),
      ).toBeVisible(),
    );
    await expect(
      c.getByRole("status", { name: "Removal destination" }),
    ).toHaveTextContent("connected::retained");
    await userEvent.click(
      dialog.getByRole("button", { name: "Check and retry removal" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("status", { name: "Removal destination" }),
      ).toHaveTextContent("disconnected:repositories:refreshed"),
    );
    await expect(
      c.getByRole("status", { name: "Removal requests" }),
    ).toHaveTextContent("0 reads; 2 writes");
  }}
/>
