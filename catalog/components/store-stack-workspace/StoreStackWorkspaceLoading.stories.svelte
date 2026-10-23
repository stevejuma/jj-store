<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceLoadingApplicationFixture from "../../fixtures/StackWorkspaceLoadingApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Loading",
    component: StackWorkspaceLoadingApplicationFixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    beforeEach: async () => {
      await Promise.all([views.stacks(), views.log()]);
    },
  });
  async function assertPending(
    element: HTMLElement,
    view: "workspace" | "branches" | "log" | "oplog" = "workspace",
  ) {
    const canvas = within(element);
    const busy = await canvas.findByRole("status", {
      name: "Loading stack workspace",
    });
    expect(busy).toHaveAttribute("aria-busy", "true");
    expect(busy).toHaveAttribute("data-view", view);
    const columns = busy.querySelectorAll('[data-ui-part="column"]');
    expect(columns).toHaveLength(view === "workspace" ? 2 : 1);
    expect(columns[0]).toHaveAttribute(
      "data-column-id",
      view === "workspace"
        ? "working"
        : view === "branches"
          ? "branches"
          : "revisions",
    );
    if (view === "workspace" || view === "branches") {
      expect(columns[0].getBoundingClientRect().width).toBeCloseTo(
        view === "workspace" ? 330 : 320,
        0,
      );
    } else {
      expect(columns[0].getBoundingClientRect().right).toBeLessThanOrEqual(
        busy.getBoundingClientRect().right,
      );
    }
    expect(busy.querySelector("[data-stack-skeleton-footer]") !== null).toBe(
      view === "workspace" || view === "branches",
    );
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
    expect(
      canvas.getAllByRole("status", { name: "Loading stack workspace" }),
    ).toHaveLength(1);
    expect(within(busy).queryByRole("button")).toBeNull();
    return columns[0].getBoundingClientRect().width;
  }
  async function load(
    element: HTMLElement,
    view: "workspace" | "branches" | "log" | "oplog" = "workspace",
  ) {
    const canvas = within(element);
    const initialWidth = await assertPending(element, view);
    await storyControl(document.body, "Connect repository");
    await assertPending(element, view);
    await storyControl(document.body, "Discover workspace binding");
    await assertPending(element, view);
    await storyControl(document.body, "Start workspace read");
    await assertPending(element, view);
    await storyControl(document.body, "Complete workspace read");
    await waitFor(() =>
      expect(
        canvas.queryByRole("status", { name: "Loading stack workspace" }),
      ).toBeNull(),
    );
    expect(
      canvas.getByRole("combobox", { name: "Active stack workspace" }),
    ).toHaveTextContent("Daily work");
    if (view === "log" || view === "oplog") {
      await waitFor(() =>
        expect(
          element
            .querySelector(
              '.community-projects-repository-log [data-column-id="revisions"]',
            )!
            .getBoundingClientRect().width,
        ).toBeCloseTo(initialWidth, 0),
      );
    }
  }
</script>

<Story
  name="Workspace startup stays skeleton through discovery and first read"
  play={async ({ canvasElement }) => {
    await load(canvasElement);
    const canvas = within(canvasElement);
    const files = canvas.getByRole("region", { name: "Unstaged files" });
    await storyControl(document.body, "Start workspace read");
    expect(files).toBeVisible();
    expect(
      canvas.queryByRole("status", { name: "Loading stack workspace" }),
    ).toBeNull();
    await storyControl(document.body, "Complete workspace read");
    expect(files).toBeVisible();
  }}
