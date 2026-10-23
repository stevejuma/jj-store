<script module lang="ts">
  import { page } from "vitest/browser";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreCapturedReactionsFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte",
      },
    },
    title: "JJ Store/Captured message reactions",
    component: Fixture,
    tags: ["visual-pending", "component"],
  });
  async function react({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement),
      body = within(document.body);
    const message = (await c.findAllByRole("article"))[0]!;
    await page
      .elementLocator(message.querySelector(".chat-text-block") ?? message)
      .hover();
    const actions = within(message).getByRole("group", {
      name: /^Actions for/,
    });
    const add = within(message).getByRole("button", {
      name: /^Add reaction(?: to|$)/,
    });
    await expect(actions.contains(add)).toBe(true);
    await waitFor(() =>
      expect(getComputedStyle(actions).pointerEvents).toBe("auto"),
    );
    await userEvent.click(add);
    await userEvent.unhover(message);
    await waitFor(async () => {
      await expect(
        await body.findByRole("button", { name: ":lapis:" }),
      ).toBeVisible();
    });
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    await waitFor(async () => {
      await expect(
        await within(message).findByRole("button", {
          name: /1 reaction with Lapis/,
        }),
      ).toBeVisible();
    });
    within(message)
      .getByRole("button", { name: /1 reaction with Lapis/ })
      .focus();
    await waitFor(async () => {
      await expect(
        await body.findByText(/1 reaction with Lapis/, {
          selector: '[data-ui-part="tooltip-content"]',
        }),
      ).toBeVisible();
    });
    await userEvent.click(
      within(message).getByRole("button", { name: /1 reaction with Lapis/ }),
    );
    await waitFor(() =>
      expect(
        within(message).queryByRole("button", {
          name: /1 reaction with Lapis/,
        }),
      ).toBeNull(),
    );
  }

  async function readOnly({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    for (const message of await c.findAllByRole("article")) {
      await page
        .elementLocator(message.querySelector(".chat-text-block") ?? message)
        .hover();
      await expect(
        within(message).queryByRole("button", {
          name: /^Add reaction(?: to|$)/,
        }),
      ).toBeNull();
    }
  }
</script>

<Story
  name="Captured agent messages"
  args={{ surface: "agents" }}
  play={react}
/>
<Story
  name="Exact review comments"
  args={{ surface: "reviews" }}
  play={react}
/>
<Story
  name="Read only captured messages"
  args={{ surface: "agents", writable: false }}
  play={readOnly}
/>
<Story
  name="Read only review comments"
  args={{ surface: "reviews", writable: false }}
  play={readOnly}
/>
