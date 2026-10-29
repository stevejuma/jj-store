<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentLayoutFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stored agent review draft",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      a11y: { test: "error" },
    },
  });
  async function review(canvasElement: HTMLElement) {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const panel = await c.findByRole("complementary", {
      name: "Agent response",
    });
    const local = within(panel);
    const trigger = await local.findByRole("button", { name: "Add review" });
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await expect(trigger.querySelector("svg")).toBeVisible();
    await expect(
      local.queryByRole("group", { name: "Agent session comment" }),
    ).toBeNull();
    await userEvent.click(trigger);
    const group = await local.findByRole("group", {
      name: "Agent session comment",
    });
    const card = group.closest(".chat-comment-list")!;
    await expect(
      card.querySelector('[data-ui-component="community-member-avatar"]'),
    ).toBeVisible();
    await expect(getComputedStyle(card).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    const editor = within(group).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(editor, "Retained review draft");
    await userEvent.click(trigger);
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await expect(
      local.queryByRole("group", { name: "Agent session comment" }),
    ).toBeNull();
    await userEvent.keyboard("{Enter}");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(
      local.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retained review draft");
  }
</script>

<Story
  name="Document review uses an expandable comment card"
  args={{ repository: true, focus: false }}
  play={async ({ canvasElement }) => review(canvasElement)}
/>
<Story
  name="Classic review uses an expandable comment card"
  args={{ repository: true, focus: false, conversationView: "classic" }}
  play={async ({ canvasElement }) => review(canvasElement)}
/>

<Story
  name="Pending document reviews keep the draft and receipt control visible"
  args={{ repository: true, focus: false, reviewOutcome: "unknown" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const panel = await c.findByRole("complementary", {
      name: "Agent response",
    });
    const local = within(panel);
    const trigger = local.getByRole("button", { name: "Add review" });
    await userEvent.click(trigger);
    const group = await local.findByRole("group", {
      name: "Agent session comment",
    });
    const editor = within(group).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(editor, "Review awaiting receipt");
    await userEvent.click(
      within(group).getByRole("button", { name: "Add comment" }),
    );
    await expect(trigger).toBeDisabled();
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(editor).toHaveTextContent("Review awaiting receipt");
    await userEvent.click(
      local.getByRole("button", { name: "Check comment status" }),
    );
    await expect(editor).not.toHaveTextContent("Review awaiting receipt");
    await local.findByText("Review awaiting receipt", { exact: true });
    await expect(trigger).toBeEnabled();
  }}
/>