/>
<Story
  name="Branches startup stays skeleton"
  args={{ view: "branches" }}
  play={async ({ canvasElement }) => {
    await load(canvasElement, "branches");
    expect(
      within(canvasElement).getByRole("region", { name: "Branches" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Commits startup stays skeleton"
  args={{ view: "log" }}
  play={async ({ canvasElement }) => {
    await load(canvasElement, "log");
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
    await load(canvasElement, "oplog");
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
    await storyControl(document.body, "Discover workspace binding");
    await storyControl(document.body, "Start workspace read");
    await storyControl(document.body, "Fail workspace read");
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
    await assertPending(canvasElement);
    await storyControl(document.body, "Complete workspace read");
    await canvas.findByRole("region", { name: "Unstaged files" });
  }}
/>

<Story
  name="Narrow Workspace retains canvas gutter and wrapped footer"
  play={async ({ canvasElement }) => {
    const host = canvasElement.querySelector<HTMLElement>(
      ".store-application__body-regions",
    )!;
    host.style.width = "380px";
    const busy = await within(canvasElement).findByRole("status", {
      name: "Loading stack workspace",
    });
    await assertPending(canvasElement);
    expect(busy.scrollWidth).toBeLessThanOrEqual(busy.clientWidth);
    const viewport = busy.querySelector(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    const column = busy.querySelector('[data-column-id="working"]')!;
    expect(
      column.getBoundingClientRect().left - busy.getBoundingClientRect().left,
    ).toBeCloseTo(12, 0);
    expect(viewport.getBoundingClientRect().right).toBeLessThanOrEqual(
      busy.getBoundingClientRect().right,
    );
  }}
/>
<Story
  name="Narrow Op Log retains scrollable history without footer"
  args={{ view: "oplog" }}
  play={async ({ canvasElement }) => {
    const host = canvasElement.querySelector<HTMLElement>(
      ".store-application__body-regions",
    )!;
    host.style.width = "380px";
    const busy = await within(canvasElement).findByRole("status", {
      name: "Loading stack workspace",
    });
    await assertPending(canvasElement, "oplog");
    expect(busy.scrollWidth).toBeLessThanOrEqual(busy.clientWidth);
  }}
/>

<Story
  name="Op Log reload retains saved widths and selected comparison"
  args={{ view: "oplog", selectedHistory: true, historyWidth: 640 }}
  play={async ({ canvasElement }) => {
    const busy = await within(canvasElement).findByRole("status", {
      name: "Loading stack workspace",
    });
    await waitFor(() =>
      expect(busy.querySelectorAll('[data-ui-part="column"]')).toHaveLength(2),
    );
    const revisions = busy.querySelector('[data-column-id="revisions"]')!;
    const diff = busy.querySelector('[data-column-id="diff"]')!;
    await waitFor(() =>
      expect(revisions.getBoundingClientRect().width).toBeCloseTo(640, 0),
    );
    expect(diff.getBoundingClientRect().width).toBeCloseTo(400, 0);
    expect(within(busy).queryByRole("button")).toBeNull();
    expect(busy.querySelector("[data-stack-skeleton-footer]")).toBeNull();
  }}
/>

<Story
  name="Loaded workspace retains bookmark framing during stack history reads"
  args={{ holdHistory: true }}
  play={async ({ canvasElement }) => {
    await load(canvasElement);
    const canvas = within(canvasElement);
    const stack = canvas.getByRole("region", { name: "Stack feature/api" });
    const loading = await within(stack).findByRole("status", {
      name: "Loading history for feature/api",
    });
    expect(loading).toHaveAttribute("data-stack-history-skeleton");
    expect(getComputedStyle(loading).borderTopWidth).toBe("1px");
    expect(
      loading.querySelector(".stack-history-skeleton__heading"),
    ).not.toBeNull();
    expect(
      loading.querySelector(".stack-history-skeleton__actions"),
    ).not.toBeNull();
    expect(within(loading).queryByRole("button")).toBeNull();
    await storyControl(document.body, "Complete stack histories");
    await within(stack).findByRole("button", { name: "Expose repository API" });
    await waitFor(() =>
      expect(
        within(stack).queryByRole("status", {
          name: "Loading history for feature/api",
        }),
      ).toBeNull(),
    );
  }}
/>
