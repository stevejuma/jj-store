<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import FixtureControls from "../../fixtures/FixtureControls.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  const { Story } = defineMeta({
    title: "JJ Store/Catalog/Fixture controls",
    component: FixtureControls,
    tags: ["visual-pending", "prototype"],
    parameters: {
      jjStoreStory: {
        kind: "prototype",
        requirement: "JST-UI-001",
        reason:
          "Catalog-only test controls are visibly distinct from the production application.",
      },
    },
  });
</script>

<Story
  name="Labelled padded controls"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      label = canvas.getByText("Fixture controls");
    const panel = label.closest("details")!;
    await expect(getComputedStyle(panel).borderTopStyle).toBe("dotted");
    await expect(
      parseFloat(getComputedStyle(panel).paddingTop),
    ).toBeGreaterThan(0);
    await expect(
      canvas.getByRole("button", { name: "Complete fixture read" }),
    ).toBeVisible();
  }}
>
  {#snippet template()}<FixtureControls floating={false}>
      <Button>Complete fixture read</Button><Button
        >Simulate fixture failure</Button
      >
    </FixtureControls>{/snippet}
</Story>
<Story
  name="Application controls collapse outside the app"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      label = canvas.getByText("Fixture controls");
    await expect(label.closest("details")).not.toHaveAttribute("open");
    await userEvent.click(label);
    await expect(
      canvas.getByRole("button", { name: "Complete fixture read" }),
    ).toBeVisible();
  }}
>
  {#snippet template()}<FixtureControls
      ><Button>Complete fixture read</Button></FixtureControls
    >{/snippet}
</Story>
