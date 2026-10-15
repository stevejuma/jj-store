<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Inline workspace",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
</script>

<Story
  name="Unknown submission keeps the draft until explicit observation"
  args={{ panelOpen: false, unknownSubmit: true }}
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
    await userEvent.click(
      within(form).getByRole("button", { name: "Create PR" }),
    );
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "Network submission outcome unknown",
      ),
    ).toBeVisible();
    await expect(form).not.toHaveTextContent(
      "Network submission outcome unknown",
    );
    await expect(
      within(form).getByRole("button", { name: "Create PR" }),
    ).toBeDisabled();
    await expect(
      within(form).getByRole("button", { name: "Cancel PR creation" }),
    ).toBeDisabled();
    await expect(
      within(form).getByRole("textbox", { name: "PR title" }),
    ).toHaveValue("feature/workspace");
    await userEvent.click(
      within(form).getByRole("button", { name: "Check submission outcome" }),
    );
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-ui",
    });
    await expect(
      within(details).getByRole("heading", { name: "feature/workspace" }),
    ).toBeVisible();
    await expect(
      within(stack).queryByRole("region", {
        name: "Create PR for feature/workspace",
      }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Create inline and float"
  args={{ panelOpen: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/workspace",
    });
    const create = within(stack).getByRole("button", {
      name: "Create PR for feature/workspace",
    });
    const actions = create.closest<HTMLElement>('[role="group"]')!;
    await expect(create.nextElementSibling).toBe(
      within(actions).getByRole("button", { name: "Push feature/workspace" }),
    );
    await userEvent.click(create);
    const form = await within(stack).findByRole("region", {
      name: "Create PR for feature/workspace",
    });
    const title = within(form).getByRole("textbox", { name: "PR title" });
    await expect(title).toHaveValue("feature/workspace");
    const editor = within(form).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(editor);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(editor, "Draft **review** context");
    await userEvent.click(
      within(form).getByRole("button", { name: "Float PR composer" }),
    );
    const page = within(canvasElement.ownerDocument.body);
    const floating = await page.findByRole("dialog", {
      name: "Create PR · feature/workspace",
    });
    await expect(
      within(floating).getByRole("textbox", { name: "Markdown editor" }),
    ).toBe(editor);
    await expect(editor).toHaveTextContent("Draft **review** context");
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await waitFor(() =>
      expect(
        within(form).getByRole("textbox", { name: "Markdown editor" }),
      ).toBe(editor),
    );
    await userEvent.click(
      within(form).getByRole("button", { name: "Cancel PR creation" }),
    );
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Create PR for feature/workspace",
      }),
    );
    await expect(
      within(stack).getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Draft **review** context");
  }}
/>
<Story
  name="PR summary stays above changes"
  args={{ panelOpen: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Pull request for feature/api",
      }),
    );
    await expect(canvas.getByText("No file selected")).toBeVisible();
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in feature/api",
      }),
    );
    await userEvent.click(
      await within(stack).findByRole("button", { name: "src/api.ts" }),
    );
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    const toggle = within(details).getByRole("button", {
      name: "Collapse pull request #42",
    });
    const summary = toggle.closest<HTMLElement>("[data-forge-summary]")!;
    await expect(summary.getBoundingClientRect().bottom).toBeLessThan(
      within(details)
        .getByRole("region", { name: "Selected file diff" })
        .getBoundingClientRect().top,
    );
    await expect(
      within(summary).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(summary).queryByRole("region", { name: "CI checks" }),
    ).not.toBeInTheDocument();
    await userEvent.click(toggle);
    await expect(
      within(details).getByRole("region", { name: "Selected file diff" }),
    ).toBeVisible();
    await expect(
      within(details).getByRole("button", { name: "Expand pull request #42" }),
    ).toBeVisible();
    await userEvent.click(
      within(details).getByRole("button", { name: "Expand pull request #42" }),
    );
    await userEvent.click(
      within(summary).getByRole("button", { name: "Float PR details" }),
    );
    const floating = await within(canvasElement.ownerDocument.body).findByRole(
      "dialog",
      { name: "PR #42 · feature/api" },
    );
    await expect(
      within(floating).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(floating).queryByRole("region", { name: "CI checks" }),
    ).not.toBeInTheDocument();
    const fileDiff = within(details).getByRole("region", {
      name: "Selected file diff",
    });
    const sections = details.querySelector<HTMLElement>(
      ".stack-workspace-details__sections",
    )!;
    await expect(
      Math.abs(
        fileDiff.getBoundingClientRect().top -
          sections.getBoundingClientRect().top,
      ),
    ).toBeLessThanOrEqual(1);
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await expect(
      within(details).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(fileDiff.getBoundingClientRect().top).toBeGreaterThan(
      summary.getBoundingClientRect().bottom,
    );
    await userEvent.click(
      within(details).getByRole("button", { name: "Close file diff" }),
    );
    await expect(within(details).getByText("No file selected")).toBeVisible();
    await expect(
      within(details).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await userEvent.click(
      within(details).getByRole("button", { name: "Inspect checks" }),
    );
    await expect(
      await canvas.findByRole("region", { name: "CI checks" }),
    ).toBeVisible();
  }}
