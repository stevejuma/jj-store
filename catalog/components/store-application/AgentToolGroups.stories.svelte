<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import { agentCommentReplies } from "./agent-comment-replies-play.js";
  import Fixture from "../../fixtures/AgentToolGroupsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent tool groups",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  const openSession = async (canvasElement: HTMLElement) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    return c;
  };
</script>

<Story
  name="Grouped rows align with messages and keep hover actions clickable"
  play={async ({ canvasElement }) => {
    const c = await openSession(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Expand 3 tool calls" }),
    );
    const heading = c.getByRole("button", { name: "Collapse 3 tool calls" });
    const group = heading.closest('[data-ui-component="ai-chat-tool-calls"]')!;
    const message = c.getAllByRole("article")[0]!;
    await expect(
      Math.abs(
        group.getBoundingClientRect().left -
          message.getBoundingClientRect().left,
      ),
    ).toBeLessThan(1);
    const calls = Array.from(
      group.querySelectorAll<HTMLElement>('[data-ui-part="agent-tool-review"]'),
    );
    const triggers = calls.map((call) =>
      call.querySelector<HTMLElement>('[data-ai-chat-part="call-trigger"]')!,
    );
    const checkSpacing = async () => {
      await waitFor(() => {
        for (let index = 0; index < calls.length; index++) {
          expect(
            Math.abs(
              triggers[index]!.getBoundingClientRect().left -
                heading.getBoundingClientRect().left,
            ),
          ).toBeLessThan(1);
          expect(
            triggers[index]!.getBoundingClientRect().height,
          ).toBeLessThanOrEqual(26);
          if (index > 0)
            expect(
              Math.abs(
                calls[index]!.getBoundingClientRect().top -
                  calls[index - 1]!.getBoundingClientRect().bottom,
              ),
            ).toBeLessThan(1);
        }
      });
    };
    await checkSpacing();
    for (const tool of calls) {
      tool.scrollIntoView({ block: "center" });
      await userEvent.hover(
        within(tool).getByRole("group", { name: /tool row$/ }),
      );
      const actions = within(tool).getByRole("group", {
        name: `Actions for ${tool.getAttribute("aria-label")}`,
      });
      await waitFor(() => expect(getComputedStyle(actions).opacity).toBe("1"));
      for (const button of within(actions).getAllByRole("button")) {
        const bounds = button.getBoundingClientRect();
        for (const y of [bounds.top + 2, bounds.bottom - 2]) {
          await expect(
            button.contains(
              document.elementFromPoint(bounds.left + bounds.width / 2, y),
            ),
          ).toBe(true);
        }
      }
      const context = tool.parentElement!;
      await expect(getComputedStyle(context).overflowY).toBe("visible");
      await userEvent.unhover(
        within(tool).getByRole("group", { name: /tool row$/ }),
      );
    }
    await userEvent.click(triggers[1]!);
    await expect(
      within(calls[1]!).getByText("Permission denied by the fixture", {
        exact: true,
      }),
    ).toBeVisible();
    await checkSpacing();
    await userEvent.click(triggers[1]!);
    await checkSpacing();
  }}
/>

