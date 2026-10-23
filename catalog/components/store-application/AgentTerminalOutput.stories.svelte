<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentTerminalOutputFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte",
      },
    },
    title: "JJ Store/Agent terminal output",
    component: Fixture,
    tags: ["visual-pending", "component"],
  });
  async function inspect(canvasElement: HTMLElement, label: string) {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: /^Show details for Ran/ }),
    );
    await expect(
      await c.findByRole("textbox", { name: label }),
    ).toHaveTextContent('export const greeting: string = "Hello 🦀";');
    const toggle = c.getByRole("button", { name: /^Show raw output for Ran/ });
    toggle.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      await c.findByRole("textbox", { name: /^Raw captured data/ }),
    ).toHaveTextContent("formatted_output");
    await userEvent.click(
      c.getByRole("button", { name: /^Show rich output for Ran/ }),
    );
    await expect(
      await c.findByRole("textbox", { name: label }),
    ).toHaveTextContent("Sample README contents");
  }
</script>

<Story
  name="Terminal-only capture renders retained output and toggles raw evidence"
  play={async ({ canvasElement }) => inspect(canvasElement, "Shell")}
/>
<Story
  name="Committed stdout renders exact text instead of the envelope"
  args={{ exact: true }}
  play={async ({ canvasElement }) => inspect(canvasElement, "Output")}
/>
