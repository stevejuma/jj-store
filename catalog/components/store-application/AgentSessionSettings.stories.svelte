<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent session settings",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  const workspace = "/device/agent-workspaces/helper/session-1234567890";
</script>

<Story
  name="Chat header menu and list toggle retain the draft"
  args={{
    workspacePath: workspace,
    savedModel: "configured-model",
    sessionPreferences: { model: null, effort: null, mode: null },
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep this session draft");
    const header = canvasElement.querySelector(
      ".lc-community-inbox__thread-header",
    )!;
    await expect(getComputedStyle(header).paddingLeft).toBe("16px");
    await expect(getComputedStyle(header).paddingTop).toBe("12px");
    await expect(
      c.getAllByRole("button", { name: "Send message" }),
    ).toHaveLength(1);
    await expect(
      c
        .getByRole("button", { name: "Send message" })
        .closest(".chat-message-footer"),
    ).not.toBeNull();
    await expect(
      c.queryByText("Conversation details", { exact: true }),
    ).not.toBeVisible();
    const toggle = c.getByRole("button", { name: "Toggle session chats" });
    await userEvent.click(toggle);
    await expect(
      c.queryByRole("complementary", { name: "Session chats" }),
    ).not.toBeInTheDocument();
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
    await expect(draft).toHaveTextContent("Keep this session draft");
    await userEvent.click(toggle);
    await c.findByRole("complementary", { name: "Session chats" });
    const menu = c.getByRole("button", { name: "Session options" });
    await userEvent.click(menu);
    for (const name of ["Activity log", "End session"])
      await expect(
        page.getByRole("menuitem", { name }).querySelector("svg"),
      ).not.toBeNull();
    await expect(page.getByRole("menu").getBoundingClientRect().width).toBe(
      288,
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Conversation details" }),
    );
    const runtime = within(
      await c.findByRole("complementary", { name: "Conversation details" }),
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    const copyPath = await runtime.findByRole("button", {
      name: `Copy full Workspace path: ${workspace}`,
    });
    const pathValue = copyPath
      .closest(".store-readonly-row")!
      .querySelector(".store-readonly-row__value")!;
    await expect(pathValue).toHaveTextContent(workspace);
    await expect(getComputedStyle(pathValue).textOverflow).toBe("ellipsis");
    await expect(pathValue.scrollWidth).toBeGreaterThan(pathValue.clientWidth);
    await expect(
      runtime.getByRole("button", {
        name: `Copy full Starting revision: ${"33".repeat(20)}`,
      }),
    ).toBeVisible();
    await expect(
      c.queryByRole("dialog", { name: "Conversation details" }),
    ).not.toBeInTheDocument();
    await expect(
      canvasElement.querySelector(".lc-community-inbox__settings"),
    ).toHaveAttribute("data-side", "right");
    await expect(
      canvasElement.querySelector(
        ".agent-runtime-settings .store-readonly-row",
      ),
    ).not.toBeNull();
    await expect(draft).toBeVisible();
    const settings = canvasElement.querySelector(
      ".lc-community-inbox__settings",
    )!;
    await expect(getComputedStyle(settings).backgroundColor).toBe(
      getComputedStyle(header).backgroundColor,
    );
    await userEvent.click(
      runtime.getByRole("button", { name: "Open starting revision" }),
    );
    await expect(
      JSON.parse(
        (await c.findByLabelText("Last opened revision")).textContent ?? "{}",
      ),
    ).toEqual({
      repository: "11".repeat(16),
      commit: "33".repeat(20),
      operation: "22".repeat(64),
    });
    await expect(
      runtime.queryByRole("button", { name: "Apply runtime preferences" }),
    ).not.toBeInTheDocument();
    await expect(
      runtime.queryByRole("button", { name: "Clean up workspace" }),
    ).not.toBeInTheDocument();
    await expect(
      runtime.getByRole("button", { name: "Edit model" }),
    ).toHaveTextContent("configured-model");
    await expect(
      runtime.getByRole("button", { name: "Edit execution mode" }),
    ).toHaveTextContent("ask");
    await userEvent.click(
      runtime.getByRole("button", { name: "Edit execution mode" }),
    );
    const editor = within(
      await page.findByRole("dialog", { name: "Edit agent" }),
    );
    await expect(
      editor.getByRole("textbox", { name: "Agent name" }),
    ).toHaveValue("Repository helper");
    await expect(
      editor.getByRole("button", { name: "Choose agent image" }),
    ).toBeVisible();
    await expect(
      editor.getByRole("form", { name: "Agent configuration" }),
    ).toBeVisible();
    const mode = editor.getByLabelText("Execution mode");
    await userEvent.click(mode);
    await userEvent.click(page.getByRole("option", { name: "Read only" }));
    await expect(mode).toHaveTextContent("Read only");
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(
      runtime.getByRole("button", { name: "Close conversation details" }),
    );
    await waitFor(() =>
      expect(
        c.queryByRole("complementary", { name: "Conversation details" }),
      ).not.toBeInTheDocument(),
    );
    await expect(menu).toHaveFocus();
    await userEvent.click(menu);
    await userEvent.click(
      page.getByRole("menuitem", { name: "Conversation details" }),
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Edit execution mode" }),
    );
    const retainedEditor = within(
      await page.findByRole("dialog", { name: "Edit agent" }),
    );
    await expect(
      retainedEditor.getByLabelText("Execution mode"),
    ).toHaveTextContent("Read only");
    await userEvent.click(
      retainedEditor.getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Close conversation details" }),
    );
    await expect(draft).toHaveTextContent("Keep this session draft");
    await userEvent.click(menu);
    await userEvent.click(page.getByRole("menuitem", { name: "End session" }));
    const command = await c.findByLabelText("Last session command");
    await expect(JSON.parse(command.textContent ?? "{}")).toMatchObject({
      session_id: "44".repeat(16),
      action: { action: "end" },
    });
    await expect(draft).toHaveTextContent("Keep this session draft");
  }}
/>

<Story
  name="Runtime settings submit exact negotiated preferences without sending the chat draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep this unapplied message");
    await userEvent.click(c.getByRole("button", { name: "Session options" }));
    await userEvent.click(
      page.getByRole("menuitem", { name: "Conversation details" }),
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Edit execution mode" }),
    );
    const editor = within(
      await page.findByRole("dialog", { name: "Edit agent" }),
    );
    await userEvent.click(editor.getByLabelText("Execution mode"));
    await userEvent.click(page.getByRole("option", { name: "Read only" }));
    await userEvent.click(editor.getByRole("button", { name: "Save agent" }));
    const command = await c.findByLabelText("Last session command");
    await expect(JSON.parse(command.textContent ?? "{}")).toMatchObject({
      session_id: "44".repeat(16),
      action: {
        action: "configure",
        preferences: { model: null, effort: null, mode: "read-only" },
      },
    });
    await expect(draft).toHaveTextContent("Keep this unapplied message");
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Edit agent" }),
      ).not.toBeInTheDocument(),
    );
    await expect(
      JSON.parse(c.getByLabelText("Session commands").textContent ?? "[]"),
    ).toHaveLength(1);
  }}
