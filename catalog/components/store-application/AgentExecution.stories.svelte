<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Interactive agent execution",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Launcher and saved setups"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "New agent session" }));
    await page.findByRole("dialog", { name: "Agent sessions" });
    await expect(page.getByLabelText("Exact starting revision")).toHaveValue(
      "33".repeat(20),
    );
    await userEvent.click(
      page.getByRole("button", { name: "Add saved setup" }),
    );
    await page.findByRole("dialog", { name: "New saved setup" });
    await userEvent.type(page.getByLabelText("Name"), "Review helper");
    await userEvent.click(page.getByRole("button", { name: /^Cancel$/ }));
    await userEvent.click(
      page.getByRole("button", { name: "Add saved setup" }),
    );
    await expect(page.getByLabelText("Name")).toHaveValue("Review helper");
    await userEvent.click(page.getByRole("tab", { name: "Instructions" }));
    await userEvent.type(
      page.getByRole("textbox", { name: "Instructions" }),
      "Explain every change.",
    );
    await userEvent.click(page.getByRole("button", { name: "Save setup" }));
    await page.findByRole("button", { name: /^🤖 Review helper$/ });
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Agent sessions" }),
      ).not.toBeInTheDocument(),
    );
  }}
/>
<Story
  name="Exact permission choices"
  args={{ permission: true }}
  play={async ({ canvasElement }) => {
    const page = within(document.body);
    await userEvent.click(
      within(canvasElement).getByRole("button", {
        name: "Open fixture conversation",
      }),
    );
    await page.findByRole("article", { name: "Agent permission request" });
    await expect(
      page.getByRole("button", { name: "Allow once" }),
    ).toBeEnabled();
    await userEvent.click(page.getByRole("button", { name: "Reject once" }));
    await expect(page.getByRole("button", { name: "Stop turn" })).toBeEnabled();
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Agent sessions" }),
      ).not.toBeInTheDocument(),
    );
  }}
/>
<Story
  name="Draft survives unknown outcome"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const page = within(document.body);
    await userEvent.click(
      within(canvasElement).getByRole("button", {
        name: "Open fixture conversation",
      }),
    );
    const draft = await page.findByRole("textbox", {
      name: "Message to agent",
    });
    await userEvent.type(draft, "Keep this draft");
    await userEvent.click(page.getByRole("button", { name: /^Send$/ }));
    await waitFor(() => expect(draft).toHaveValue("Keep this draft"));
    await page.findByRole("button", {
      name: "Keep unresolved request and review workspace",
    });
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Agent sessions" }),
      ).not.toBeInTheDocument(),
    );
  }}
/>
