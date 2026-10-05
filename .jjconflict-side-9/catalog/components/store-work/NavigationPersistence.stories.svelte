<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/NavigationPersistenceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Navigation persistence",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Disclosure choices survive remount"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Expand Native release" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Repositories" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Remount navigation" }),
    );
    await expect(
      await canvas.findByRole("button", { name: "Collapse Native release" }),
    ).toHaveAttribute("aria-expanded", "true");
    await expect(
      canvas.getByRole("button", { name: "Repositories" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      canvas.getByRole("button", { name: "Collapse Native release" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Your projects" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Remount navigation" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Your projects" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      canvas.getByRole("button", { name: "Your projects" }),
    );
    await expect(
      await canvas.findByRole("button", { name: "Expand Native release" }),
    ).toHaveAttribute("aria-expanded", "false");
  }}
/>
