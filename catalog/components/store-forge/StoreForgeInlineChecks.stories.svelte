<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Inline checks",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
  async function headerHover({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const section = await within(canvasElement).findByRole("region", {
      name: "Pull request for feature/api",
    });
    const header = section.querySelector("header")!;
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const sample = document.createElement("canvas");
    sample.width = sample.height = 1;
    const context = sample.getContext("2d")!;
    const color = (value: string) => {
      context.clearRect(0, 0, 1, 1);
      context.fillStyle = value;
      context.fillRect(0, 0, 1, 1);
      return [...context.getImageData(0, 0, 1, 1).data];
    };
    const difference = (a: number[], b: number[]) =>
      Math.max(...a.slice(0, 3).map((v, i) => Math.abs(v - b[i]!)));
    for (const button of within(
      header.querySelector<HTMLElement>(".forge-inline__actions")!,
    ).getAllByRole("button")) {
      if ((button as HTMLButtonElement).disabled) continue;
      button.scrollIntoView({ block: "nearest", inline: "nearest" });
      await pointer.hover(header, { position: { x: 4, y: 4 } });
      await pointer.hover(button);
      await waitFor(() => {
        expect(button.matches(":hover")).toBe(true);
        const hover = color(getComputedStyle(button).backgroundColor);
        expect(hover[3]).toBe(255);
        expect(
          difference(hover, color(getComputedStyle(header).backgroundColor)),
          button.getAttribute("aria-label") ?? "Header action",
        ).toBeGreaterThanOrEqual(12);
      });
      button.focus();
      await expect(button).toHaveFocus();
    }
    await pointer.hover(canvasElement, { position: { x: 0, y: 0 } });
  }
</script>

<Story
  name="Checks and logs stay in the PR body through float and redock"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    const section = within(details).getByRole("region", {
      name: "Pull request for feature/api",
    });
    const file = within(details).getByRole("region", {
      name: "Selected file diff",
    });
    const inspect = within(section).getByRole("button", {
      name: "Toggle check results",
    });
    await expect(inspect).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(inspect);
    const checks = await within(section).findByRole("region", {
      name: "CI checks",
    });
    await expect(inspect).toHaveAttribute("aria-expanded", "true");
    await expect(
      within(section).getByRole("region", { name: "PR 42" }),
    ).toContainElement(checks);
    await expect(
      canvas.queryByRole("region", { name: "Pull requests" }),
    ).not.toBeInTheDocument();
    await expect(file).toBeVisible();
    await userEvent.click(
      within(checks).getAllByRole("button", { name: /^Actions for / })[0]!,
    );
    const actionsMenu = within(canvasElement.ownerDocument.body).getByRole(
      "menu",
    );
    for (const item of within(actionsMenu).getAllByRole("menuitem")) {
      await expect(
        item.scrollWidth,
        item.textContent ?? "Check action",
      ).toBeLessThanOrEqual(item.clientWidth + 1);
    }
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "Jobs",
      }),
    );
    await userEvent.click(
      (
        await within(checks).findAllByRole("button", { name: /^Actions for / })
      )[0]!,
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "View logs",
      }),
    );
    const logs = await within(checks).findByLabelText("Job logs");
    await expect(logs).toHaveTextContent("42 passed");
    await userEvent.click(
      within(section).getByRole("button", { name: "Edit details" }),
    );
    const title = within(section).getByRole("textbox", { name: "Title" });
    await userEvent.clear(title);
    await userEvent.type(title, "Retained review title");
    await userEvent.click(
      within(section).getByRole("button", { name: "Float PR details" }),
    );
    const floating = await within(canvasElement.ownerDocument.body).findByRole(
      "dialog",
      { name: "PR #42 · feature/api" },
    );
    await expect(
      within(floating).getByRole("region", { name: "CI checks" }),
    ).toBe(checks);
    await expect(within(floating).getByLabelText("Job logs")).toBe(logs);
    await expect(within(floating).getByRole("textbox", { name: "Title" })).toBe(
      title,
    );
    await expect(title).toHaveValue("Retained review title");
    await expect(file).toBeVisible();
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await expect(
      within(section).getByRole("region", { name: "CI checks" }),
    ).toBe(checks);
    await expect(
      within(details).getByRole("region", { name: "Selected file diff" }),
    ).toBe(file);
    await userEvent.click(inspect);
    await expect(inspect).toHaveAttribute("aria-expanded", "false");
    await expect(logs).not.toBeVisible();
    await userEvent.click(inspect);
    await expect(within(section).getByLabelText("Job logs")).toHaveTextContent(
      "42 passed",
    );
    await expect(title).toHaveValue("Retained review title");
  }}
/>

<Story
  name="Checks footer starts collapsed with trailing chevron and hover"
  play={async ({ canvasElement }) => {
    const section = await within(canvasElement).findByRole("region", {
      name: "Pull request for feature/api",
    });
    const header = section.querySelector("header")!;
    await expect(
      within(header).queryByRole("button", { name: "Inspect checks" }),
    ).not.toBeInTheDocument();
    const checks = within(section).getByRole("region", { name: "CI checks" });
    const trigger = within(checks).getByRole("button", {
      name: "Toggle check results",
    });
    const chevron = within(checks).getByRole("button", {
      name: "Expand checks",
    });
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await expect(chevron.parentElement!.lastElementChild).toBe(chevron);
    const pr = within(section).getByRole("region", { name: "PR 42" });
    await expect(checks.getBoundingClientRect().left).toBe(
      pr.getBoundingClientRect().left,
    );
    await expect(checks.getBoundingClientRect().right).toBe(
      pr.getBoundingClientRect().right,
    );
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      const row = trigger.parentElement!;
      const before = getComputedStyle(row).backgroundColor;
      await pointer.hover(trigger);
      await waitFor(() =>
        expect(getComputedStyle(row).backgroundColor).not.toBe(before),
      );
    }
    chevron.focus();
    await userEvent.keyboard("{Enter}");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(
      within(section).getByRole("button", { name: /PR actions:/ }),
    );
    const menu = within(canvasElement.ownerDocument.body).getByRole("menu");
    for (const item of within(menu).getAllByRole("menuitem")) {
      await expect(item.querySelector("svg")).not.toBeNull();
      await expect(getComputedStyle(item).whiteSpace).toBe("nowrap");
      await expect(item.scrollWidth).toBeLessThanOrEqual(item.clientWidth + 1);
    }
    await userEvent.keyboard("{Escape}");
    const actions = header.querySelector(".forge-inline__actions")!;
    await expect(
      actions.lastElementChild?.previousElementSibling,
    ).toHaveAttribute("title", "PR actions");
  }}
/>

<Story
  name="PR header actions have visible hover in light mode"
  globals={{ colorMode: "light" }}
  play={headerHover}
/>
<Story
  name="PR header actions have visible hover in dark mode"
  globals={{ colorMode: "dark" }}
  play={headerHover}
/>
