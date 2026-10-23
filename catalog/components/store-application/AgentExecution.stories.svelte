<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Interactive agent execution",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Standalone agent management"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByRole("button", { name: "View Repository helper" });
    await expect(c.getByRole("heading", { name: "Agents" })).toBeVisible();
    await expect(
      c.queryByRole("navigation", { name: "Agent sessions" }),
    ).not.toBeInTheDocument();
    await expect(
      page.queryByRole("dialog", { name: "Agent sessions" }),
    ).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Add agent" }));
    const editor = await page.findByRole("dialog", { name: "Add agent" });
    await userEvent.type(
      within(editor).getByLabelText("Agent name"),
      "Review helper",
    );
    await userEvent.click(
      within(editor).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(c.getByRole("button", { name: "Add agent" }));
    await expect(page.getByLabelText("Agent name")).toHaveValue(
      "Review helper",
    );
    await userEvent.type(
      within(page.getByRole("group", { name: "Agent instructions" })).getByRole(
        "textbox",
        { name: "Markdown editor" },
      ),
      "Explain every change.",
    );
    await userEvent.click(page.getByRole("button", { name: "Save agent" }));
    await c.findByRole("button", { name: "View Review helper" });
    await expect(
      page.queryByRole("dialog", { name: "Add agent" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Offline agent"
  args={{ status: "Offline" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByTitle("Offline");
  }}
/>
<Story
  name="Starting agent"
  args={{ status: "Starting" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByTitle("Starting");
  }}
/>
<Story
  name="Online agent"
  args={{ status: "Online" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByTitle("Online");
  }}
/>
<Story
  name="Working agent"
  args={{ status: "Working" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByTitle("Working");
  }}
/>
<Story
  name="Unknown retains gallery"
  args={{ status: "Unknown" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByTitle("Unknown");
    await expect(
      c.getByRole("button", { name: "View Repository helper" }),
    ).toBeVisible();
    await expect(c.getByRole("button", { name: "Add agent" })).toBeDisabled();
  }}
/>
<Story
  name="Empty agent gallery"
  args={{ status: "Empty" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByText(/No agents yet/);
  }}
/>
<Story
  name="Exact permission choices"
  args={{ permission: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    await c.findByRole("article", { name: "Agent permission request" });
    await expect(c.getByRole("button", { name: "Allow once" })).toBeEnabled();
    await userEvent.click(c.getByRole("button", { name: "Reject once" }));
    await expect(c.getByRole("button", { name: "Stop agent" })).toBeEnabled();
  }}
/>
<Story
  name="Draft survives unknown outcome and navigation"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep this draft");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await within(document.body).findByRole("button", {
      name: "Review workspace",
    });
    await expect(draft).toHaveTextContent("Keep this draft");
    await storyControl(document.body, "Manage fixture agents");
    await c.findByRole("heading", { name: "Agents" });
    await storyControl(document.body, "Open fixture conversation");
    await expect(
      await c.findByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Keep this draft");
  }}
/>
<Story
  name="Launch opens a draft chat in the inbox"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "New agent session" }));
    await c.findByRole("form", { name: "Start agent session" });
    await c.findByRole("button", {
      name: "Open session New chat · new-session",
    });
    await c.findByRole("textbox", { name: "Markdown editor" });
    await expect(
      c.getByRole("combobox", { name: "Starting revision" }),
    ).toHaveAttribute("title", "33".repeat(20));
    await expect(
      c.queryByRole("heading", { name: "Agents" }),
    ).not.toBeInTheDocument();
    await storyControl(document.body, "Manage fixture agents");
    await c.findByRole("heading", { name: "Agents" });
  }}
/>

