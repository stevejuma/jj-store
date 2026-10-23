<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Workspace",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
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
    const row = within(stack).getByRole("group", {
      name: "Branch status for feature/api",
    });
    await expect(button.parentElement).toBe(row);
    await expect(row.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      title.getBoundingClientRect().bottom,
    );
    await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth + 1);
    const lower = within(stack).getByRole("button", {
      name: "Pull request for feature/model",
    });
    const lowerRow = within(stack).getByRole("group", {
      name: "Branch status for feature/model",
    });
    await expect(lower.parentElement).toBe(lowerRow);
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
        name: "Create PR for feature/workspace",
      }),
    ).toHaveTextContent("Create PR");
    await expect(
      within(unsubmitted).queryByText(/Checks /),
    ).not.toBeInTheDocument();
    const badge = within(stack).getByRole("button", {
      name: "Pull request for feature/api",
    });
    await expect(badge).toHaveTextContent("#42");
    await expect(
      within(badge).getByRole("img", { name: "GitHub" }),
    ).toBeInTheDocument();
    await expect(
      within(stack).getByRole("status", { name: "Checks running" }),
    ).toHaveTextContent("Running");
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    const panel = within(details).getByRole("region", {
      name: "Pull request for feature/api",
    });
    await expect(
      within(stack).queryByRole("region", {
        name: "Pull request for feature/api",
      }),
    ).not.toBeInTheDocument();
    const header = panel.querySelector("header")!;
    await expect(
      within(header).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(header).getByRole("link", { name: "#42" }),
    ).toBeVisible();
    await expect(
      within(header).getByText("Draft", { exact: true }),
    ).toBeVisible();
    await expect(
      within(header).getByRole("button", { name: "Float PR details" }),
    ).toBeVisible();
    await expect(
      within(header).queryByRole("button", { name: "Inspect checks" }),
    ).not.toBeInTheDocument();
    const actions = within(header).getByRole("button", {
      name: "PR actions: Squash",
    });
    await userEvent.click(actions);
    const menu = within(canvasElement.ownerDocument.body);
    await waitFor(() =>
      expect(
        menu.getByRole("menuitem", { name: "Ready for review" }),
      ).toBeVisible(),
    );
    await expect(
      menu.getByRole("menuitem", { name: "Restack…" }),
    ).toBeVisible();
    await expect(
      menu.getByRole("menuitemradio", { name: "Squash" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.keyboard("{Escape}");
    await expect(actions).toHaveFocus();
    const body = within(panel).getByRole("region", { name: "PR 42" });
    await expect(
      within(body).queryByText("team/repo", { exact: true }),
    ).not.toBeInTheDocument();
    await expect(body.querySelector(".forge-pr__identity")).toBeNull();
    await expect(
      within(body).getAllByRole("link", { name: "#42" }),
    ).toHaveLength(1);
    await expect(
      within(body).queryByRole("heading", { name: "Expose repository API" }),
    ).not.toBeInTheDocument();
    await expect(
      within(body).queryByText("Draft", { exact: true }),
    ).not.toBeInTheDocument();
    const edit = within(panel).getByRole("button", { name: "Edit details" });
    await expect(edit.closest("header")).toBeInTheDocument();
    await expect(edit.textContent?.trim()).toBe("");
    await expect(edit.querySelector("svg")).toBeInTheDocument();
    await expect(
      within(panel).getAllByRole("button", { name: "Edit details" }),
    ).toHaveLength(1);
    await expect(
      within(panel).queryByText("Description", { exact: true }),
    ).not.toBeInTheDocument();
    await expect(
      within(panel).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(panel).getByRole("link", { name: "#41" }),
    ).toHaveAttribute("href", "https://github.com/team/repo/pull/41");
    await expect(
      within(panel).getByRole("region", { name: "CI checks" }),
    ).toBeVisible();
    await expect(
      within(panel).getByRole("button", { name: "Expand checks" }),
    ).toBeEnabled();
    await expect(panel.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      within(details)
        .getByRole("region", { name: "Selected file diff" })
        .getBoundingClientRect().top,
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
    await expect(
      canvas.getByRole("heading", { name: "Add workspace toolbar" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Collapse pull request #43" }),
    );
    const api = canvas.getByRole("region", { name: "Stack feature/api" });
    await userEvent.click(
      within(api).getByRole("button", { name: "Submit stack feature/api" }),
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
      within(panel.querySelector("header")!).queryByRole("button", {
        name: "Edit details",
      }),
    ).not.toBeInTheDocument();
    await expect(
      within(preview).queryByText("Description", { exact: true }),
    ).not.toBeInTheDocument();
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
    await waitFor(() =>
      expect(
        page.getByRole("menuitem", { name: "Submit stack" }),
      ).toBeVisible(),
    );
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
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Create PR for feature/workspace",
      }),
    );
    const form = await within(stack).findByRole("region", {
      name: "Create PR for feature/workspace",
    });
    await expect(
      within(form).getByRole("textbox", { name: "PR title" }),
    ).toHaveValue("feature/workspace");
    await expect(form).toHaveTextContent("feature/workspace → main");
    await userEvent.click(
      within(form).getByRole("button", { name: "Create PR" }),
    );
    const badge = await within(stack).findByRole("button", {
      name: "Pull request for feature/workspace",
    });
    await expect(badge).toHaveTextContent("#43");
    await expect(
      await canvas.findByRole("heading", { name: "feature/workspace" }),
    ).toBeVisible();
    await expect(
      within(stack).queryByRole("region", { name: "CI checks" }),
    ).not.toBeInTheDocument();
  }}
/>
