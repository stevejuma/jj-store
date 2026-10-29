<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentChatGptFixture.svelte";
  const { Story } = defineMeta({
    title: "Application/ChatGPT accounts",
    component: Fixture,
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-settings/StoreAgentChatGptAccounts.svelte",
      },
      layout: "padded",
      a11y: { test: "error" },
      visualDelta: { status: "visual-pending" },
    },
  });
</script>

<Story
  name="Account lifecycle"
  play={async ({ canvasElement }) => {
    const page = within(canvasElement);
    await expect(
      await page.findByText("Using your ChatGPT plan"),
    ).toBeVisible();
    await userEvent.click(
      page.getByRole("button", { name: "Continue with ChatGPT" }),
    );
    await expect(
      await page.findByText("Complete sign-in in your browser."),
    ).toBeVisible();
    await userEvent.click(page.getByRole("button", { name: "Cancel sign-in" }));
    await expect(
      page.queryByText("Complete sign-in in your browser."),
    ).toBeNull();
    await userEvent.click(
      page.getAllByRole("button", { name: "Reconnect" })[1],
    );
    await userEvent.click(
      page.getByRole("button", { name: "Complete fixture sign-in" }),
    );
    await expect(
      await page.findAllByText("Using your ChatGPT plan"),
    ).toHaveLength(2);
    await userEvent.click(page.getAllByRole("button", { name: "Sign out" })[1]);
    await expect(await page.findByText("Signed out")).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Manage usage" }),
    ).toHaveAttribute("rel", "noopener noreferrer");
    await expect(page.getByLabelText("Account commands")).toHaveTextContent(
      "connect, cancel, connect, sign_out",
    );
  }}
/>
<Story
  name="Unsupported platform"
  args={{ unsupported: true }}
  play={async ({ canvasElement }) => {
    const page = within(canvasElement);
    await expect(
      await page.findByText("ChatGPT plan connections require local macOS."),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Continue with ChatGPT" }),
    ).toBeDisabled();
  }}
/>
