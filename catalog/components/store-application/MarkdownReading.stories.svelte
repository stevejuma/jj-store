<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/MarkdownReadingFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Transcript Markdown",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="File links and list bullets"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const link = await canvas.findByRole("link", { name: "LICENSE" });
    const bullets = canvasElement.querySelectorAll(
      ".cm-formatting-list-bullet > .cm-meta",
    );
    expect(bullets).toHaveLength(2);
    for (const bullet of bullets) {
      expect(getComputedStyle(bullet).color).toBe("rgba(0, 0, 0, 0)");
      expect(
        parseFloat(getComputedStyle(bullet, "::after").width),
      ).toBeGreaterThan(0);
    }
    expect(
      canvas.getByRole("textbox", { name: "Assistant message" }),
    ).toHaveTextContent("Location: $WORKSPACE/LICENSE");
    await userEvent.click(link);
    expect(canvas.getByLabelText("Selected file")).toHaveTextContent(
      "LICENSE line 3 (1)",
    );
    link.focus();
    await userEvent.keyboard("{Enter}");
    expect(canvas.getByLabelText("Selected file")).toHaveTextContent(
      "LICENSE line 3 (2)",
    );
  }}
/>
