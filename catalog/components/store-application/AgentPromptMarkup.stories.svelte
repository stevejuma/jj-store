<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentPromptFixture.svelte";
  import { inertPromptMarkup } from "../../fixtures/agent-prompt-markup.js";
  const { Story } = defineMeta({
    title: "JJ Store/Agent prompt markup",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function nested({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const context = await c.findByRole("button", {
      name: '<harness:context source="cli > desktop">',
    });
    await expect(context).toHaveAttribute("aria-expanded", "false");
    await expect(c.getByText("Before", { exact: false })).toBeVisible();
    await userEvent.click(context);
    await expect(
      c.getByRole("heading", { name: /Session notes/ }),
    ).toBeVisible();
    await expect(c.getByRole("link", { name: "safe links" })).toHaveAttribute(
      "href",
      "https://example.com/",
    );
    const child = c.getByRole("button", {
      name: '<arbitrary_limits category="host">',
    });
    await userEvent.click(child);
    await expect(c.getByText("cargo test", { exact: true })).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "<environment_context>" }),
    );
    await expect(c.getByText("/workspace/demo", { exact: true })).toBeVisible();
    await expect(canvasElement.querySelector("pre")!).toHaveTextContent(
      "<literal><keep_this /></literal>",
    );
    await expect(c.queryByRole("button", { name: "<literal>" })).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "<another-tool.context>" }),
    );
    await expect(c.getByText("Markdown", { exact: true })).toBeVisible();
    await context.focus();
    await userEvent.keyboard("{Enter}");
    await expect(context).toHaveAttribute("aria-expanded", "false");
    await waitFor(() =>
      expect(c.getByText("cargo test", { exact: true })).not.toBeVisible(),
    );
    await expect(c.getByText("<example />", { exact: true })).toBeVisible();
  }
</script>

<Story
  name="Nested arbitrary prompt elements preserve Markdown"
  play={nested}
/>

<Story
  name="Ordinary Markdown retains safe HTML rendering"
  args={{
    ordinary: true,
    content:
      "Before **Markdown**.\n\n<em>ordinary emphasis</em>\n\nAfter **Markdown**.",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByText("ordinary emphasis", { exact: true }),
    ).toBeVisible();
    await expect(canvasElement.querySelector("em")).toHaveTextContent(
      "ordinary emphasis",
    );
    await expect(
      canvasElement.querySelectorAll("[data-prompt-element]"),
    ).toHaveLength(0);
    await expect(canvasElement.querySelectorAll("strong")).toHaveLength(2);
  }}
/>
<Story
  name="Recorded transcript chunks use the same extension"
  args={{ fallback: true }}
  play={nested}
/>

<Story
  name="Unfinished markup and code stay readable"
  args={{
    content:
      "<unclosed>\n\n**Keep this text**\n\n`<literal />`\n\n```xml\n<fenced><x /></fenced>\n```\n\n    <indented />\n\n\\<escaped>\n\n<wrong></other>\n",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByText("Keep this text", { exact: true }),
    ).toBeVisible();
    await expect(canvasElement.querySelector("strong")).toHaveTextContent(
      "Keep this text",
    );
    await expect(canvasElement.textContent).toContain("<unclosed>");
    await expect(canvasElement.textContent).toContain("<wrong></other>");
    await expect(
      canvasElement.querySelectorAll("[data-prompt-element]"),
    ).toHaveLength(0);
    await expect(canvasElement.querySelectorAll("pre")).toHaveLength(2);
  }}
/>

<Story
  name="Markup attributes remain inert and references span sections"
  args={{
    content: inertPromptMarkup,
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: '<payload onclick="alert(1)">' }),
    );
    await expect(c.getByRole("link", { name: "reference" })).toHaveAttribute(
      "href",
      "https://example.com/",
    );
    await expect(
      c.getByText('<image src="https://example.invalid/leak" />', {
        exact: true,
      }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "<script>" }));
    await expect(
      c.getByText("window.promptExecuted = true", { exact: true }),
    ).toBeVisible();
    await expect(
      canvasElement.querySelectorAll(
        ".community-markdown-content script, .community-markdown-content img, .community-markdown-content [onclick]",
      ),
    ).toHaveLength(0);
  }}
/>
