<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import CommunityProjectsColumnAlert from "../../../web/src/lib/ui/community-projects/CommunityProjectsColumnAlert.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Components/Column alert",
    component: CommunityProjectsColumnAlert,
    tags: ["visual-pending"],
    args: {
      title: "Comparison needs a target",
      description:
        "One affected change has ambiguous lineage and needs an explicit target.",
    },
  });

  async function expectFlatAlert(
    canvasElement: HTMLElement,
    tone: "warning" | "destructive",
  ) {
    const alert = within(canvasElement).getByRole("alert");
    const style = getComputedStyle(alert);
    await expect(alert).toHaveAttribute("data-tone", tone);
    await expect(alert.querySelector(":scope > svg")).not.toBeNull();
    await expect(style.borderTopWidth).toBe("0px");
    await expect(style.borderRightWidth).toBe("0px");
    await expect(style.borderBottomWidth).toBe("1px");
    await expect(style.borderLeftWidth).toBe("0px");
    await expect(style.borderRadius).toBe("0px");
    await expect(style.backgroundColor).not.toBe("rgba(0, 0, 0, 0)");
  }
</script>

<Story
  name="Warning"
  play={async ({ canvasElement }) => {
    await expectFlatAlert(canvasElement, "warning");
  }}
/>

<Story
  name="Destructive"
  args={{
    title: "History unavailable",
    description: "The selected history window could not be loaded.",
    tone: "destructive",
  }}
  play={async ({ canvasElement }) => {
    await expectFlatAlert(canvasElement, "destructive");
  }}
/>
