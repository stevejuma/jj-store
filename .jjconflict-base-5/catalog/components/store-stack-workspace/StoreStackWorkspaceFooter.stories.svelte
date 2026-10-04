<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Footer",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function verifyFooter({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const canvas = within(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    await userEvent.click(
      canvas.getByRole("button", { name: /^Create stack$/ }),
    );
    const portal = within(canvasElement.ownerDocument.body);
    const dialog = await portal.findByRole("dialog", { name: "Create stack" });
    await userEvent.type(
      within(dialog).getByRole("textbox", { name: "Name" }),
      "feature/retained",
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel" }),
    );
    const action = canvas.getByRole("button", { name: "Edit retained draft" });
    const footer = action.closest<HTMLElement>("footer")!;
    const chips = footer.querySelectorAll<HTMLElement>(
      '.stack-workspace__footer-items [data-ui-part="community-chip"]',
    );
    expect(chips).toHaveLength(5);
    expect(action.querySelector("svg")).not.toBeNull();
    for (const chip of chips) {
      expect(getComputedStyle(chip).backgroundColor).toBe(
        getComputedStyle(action).backgroundColor,
      );
    }
    expect(footer.textContent).not.toMatch(/Folder:|Base:/);
    await userEvent.click(action);
    expect(
      within(
        await portal.findByRole("dialog", { name: "Create stack" }),
      ).getByRole("textbox", { name: "Name" }),
    ).toHaveValue("feature/retained");
    await userEvent.click(portal.getByRole("button", { name: "Cancel" }));
  }
</script>

<Story
  name="Retained draft matches the light footer chips"
  play={verifyFooter}
/>
<Story
  name="Retained draft matches the dark footer chips"
  globals={{ colorMode: "dark" }}
  play={verifyFooter}
/>
