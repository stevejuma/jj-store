<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import ForgeFixture from "../../fixtures/ForgeFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Pull requests",
    component: ForgeFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Edit description and inspect CI logs"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Edit details" }),
    );
    const description = canvas.getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(description);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(description, "Updated explanation");
    await userEvent.click(canvas.getByRole("button", { name: "Save changes" }));
    await expect(await canvas.findByText("Updated explanation")).toBeVisible();
    await expect(
      canvas.queryByRole("textbox", { name: "Markdown editor" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Refresh checks" }),
    );
    await expect(
      await canvas.findByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("running");
    await userEvent.click(
      canvas.getByRole("button", { name: "Expand checks" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Actions for Build and test" }),
    );
    await expect(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "Cancel run",
      }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");

    await userEvent.click(
      canvas.getByRole("button", { name: "Actions for Build and test" }),
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "Jobs",
      }),
    );
    await userEvent.click(
      (await canvas.findAllByRole("button", { name: /^Actions for / }))[0]!,
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "View logs",
      }),
    );
    await expect(await canvas.findByLabelText("Job logs")).toHaveTextContent(
      "Assertion failed",
    );
  }}
/>
<Story
  name="Failed save keeps draft and publishes a toast"
  args={{ failure: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Edit details" }),
    );
    const description = canvas.getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(description);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(description, "Keep this draft");
    await userEvent.click(canvas.getByRole("button", { name: "Save changes" }));
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "The forge refused this description update",
      ),
    ).toBeVisible();
    await expect(description).toHaveTextContent("Keep this draft");
    await userEvent.click(
      canvas.getByRole("button", { name: "Preview description" }),
    );
    await expect(canvas.getByText("Keep this draft")).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Edit details" }));
    await expect(
      canvas.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Keep this draft");
  }}
/>

<Story
  name="Markdown description and merge method menu"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("heading", { name: "Review summary" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("link", { name: "review guidance" }),
    ).toHaveAttribute("href", "https://example.com/review");
    const edit = canvas.getByRole("button", { name: "Edit details" });
    await expect(edit.closest("header")).toHaveAttribute(
      "data-ui-part",
      "column-header",
    );
    await expect(edit.textContent?.trim()).toBe("");
    await expect(edit.querySelector("svg")).toBeInTheDocument();
    await expect(
      canvas.queryByText("Description", { exact: true }),
    ).not.toBeInTheDocument();
    await expect(edit).toHaveAttribute("aria-pressed", "false");
    edit.focus();
    await userEvent.keyboard("{Enter}");
    await expect(edit).toHaveAttribute("aria-pressed", "true");
    await expect(canvas.getByRole("textbox", { name: "Title" })).toBeVisible();
    const composer = canvas.getByRole("group", { name: "PR description" });
    await expect(
      within(composer).getByRole("textbox", { name: "Markdown editor" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Preview description" }),
    );
    await expect(
      canvas.getByRole("heading", { name: "Review summary" }),
    ).toBeVisible();
    await expect(edit).toHaveAttribute("aria-pressed", "false");
    await userEvent.click(
      canvas.getByRole("button", { name: "PR actions: Squash" }),
    );
    const menu = within(canvasElement.ownerDocument.body);
    await expect(
      menu.getByRole("menuitemradio", { name: "Squash" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.keyboard("{End}");
    await expect(
      menu.getByRole("menuitemradio", { name: "Rebase" }),
    ).toHaveFocus();
    await userEvent.keyboard("{Enter}");
    await expect(
      canvas.getByRole("button", { name: "PR actions: Rebase" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Provider-supported merge methods"
  args={{ mergeMethods: ["rebase"] }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "PR actions: Rebase" }),
    );
    const menu = within(canvasElement.ownerDocument.body);
    await expect(menu.getAllByRole("menuitemradio")).toHaveLength(1);
    await expect(
      menu.getByRole("menuitemradio", { name: "Rebase" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.keyboard("{Escape}");
    await expect(
      canvas.getByRole("button", { name: "PR actions: Rebase" }),
    ).toHaveFocus();
  }}
/>

<Story
  name="Rendered details with automatic checks"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("heading", { name: "Add API" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("textbox", { name: "Title" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.queryByLabelText("Session token"),
    ).not.toBeInTheDocument();
    const checks = await canvas.findByRole("region", { name: "CI checks" });
    await expect(
      within(checks).getByRole("button", { name: "Expand checks" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      within(checks).getByRole("button", { name: "Expand checks" }),
    );
    await expect(within(checks).getByText("Build and test")).toBeVisible();
    const description = canvas.getByRole("heading", { name: "Review summary" });
    await expect(checks.getBoundingClientRect().top).toBeGreaterThan(
      description.getBoundingClientRect().bottom,
    );
    await expect(
      canvas
        .getByRole("button", { name: "PR actions: Squash" })
        .getBoundingClientRect().bottom,
    ).toBeLessThan(checks.getBoundingClientRect().top);
  }}
/>

<Story
  name="Check loading failure keeps rendered details"
  args={{ failureChecks: true, provider: "forgejo" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("heading", { name: "Add API" }),
    ).toBeVisible();
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "The forge could not load checks",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("Checks unavailable");
    await expect(
      canvas.getByRole("heading", { name: "Review summary" }),
    ).toBeVisible();
    await expect(canvas.getAllByRole("img", { name: "Forgejo" })).toHaveLength(
      1,
    );
  }}
/>

<Story
  name="Forge connection lives in the menu"
  args={{ provider: "gitlab" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("heading", { name: "Add API" }),
    ).toBeVisible();
    await expect(canvas.getAllByRole("img", { name: "GitLab" })).toHaveLength(
      1,
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Pull request options" }),
    );
    const document = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      document.getByRole("menuitem", { name: "Forge connection…" }),
    );
    const dialog = await document.findByRole("dialog", {
      name: "Forge connection",
    });
    await expect(within(dialog).getByLabelText("Session token")).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await expect(
      canvas.queryByLabelText("Session token"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Earlier check results cannot imply success"
  args={{ staleChecks: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const checks = await canvas.findByRole("region", { name: "CI checks" });
    await expect(
      await within(checks).findByRole("heading", {
        name: "Checks need refresh",
      }),
    ).toBeVisible();
    await expect(checks).toHaveTextContent("Observed revision ffffffff");
    await expect(
      within(checks).queryByRole("heading", { name: "Checks passed" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Editing unavailable keeps the header pencil disabled"
  args={{ canEdit: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const edit = await canvas.findByRole("button", { name: "Edit details" });
    await expect(edit.closest("header")).toHaveAttribute(
      "data-ui-part",
      "column-header",
    );
    await expect(edit).toBeDisabled();
    await expect(edit.textContent?.trim()).toBe("");
    await expect(
      canvas.getAllByRole("button", { name: "Edit details" }),
    ).toHaveLength(1);
    await expect(
      canvas.queryByRole("textbox", { name: "Title" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.queryByText("Description", { exact: true }),
    ).not.toBeInTheDocument();
  }}
/>
