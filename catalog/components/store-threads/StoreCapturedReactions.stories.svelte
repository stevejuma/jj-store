<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreCapturedReactionsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Captured message reactions",
    component: Fixture,
    tags: ["visual-pending"],
  });
  async function react({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement),
      body = within(document.body);
    const groups = await c.findAllByRole("group", {
      name: "Message reactions",
    });
    const message = groups[0]!.closest("article")!;
    await userEvent.click(
      within(groups[0]!).getByRole("button", { name: "Add reaction" }),
    );
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    await expect(
      await within(message).findByRole("button", { name: "Lapis" }),
    ).toBeVisible();
    await userEvent.hover(
      within(message).getByRole("button", { name: "Lapis" }),
    );
    await expect(
      await body.findByRole("group", { name: "People who reacted with Lapis" }),
    ).toBeVisible();
    await userEvent.click(
      within(message).getByRole("button", { name: "Lapis" }),
    );
    await waitFor(() =>
      expect(
        within(message).queryByRole("button", { name: "Lapis" }),
      ).toBeNull(),
    );
  }
</script>

<Story
  name="Captured agent messages"
  args={{ surface: "agents" }}
  play={react}
/>
<Story
  name="Exact review comments"
  args={{ surface: "reviews" }}
  play={react}
/>