<Story
  name="Community avatar and harness pickers"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByRole("button", { name: "View Repository helper" });
    const add = await c.findByRole("button", { name: "Add agent" });
    await waitFor(() => expect(add).toBeEnabled());
    await userEvent.click(add);
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    await userEvent.type(editor.getByLabelText("Agent name"), "Cursor helper");
    await userEvent.click(
      editor.getByRole("button", { name: "Choose agent image" }),
    );
    await userEvent.click(page.getByRole("tab", { name: "Emoji" }));
    await userEvent.click(
      page.getByRole("button", { name: "Use 🤖 as avatar" }),
    );
    await userEvent.click(
      page.getByRole("button", { name: "Use #30a46c background" }),
    );
    await userEvent.keyboard("{Escape}");
    await userEvent.click(editor.getByLabelText("Harness", { exact: true }));
    await userEvent.click(page.getByRole("option", { name: "Cursor" }));
    await expect(
      editor.getByLabelText("Harness", { exact: true }),
    ).toHaveTextContent("Cursor");
    await expect(
      editor.getByRole("combobox", { name: "Model" }),
    ).toHaveTextContent("Harness default");
    await expect(editor.queryByLabelText("Runner")).not.toBeInTheDocument();
    await userEvent.click(editor.getByRole("button", { name: "Save agent" }));
    const card = (await c.findByRole("button", { name: "View Cursor helper" }))
      .parentElement!;
    const image = card.querySelector("img")!;
    await expect(decodeURIComponent(image.getAttribute("src") ?? "")).toContain(
      'fill="#30a46c"',
    );
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "View Cursor helper" }),
    );
    await c.findByRole("complementary", { name: "Agent details sidebar" });
    await userEvent.click(
      c.getByRole("button", { name: "Edit Cursor helper" }),
    );
    await expect(
      page.getByLabelText("Harness", { exact: true }),
    ).toHaveTextContent("Cursor");
    await userEvent.click(page.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
  }}
/>

<Story
  name="Stale save retains draft"
  args={{ staleSave: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByRole("button", { name: "View Repository helper" });
    const add = await c.findByRole("button", { name: "Add agent" });
    await waitFor(() => expect(add).toBeEnabled());
    await userEvent.click(add);
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    await userEvent.type(
      editor.getByLabelText("Agent name"),
      "Retained helper",
    );
    await userEvent.click(editor.getByRole("button", { name: "Save agent" }));
    await editor.findByText(/Agent configurations changed/);
    await expect(editor.getByLabelText("Agent name")).toHaveValue(
      "Retained helper",
    );
    await userEvent.click(
      editor.getByRole("button", { name: "Keep draft and use latest version" }),
    );
    await userEvent.click(editor.getByRole("button", { name: "Save agent" }));
    await c.findByRole("button", { name: "View Retained helper" });
  }}
/>

<Story
  name="Agent card opens Community details"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    const card = await c.findByRole("button", {
      name: "View Repository helper",
    });
    card.focus();
    await userEvent.keyboard("{Enter}");
    const panel = within(
      await c.findByRole("complementary", { name: "Agent details sidebar" }),
    );
    await panel.findByRole("article", {
      name: "Repository helper agent profile",
    });
    await expect(
      page.queryByRole("dialog", { name: "Edit agent" }),
    ).not.toBeInTheDocument();
    await userEvent.click(panel.getByRole("tab", { name: "Runtime" }));
    await panel.findByRole("button", { name: "Edit agent harness" });
    await userEvent.click(
      panel.getByRole("button", { name: "Close agent details" }),
    );
    await expect(card).toHaveFocus();
  }}
/>

<Story
  name="Configuration preparation retains draft"
  args={{ preparing: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByRole("button", { name: "View Repository helper" });
    const add = await c.findByRole("button", { name: "Add agent" });
    await waitFor(() => expect(add).toBeEnabled());
    await userEvent.click(add);
    const editor = within(
      await page.findByRole("dialog", { name: "Add agent" }),
    );
    await userEvent.type(editor.getByLabelText("Agent name"), "Waiting helper");
    await editor.findByText(/Preparing agent configuration/);
    await expect(
      editor.getByRole("button", { name: "Save agent" }),
    ).toBeDisabled();
    await expect(editor.getByRole("button", { name: "Cancel" })).toBeEnabled();
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
  }}
/>
