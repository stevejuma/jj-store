<script module lang="ts">
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import { IDS } from "../../fixtures/repository.js";
  import Fixture from "../../fixtures/StoreEvolutionStatesFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Evolution column",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await Promise.all([views.log()]);
    },
  });
</script>

<Story
  name="Loading"
  args={{ state: "loading" }}
  play={async ({ canvasElement }) => {
    await expect(
      await within(canvasElement).findByRole("status", {
        name: "Loading evolution log",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Empty"
  args={{ state: "empty" }}
  play={async ({ canvasElement }) => {
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "No evolution history",
      ),
    ).toBeVisible();
  }}
/>
<Story
  name="Failed"
  args={{ state: "failed" }}
  play={async ({ canvasElement }) => {
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "Evolution unavailable",
      ),
    ).toBeVisible();
    await expect(
      within(canvasElement).getByRole("button", {
        name: "Close Evo Log column",
      }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Incomplete native history"
  args={{ state: "incomplete" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await expect(
      await page.findByText("Incomplete predecessor history"),
    ).toBeVisible();
    await expect(
      [
        ...canvas
          .getByRole("list", { name: "Evolution log" })
          .querySelectorAll("[data-revision-id]"),
      ].map((row) => row.getAttribute("data-revision-id")),
    ).toEqual([IDS.merge, "ec".repeat(64), "ed".repeat(64)]);
  }}
/>
