<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceLoadingApplicationFixture from "../../fixtures/StackWorkspaceLoadingApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Loading",
    component: StackWorkspaceLoadingApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  function assertPending(element: HTMLElement) {
    const canvas = within(element);
    const busy = canvas.getByRole("status", {
      name: "Loading stack workspace",
    });
    expect(busy).toHaveAttribute("aria-busy", "true");
    expect(busy.querySelector('[data-ui-component="skeleton"]')).not.toBeNull();
    expect(canvas.queryByText("Local stack workspace unavailable")).toBeNull();
    expect(
      canvas.queryByText("No stack workspace", {
        selector: '[data-ui-part="empty-title"]',
      }),
    ).toBeNull();
    expect(
      element.querySelector('.store-application__body-regions [role="alert"]'),
    ).toBeNull();
  }
  async function load(element: HTMLElement) {
    const canvas = within(element);
    assertPending(element);
    await userEvent.click(
      canvas.getByRole("button", { name: "Connect repository" }),
    );
    assertPending(element);
    await userEvent.click(
      canvas.getByRole("button", { name: "Discover workspace binding" }),
    );
    assertPending(element);
    await userEvent.click(
      canvas.getByRole("button", { name: "Start workspace read" }),
    );
    assertPending(element);
    await userEvent.click(
      canvas.getByRole("button", { name: "Complete workspace read" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("status", { name: "Loading stack workspace" }),
      ).toBeNull(),
    );
    expect(
      canvas.getByRole("combobox", { name: "Active stack workspace" }),
    ).toHaveTextContent("Daily work");
  }
</script>

<Story
  name="Workspace startup stays skeleton through discovery and first read"
  play={async ({ canvasElement }) => {
    await load(canvasElement);
    const canvas = within(canvasElement);
    const files = canvas.getByRole("region", { name: "Unstaged files" });
    await userEvent.click(
      canvas.getByRole("button", { name: "Start workspace read" }),
    );
    expect(files).toBeVisible();
    expect(
      canvas.queryByRole("status", { name: "Loading stack workspace" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Complete workspace read" }),
    );
    expect(files).toBeVisible();
  }}
/>
<Story
  name="Branches startup stays skeleton"
  args={{ view: "branches" }}
  play={async ({ canvasElement }) => {
    await load(canvasElement);
    expect(
      within(canvasElement).getByRole("region", { name: "Branches" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Commits startup stays skeleton"
  args={{ view: "log" }}
  play={async ({ canvasElement }) => {
    await load(canvasElement);
    expect(
      await within(canvasElement).findByRole("list", {
        name: "Revision graph",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Op Log startup stays skeleton"
  args={{ view: "oplog" }}
  play={async ({ canvasElement }) => {
    await load(canvasElement);
    expect(
      await within(canvasElement).findByRole("list", {
        name: "Operation graph",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Failed initial read uses toast and empty recovery then retry skeleton"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Discover workspace binding" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Start workspace read" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Fail workspace read" }),
    );
    await within(canvasElement.ownerDocument.body).findByText(
      "The workspace read failed. Try again.",
    );
    expect(
      canvasElement.querySelector(
        '.store-application__body-regions [role="alert"]',
      ),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Retry workspace" }),
    );
    assertPending(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Complete workspace read" }),
    );
    await canvas.findByRole("region", { name: "Unstaged files" });
  }}
/>
