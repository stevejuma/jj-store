<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Workspace",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
  async function verifyForgeTitleButtons({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const button = within(stack).getByRole("button", {
      name: "Pull request for feature/api",
    });
    const title = within(stack).getByRole("button", {
      name: "Show all changes in feature/api",
    });
    const row = title.closest<HTMLElement>(
      '[data-slot="card-title"]',
    )!.parentElement!;
    const centre = (element: HTMLElement) => {
      const r = element.getBoundingClientRect();
      return r.top + r.height / 2;
    };
    await expect(Math.abs(centre(button) - centre(title))).toBeLessThanOrEqual(
      1,
    );
    await expect(
      Math.abs(
        button.getBoundingClientRect().right -
          row.getBoundingClientRect().right,
      ),
    ).toBeLessThanOrEqual(1);
    await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth + 1);
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const sample = document.createElement("canvas");
    sample.width = sample.height = 1;
    const context = sample.getContext("2d")!;
    function color(value: string) {
      context.clearRect(0, 0, 1, 1);
      context.fillStyle = value;
      context.fillRect(0, 0, 1, 1);
      return [...context.getImageData(0, 0, 1, 1).data];
    }
    const difference = (a: number[], b: number[]) =>
      Math.max(...a.slice(0, 3).map((v, i) => Math.abs(v - b[i]!)));
    for (const control of [
      button,
      within(stack).getByRole("button", { name: "Submit stack feature/api" }),
    ]) {
      const parent = control.closest<HTMLElement>(
        '[data-ui-part="card-header"], .stack-workspace__bookmark-actions',
      )!;
      control.scrollIntoView({ block: "nearest", inline: "nearest" });
      await pointer.hover(parent, { position: { x: 5, y: 5 } });
      await waitFor(() => expect(parent.matches(":hover")).toBe(true));
      const rest = color(getComputedStyle(control).backgroundColor);
      const background = color(getComputedStyle(parent).backgroundColor);
      await expect(rest[3]).toBe(255);
      await expect(difference(rest, background)).toBeGreaterThanOrEqual(12);
      await expect(
        Number.parseFloat(getComputedStyle(control).borderTopWidth),
      ).toBeGreaterThanOrEqual(1);
      await expect(
        difference(color(getComputedStyle(control).borderTopColor), rest),
      ).toBeGreaterThanOrEqual(12);
      await pointer.hover(control);
      await waitFor(() => {
        const hover = color(getComputedStyle(control).backgroundColor);
        expect(control.matches(":hover")).toBe(true);
        expect(hover[3]).toBe(255);
        expect(
          difference(hover, color(getComputedStyle(parent).backgroundColor)),
          `${control.getAttribute("aria-label")}: button ${getComputedStyle(control).backgroundColor}; row ${getComputedStyle(parent).backgroundColor}`,
        ).toBeGreaterThanOrEqual(12);
        expect(difference(hover, rest)).toBeGreaterThanOrEqual(12);
      });
    }
    await pointer.hover(canvasElement, { position: { x: 0, y: 0 } });
    await userEvent.click(button);
    await expect(
      await canvas.findByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(stack).getByRole("button", {
        name: "Show all changes in feature/api",
      }),
    ).toHaveAttribute("aria-pressed", "false");
  }
</script>

