<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Commit panel spacing",
    component: StackWorkspaceFixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Commit panel gap matches the space between branch stacks"
  play={async ({ canvasElement }) => {
    const stack = await within(canvasElement).findByRole("region", {
      name: "Stack feature/api",
    });
    const composer = within(stack).getByRole("group", {
      name: "Commit composer for feature/api",
    });
    const cards = [
      ...stack.querySelectorAll<HTMLElement>(".stack-workspace__bookmark-card"),
    ];
    function verify() {
      const branchesGap =
        cards[1]!.getBoundingClientRect().top -
        cards[0]!.getBoundingClientRect().bottom;
      const composerGap =
        cards[0]!.getBoundingClientRect().top -
        composer.getBoundingClientRect().bottom;
      expect(branchesGap).toBe(12);
      expect(composerGap).toBeCloseTo(branchesGap, 1);
    }
    verify();
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Create commit in feature/api",
      }),
    );
    verify();
  }}
/>
