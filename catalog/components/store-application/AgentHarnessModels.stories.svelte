<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent harness models",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Offline agent retains the negotiated harness models"
  args={{ status: "Offline", advertisedModels: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      await c.findByRole("button", { name: "View Repository helper" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Edit Repository helper" }),
    );
    const editor = within(
      await page.findByRole("dialog", { name: "Edit agent" }),
    );
    await expect(
      await editor.findByText(
        /Showing choices from this harness's last session/,
      ),
    ).toBeVisible();
    await userEvent.click(editor.getByRole("option", { name: /Codex Fast/ }));
    await expect(editor.getByText("Selected model: Codex Fast")).toBeVisible();
    await userEvent.click(editor.getByRole("button", { name: "Save agent" }));
    await expect(
      await within(c.getByRole("region", { name: "Your agents" })).findByText(
        "codex-fast",
        { exact: true },
      ),
    ).toBeVisible();
    await expect(
      c.queryByLabelText("Last session command"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Missing harness catalog explains availability and preserves saved model"
  args={{ status: "Offline", savedModel: "remembered-model" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      await c.findByRole("button", { name: "View Repository helper" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Edit Repository helper" }),
    );
    const editor = within(
      await page.findByRole("dialog", { name: "Edit agent" }),
    );
    await expect(
      await editor.findByText(
        /No model or thinking choices have been advertised/,
      ),
    ).toBeVisible();
    await expect(
      editor.getByRole("option", {
        name: /remembered-model \(saved preference\)/,
      }),
    ).toHaveAttribute("aria-disabled", "true");
    await userEvent.click(editor.getByRole("button", { name: "Save agent" }));
    await expect(
      await within(c.getByRole("region", { name: "Your agents" })).findByText(
        "remembered-model",
        { exact: true },
      ),
    ).toBeVisible();
  }}
/>
