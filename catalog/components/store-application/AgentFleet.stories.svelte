<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentFleetApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Reusable agent chat",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

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
    const editor = within(
      c.getByRole("group", { name: "Message to agents" }),
    ).getByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(editor);
    await userEvent.type(editor, "@Help");
    const picker = within(document.body);
    await userEvent.click(
      await picker.findByRole("option", { name: /Helper/ }),
    );
    await expect(
      within(editor).getByRole("img", { name: "Helper, Offline" }),
    ).toBeVisible();
    await userEvent.click(c.getByLabelText("Automatically mention agents"));
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
    await expect(editor).toHaveTextContent("Helper");
    await expect(
      within(editor).getByRole("img", { name: "Helper, Idle" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: /^Close$/ }));
    await userEvent.click(chat);
    await expect(
      c.getByLabelText("Automatically mention agents"),
    ).toBeChecked();
    await userEvent.click(
      c.getByRole("button", { name: "Stop always mentioning Helper" }),
    );
    await expect(
      c.queryByRole("button", { name: "Stop always mentioning Helper" }),
    ).not.toBeInTheDocument();
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
    await userEvent.click(await c.findByRole("button", { name: "Add agent" }));
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    await userEvent.click(
      await editor.findByRole("option", { name: /Fixture model/ }),
    );
    await expect(
      editor.getByText("Selected model: Fixture model"),
    ).toBeVisible();
    await expect(editor.getByLabelText("Thinking")).toBeVisible();
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
    await userEvent.click(await c.findByRole("button", { name: "Add agent" }));
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
