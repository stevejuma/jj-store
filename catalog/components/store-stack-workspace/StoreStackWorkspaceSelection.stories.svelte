<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Selection",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Selected commits and headers toggle closed without losing adjacent diffs"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const commit = within(stack).getByRole("button", {
      name: "Expose repository API",
    });
    await userEvent.click(commit);
    const files = within(stack).getByRole("region", {
      name: "Changed files in feature/api",
    });
    await userEvent.click(
      within(files).getByRole("button", { name: "src/api.ts" }),
    );
    const diff = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    commit.focus();
    await userEvent.keyboard("{Enter}");
    expect(
      within(stack).queryByRole("region", {
        name: "Changed files in feature/api",
      }),
    ).toBeNull();
    expect(diff).toBeVisible();
    for (const name of ["feature/model", "feature/api", "stack feature/api"]) {
      const header = within(stack).getByRole("button", {
        name: `Show all changes in ${name}`,
      });
      await userEvent.click(header);
      expect(
        within(stack).getByRole("region", {
          name: `Changed files in branch ${name === "feature/model" ? name : "feature/api"}`,
        }),
      ).toBeVisible();
      await userEvent.click(header);
      expect(
        within(stack).queryByRole("region", {
          name: /^Changed files in branch/,
        }),
      ).toBeNull();
    }
  }}
/>

<Story
  name="Branches and target options toggle off and retain their history"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const branches = await canvas.findByRole("region", { name: "Branches" });
    for (const name of ["feature/api", "main"]) {
      const option = within(branches).getByRole("option", { name });
      expect(getComputedStyle(option).cursor).toBe("pointer");
      await userEvent.click(option);
      const stack = await canvas.findByRole("region", {
        name: `Stack ${name}`,
      });
      const commit = await within(stack).findByRole("button", {
        name: name === "main" ? "Main history 240" : "Expose repository API",
      });
      await userEvent.click(commit);
      await within(stack).findByRole("region", {
        name: `Changed files in ${name}`,
      });
      await userEvent.click(commit);
      expect(
        within(stack).queryByRole("region", {
          name: `Changed files in ${name}`,
        }),
      ).toBeNull();
      await userEvent.click(option);
      expect(
        canvas.queryByRole("region", { name: `Stack ${name}` }),
      ).toBeNull();
      expect(option).not.toHaveAttribute("data-current", "true");
      await userEvent.click(option);
      expect(
        await canvas.findByRole("region", { name: `Stack ${name}` }),
      ).toBeVisible();
    }
  }}
/>
