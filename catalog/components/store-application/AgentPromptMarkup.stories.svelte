<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentPromptFixture.svelte";
  import { inertPromptMarkup } from "../../fixtures/agent-prompt-markup.js";
  const openCodeParts = JSON.stringify([
    { id: "part_1", type: "text", text: "Update the **parser**." },
    { id: "part_2", type: "text", text: "Keep the review anchors exact." },
  ]);
  const { Story } = defineMeta({
    title: "JJ Store/Agent prompt markup",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function nested({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const time = canvasElement.querySelector(
      "time[datetime='2024-01-02T03:05:02.456Z']",
    );
    await expect(time).toBeVisible();
    await expect(time?.getAttribute("title")).toMatch(/2024/);
    await expect(time).toHaveTextContent(/\d{2}:\d{2}:\d{2}/);
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
    await expect(
      canvasElement.querySelector(
        '[data-prompt-element^="<arbitrary_limits "] button',
      ),
    ).toBeNull();
    await expect(c.getByText("cargo test", { exact: true })).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "<environment_context>" }),
    );
    await expect(c.getByText("/workspace/demo", { exact: true })).toBeVisible();
    await expect(canvasElement.querySelector("pre")!).toHaveTextContent(
      "<literal><keep_this /></literal>",
    );
    await expect(c.queryByRole("button", { name: "<literal>" })).toBeNull();
    await expect(
      c.queryByRole("button", { name: "<another-tool.context>" }),
    ).toBeNull();
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
  name="OpenCode typed text message parts"
  args={{
    integration: "open_code",
    mediaType: "json",
    content: openCodeParts,
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const message = await c.findByRole("article", {
      name: "Message from Maya Chen",
    });
    await expect(
      within(message).getByText("parser", { selector: "strong", exact: true }),
    ).toBeVisible();
    await expect(message).toHaveTextContent("Keep the review anchors exact.");
    await expect(message.querySelector("pre")).toBeNull();
    await expect(message).not.toHaveTextContent("part_1");
    const write = fn();
    const clipboardOwner = canvasElement.ownerDocument.defaultView!.navigator;
    const descriptor = Object.getOwnPropertyDescriptor(
      clipboardOwner,
      "clipboard",
    );
    Object.defineProperty(clipboardOwner, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      await userEvent.hover(message);
      await waitFor(() =>
        expect(message).toHaveAttribute("data-actions-visible"),
      );
      await userEvent.click(
        within(message).getByRole("button", { name: "More message actions" }),
      );
      const page = within(canvasElement.ownerDocument.body);
      const copy = page.getByRole("menuitem", { name: /^Copy$/ });
      await userEvent.click(copy);
      const pointerMessage = await page.findByRole("menuitem", {
        name: /^Copy message$/,
      });
      await waitFor(() => expect(pointerMessage).toBeVisible());
      await userEvent.click(pointerMessage);
      await waitFor(() => expect(write).toHaveBeenCalledWith(openCodeParts));
      // The clipboard mock resolves before the portal finishes closing. Wait
      // for that visible transition before reopening for the keyboard path.
      await waitFor(() =>
        expect(page.queryByRole("menuitem", { name: /^Copy$/ })).toBeNull(),
      );
      // Also exercise a complete keyboard path without moving the pointer out
      // of a keyboard-opened submenu during its focus transition.
      await userEvent.click(
        within(message).getByRole("button", { name: "More message actions" }),
      );
      const keyboardCopy = await page.findByRole("menuitem", {
        name: /^Copy$/,
      });
      keyboardCopy.focus();
      await userEvent.keyboard("{ArrowRight}");
      const keyboardMessage = await page.findByRole("menuitem", {
        name: /^Copy message$/,
      });
      await waitFor(() => expect(keyboardMessage).toBeVisible());
      keyboardMessage.focus();
      await userEvent.keyboard("{Enter}");
      await waitFor(() => expect(write).toHaveBeenCalledTimes(2));
      await expect(write).toHaveBeenLastCalledWith(openCodeParts);
    } finally {
      if (descriptor)
        Object.defineProperty(clipboardOwner, "clipboard", descriptor);
      else Reflect.deleteProperty(clipboardOwner, "clipboard");
    }
  }}
/>

<Story
  name="OpenCode transcript fallback text parts"
  args={{
    fallback: true,
    integration: "open_code",
    mediaType: "json",
    content: JSON.stringify([
      { id: "part_1", type: "text", text: "Update the parser" },
    ]),
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByText("Update the parser", { exact: true }),
    ).toBeVisible();
    await expect(canvasElement.querySelector("pre")).toBeNull();
  }}
/>

