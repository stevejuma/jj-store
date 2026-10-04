<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  import { applyPreview } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Branches",
    component: StackWorkspaceApplicationFixture,
    args: { page: "branches" },
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  function branchGroup(element: HTMLElement, heading: string) {
    return within(element)
      .getByText(heading, { exact: true })
      .closest<HTMLElement>('[data-ui-part="command-group"]')!;
  }
</script>

<Story
  name="Applied branches have their own group"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const column = await canvas.findByRole("region", { name: "Branches" });
    const applied = branchGroup(column, "Applied");
    expect(
      within(applied)
        .getAllByRole("option")
        .map((item) => item.getAttribute("aria-label")),
    ).toEqual(["feature/api", "feature/model", "feature/workspace"]);
    expect(
      within(branchGroup(column, "Local branches")).getAllByRole("option"),
    ).toHaveLength(1);
    const search = within(column).getByPlaceholderText("Search branches");
    await userEvent.type(search, "model");
    await waitFor(() =>
      expect(
        within(branchGroup(column, "Applied")).getAllByRole("option"),
      ).toHaveLength(1),
    );
    expect(
      within(column).queryByText("Local branches", { exact: true }),
    ).toBeNull();
    await userEvent.clear(search);
    await userEvent.click(
      within(column).getByRole("tab", { name: "Workspace" }),
    );
    await waitFor(() =>
      expect(within(column).getAllByRole("option")).toHaveLength(3),
    );
    expect(
      within(column).queryByText("Local branches", { exact: true }),
    ).toBeNull();
  }}
/>

<Story
  name="Apply and unapply actions occupy the stack body"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: "feature/search" }),
    );
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/search",
    });
    const apply = within(stack).getByRole("button", {
      name: "Apply feature/search",
    });
    const header = stack.querySelector('[data-ui-part="column-header"]')!;
    const body = stack.querySelector('[data-ui-part="column-body"]')!;
    expect(header.contains(apply)).toBe(false);
    expect(body.contains(apply)).toBe(true);
    expect(apply).toHaveTextContent("Apply to workspace");
    expect(apply.getAttribute("data-variant")).toBe("default");
    expect(
      within(body as HTMLElement).getByRole("button", { name: "Delete local" }),
    ).toBeDisabled();
    await userEvent.click(apply);
    await applyPreview(canvasElement);
    const unapply = within(stack).getByRole("button", {
      name: "Unapply feature/search",
    });
    expect(unapply).toHaveTextContent("Unapply from workspace");
    const branches = canvas.getByRole("region", { name: "Branches" });
    expect(
      within(branchGroup(branches, "Applied")).getByRole("option", {
        name: "feature/search",
      }),
    ).toBeVisible();
    await userEvent.click(unapply);
    await applyPreview(canvasElement);
    await waitFor(() =>
      expect(
        within(stack).getByRole("button", { name: "Apply feature/search" }),
      ).toBeVisible(),
    );
    expect(
      within(branchGroup(branches, "Local branches")).getByRole("option", {
        name: "feature/search",
      }),
    ).toBeVisible();
    await userEvent.click(
      within(
        canvas.getByRole("navigation", { name: "Stack workspace navigation" }),
      ).getByRole("tab", { name: "Workspace" }),
    );
    const appliedStack = canvas.getByRole("region", {
      name: "Stack feature/api",
    });
    expect(
      within(appliedStack).queryByRole("button", {
        name: "Unapply feature/api",
      }),
    ).toBeNull();
  }}
/>

<Story
  name="Workspace target branch appears above body tabs"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const target = await canvas.findByRole("group", {
      name: "Workspace target branch",
    });
    expect(within(target).getByRole("heading", { name: "main" })).toBeVisible();
    const navigation = canvas.getByRole("navigation", {
      name: "Stack workspace navigation",
    });
    expect(target.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      navigation.getBoundingClientRect().top,
    );
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/search" }),
    );
    expect(within(target).getByRole("heading", { name: "main" })).toBeVisible();
    await userEvent.click(
      within(navigation).getByRole("tab", { name: "Workspace" }),
    );
    expect(
      canvas.queryByRole("group", { name: "Workspace target branch" }),
    ).toBeNull();
    await userEvent.click(
      within(navigation).getByRole("tab", { name: "Branches" }),
    );
    expect(
      within(
        canvas.getByRole("group", { name: "Workspace target branch" }),
      ).getByRole("heading", { name: "main" }),
    ).toBeVisible();
  }}
/>
