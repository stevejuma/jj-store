<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Navigation",
    component: StackWorkspaceApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  const views = (element: HTMLElement) =>
    within(
      within(element).getByRole("tablist", { name: "Local repository views" }),
    );
</script>

<Story
  name="Workspace rail uses repository Details navigation"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const sidebar = canvas.getByRole("complementary", {
      name: "Repository details",
    });
    const rail = within(sidebar);
    const repositoryLink = rail.getByRole("button", { name: "Branches" });
    const style = getComputedStyle(repositoryLink);
    const expectedStyle = [
      style.height,
      style.paddingInline,
      style.fontSize,
      style.fontWeight,
      style.gap,
    ];
    await userEvent.click(canvas.getByRole("tab", { name: "Stacks" }));
    await canvas.findByRole("region", { name: "Unstaged files" });
    const navigation = await rail.findByRole("navigation", {
      name: "Stack workspace views",
    });
    expect(
      within(navigation).getByText("Details", { exact: true }),
    ).toBeVisible();
    expect(
      within(navigation)
        .getAllByRole("button")
        .map((button) => button.textContent?.trim()),
    ).toEqual(["Branches", "Workspace", "Commits", "Op Log"]);
    expect(
      rail.queryByRole("navigation", { name: "Repository views" }),
    ).toBeNull();
    expect(sidebar.querySelector("dl")).toBeNull();
    for (const name of [
      "Workspace ID",
      "Target branch",
      "Applied stacks",
      "Local folder",
    ])
      expect(rail.queryByText(name, { exact: true })).toBeNull();
    const actions = rail.getByRole("region", { name: "Workspace actions" });
    expect(
      within(actions).queryByRole("button", { name: "Resolve conflicts" }),
    ).toBeNull();
    expect(
      within(actions).queryByRole("button", { name: "Reapply saved branches" }),
    ).toBeNull();
    for (const name of ["Branches", "Workspace", "Commits", "Op Log"]) {
      const button = within(navigation).getByRole("button", { name });
      const computed = getComputedStyle(button);
      expect([
        computed.height,
        computed.paddingInline,
        computed.fontSize,
        computed.fontWeight,
        computed.gap,
      ]).toEqual(expectedStyle);
      expect(button.querySelector("svg")).not.toBeNull();
      await userEvent.click(button);
      await waitFor(() =>
        expect(button).toHaveAttribute("aria-current", "page"),
      );
      expect(canvas.getByRole("tab", { name })).toHaveAttribute(
        "data-state",
        "active",
      );
      expect(sidebar.querySelector("button:disabled")).toBeNull();
    }
  }}
/>

