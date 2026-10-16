<script module lang="ts">
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositoryDirectoryNavigationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Directory navigation",
    component: Fixture,
    beforeEach: async () => {
      await views.files();
    },
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Folder navigation shows a skeleton with a retained snapshot"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Open directory src" }),
    );
    await expect(
      canvas.getByRole("status", { name: "Loading repository files" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("table", { name: "Files in src" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.queryByText("This directory does not contain any files."),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish folder load" }),
    );
    await expect(
      canvas.queryByRole("status", { name: "Loading repository files" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("button", { name: "Open file lib.rs" }),
    ).toBeEnabled();
  }}
/>
