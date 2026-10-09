<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentFleetApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent completed changes",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Selected live avatar and completed file panel"
  args={{ completedChanges: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const header = canvasElement.querySelector<HTMLElement>(
      ".agent-chat-heading",
    )!;
    await waitFor(() =>
      expect(
        within(header).getByRole("img", { name: "Helper, Idle" }),
      ).toBeVisible(),
    );
    const presence = header.querySelector('[data-ui-part="presence"]')!;
    expect(presence.textContent?.trim()).toBe("");
    expect(presence.children).toHaveLength(0);
    await expect(within(header).getByText("Helper")).toBeVisible();
    await expect(
      within(header).getByRole("button", { name: /^Close$/ }),
    ).toBeVisible();
    const panel = await c.findByRole(
      "region",
      { name: "Changes in aaaaaaaa" },
      { timeout: 10000 },
    );
    const p = within(panel);
    await expect(p.getByText("Edited 20 files")).toBeVisible();
    await expect(p.getByText("+40")).toBeVisible();
    await expect(p.getByText("-20")).toBeVisible();
    await expect(
      p.getAllByRole("button", { name: /View changes to/ }),
    ).toHaveLength(3);
    await userEvent.click(
      p.getByRole("button", { name: "Show 17 more files" }),
    );
    await expect(
      p.getAllByRole("button", { name: /View changes to/ }),
    ).toHaveLength(20);
    await userEvent.click(p.getByRole("button", { name: "Show fewer files" }));
    await expect(
      p.getAllByRole("button", { name: /View changes to/ }),
    ).toHaveLength(3);
    await expect(
      p.getByRole("button", { name: "Undo unavailable" }),
    ).toBeDisabled();
    await userEvent.click(
      p.getByRole("button", { name: "View changes to src/example-2.ts" }),
    );
    const dialog = await within(document.body).findByRole("dialog");
    await expect(
      within(dialog).getByRole("combobox", { name: "Changed file" }),
    ).toHaveValue("src/example-2.ts");
    await expect(
      within(dialog).getByText("old", { exact: true }),
    ).toBeVisible();
    await expect(
      within(dialog).getByText("more", { exact: true }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        within(document.body).queryByRole("dialog"),
      ).not.toBeInTheDocument(),
    );
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>

<Story
  name="Reconnect warning stays inside the composer and clears after refresh"
  args={{ reconnect: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const warning = await c.findByText(
      "Chat updates are unavailable. Reconnecting…",
    );
    await expect(
      warning.closest('[data-ui-component="ai-chat-composer"]'),
    ).not.toBeNull();
    const editor = c.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(editor);
    await userEvent.type(editor, "Retain this draft");
    await userEvent.click(
      c.getByRole("button", { name: "Restore chat updates" }),
    );
    await waitFor(() =>
      expect(
        c.queryByText("Chat updates are unavailable. Reconnecting…"),
      ).not.toBeInTheDocument(),
    );
    await expect(editor).toHaveTextContent("Retain this draft");
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>