/>

<Story
  name="Ended session groups review and destructive cleanup after titled runtime sections"
  args={{ status: "Offline" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await userEvent.click(c.getByRole("button", { name: "Session options" }));
    await userEvent.click(
      page.getByRole("menuitem", { name: "Conversation details" }),
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    for (const name of ["Workspace", "Agent settings", "Actions"])
      await expect(c.getByRole("heading", { name })).toBeVisible();
    const cleanup = c.getByRole("button", { name: "Clean up workspace" });
    const actions = cleanup.closest(
      '[aria-label="Session workspace actions"]',
    )!;
    await expect(
      within(actions as HTMLElement).getByRole("button", {
        name: "Review changes",
      }),
    ).toBeVisible();
    await expect(cleanup).toHaveClass("lc-channel-settings__destructive");
    await expect(cleanup.querySelector("svg")).not.toBeNull();
    await expect(actions.getBoundingClientRect().top).toBeGreaterThan(
      c
        .getByRole("button", { name: "Edit execution mode" })
        .getBoundingClientRect().bottom,
    );
    await userEvent.click(c.getByRole("button", { name: "Edit model" }));
    const editor = within(
      await page.findByRole("dialog", { name: "Edit agent" }),
    );
    await expect(
      editor.getByRole("button", { name: "Save agent" }),
    ).toBeEnabled();
    await expect(
      editor.getByText(/Save changes the agent defaults for future chats/),
    ).toBeVisible();
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(cleanup);
    await expect(
      JSON.parse(
        (await c.findByLabelText("Last session command")).textContent ?? "{}",
      ),
    ).toMatchObject({
      session_id: "44".repeat(16),
      action: { action: "cleanup" },
    });
  }}
/>

<Story
  name="Unassigned session retains its runtime values without borrowing another agent"
  args={{
    agentAssociated: false,
    sessionPreferences: {
      model: "retained-model",
      effort: "high",
      mode: "read-only",
    },
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await userEvent.click(c.getByRole("button", { name: "Session options" }));
    await userEvent.click(
      page.getByRole("menuitem", { name: "Conversation details" }),
    );
    await expect(
      c.getByRole("button", { name: "Edit model" }),
    ).toHaveTextContent("retained-model");
    await expect(
      c.getByRole("button", { name: "Edit thinking" }),
    ).toHaveTextContent("high");
    await expect(
      c.getByRole("button", { name: "Edit execution mode" }),
    ).toHaveTextContent("read-only");
    await expect(
      c.queryByLabelText("Last session command"),
    ).not.toBeInTheDocument();
  }}
/>
