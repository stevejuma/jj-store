<script module lang="ts">
  import { views as applicationViews } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Navigation",
    component: StackWorkspaceApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await Promise.all([
        applicationViews.stacks(),
        applicationViews.files(),
        applicationViews.log(),
        applicationViews.stackDetails(),
        applicationViews.stackConflicts(),
      ]);
    },
  });
  const views = (element: HTMLElement) =>
    within(
      within(element).getByRole("tablist", { name: "Local repository views" }),
    );
</script>

<Story
  name="Details navigation retains dependent branch and target selections in the body"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: /feature\/api/ }),
    );
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in feature/model",
      }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const details = canvas.getByRole("region", {
      name: "Stack workspace details",
    });
    const rail = within(details);
    expect(rail.getByText("Branches/Stacks")).toBeVisible();
    expect(rail.getByRole("button", { name: "Branches" })).toHaveAttribute(
      "aria-current",
      "page",
    );
    expect(
      within(stack).getByRole("region", {
        name: "Changed files in branch feature/model",
      }),
    ).toBeVisible();
    expect(details.querySelector("dl")).toBeNull();
    await userEvent.click(canvas.getByRole("option", { name: "main" }));
    const target = await canvas.findByRole("region", { name: "Stack main" });
    await userEvent.click(
      await within(target).findByRole("button", { name: "Main history 240" }),
    );
    await waitFor(() =>
      expect(
        within(target)
          .getByRole("button", { name: "Main history 240" })
          .closest('[role="listitem"]'),
      ).toHaveAttribute("aria-current", "true"),
    );
    expect(rail.queryByText("feature/model", { exact: true })).toBeNull();
  }}
/>