<Story
  name="Compact XML leaves retain closing tags, Markdown and highlighted syntax"
  args={{
    content:
      "<context>\n<current_date>\n2026-10-04\n</current_date>\n<timezone>Europe/London</timezone>\n<value source=\"cli > desktop\" mode='local'>Use **Markdown**.</value>\n<empty />\n<child><leaf>Nested</leaf></child>\n<notes>First line\nSecond line</notes>\n</context>",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: "<context>" }));
    const date = canvasElement.querySelector<HTMLElement>(
      '[data-prompt-element="<current_date>"]',
    )!;
    const timezone = canvasElement.querySelector<HTMLElement>(
      '[data-prompt-element="<timezone>"]',
    )!;
    await waitFor(() => expect(date).toBeVisible());
    await expect(date).toHaveTextContent(
      "<current_date>2026-10-04</current_date>",
    );
    await expect(date.querySelector("button")).toBeNull();
    await expect(date.querySelector("svg")).toBeNull();
    await expect(date.querySelectorAll("p")).toHaveLength(0);
    const row = (label: string) =>
      Array.from(canvasElement.querySelectorAll("[data-prompt-element]")).find(
        (element) => element.getAttribute("data-prompt-element") === label,
      )!;
    const tagLeft = (label: string) =>
      row(label).querySelector(".store-prompt-tag")!.getBoundingClientRect()
        .left;
    await waitFor(() => {
      expect(
        tagLeft("<current_date>") - tagLeft("<context>"),
      ).toBeGreaterThanOrEqual(16);
      expect(tagLeft("<current_date>")).toBe(tagLeft("<child>"));
    });
    await userEvent.click(c.getByRole("button", { name: "<child>" }));
    const leaf = canvasElement.querySelector('[data-prompt-element="<leaf>"]')!;
    await waitFor(() => expect(leaf).toBeVisible());
    await waitFor(() =>
      expect(tagLeft("<leaf>") - tagLeft("<child>")).toBeGreaterThanOrEqual(16),
    );
    for (const element of [date, row("<child>"), leaf]) {
      await expect(getComputedStyle(element).borderTopWidth).toBe("0px");
    }
    await expect(date.querySelectorAll('[data-xml-token="name"]')).toHaveLength(
      2,
    );
    await expect(
      date.querySelectorAll('[data-xml-token="punctuation"]'),
    ).toHaveLength(4);
    const value = canvasElement.querySelector<HTMLElement>(
      '[data-prompt-element^="<value "]',
    )!;
    await expect(value.querySelector("strong")).toHaveTextContent("Markdown");
    await expect(
      Array.from(value.querySelectorAll('[data-xml-token="attribute"]')).map(
        (el) => el.textContent,
      ),
    ).toEqual(["source", "mode"]);
    await expect(
      Array.from(value.querySelectorAll('[data-xml-token="string"]')).map(
        (el) => el.textContent,
      ),
    ).toEqual(['"cli > desktop"', "'local'"]);
    await expect(
      timezone.getBoundingClientRect().top -
        date.getBoundingClientRect().bottom,
    ).toBeLessThanOrEqual(4);
    await expect(c.getByRole("button", { name: "<notes>" })).toHaveAttribute(
      "aria-expanded",
      "false",
    );
    const tag = value.querySelector('[data-xml-token="name"]')!;
    const attr = value.querySelector('[data-xml-token="attribute"]')!;
    const str = value.querySelector('[data-xml-token="string"]')!;
    await expect(getComputedStyle(attr).color).not.toBe(
      getComputedStyle(tag).color,
    );
    await expect(getComputedStyle(attr).color).not.toBe(
      getComputedStyle(str).color,
    );
    await expect(
      new Set([tag, attr, str].map((el) => getComputedStyle(el).color)).size,
    ).toBeGreaterThan(1);
  }}
/>

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
    const imageLabel = canvasElement.querySelector(
      '[data-prompt-element^="<image "]',
    )!;
    await expect(imageLabel).toHaveTextContent(
      '<image src="https://example.invalid/leak" />',
    );
    await expect(c.queryByRole("button", { name: "<script>" })).toBeNull();
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

<Story
  name="Messages without captured timestamps stay unknown"
  args={{ occurredAt: null, content: "An untimed response" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByText("Time unknown", { exact: true }),
    ).toBeVisible();
    await expect(canvasElement.querySelector("time")).toBeNull();
  }}
/>

<Story
  name="Recorded owned prompt has one context inspector before authored text"
  args={{
    integration: "owned_controller",
    mediaType: "json",
    content: JSON.stringify({
      schema: "jj-store.sent-context.v1",
      text: "Authored request stays separate.",
      context: {
        version: 1,
        delivery: "sent",
        wireContext:
          "<agent_context>\n\n<origin>\n\n## Recorded context\n\n**Captured Markdown**\n\n</origin>\n\n</agent_context>",
        blocks: [],
      },
    }),
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const inspector = canvasElement.querySelector<HTMLDetailsElement>(
      '[data-ui-component="agent-sent-context"]',
    )!;
    expect(
      canvasElement.querySelectorAll(
        '[data-ui-component="agent-sent-context"]',
      ),
    ).toHaveLength(1);
    expect(inspector.open).toBe(false);
    expect(
      inspector.compareDocumentPosition(
        c.getByText("Authored request stays separate."),
      ) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).not.toBe(0);
    await userEvent.click(c.getByText("Sent context", { exact: true }));
    await userEvent.click(c.getByRole("button", { name: "<agent_context>" }));
    await userEvent.click(c.getByRole("button", { name: "<origin>" }));
    await expect(
      c.getByRole("heading", { name: "Recorded context" }),
    ).toBeVisible();
    expect(c.getByText("Captured Markdown").tagName).toBe("STRONG");
  }}
/>
