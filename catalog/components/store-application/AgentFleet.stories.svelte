<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentFleetApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Reusable agent chat",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Agent Advanced uses Lucide and keeps unsaved environment input"
  args={{ directory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true });
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Add agent" })).toBeEnabled(),
    );
    await userEvent.click(c.getByRole("button", { name: "Add agent" }));
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    const advanced = editor.getByRole("button", {
      name: "Advanced",
    });
    await expect(
      advanced.querySelector("svg.lucide-chevron-right"),
    ).not.toBeNull();
    await expect(advanced).toHaveAttribute("aria-expanded", "false");
    advanced.focus();
    await userEvent.keyboard("{Enter}");
    const name = editor.getByRole("textbox", {
      name: "Environment variable name",
    });
    await userEvent.type(name, "AGENT_DRAFT");
    await userEvent.click(advanced);
    await expect(name).not.toBeVisible();
    await userEvent.click(advanced);
    await expect(name).toHaveValue("AGENT_DRAFT");
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
  }}
/>

<Story
  name="Contextual sidebar and persistent agent tags"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const chat = await c.findByRole("button", { name: "Open chat" });
    await userEvent.click(chat);
    await c.findByRole("group", { name: "Message to agents" });
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
    let editor = within(
      c.getByRole("group", { name: "Message to agents" }),
    ).getByRole("textbox", { name: "Markdown editor" });
    await expect(
      c.queryByRole("switch", {
        name: "Automatically mention agents",
      }),
    ).not.toBeInTheDocument();
    await userEvent.click(editor);
    await userEvent.type(editor, "@Help");
    const picker = within(document.body);
    const automatic = await picker.findByRole("switch", {
      name: "Automatically mention agents",
    });
    await expect(
      automatic.closest(
        '[data-ui-component="community-composer-mention-picker"]',
      ),
    ).not.toBeNull();
    await userEvent.click(automatic);
    await userEvent.click(
      await picker.findByRole("option", { name: /Helper/ }),
    );
    await expect(
      within(editor).getByRole("img", { name: "Helper, Offline" }),
    ).toBeVisible();
    const addressed = await c.findByRole("button", {
      name: "Don't automatically mention Helper in this thread",
    });
    await expect(
      addressed.closest(
        '[data-ui-component="community-composer-mention-button"]',
      ),
    ).toHaveAttribute("data-addressed", "true");
    await expect(
      c.queryByRole("switch", {
        name: "Automatically mention agents",
      }),
    ).not.toBeInTheDocument();
    await userEvent.type(editor, "Explain this repository");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await waitFor(() =>
      expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
        '"agent_ids":["agent-0"]',
      ),
    );
    await expect(
      await c.findByText("Hello from the reusable agent."),
    ).toBeVisible();
    editor = within(
      c.getByRole("group", { name: "Message to agents" }),
    ).getByRole("textbox", { name: "Markdown editor" });
    await expect(editor).toHaveTextContent("Helper");
    await expect(
      within(editor).getByRole("img", { name: "Helper, Idle" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: /^Close$/ }));
    await userEvent.click(chat);
    await userEvent.click(c.getByRole("button", { name: "Mention someone" }));
    await expect(
      picker.getByRole("switch", {
        name: "Automatically mention agents",
      }),
    ).toBeChecked();
    await userEvent.click(
      within(
        c.getByRole("toolbar", { name: "Message formatting and actions" }),
      ).getByRole("button", {
        name: "Don't automatically mention Helper in this thread",
      }),
    );
    await expect(
      c.queryByRole("button", {
        name: "Don't automatically mention Helper in this thread",
      }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Picker pin selects an agent without sending"
  args={{ general: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Mention someone" }),
    );
    const automatic = await c.findByRole("switch", {
      name: "Automatically mention agents",
    });
    const picker = within(
      automatic.closest<HTMLElement>(
        '[data-ui-component="community-composer-mention-picker"]',
      )!,
    );
    await userEvent.click(
      picker.getByRole("button", {
        name: /^Automatically mention Helper$/,
      }),
    );
    await expect(automatic).toBeChecked();
    await expect(
      picker.getByRole("button", {
        name: "Don't automatically mention Helper in this thread",
      }),
    ).toHaveAttribute("aria-pressed", "true");
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Helper");
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
    await userEvent.click(
      picker.getByRole("button", {
        name: "Don't automatically mention Helper in this thread",
      }),
    );
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).not.toHaveTextContent("Helper");
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>
<Story
  name="Repository-free draft"
  args={{ general: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByText("General chat");
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
    await expect(
      c.getByRole("group", { name: "Message to agents" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Body repository and workspace selections"
  args={{ multipleWorkspaces: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(await c.findByRole("button", { name: "Open chat" }));
    const selections = c.getByLabelText("Chat context");
    const header = canvasElement.querySelector<HTMLElement>(
      ".agent-chat-heading",
    );
    await expect(header).not.toBeNull();
    await expect(getComputedStyle(header!).borderBottomWidth).toBe("0px");
    await expect(within(header!).getAllByRole("button")).toHaveLength(1);
    await expect(
      within(header!).getByRole("button", { name: /^Close$/ }),
    ).toBeVisible();
    await expect(
      selections.closest('[data-ui-part="sidebar-body"]'),
    ).not.toBeNull();
    await expect(
      selections.closest('[data-ui-part="sidebar-header"]'),
    ).toBeNull();
    const workspace = await c.findByRole("combobox", {
      name: "Workspace for jj-store",
    });
    await userEvent.click(workspace);
    await userEvent.click(
      await page.findByRole("option", { name: /Feature workspace/ }),
    );
    await expect(workspace).toHaveTextContent("Feature workspace");
    const editor = within(
      c.getByRole("group", { name: "Message to agents" }),
    ).getByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(editor, "@Help");
    await userEvent.click(await page.findByRole("option", { name: /Helper/ }));
    await userEvent.type(editor, "Use this workspace");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await waitFor(() =>
      expect(
        JSON.parse(
          c.getByLabelText("Agent fleet requests").textContent ?? "[]",
        )[0].attachments[0],
      ).toMatchObject({
        repository: "jj-store",
        target_id: "workspace-1",
        operation: "2".repeat(128),
        commit: "2".repeat(40),
      }),
    );
    const close = c.getByRole("button", { name: /^Close$/ });
    await expect(close).toHaveAttribute("data-ui-part", "sidebar-close");
    await userEvent.click(close);
    await expect(
      c.queryByRole("group", { name: "Message to agents" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Models before the first chat"
  args={{ directory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true });
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Add agent" })).toBeEnabled(),
    );
    await userEvent.click(c.getByRole("button", { name: "Add agent" }));
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    await userEvent.click(editor.getByRole("combobox", { name: "Model" }));
    await userEvent.click(
      await page.findByRole("option", { name: /Fixture model/ }),
    );
    await expect(
      editor.getByRole("combobox", { name: "Model" }),
    ).toHaveTextContent("Fixture model");
    await expect(await editor.findByLabelText("Thinking")).toBeVisible();
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
  }}
/>
<Story
  name="Unavailable models retain the editor and retry"
  args={{ directory: true, modelFailure: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true });
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Add agent" })).toBeEnabled(),
    );
    await userEvent.click(c.getByRole("button", { name: "Add agent" }));
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    await editor.findByText(/Models could not be loaded/);
    await expect(
      editor.getByRole("button", { name: "Retry models" }),
    ).toBeEnabled();
    await expect(
      editor.getByRole("textbox", { name: "Agent name" }),
    ).toBeEnabled();
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
  }}
/>