/>

<Story
  name="File selection brings the matching PR into the diff column"
  args={{ panelOpen: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in feature/model",
      }),
    );
    await userEvent.click(
      await within(stack).findByRole("button", { name: "src/model.ts" }),
    );
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    await expect(
      within(details).getByRole("heading", { name: "Add repository model" }),
    ).toBeVisible();
    await expect(
      within(details).getByRole("region", { name: "Selected file diff" }),
    ).toHaveTextContent("src/model.ts");
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in feature/api",
      }),
    );
    await userEvent.click(
      await within(stack).findByRole("button", { name: "src/api.ts" }),
    );
    await expect(
      within(details).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(details).getByRole("region", { name: "Selected file diff" }),
    ).toHaveTextContent("src/api.ts");
    await expect(
      canvas.getAllByRole("region", {
        name: "Selected changes from stack-api",
      }),
    ).toHaveLength(1);
    await userEvent.click(
      within(details).getByRole("button", { name: "Close details column" }),
    );
    await expect(
      canvas.queryByRole("region", { name: "Selected changes from stack-api" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Only the top unsubmitted change offers creation"
  args={{ panelOpen: false, apiPullRequests: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await expect(
      within(stack).getAllByRole("button", { name: /^Create PR for / }),
    ).toHaveLength(1);
    await expect(
      within(stack).queryByRole("button", {
        name: "Create PR for feature/model",
      }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      within(stack).getByRole("button", { name: "Create PR for feature/api" }),
    );
    const form = await within(stack).findByRole("region", {
      name: "Create PR for feature/api",
    });
    await expect(
      within(form).getByRole("textbox", { name: "PR title" }),
    ).toHaveValue("feature/api");
    await userEvent.click(within(form).getByText("Stack dependencies (1)"));
    await expect(
      within(form).getByRole("textbox", { name: "PR title for feature/model" }),
    ).toHaveValue("feature/model");
  }}
/>
<Story
  name="Preview failure uses a toast and cannot submit"
  args={{ panelOpen: false, failPreview: true }}
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
      await within(canvasElement.ownerDocument.body).findByText(
        "The forge could not prepare this pull request",
      ),
    ).toBeVisible();
    await expect(
      within(form).queryByRole("button", { name: "Create PR" }),
    ).not.toBeInTheDocument();
    await expect(form).not.toHaveTextContent(
      "The forge could not prepare this pull request",
    );
    await expect(
      within(form).getByRole("button", { name: "Retry PR preview" }),
    ).toBeEnabled();
    await userEvent.click(
      within(form).getByRole("button", { name: "Cancel PR creation" }),
    );
    await expect(
      within(stack).getByRole("button", {
        name: "Create PR for feature/workspace",
      }),
    ).toBeEnabled();
  }}
/>
