<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceLazyApplicationFixture from "../../fixtures/StackWorkspaceLazyApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Lazy reads",
    component: StackWorkspaceLazyApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function openApi(canvasElement: HTMLElement) {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    await userEvent.click(
      await within(stack).findByRole("button", { name: "src/api.ts" }),
    );
    return {
      canvas,
      stack,
      diff: await canvas.findByRole("region", {
        name: "Selected changes from stack-api",
      }),
    };
  }
</script>

<Story
  name="Read-only browsing with independent adjacent diffs"
  play={async ({ canvasElement }) => {
    const { canvas, stack, diff } = await openApi(canvasElement);
    await expect(
      await within(diff).findByRole("group", {
        name: "Draggable hunk query endpoint",
      }),
    ).toHaveAttribute("draggable", "false");
    expect(within(diff).queryByRole("checkbox")).toBeNull();
    expect(stack.querySelector('[data-revision-id="api2"]')).toHaveAttribute(
      "draggable",
      "false",
    );
    expect(canvas.getByRole("button", { name: "Create stack" })).toBeDisabled();
    expect(
      within(stack).getByRole("button", {
        name: "Create commit in feature/api",
      }),
    ).toBeDisabled();
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Actions for Expose repository API",
      }),
    );
    const portal = within(canvasElement.ownerDocument.body);
    expect(
      await portal.findByRole("menuitem", { name: "Reword commit" }),
    ).toHaveAttribute("aria-disabled", "true");
    expect(
      portal.getByRole("menuitem", { name: /^Copy$/ }),
    ).not.toHaveAttribute("aria-disabled", "true");
    await userEvent.keyboard("{Escape}");
    const ui = canvas.getByRole("region", { name: "Stack feature/workspace" });
    await userEvent.click(
      within(ui).getByRole("button", { name: "Add workspace toolbar" }),
    );
    await userEvent.click(
      await within(ui).findByRole("button", { name: "src/toolbar.ts" }),
    );
    const other = await canvas.findByRole("region", {
      name: "Selected changes from stack-ui",
    });
    await within(other).findByRole("group", {
      name: "Draggable hunk workspace picker",
    });
    await userEvent.click(canvas.getByRole("button", { name: /^Refresh$/ }));
    await waitFor(() =>
      expect(
        within(diff).getByRole("group", {
          name: "Draggable hunk query endpoint",
        }),
      ).toBeInTheDocument(),
    );
    expect(other).toBeInTheDocument();
    const unstaged = canvas.getByRole("region", { name: "Unstaged files" });
    await userEvent.click(
      await within(unstaged).findByRole("button", { name: "README.md" }),
    );
    const working = await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    await within(working).findByRole("group", { name: /^Draggable hunk/ });
    expect(other).toBeInTheDocument();
    await userEvent.click(
      within(diff).getByRole("button", { name: "Close Diff column" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("region", {
          name: "Selected changes from stack-api",
        }),
      ).toBeNull(),
    );
    expect(other).toBeInTheDocument();
  }}
/>
<Story
  name="Selected patch retry"
  args={{ failFirstPatch: true }}
  play={async ({ canvasElement }) => {
    const { diff } = await openApi(canvasElement);
    await userEvent.click(
      await within(diff).findByRole("button", { name: "Retry diff" }),
    );
    await expect(
      await within(diff).findByRole("group", {
        name: "Draggable hunk query endpoint",
      }),
    ).toBeInTheDocument();
    expect(
      within(diff).queryByRole("button", { name: "Retry diff" }),
    ).toBeNull();
  }}
/>
<Story
  name="Same operation refresh retains an in-flight patch"
  args={{ refreshDuringPatch: true }}
  play={async ({ canvasElement }) => {
    const { diff } = await openApi(canvasElement);
    await expect(
      await within(diff).findByRole("group", {
        name: "Draggable hunk query endpoint",
      }),
    ).toBeInTheDocument();
    expect(
      within(diff).queryByRole("status", { name: /Loading diff/ }),
    ).toBeNull();
  }}
/>
<Story
  name="Changed operation reloads retained adjacent patches"
  args={{ simulateOperationChange: true }}
  play={async ({ canvasElement }) => {
    const { canvas, diff } = await openApi(canvasElement);
    await within(diff).findByRole("group", {
      name: "Draggable hunk query endpoint",
    });
    const unstaged = canvas.getByRole("region", { name: "Unstaged files" });
    await userEvent.click(
      await within(unstaged).findByRole("button", { name: "README.md" }),
    );
    const working = await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    await within(working).findByRole("group", { name: /^Draggable hunk/ });
    await userEvent.click(
      canvas.getByRole("button", { name: "Simulate native operation change" }),
    );
    await waitFor(() => {
      expect(canvas.getByLabelText("Fixture patch reads")).toHaveTextContent(
        "4",
      );
      expect(
        within(diff).queryByRole("status", { name: /Loading diff/ }),
      ).toBeNull();
      expect(
        within(working).queryByRole("status", { name: /Loading diff/ }),
      ).toBeNull();
      expect(
        within(diff).getByRole("group", {
          name: "Draggable hunk query endpoint",
        }),
      ).toBeInTheDocument();
      expect(
        within(working).getByRole("group", {
          name: /^Draggable hunk/,
        }),
      ).toBeInTheDocument();
    });
  }}
/>
<Story
  name="Unavailable patch stays explicit"
  args={{ unavailable: true }}
  play={async ({ canvasElement }) => {
    const { diff } = await openApi(canvasElement);
    await expect(
      await within(diff).findByText("Text diff unavailable"),
    ).toBeVisible();
    expect(within(diff).getByText(/binary/)).toBeVisible();
    expect(
      within(diff).queryByRole("group", { name: /^Draggable hunk/ }),
    ).toBeNull();
  }}
/>
