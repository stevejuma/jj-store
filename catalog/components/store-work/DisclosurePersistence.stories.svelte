<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/DisclosurePersistenceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Page disclosure persistence",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function remount(c: ReturnType<typeof within>) {
    await userEvent.click(c.getByRole("button", { name: "Remount page" }));
  }
  async function transparency(button: HTMLElement) {
    await userEvent.unhover(button);
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      await pointer.unhover(button);
    }
    button.blur();
    await waitFor(() => {
      expect(button.matches(":hover, :focus-visible")).toBe(false);
      expect(getComputedStyle(button).backgroundColor).toBe("rgba(0, 0, 0, 0)");
    });
  }
</script>

<Story
  name="Projects collection choices survive tab switches and remount"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const group = await c.findByRole("region", {
      name: "Agents in UI repository",
    });
    const toggle = within(group).getByRole("button", {
      name: /^UI repository(?: \d+)?$/,
    });
    await userEvent.click(toggle);
    await expect(toggle).toHaveAttribute("aria-expanded", "false");
    await expect(
      within(group).queryByRole("button", { name: /Open Codex/ }),
    ).toBeNull();
    await userEvent.click(c.getByRole("tab", { name: "Tasks" }));
    await userEvent.click(c.getByRole("tab", { name: "Agents" }));
    await expect(
      within(
        await c.findByRole("region", { name: "Agents in UI repository" }),
      ).getByRole("button", { name: /^UI repository(?: \d+)?$/ }),
    ).toHaveAttribute("aria-expanded", "false");
    await remount(c);
    const restored = within(
      await c.findByRole("region", { name: "Agents in UI repository" }),
    ).getByRole("button", { name: /^UI repository(?: \d+)?$/ });
    await expect(restored).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(restored);
    await remount(c);
    await expect(
      within(
        await c.findByRole("region", { name: "Agents in UI repository" }),
      ).getByRole("button", { name: /^UI repository(?: \d+)?$/ }),
    ).toHaveAttribute("aria-expanded", "true");
  }}
/>
<Story
  name="Repository collection choices survive tab switches and remount"
  args={{ repository: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      toggle = await c.findByRole("button", {
        name: /^Agent sessions(?: \d+)?$/,
      });
    await userEvent.click(toggle);
    await userEvent.click(c.getByRole("tab", { name: "Reviews" }));
    await userEvent.click(c.getByRole("tab", { name: "Agents" }));
    await expect(
      await c.findByRole("button", { name: /^Agent sessions(?: \d+)?$/ }),
    ).toHaveAttribute("aria-expanded", "false");
    await remount(c);
    await expect(
      await c.findByRole("button", { name: /^Agent sessions(?: \d+)?$/ }),
    ).toHaveAttribute("aria-expanded", "false");
  }}
/>
<Story
  name="Task section choices survive remount with composer retained"
  args={{ task: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      task = within(await c.findByRole("article", { name: "Task details" }));
    for (const name of ["Description", "Activity", "Subtasks"])
      await userEvent.click(task.getByRole("button", { name }));
    await expect(task.getByRole("group", { name: "Comment" })).toBeVisible();
    await remount(c);
    const restored = within(
      await c.findByRole("article", { name: "Task details" }),
    );
    for (const name of ["Description", "Activity"])
      await expect(restored.getByRole("button", { name })).toHaveAttribute(
        "aria-expanded",
        "false",
      );
    await expect(
      restored.getByRole("button", { name: "Subtasks" }),
    ).toHaveAttribute("aria-expanded", "true");
    await expect(
      restored.getByRole("group", { name: "Comment" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Review section choices survive remount"
  args={{ repository: true, review: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Revision open/ }),
    );
    const detail = within(c.getByRole("article", { name: "Review details" }));
    for (const name of [
      "Description",
      "Files changed",
      "Commits",
      "Agent context",
      "Activity",
    ])
      await userEvent.click(await detail.findByRole("button", { name }));
    await remount(c);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Revision open/ }),
    );
    const restored = within(c.getByRole("article", { name: "Review details" }));
    for (const name of ["Description", "Activity"])
      await expect(restored.getByRole("button", { name })).toHaveAttribute(
        "aria-expanded",
        "false",
      );
    for (const name of ["Files changed", "Commits"])
      await expect(
        restored.getByRole("button", {
          name: new RegExp(`^${name}(?: \\d+)?$`),
        }),
      ).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(restored.getByRole("button", { name: "Activity" }));
    await expect(
      await restored.findByRole("button", { name: "Agent context" }),
    ).toHaveAttribute("aria-expanded", "true");
  }}
/>
<Story
  name="Op Log operation disclosure survives remount"
  args={{ repository: true, opLog: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    async function select() {
      await userEvent.click(
        await c.findByRole("button", {
          name: "describe: native object history",
        }),
      );
      return within(
        await c.findByRole("region", { name: "Selected operation" }),
      ).getByRole("button", { name: "describe: native object history" });
    }
    await userEvent.click(await select());
    await remount(c);
    await expect(await select()).toHaveAttribute("aria-expanded", "false");
  }}
/>
<Story
  name="Header toggles stay transparent in both expansion states"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    let left = await c.findByRole("button", { name: "Collapse left sidebar" });
    await transparency(left);
    await userEvent.click(left);
    left = c.getByRole("button", { name: "Expand left sidebar" });
    await transparency(left);
    await userEvent.click(left);
    await expect(
      c.getByRole("button", { name: "Collapse left sidebar" }),
    ).toHaveAttribute("aria-expanded", "true");
    const right = c.getByRole("button", { name: "Toggle details" });
    await transparency(right);
    await userEvent.click(right);
    await expect(right).toHaveAttribute("aria-expanded", "true");
    await transparency(right);
    // CSS :hover requires a native pointer, not Testing Library's synthetic events.
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      await pointer.hover(right);
      await waitFor(() =>
        expect(getComputedStyle(right).backgroundColor).not.toBe(
          "rgba(0, 0, 0, 0)",
        ),
      );
      await pointer.unhover(right);
    }
  }}
/>
