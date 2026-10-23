<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/UnavailableContentFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Unavailable content",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
  async function expectEmpty(
    canvas: HTMLElement,
    title: string,
    description: string,
  ) {
    const c = within(canvas);
    const empty = await c.findByRole("status", { name: title });
    await expect(empty).toBeVisible();
    await expect(empty).toHaveTextContent(description);
    await expect(empty).toHaveAttribute("data-ui-component", "empty");
    await expect(
      empty.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
    await expect(empty.querySelector("svg")).toHaveAttribute(
      "aria-hidden",
      "true",
    );
  }
</script>

<Story
  name="Missing session uses an icon empty state"
  play={async ({ canvasElement }) =>
    expectEmpty(
      canvasElement,
      "Session unavailable",
      "This session is unavailable or no longer accessible.",
    )}
/>
<Story
  name="Unsupported device explains agent execution inline"
  args={{ surface: "device" }}
  play={async ({ canvasElement }) =>
    expectEmpty(
      canvasElement,
      "Agent execution unavailable",
      "Agent execution is unavailable on this device.",
    )}
/>
<Story
  name="Unsupported agent directory retains session navigation"
  args={{ surface: "agents" }}
  play={async ({ canvasElement }) => {
    await expectEmpty(
      canvasElement,
      "Agent execution unavailable",
      "Local agent management requires an enabled local execution service on this device.",
    );
    await expect(
      within(canvasElement).getByRole("button", { name: "View sessions" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Missing Thread retains its refresh control"
  args={{ surface: "thread" }}
  play={async ({ canvasElement }) => {
    await expectEmpty(
      canvasElement,
      "Thread unavailable",
      "The selected Thread could not be loaded.",
    );
    await expect(
      within(canvasElement).getByRole("button", { name: "Refresh Threads" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Missing review uses an icon empty state"
  args={{ surface: "review" }}
  play={async ({ canvasElement }) =>
    expectEmpty(
      canvasElement,
      "Review unavailable",
      "The selected review could not be loaded.",
    )}
/>
<Story
  parameters={{
    jjStoreStory: {
      kind: "component",
      subject:
        "web/src/lib/ui/store-agent-workspace/StoreWorkspaceInspector.svelte",
    },
  }}
  name="Unavailable workspace file retains its close control"
  args={{ surface: "file" }}
  play={async ({ canvasElement }) => {
    await expectEmpty(
      canvasElement,
      "File unavailable",
      "This file is unavailable or no longer accessible.",
    );
    await userEvent.click(
      within(canvasElement).getByRole("button", {
        name: "Close workspace file",
      }),
    );
    await expect(
      within(canvasElement).queryByRole("status", { name: "File unavailable" }),
    ).not.toBeInTheDocument();
  }}
/>
