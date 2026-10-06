<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
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
    const folder = within(footer).getByRole("button", {
      name: "Copy folder path: /home/example/code/local-demo",
    });
    expect(folder.querySelector("svg.lucide-folder")).not.toBeNull();
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      await pointer.hover(canvasElement);
      const rest = getComputedStyle(folder).backgroundColor;
      await pointer.hover(folder);
      await waitFor(() =>
        expect(getComputedStyle(folder).backgroundColor).not.toBe(rest),
      );
      await pointer.hover(canvasElement);
    }
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      folder.focus();
      await userEvent.keyboard("{Enter}");
      expect(write).toHaveBeenCalledWith("/home/example/code/local-demo");
      expect(await portal.findByText("Folder path copied")).toBeVisible();
      write.mockRejectedValueOnce(new Error("Clipboard denied"));
      await userEvent.click(folder);
      expect(
        await portal.findByText("Could not copy the folder path. Try again."),
      ).toBeVisible();
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
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
