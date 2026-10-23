<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/LazyViewFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-application/StoreLazyView.svelte",
      },
    },
    title: "JJ Store/Lazy views",
    component: Fixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story
  name="Demand loading and retained closed drafts"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByText("Imports: 0")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Open view" }));
    await expect(
      c.getByRole("status", { name: "Loading Example view" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Resolve first" }));
    const input = await c.findByRole("textbox", { name: "First draft" });
    await userEvent.type(input, "Retained draft");
    await userEvent.click(c.getByRole("button", { name: "Close view" }));
    await expect(c.queryByRole("textbox")).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Open view" }));
    await expect(c.getByRole("textbox", { name: "First draft" })).toHaveValue(
      "Retained draft",
    );
    await expect(c.getByText("Imports: 1")).toBeVisible();
  }}
/>

<Story
  name="Failed import and explicit retry"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "Open view" }));
    await userEvent.click(c.getByRole("button", { name: "Reject first" }));
    await userEvent.click(await body.findByRole("button", { name: "Retry" }));
    await waitFor(() => expect(c.getByText("Imports: 2")).toBeVisible());
    await userEvent.click(c.getByRole("button", { name: "Resolve first" }));
    await expect(
      await c.findByRole("textbox", { name: "First draft" }),
    ).toBeVisible();
    await expect(body.queryByText("The view could not be loaded.")).toBeNull();
  }}
/>

<Story
  name="Closing a pending overlay hides loading and error feedback"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "Open view" }));
    await userEvent.click(c.getByRole("button", { name: "Close view" }));
    await expect(
      c.queryByRole("status", { name: "Loading Example view" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Reject first" }));
    await expect(body.queryByText("The view could not be loaded.")).toBeNull();
  }}
/>

<Story
  name="Obsolete and unmounted import completion"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Open view" }));
    await userEvent.click(c.getByRole("button", { name: "Switch view" }));
    await userEvent.click(c.getByRole("button", { name: "Resolve first" }));
    await expect(c.queryByRole("textbox")).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Navigate away" }));
    await userEvent.click(c.getByRole("button", { name: "Resolve second" }));
    await expect(c.queryByRole("textbox")).toBeNull();
    await expect(
      c.queryByRole("status", { name: "Loading Example view" }),
    ).toBeNull();
  }}
/>