<Story
  name="Enter from repository and return to exact selection"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const tabs = await canvas.findByRole("tablist", {
      name: "Repository sections",
    });
    await waitFor(() =>
      expect(
        within(tabs)
          .getAllByRole("tab")
          .map((tab) => tab.textContent?.trim())
          .slice(0, 3),
      ).toEqual(["Overview", "Stacks", "Branches"]),
    );
    const sidebar = within(
      canvas.getByRole("complementary", {
        name: "Repository navigation sidebar",
      }),
    );
    expect(sidebar.queryByRole("button", { name: "Stacks" })).toBeNull();
    await userEvent.click(within(tabs).getByRole("tab", { name: "Stacks" }));
    const navigation = await canvas.findByRole("navigation", {
      name: "Stack workspace navigation",
    });
    const back = canvas.getByRole("button", { name: "Back to repository" });
    expect(back.getBoundingClientRect().width).toBeCloseTo(
      back.getBoundingClientRect().height,
      1,
    );
    expect(
      parseFloat(getComputedStyle(back).borderRadius),
    ).toBeGreaterThanOrEqual(back.getBoundingClientRect().width / 2);
    expect(getComputedStyle(back).backgroundColor).toBe("rgba(0, 0, 0, 0)");
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      await pointer.hover(back);
      await waitFor(() =>
        expect(getComputedStyle(back).backgroundColor).not.toBe(
          "rgba(0, 0, 0, 0)",
        ),
      );
      await pointer.hover(navigation);
      await waitFor(() =>
        expect(getComputedStyle(back).backgroundColor).toBe("rgba(0, 0, 0, 0)"),
      );
    }
    expect(
      within(navigation)
        .getAllByRole("tab")
        .map((tab) => tab.textContent?.trim()),
    ).toEqual(["Branches", "Workspace", "Commits", "Op Log"]);
    expect(
      canvas.queryByRole("tablist", { name: "Repository sections" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("button", { name: "Refresh snapshot" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("button", { name: "Repository actions" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("button", { name: /Show terminal|Hide terminal/ }),
    ).toBeNull();
    expect(
      canvas.getByRole("button", { name: "Toggle details" }),
    ).toBeVisible();
    expect(sidebar.getByRole("button", { name: "jj-store" })).toHaveAttribute(
      "aria-current",
      "page",
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const details = within(
      canvas.getByRole("complementary", { name: "Repository details" }),
    );
    expect(
      details.queryByRole("navigation", { name: "Repository views" }),
    ).toBeNull();
    const workspaceDetails = details.getByRole("region", {
      name: "Stack workspace details",
    });
    expect(workspaceDetails).toHaveTextContent("Workspace/Stacks");
    expect(
      within(workspaceDetails).getByRole("navigation", {
        name: "Stack workspace views",
      }),
    ).toBeVisible();
    expect(workspaceDetails.querySelector("dl")).toBeNull();
    expect(details.queryByText("Reviews/Threads")).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to repository" }),
    );
    expect(
      await canvas.findByRole("tab", { name: "Overview" }),
    ).toHaveAttribute("data-state", "active");
    expect(
      canvas.getByRole("combobox", { name: "Repository bookmark or change" }),
    ).toHaveTextContent("main");
  }}
/>

<Story
  name="History retains canvas nodes diffs drafts and selected commits"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    await userEvent.click(
      within(stack).getByRole("button", { name: "src/api.ts" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const rail = within(
      canvas.getByRole("region", { name: "Stack workspace details" }),
    );
    expect(rail.getByRole("button", { name: "Workspace" })).toHaveAttribute(
      "aria-current",
      "page",
    );
    expect(rail.queryByText("Expose repository API")).toBeNull();
    const diff = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Create commit in feature/api",
      }),
    );
    const title = within(stack).getByRole("textbox", { name: "Commit title" });
    await userEvent.type(title, "Retained while reading history");
    const commits = views(canvasElement).getByRole("tab", { name: "Commits" });
    await userEvent.click(commits);
    expect(stack).not.toBeVisible();
    expect(
      canvas.getByRole("combobox", { name: "Active stack workspace" }),
    ).toBeVisible();
    expect(
      canvas.getByRole("button", { name: "Workspace actions" }),
    ).toBeVisible();
    await userEvent.click(
      within(canvas.getByRole("list", { name: "Revision graph" })).getByRole(
        "button",
        {
          name: "Merge native storage and publication",
        },
      ),
    );
    expect(rail.getByText("JJ Log/Changes")).toBeVisible();
    expect(rail.getByRole("button", { name: "Commits" })).toHaveAttribute(
      "aria-current",
      "page",
    );
    await userEvent.click(
      views(canvasElement).getByRole("tab", { name: "Op Log" }),
    );
    await userEvent.click(
      within(canvas.getByRole("list", { name: "Operation graph" })).getByRole(
        "button",
        { name: "describe: native object history" },
      ),
    );
    await userEvent.click(
      views(canvasElement).getByRole("tab", { name: "Workspace" }),
    );
    expect(canvas.getByRole("region", { name: "Stack feature/api" })).toBe(
      stack,
    );
    expect(
      canvas.getByRole("region", { name: "Selected changes from stack-api" }),
    ).toBe(diff);
    expect(title).toHaveValue("Retained while reading history");
    expect(rail.getByRole("button", { name: "Workspace" })).toHaveAttribute(
      "aria-current",
      "page",
    );
    await userEvent.click(
      views(canvasElement).getByRole("tab", { name: "Commits" }),
    );
    expect(
      within(canvas.getByRole("list", { name: "Revision graph" }))
        .getByRole("button", { name: "Merge native storage and publication" })
        .closest('[role="listitem"]'),
    ).toHaveAttribute("aria-current", "true");
    await userEvent.click(
      views(canvasElement).getByRole("tab", { name: "Op Log" }),
    );
    expect(
      within(canvas.getByRole("list", { name: "Operation graph" }))
        .getByRole("button", { name: "describe: native object history" })
        .closest('[role="listitem"]'),
    ).toHaveAttribute("aria-current", "true");
  }}
/>

<Story
  name="Focused tabs fit narrow body and support keyboard"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("region", { name: "Unstaged files" });
    const region = await canvas.findByRole("region", {
      name: "Local stack workspace",
    });
    const original = region.style.width;
    try {
      region.style.width = "360px";
      region.style.flex = "none";
      const more = await canvas.findByRole("button", {
        name: "More local repository views",
      });
      await waitFor(() => expect(more).toBeEnabled());
      await userEvent.click(more);
      expect(more.isConnected).toBe(true);
      const portal = within(canvasElement.ownerDocument.body);
      await userEvent.click(
        await portal.findByRole("menuitem", { name: "Op Log" }),
      );
      expect(more).toHaveAttribute("data-selected");
      expect(
        canvas.getByRole("button", { name: "Workspace actions" }),
      ).toBeVisible();
      await userEvent.click(
        canvas.getByRole("button", { name: "Back to repository" }),
      );
      await canvas.findByRole("tab", { name: "Overview" });
      await userEvent.click(canvas.getByRole("tab", { name: "Stacks" }));
      const branches = views(canvasElement).getByRole("tab", {
        name: "Branches",
      });
      branches.focus();
      await userEvent.keyboard("{ArrowRight}");
      expect(
        views(canvasElement).getByRole("tab", { name: "Workspace" }),
      ).toHaveFocus();
    } finally {
      region.style.width = original;
      region.style.flex = "";
    }
  }}
/>

<Story
  name="Unlinked repository has no Stacks tab"
  args={{ initialPage: "files", linked: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("tablist", { name: "Repository sections" });
    expect(canvas.queryByRole("tab", { name: "Stacks" })).toBeNull();
  }}
/>

<Story
  name="Details navigation retains dependent branch and target selections in the body"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: /feature\/api/ }),
    );
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in feature/model",
      }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const details = canvas.getByRole("region", {
      name: "Stack workspace details",
    });
    const rail = within(details);
    expect(rail.getByText("Branches/Stacks")).toBeVisible();
    expect(rail.getByRole("button", { name: "Branches" })).toHaveAttribute(
      "aria-current",
      "page",
    );
    expect(
      within(stack).getByRole("region", {
        name: "Changed files in branch feature/model",
      }),
    ).toBeVisible();
    expect(details.querySelector("dl")).toBeNull();
    await userEvent.click(canvas.getByRole("option", { name: "main" }));
    const target = await canvas.findByRole("region", { name: "Stack main" });
    await userEvent.click(
      await within(target).findByRole("button", { name: "Main history 240" }),
    );
    await waitFor(() =>
      expect(
        within(target)
          .getByRole("button", { name: "Main history 240" })
          .closest('[role="listitem"]'),
      ).toHaveAttribute("aria-current", "true"),
    );
    expect(rail.queryByText("feature/model", { exact: true })).toBeNull();
  }}
/>