<Story
  name="Pull request details beside stacks"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const unsubmitted = canvas.getByRole("region", {
      name: "Stack feature/workspace",
    });
    await expect(
      canvas.getAllByRole("region", { name: /^Stack feature\// })[0],
    ).toBe(unsubmitted);
    await expect(
      within(unsubmitted).getByRole("button", {
        name: "Pull request for feature/workspace",
      }),
    ).toHaveTextContent("Create PR");
    await expect(
      within(unsubmitted).queryByText(/Checks /),
    ).not.toBeInTheDocument();
    await expect(
      within(stack).getByRole("button", {
        name: "Pull request for feature/api",
      }),
    ).toHaveTextContent("#42");
    await expect(within(stack).getByText("Checks running")).toBeInTheDocument();
    await expect(
      within(
        within(stack).getByRole("button", {
          name: "Pull request for feature/api",
        }),
      ).getByRole("img", { name: "GitHub" }),
    ).toBeInTheDocument();
    const panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    panel.scrollIntoView({ block: "nearest", inline: "end" });
    const stackHeader = stack.querySelector("header")!;
    const panelHeader = panel.querySelector("header")!;
    await expect(panelHeader).toHaveAttribute("data-ui-part", "column-header");
    await expect(panelHeader.getBoundingClientRect().height).toBe(
      stackHeader.getBoundingClientRect().height,
    );
    await expect(getComputedStyle(panelHeader).backgroundColor).toBe(
      getComputedStyle(stackHeader).backgroundColor,
    );
    const edit = within(panelHeader).getByRole("button", {
      name: "Edit details",
    });
    await expect(edit.textContent?.trim()).toBe("");
    await expect(edit.querySelector("svg")).toBeInTheDocument();
    await expect(
      within(panel).getAllByRole("button", { name: "Edit details" }),
    ).toHaveLength(1);
    await expect(
      within(panel).queryByText("Description", { exact: true }),
    ).not.toBeInTheDocument();
    const close = within(panelHeader).getByRole("button", {
      name: "Close pull requests",
    });
    await expect(close.textContent?.trim()).toBe("");
    await expect(close.querySelector("svg")).toBeInTheDocument();
    await expect(
      within(panel).getByRole("heading", { name: "Stack" }),
    ).toBeInTheDocument();
    await expect(
      within(panel).getByRole("link", { name: "#41" }),
    ).toHaveAttribute("href", "https://github.com/team/repo/pull/41");
    await expect(
      within(panel).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(panel).getByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("running");
    await expect(
      within(panel).getByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("2 successful · 1 in progress");
    await expect(panel.getBoundingClientRect().left).toBeGreaterThanOrEqual(
      stack.getBoundingClientRect().right - 1,
    );
  }}
/>

<Story
  name="Open another PR and preview stack submission"
  args={{ panelOpen: false, workspacePullRequest: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/workspace",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Pull request for feature/workspace",
      }),
    );
    let panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    await expect(
      within(panel).getByRole("heading", { name: "Add workspace toolbar" }),
    ).toBeVisible();
    await userEvent.click(
      within(panel).getByRole("button", { name: "Close pull requests" }),
    );
    const api = canvas.getByRole("region", { name: "Stack feature/api" });
    await userEvent.click(
      within(api).getByRole("button", { name: "Submit stack feature/api" }),
    );
    panel = await canvas.findByRole("region", { name: "Pull request details" });
    panel.scrollIntoView({ block: "nearest", inline: "end" });
    const preview = within(panel).getByRole("region", {
      name: "Stack submission preview",
    });
    await expect(
      within(panel.querySelector("header")!).queryByRole("button", {
        name: "Edit details",
      }),
    ).not.toBeInTheDocument();
    await expect(
      within(preview).queryByText("Description", { exact: true }),
    ).not.toBeInTheDocument();
    await expect(preview).toHaveTextContent("feature/model → main");
    await expect(preview).toHaveTextContent("feature/api → feature/model");
    const composer = within(preview).getByRole("group", {
      name: "PR description for feature/api",
    });
    const description = within(composer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(description);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(description, "## API review");
    await userEvent.click(
      within(composer.closest("fieldset")!).getByRole("button", {
        name: "Preview description",
      }),
    );
    await expect(
      within(preview).getByRole("heading", { name: "API review" }),
    ).toBeVisible();
    await expect(
      within(preview).getByRole("button", { name: "Push branches and submit" }),
    ).toBeEnabled();
  }}
/>

<Story
  name="Forge buttons align with titles and retain hover contrast"
  args={{ panelOpen: false }}
  play={verifyForgeTitleButtons}
/>
<Story
  name="Dark forge buttons align with titles and retain hover contrast"
  args={{ panelOpen: false }}
  globals={{ colorMode: "dark" }}
  play={verifyForgeTitleButtons}
/>

<Story
  name="Submit stack from column menu"
  args={{ panelOpen: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const rows = within(stack).getAllByRole("group", {
      name: /^Branch actions for /,
    });
    await expect(rows.length).toBeGreaterThan(1);
    const submit = within(rows[0]!).getByRole("button", {
      name: "Submit stack feature/api",
    });
    const push = within(rows[0]!).getByRole("button", {
      name: "Push feature/api",
    });
    await expect(push.nextElementSibling).toBe(submit);
    for (const row of rows.slice(1))
      await expect(
        within(row).queryByRole("button", { name: /Submit stack/ }),
      ).not.toBeInTheDocument();
    await expect(
      within(stack.querySelector("header")!).queryByRole("button", {
        name: /Submit stack/,
      }),
    ).not.toBeInTheDocument();
    await expect(
      within(stack).getAllByRole("button", { name: /Submit stack/ }),
    ).toHaveLength(1);
    const menu = within(stack).getByRole("button", {
      name: "Actions for feature/api",
    });
    await userEvent.click(menu);
    await waitFor(() => expect(menu).toHaveAttribute("aria-expanded", "true"));
    const page = within(canvasElement.ownerDocument.body);
    await expect(
      await page.findByRole("menuitem", { name: "Submit stack" }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await expect(menu).toHaveFocus();
    await userEvent.click(menu);
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Submit stack" }),
    );
    const panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    const preview = within(panel).getByRole("region", {
      name: "Stack submission preview",
    });
    await expect(preview).toHaveTextContent("feature/model → main");
    await expect(preview).toHaveTextContent("feature/api → feature/model");
    await expect(
      within(preview).getByRole("button", { name: "Push branches and submit" }),
    ).toBeEnabled();
  }}
/>

<Story
  name="Preview and create PR for unsubmitted stack"
  args={{ panelOpen: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/workspace",
    });
    const create = within(stack).getByRole("button", {
      name: "Pull request for feature/workspace",
    });
    await expect(create).toHaveTextContent("Create PR");
    await userEvent.click(create);
    const panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    const preview = within(panel).getByRole("region", {
      name: "Stack submission preview",
    });
    await expect(preview).toHaveTextContent("feature/workspace → main");
    await expect(preview).toHaveTextContent("push, create, description");
    await expect(create).toHaveTextContent("Create PR");
    await userEvent.click(
      within(preview).getByRole("button", { name: "Push branches and submit" }),
    );
    await waitFor(() => expect(create).toHaveTextContent("#43"));
    await userEvent.click(create);
    await expect(
      await within(panel).findByRole("heading", {
        name: "Add workspace toolbar",
      }),
    ).toBeVisible();
  }}
/>
