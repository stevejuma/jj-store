<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository navigation",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initialPage: "files", initiallyCollapsed: false },
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  const labels = [
    "Overview",
    "Branches",
    "Commits",
    "Op Log",
    "Agents",
    "Reviews",
  ];
</script>

<Story
  name="Right rail navigation"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const tabs = canvas.getByRole("tablist", { name: "Repository sections" });
    await expect(
      within(tabs)
        .getAllByRole("tab")
        .map((tab) => tab.textContent?.trim()),
    ).toEqual(labels);
    const left = within(
      canvas.getByRole("complementary", {
        name: "Repository navigation sidebar",
      }),
    );
    const section = canvasElement.querySelector(
      ".lc-community-navigation__section",
    )!;
    await expect(getComputedStyle(section).marginBlockStart).toBe("0px");
    for (const label of ["Branches", "JJ Log", "Op Log", "Agents", "Reviews"])
      await expect(left.queryByRole("button", { name: label })).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const right = within(
      canvas.getByRole("complementary", { name: "Repository details" }),
    );
    const sidebar = canvas.getByRole("complementary", {
      name: "Repository details",
    });
    const heading = right.getByText("Files/Repository");
    const menu = right.getByRole("navigation", { name: "Repository views" });
    await expect(
      heading.compareDocumentPosition(menu) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).toBeTruthy();
    await expect(sidebar.querySelectorAll("section, dl")).toHaveLength(0);
    for (const [label, tab] of [
      ["Branches", "Branches"],
      ["JJ Log", "Commits"],
      ["Op Log", "Op Log"],
      ["Agents", "Agents"],
      ["Reviews", "Reviews"],
    ]) {
      const item = right.getByRole("button", { name: label });
      await expect(item.querySelector("svg")).not.toBeNull();
      await userEvent.click(item);
      await waitFor(() => expect(item).toHaveAttribute("aria-current", "page"));
      await expect(canvas.getByRole("tab", { name: tab })).toHaveAttribute(
        "data-state",
        "active",
      );
    }
  }}
/>

<Story
  name="Trailing sections overflow and return"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    const header = canvasElement.querySelector<HTMLElement>(
      ".store-application__tabs",
    )!;
    const originalWidth = header.style.width;
    header.style.width = "220px";
    try {
      const more = await canvas.findByRole("button", {
        name: "More repository sections",
      });
      await waitFor(() =>
        expect(header.scrollWidth).toBeLessThanOrEqual(header.clientWidth),
      );
      await expect(canvas.getByRole("tab", { name: "Overview" })).toBeVisible();
      await userEvent.click(more);
      await userEvent.click(
        await portal.findByRole("menuitem", { name: "Reviews" }),
      );
      await waitFor(() => expect(more).toHaveAttribute("aria-current", "page"));
      await userEvent.click(more);
      await expect(
        await portal.findByRole("menuitem", { name: "Reviews" }),
      ).toHaveAttribute("aria-current", "page");
      await userEvent.keyboard("{Escape}");
    } finally {
      header.style.width = originalWidth;
    }
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", { name: "More repository sections" }),
      ).toBeNull(),
    );
    await expect(canvas.getByRole("tab", { name: "Reviews" })).toHaveAttribute(
      "data-state",
      "active",
    );
  }}
/>

<Story
  name="Selected change actions"
  args={{ initialPage: "log" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const rail = within(
      canvas.getByRole("complementary", { name: "Repository details" }),
    );
    await expect(
      rail.queryByRole("group", { name: "Selected repository actions" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Merge native storage and publication",
      }),
    );
    const actions = await rail.findByRole("group", {
      name: "Selected repository actions",
    });
    for (const name of [
      "Edit description",
      "Create bookmark",
      "Abandon change",
      "Create change",
    ])
      await expect(within(actions).getByRole("button", { name })).toBeEnabled();
    await expect(
      within(actions).queryByRole("button", { name: "Reconcile operations" }),
    ).toBeNull();
    await userEvent.click(
      within(actions).getByRole("button", { name: "Edit description" }),
    );
    const dialog = await page.findByRole("dialog", {
      name: "Edit description",
    });
    await expect(
      within(dialog).getByRole("group", { name: "Selected commit" }),
    ).toHaveTextContent("Merge native storage and publication");
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Edit description" }),
      ).toBeNull(),
    );
    await userEvent.click(rail.getByRole("button", { name: "Op Log" }));
    await expect(
      rail.queryByRole("group", { name: "Selected repository actions" }),
    ).toBeNull();
  }}
/>

<Story
  name="Selected operation actions"
  args={{ initialPage: "oplog" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const rail = within(
      canvas.getByRole("complementary", { name: "Repository details" }),
    );
    await expect(
      rail.queryByRole("group", { name: "Selected repository actions" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "describe: native object history" }),
    );
    const actions = await rail.findByRole("group", {
      name: "Selected repository actions",
    });
    await expect(
      within(actions).getByRole("button", { name: "Reconcile operations" }),
    ).toBeEnabled();
    await expect(
      within(actions).getByRole("button", { name: "Create change" }),
    ).toBeEnabled();
    await expect(
      within(actions).queryByRole("button", { name: "Edit description" }),
    ).toBeNull();
    await userEvent.click(
      within(actions).getByRole("button", { name: "Reconcile operations" }),
    );
    const dialog = await page.findByRole("dialog", {
      name: "Reconcile operations",
    });
    await expect(dialog).toBeVisible();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Reconcile operations" }),
      ).toBeNull(),
    );
    await userEvent.click(rail.getByRole("button", { name: "JJ Log" }));
    await expect(
      rail.queryByRole("group", { name: "Selected repository actions" }),
    ).toBeNull();
  }}
/>
