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
