<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentFleetApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent completed changes",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Selected live avatar and completed file panel"
  args={{ completedChanges: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(canvasElement.querySelector(".agent-chat-heading")).not.toBeNull(),
    );
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
    await waitFor(
      () =>
        expect(
          c.getAllByRole("region", { name: /Changes in aaaaaaaa/ }),
        ).toHaveLength(2),
      { timeout: 10000 },
    );
    const panels = c.getAllByRole("region", { name: /Changes in aaaaaaaa/ });
    const panel = panels[1]!;
    const p = within(panel);
    await expect(p.getByText("Edited 20 files")).toBeVisible();
    await waitFor(() => expect(p.getByText("+40")).toBeVisible());
    await expect(p.getByText("-20")).toBeVisible();
    await expect(
      panel.querySelectorAll('[data-ui-part="file-row"]'),
    ).toHaveLength(3);
    await userEvent.click(
      p.getByRole("button", { name: "Show 17 more files" }),
    );
    await expect(
      panel.querySelectorAll('[data-ui-part="file-row"]'),
    ).toHaveLength(20);
    await userEvent.click(p.getByRole("button", { name: "Show fewer files" }));
    await expect(
      panel.querySelectorAll('[data-ui-part="file-row"]'),
    ).toHaveLength(3);
    await expect(
      p.getByRole("button", { name: "Undo unavailable" }),
    ).toBeDisabled();
    const cardBounds = panel.getBoundingClientRect();
    const hostBounds = panel.parentElement!.getBoundingClientRect();
    expect(Math.abs(cardBounds.left - hostBounds.left)).toBeLessThan(1);
    expect(Math.abs(cardBounds.right - hostBounds.right)).toBeLessThan(1);
    await expect(
      p.getByRole("button", { name: /^Copy Git commit SHA/ }),
    ).toBeVisible();
    await expect(
      p.getByRole("button", { name: /^Copy JJ Change ID/ }),
    ).toBeVisible();
    await userEvent.click(p.getByRole("radio", { name: "Show files as list" }));
    await expect(
      panel.querySelector('[data-ui-component="file-listing"]'),
    ).toHaveAttribute("data-mode", "list");
    await userEvent.click(p.getByRole("button", { name: "View changes" }));
    await expect(
      c.queryByRole("complementary", { name: "Workspace file" }),
    ).not.toBeInTheDocument();
    await userEvent.click(p.getByRole("button", { name: /example-2.ts/ }));
    const rail = await c.findByRole("complementary", {
      name: "Workspace files",
    });
    await expect(
      await within(rail).findByRole("tab", { name: "Changes" }),
    ).toHaveAttribute("data-state", "active");
    await expect(
      await within(rail).findByRole("button", { name: "Comparison range" }),
    ).toHaveTextContent("Selected turn");
    const inspector = await c.findByRole("complementary", {
      name: "Workspace file",
    });
    await expect(
      await within(inspector).findByText("old", { exact: true }),
    ).toBeVisible();
    await expect(
      await within(inspector).findByText("more", { exact: true }),
    ).toBeVisible();
    await expect(
      within(document.body).queryByRole("dialog"),
    ).not.toBeInTheDocument();
    await userEvent.click(
      within(inspector).getByRole("button", { name: "Close workspace file" }),
    );
    await waitFor(() =>
      expect(
        c.queryByRole("complementary", { name: "Workspace file" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(
      within(
        await c.findByRole("region", { name: /Changes in aaaaaaaa.*earlier-/ }),
      ).getByRole("button", { name: /example-1.ts/ }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("complementary", { name: "Workspace file" }),
      ).toHaveTextContent("historic"),
    );
    await expect(
      c.queryByText("more", { exact: true }),
    ).not.toBeInTheDocument();
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
      warning.closest('[data-ui-component="community-chat-composer"]'),
    ).not.toBeNull();
    const editor = c.getByRole("textbox", { name: "Markdown editor" });
    await waitFor(() =>
      expect(getComputedStyle(editor).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(editor);
    await userEvent.type(editor, "Retain this draft");
    await storyControl(document.body, "Restore chat updates");
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

<Story
  name="Saved changes warning offers a styled retry without dispatching"
  args={{ completedChanges: true, changesFailure: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const title = await c.findByText(
      "File changes unavailable",
      {},
      { timeout: 10000 },
    );
    const alert = title.closest('[data-ui-part="alert"]')!;
    expect(alert).toHaveAttribute("role", "alert");
    expect(alert.querySelector("svg")).not.toBeNull();
    const retry = within(alert as HTMLElement).getByRole("button", {
      name: "Retry file changes",
    });
    expect(retry.querySelector("svg")).not.toBeNull();
    expect(retry).toHaveAttribute("data-variant", "secondary");
    const description = alert.querySelector<HTMLElement>(
      '[data-slot="alert-description"]',
    )!;
    expect(retry.parentElement).toHaveAttribute("data-ui-part", "alert-action");
    const retryBounds = retry.getBoundingClientRect();
    const alertBounds = alert.getBoundingClientRect();
    expect(retryBounds.left).toBeGreaterThan(
      description.getBoundingClientRect().right,
    );
    expect(
      Math.abs(
        retryBounds.top +
          retryBounds.height / 2 -
          (alertBounds.top + alertBounds.height / 2),
      ),
    ).toBeLessThan(1);
    expect(getComputedStyle(retry).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await userEvent.click(retry);
    await waitFor(() =>
      expect(c.queryByText("File changes unavailable")).not.toBeInTheDocument(),
    );
    await waitFor(() =>
      expect(
        c.getAllByRole("region", { name: /Changes in aaaaaaaa/ }),
      ).toHaveLength(2),
    );
    expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent("[]");
  }}
/>
