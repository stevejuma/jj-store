<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  const { Story } = defineMeta({
    title: "JJ Store/Application",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: {
        story: { inline: false, iframeHeight: 700 },
        source: {
          code: '<script>\n  import { StoreRepositoryApplication, createInMemoryStoreController } from "./catalog/index.js";\n  const controller = createInMemoryStoreController();\n<\/script>\n<StoreRepositoryApplication {controller} />',
        },
      },
    },
  });
</script>

<Story
  name="Repository workspace"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await views.log();
    await expect(
      await canvas.findByRole("button", { name: "Collapse Branches column" }),
    ).toBeVisible();
    await userEvent.click(await canvas.findByRole("option", { name: "main" }));
    await expect(
      await canvas.findByRole("button", {
        name: "Merge native storage and publication",
      }),
    ).toBeVisible();
  }}
>
  {#snippet template()}<StoreRepositoryApplication />{/snippet}
</Story>
<Story name="Expanded workspace">
  {#snippet template()}<StoreRepositoryApplication
      initiallyCollapsed={false}
    />{/snippet}
</Story>
<Story
  name="Mobile workspace"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("option", { name: "main" }),
    ).toBeVisible();
  }}
>
  {#snippet template()}<StoreRepositoryApplication
      displayMode="mobile"
    />{/snippet}
</Story>
<Story
  name="Full app"
  parameters={{
    docs: {
      description: {
        story:
          "Complete application with an independent in-memory controller. No play function changes the initial selection; use the navigation, bookmark cards, file browser, snapshot picker, history controls and profile footer freely.",
      },
    },
  }}
>
  {#snippet template()}<StoreRepositoryApplication />{/snippet}
</Story>