<Story
  name="Adjacent calls group and failed details retain the actual error"
  play={async ({ canvasElement }) => {
    const c = await openSession(canvasElement);
    const group = c.getByRole("button", { name: "Expand 3 tool calls" });
    await expect(group).toHaveAttribute("aria-expanded", "false");
    await expect(c.queryByText("Tool failed")).toBeNull();
    await expect(
      c.getByRole("button", { name: "Show details for Search" }),
    ).toBeVisible();
    await userEvent.click(group);
    const calls = canvasElement.querySelectorAll(
      '[data-ui-part="agent-tool-review"]',
    );
    await expect(
      Array.from(calls).map((call) => call.getAttribute("aria-label")),
    ).toEqual([
      "apply_patch tool call",
      "Bash tool call",
      "Read tool call",
      "Search tool call",
    ]);
    await expect(c.queryByText("Tool failed")).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Bash" }),
    );
    await expect(
      within(c.getByRole("group", { name: "Bash tool call" })).getByText(
        "Permission denied by the fixture",
        { exact: true },
      ),
    ).toBeVisible();
    await expect(
      c
        .getByRole("group", { name: "Bash tool call" })
        .querySelector('[data-ui-part="call-status"]'),
    ).toHaveAttribute("data-status", "error");
    await userEvent.click(
      c.getByRole("button", { name: "Collapse 3 tool calls" }),
    );
    await expect(
      c.queryByRole("button", { name: "Hide details for Bash" }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Expand 3 tool calls" }),
    );
    await expect(
      c.getByRole("button", { name: "Hide details for Bash" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Grouped calls retain native menus and independent reply drafts"
  play={async ({ canvasElement }) => {
    const c = await openSession(canvasElement),
      body = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Expand 3 tool calls" }),
    );
    const tool = c.getByRole("group", { name: "Bash tool call" });
    const write = fn(),
      descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      await userEvent.hover(
        within(tool).getByRole("group", { name: /tool row$/ }),
      );
      await userEvent.click(
        within(tool).getByRole("button", { name: "More Bash tool actions" }),
      );
      await userEvent.click(body.getByRole("menuitem", { name: /^Copy$/ }));
      await body.findByRole("menu", { name: /^Copy$/ });
      await userEvent.click(
        body.getByRole("menuitem", { name: "Copy tool call ID" }),
      );
      await expect(write).toHaveBeenLastCalledWith("e1".repeat(16));
      await userEvent.click(
        within(tool).getByRole("button", { name: "Reply to Bash tool call" }),
      );
      await userEvent.type(
        within(tool).getByRole("textbox", { name: "Markdown editor" }),
        "Bash draft",
      );
      const read = c.getByRole("group", { name: "Read tool call" });
      await userEvent.hover(
        within(read).getByRole("group", { name: /tool row$/ }),
      );
      await userEvent.click(
        within(read).getByRole("button", { name: "Reply to Read tool call" }),
      );
      await userEvent.type(
        within(read).getByRole("textbox", { name: "Markdown editor" }),
        "Read draft",
      );
      await userEvent.click(
        c.getByRole("button", { name: /Open profile menu for/ }),
      );
      await userEvent.click(body.getByRole("menuitem", { name: "Settings" }));
      await userEvent.click(
        await c.findByRole("button", { name: "Appearance" }),
      );
      await userEvent.click(c.getByRole("radio", { name: "Focus" }));
      await userEvent.click(
        c.getByRole("button", { name: "Back to jj-store" }),
      );
      const sheet = await body.findByRole("region", { name: "Agent response" });
      const restoredBash = within(sheet).getByRole("group", {
        name: "Bash tool call",
      });
      const restoredRead = within(sheet).getByRole("group", {
        name: "Read tool call",
      });
      await expect(
        within(restoredRead).getByRole("textbox", { name: "Markdown editor" }),
      ).toHaveTextContent("Read draft");
      await expect(
        within(restoredBash).getByRole("textbox", { name: "Markdown editor" }),
      ).toHaveTextContent("Bash draft");
      await expect(
        document.querySelectorAll('[data-ui-part="surface-layer-surface"]'),
      ).toHaveLength(1);
      await userEvent.click(
        within(restoredRead).getByRole("button", { name: "Cancel" }),
      );
      await expect(within(restoredRead).queryByRole("textbox")).toBeNull();
      await expect(within(restoredBash).getByRole("textbox")).toHaveTextContent(
        "Bash draft",
      );
      await userEvent.keyboard("{Escape}");
      await waitFor(() =>
        expect(
          body.queryByRole("region", { name: "Agent response" }),
        ).toBeNull(),
      );
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>

<Story
  name="Tool comments and reply composer stay in a full row below their call"
  play={async ({ canvasElement }) => {
    const c = await openSession(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Expand 3 tool calls" }),
    );
    const tool = c.getByRole("group", { name: "apply_patch tool call" });
    await userEvent.hover(
      within(tool).getByRole("group", { name: /tool row$/ }),
    );
    await userEvent.click(
      within(tool).getByRole("button", {
        name: "Reply to apply_patch tool call",
      }),
    );
    const editor = await within(tool).findByRole("textbox", {
      name: "Markdown editor",
    });
    const trigger = within(tool).getByRole("button", {
      name: "Show details for apply_patch",
    });
    const composer = editor.closest<HTMLElement>(
      '[data-ui-part="agent-tool-reply-composer"]',
    )!;
    await expect(composer.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      trigger.getBoundingClientRect().bottom,
    );
    await expect(
      Math.abs(
        composer.getBoundingClientRect().width -
          tool.getBoundingClientRect().width,
      ),
    ).toBeLessThan(1);
    await expect(
      within(tool).getByText(
        "Can we preserve the exact operation when this retry races?",
        { exact: true },
      ),
    ).toBeVisible();
    await userEvent.type(editor, "Keep this comment under the tool");
    await userEvent.click(
      within(composer).getByRole("button", { name: "Reply" }),
    );
    await within(tool).findByText("Keep this comment under the tool", {
      exact: true,
    });
    await userEvent.click(
      within(composer).getByRole("button", { name: "Cancel" }),
    );
    await expect(
      within(tool).queryByRole("textbox", { name: "Markdown editor" }),
    ).toBeNull();
    await expect(
      within(tool).getByText("Keep this comment under the tool", {
        exact: true,
      }),
    ).toBeVisible();
    const other = c.getByRole("group", { name: "Bash tool call" });
    await userEvent.hover(
      within(other).getByRole("group", { name: /tool row$/ }),
    );
    await userEvent.click(
      within(other).getByRole("button", { name: "Reply to Bash tool call" }),
    );
    await expect(
      within(tool).getByText("Keep this comment under the tool", {
        exact: true,
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Tool and comment hover actions stay independent"
  play={async ({ canvasElement }) => {
    const c = await openSession(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Expand 3 tool calls" }),
    );
    const tool = c.getByRole("group", { name: "apply_patch tool call" });
    const comment = (
      await within(tool).findByText(
        "Can we preserve the exact operation when this retry races?",
        { exact: true },
      )
    ).closest(".community-message") as HTMLElement;
    const actions = within(tool).getByRole("group", {
      name: "Actions for apply_patch tool call",
    });
    await userEvent.hover(
      within(tool).getByRole("button", {
        name: "Show details for apply_patch",
      }),
    );
    await waitFor(() => expect(getComputedStyle(actions).opacity).toBe("1"));
    await userEvent.unhover(
      within(tool).getByRole("button", {
        name: "Show details for apply_patch",
      }),
    );
    await userEvent.hover(comment);
    await waitFor(() => {
      expect(getComputedStyle(actions).opacity).toBe("0");
      expect(
        getComputedStyle(
          comment.querySelector('[data-ui-part="message-actions"]')!,
        ).opacity,
      ).toBe("1");
    });
    await expect(actions).not.toHaveAttribute("data-visible");
  }}
/>

<Story
  name="Agent comment replies use Thread indentation and connectors"
  play={agentCommentReplies}
/>
