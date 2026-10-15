<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Header controls",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
</script>

<Story
  name="Draft and guarded merge use the PR header menu"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    const section = within(details).getByRole("region", {
      name: "Pull request for feature/api",
    });
    const header = section.querySelector("header")!;
    const actions = within(header).getByRole("button", {
      name: "PR actions: Squash",
    });
    await expect(
      within(header).getByText("Draft", { exact: true }),
    ).toBeVisible();
    await expect(
      within(section).queryByRole("button", { name: "Ready for review" }),
    ).not.toBeInTheDocument();
    await waitFor(() =>
      expect(getComputedStyle(actions).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(actions);
    await expect(
      await page.findByRole("menuitem", { name: "Merge next…" }),
    ).toHaveAttribute("aria-disabled", "true");
    await userEvent.click(
      page.getByRole("menuitem", { name: "Ready for review" }),
    );
    await waitFor(() =>
      expect(within(header).getByText("open", { exact: true })).toBeVisible(),
    );
    await waitFor(() =>
      expect(getComputedStyle(actions).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(actions);
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Merge next…" }),
    );
    let confirm = await page.findByRole("dialog", { name: "Merge PR #42?" });
    await expect(confirm).toHaveTextContent("Merge PR #42 into feature/model?");
    await userEvent.click(
      within(confirm).getByRole("button", { name: "Cancel" }),
    );
    await expect(
      within(header).getByText("open", { exact: true }),
    ).toBeVisible();
    await waitFor(() =>
      expect(getComputedStyle(actions).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(actions);
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Merge next…" }),
    );
    confirm = await page.findByRole("dialog", { name: "Merge PR #42?" });
    await userEvent.click(
      within(confirm).getByRole("button", { name: "Confirm merge" }),
    );
    await waitFor(() =>
      expect(within(header).getByText("merged", { exact: true })).toBeVisible(),
    );
    await expect(
      within(details).getByRole("region", { name: "Selected file diff" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Header pencil expands a collapsed PR into its retained editor"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const section = await canvas.findByRole("region", {
      name: "Pull request for feature/api",
    });
    const header = section.querySelector("header")!;
    await userEvent.click(
      within(header).getByRole("button", { name: "Collapse pull request #42" }),
    );
    await userEvent.click(
      within(header).getByRole("button", { name: "Edit details" }),
    );
    await expect(
      within(section).getByRole("textbox", { name: "Title" }),
    ).toHaveValue("Expose repository API");
    await expect(
      within(section).getByRole("textbox", { name: "Markdown editor" }),
    ).toBeVisible();
    await expect(
      within(header).getByRole("button", { name: "Edit details" }),
    ).toHaveAttribute("aria-pressed", "true");
  }}
/>
