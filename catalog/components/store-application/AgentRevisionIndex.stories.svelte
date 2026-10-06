<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent revision index",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Revision picker explicitly rebuilds missing index and retains launch draft"
  args={{ revisionIndexRequired: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "New agent session" }));
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep my starting revision");
    const picker = c.getByRole("combobox", { name: "Starting revision" });
    await userEvent.click(picker);
    await userEvent.click(
      await page.findByRole("button", { name: "Rebuild index" }),
    );
    await page.findByRole("option", { name: /main Main branch/ });
    await waitFor(() =>
      expect(
        page.queryByRole("button", { name: "Rebuild index" }),
      ).not.toBeInTheDocument(),
    );
    await expect(picker).toHaveAttribute("title", "33".repeat(20));
    await expect(draft).toHaveTextContent("Keep my starting revision");
    await expect(
      c.queryByLabelText("Last session command"),
    ).not.toBeInTheDocument();
  }}
/>
