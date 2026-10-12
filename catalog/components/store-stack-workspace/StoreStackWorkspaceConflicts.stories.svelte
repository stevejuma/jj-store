<script module lang="ts">
  import { views as applicationViews } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceConflictApplicationFixture from "../../fixtures/StackWorkspaceConflictApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Conflict presentation",
    component: StackWorkspaceConflictApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await Promise.all([
        applicationViews.stacks(),
        applicationViews.files(),
        applicationViews.log(),
        applicationViews.stackDetails(),
        applicationViews.stackConflicts(),
      ]);
    },
  });
  import {
    rail,
    closeEditor,
    loadingConflict,
  } from "./conflict-story-actions.js";
</script>

<Story
  name="Conflict file loading shows merge skeleton until the read completes"
  args={{ deferredConflict: true }}
  play={loadingConflict}
/>

<Story
  name="Unstaged conflict indicators and full width diff alert"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const column = await canvas.findByRole("region", {
      name: "Unstaged files",
    });
    const banner = canvas
      .getByText(/^Workspace has unresolved conflicts/)
      .closest('[role="alert"]')!;
    expect(banner.querySelector('[data-ui-part="alert-title"]')).toBeNull();
    const message = within(banner as HTMLElement).getByText(
      /^Workspace has unresolved conflicts/,
    );
    const resolve = within(banner as HTMLElement).getByRole("button", {
      name: "Resolve conflicts",
    });
    const messageRect = message.getBoundingClientRect(),
      actionRect = resolve.getBoundingClientRect();
    expect(
      Math.abs(
        messageRect.top +
          messageRect.height / 2 -
          actionRect.top -
          actionRect.height / 2,
      ),
    ).toBeLessThanOrEqual(1);
    expect(messageRect.right).toBeLessThanOrEqual(actionRect.left);
    const count = within(banner as HTMLElement).getByLabelText("1 conflict");
    expect(count).toHaveTextContent("1");
    const countRect = count.getBoundingClientRect();
    expect(Math.abs(countRect.width - countRect.height)).toBeLessThanOrEqual(1);
    expect(parseFloat(getComputedStyle(count).fontSize)).toBeLessThan(
      parseFloat(getComputedStyle(message).fontSize),
    );
    expect(getComputedStyle(count).backgroundColor).not.toBe(
      getComputedStyle(banner).backgroundColor,
    );
    expect(message).not.toHaveTextContent("src/settings.ts");
    expect(getComputedStyle(banner).paddingTop).toBe("4px");
    expect(getComputedStyle(banner).paddingBottom).toBe("4px");
    expect(getComputedStyle(banner).marginBottom).toBe("8px");
    const iconRect = banner.querySelector("svg")!.getBoundingClientRect();
    expect(
      Math.abs(
        iconRect.top +
          iconRect.height / 2 -
          messageRect.top -
          messageRect.height / 2,
      ),
    ).toBeLessThanOrEqual(1);
    for (const mode of ["folders", "list", "packages"]) {
      await userEvent.click(
        within(column).getByRole("radio", { name: `Show files as ${mode}` }),
      );
      for (const path of ["src/settings.ts", "src/metadata.ts"]) {
        const row = await within(column).findByRole("button", {
          name: path,
        });
        expect(
          within(row).getByRole("img", { name: "Conflicted file" }),
        ).toBeVisible();
        expect(getComputedStyle(row).backgroundColor).toBe(
          getComputedStyle(banner).backgroundColor,
        );
      }
      await waitFor(() => {
        const rows = [
          ...column.querySelectorAll<HTMLElement>(
            '[data-ui-part="file-row"], [data-ui-part="directory-row"]',
          ),
        ];
        expect(rows.length).toBeGreaterThan(3);
        for (let index = 1; index < rows.length; index += 1) {
          expect(
            rows[index].getBoundingClientRect().top -
              rows[index - 1].getBoundingClientRect().bottom,
          ).toBeCloseTo(2, 1);
        }
      });
      for (const path of ["README.md", "src/resolved.ts"]) {
        const row = within(column).getByRole("button", {
          name: path,
        });
        expect(
          within(row).queryByRole("img", { name: "Conflicted file" }),
        ).toBeNull();
      }
    }
    await userEvent.click(
      within(column).getByRole("button", {
        name: "src/settings.ts",
      }),
    );
    const diff = await canvas.findByRole("region", {
      name: "Selected changes from working",
    });
    const alert = within(diff).getByRole("alert");
    expect(alert).toHaveTextContent(
      "This file cannot be displayed as a text diff (conflict).",
    );
    expect(within(diff).queryByText("Text diff unavailable")).toBeNull();
    expect(alert.querySelector('[data-ui-part="alert-title"]')).toBeNull();
    const header = diff.querySelector('[data-ui-part="column-header"]')!;
    expect(
      Math.abs(
        alert.getBoundingClientRect().left -
          header.getBoundingClientRect().left,
      ),
    ).toBeLessThanOrEqual(1);
    expect(
      Math.abs(
        alert.getBoundingClientRect().right -
          header.getBoundingClientRect().right,
      ),
    ).toBeLessThanOrEqual(1);
    await userEvent.click(
      canvas.getByRole("button", { name: "Simulate conflict recovery" }),
    );
    await waitFor(() =>
      expect(column.querySelectorAll("[data-workspace-conflict]")).toHaveLength(
        0,
      ),
    );
    expect(
      canvas.queryByText(/^Workspace has unresolved conflicts/),
    ).toBeNull();
  }}
/>

<Story
  name="Rail actions and notification open the retained conflict editor"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      details = await rail(canvasElement);
    const actions = within(
      details.getByRole("region", { name: "Workspace actions" }),
    );
    expect(
      actions.getByRole("button", { name: "Workspace settings" }),
    ).toBeEnabled();
    expect(
      details.getByRole("navigation", { name: "Stack workspace views" }),
    ).toBeVisible();
    expect(
      actions.getByRole("button", { name: "Refresh workspace" }),
    ).toBeEnabled();
    const tabs = within(
      canvas.getByRole("tablist", { name: "Local repository views" }),
    );
    for (const name of ["Branches", "Workspace", "Commits", "Op Log"]) {
      await userEvent.click(tabs.getByRole("tab", { name }));
      await waitFor(() => {
        const banner = canvas.queryByText(
          /^Workspace has unresolved conflicts/,
        );
        if (name === "Workspace") expect(banner).toBeVisible();
        else expect(banner).toBeNull();
      });
      await userEvent.click(
        actions.getByRole("button", { name: "Resolve conflicts" }),
      );
      await closeEditor(canvasElement);
      expect(tabs.getByRole("tab", { name })).toHaveAttribute(
        "data-state",
        "active",
      );
    }
    await userEvent.click(tabs.getByRole("tab", { name: "Workspace" }));
    await waitFor(() =>
      expect(
        canvas.getByText(/^Workspace has unresolved conflicts/),
      ).toBeVisible(),
    );
    const portal = within(canvasElement.ownerDocument.body);
    const toast =
      portal
        .getByText("Local workspace needs attention")
        .closest('[data-ui-part="notification-toast"]') ??
      portal.getByText("Local workspace needs attention").closest("section");
    expect(toast).not.toBeNull();
    const resolve = within(toast as HTMLElement).getByRole("button", {
      name: "Resolve conflicts",
    });
    expect(getComputedStyle(resolve).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    expect(getComputedStyle(resolve).backgroundColor).not.toBe("transparent");
    const resting = getComputedStyle(resolve).backgroundColor;
    const { page: native } = await import("vitest/browser");
    await native
      .getByRole("region", { name: "Notifications" })
      .getByRole("button", { name: "Resolve conflicts" })
      .hover();
    expect(getComputedStyle(resolve).backgroundColor).not.toBe(resting);
    await native.getByRole("button", { name: "Toggle details" }).hover();
    resolve.focus();
    expect(resolve).toHaveFocus();
    expect(getComputedStyle(resolve).outlineStyle).toBe("solid");
    await userEvent.keyboard("{Enter}");
    await closeEditor(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Simulate conflict recovery" }),
    );
    await waitFor(() =>
      expect(
        details.queryByRole("button", { name: "Resolve conflicts" }),
      ).toBeNull(),
    );
    await waitFor(() =>
      expect(portal.queryByText("Local workspace needs attention")).toBeNull(),
    );
  }}
/>

<Story
  name="Pending reconciliation hides rail actions until cancellation"
  args={{ membershipChanged: true }}
  play={async ({ canvasElement }) => {
    const details = await rail(canvasElement);
    await userEvent.click(
      within(canvasElement).getByRole("button", {
        name: "Simulate conflict recovery",
      }),
    );
    await waitFor(() =>
      expect(
        details.queryByRole("button", { name: "Resolve conflicts" }),
      ).toBeNull(),
    );
    await userEvent.click(
      details.getByRole("button", { name: "Reapply saved branches" }),
    );
    const portal = within(canvasElement.ownerDocument.body);
    const dialog = await portal.findByRole("dialog");
    expect(
      details.queryByRole("region", { name: "Workspace actions" }),
    ).toBeNull();
    const navigation = details.getByRole("navigation", {
      name: "Stack workspace views",
    });
    for (const button of within(navigation).getAllByRole("button"))
      expect(button).toBeDisabled();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel preview" }),
    );
    await waitFor(() =>
      expect(
        details.getByRole("button", { name: "Reapply saved branches" }),
      ).toBeEnabled(),
    );
    expect(
      details.getByRole("button", { name: "Workspace settings" }),
    ).toBeEnabled();
    expect(details.getByRole("button", { name: "Workspace" })).toBeEnabled();
  }}
/>

<Story
  name="Unavailable resolution has no executable notification action"
  args={{ writable: false, membershipChanged: true }}
  play={async ({ canvasElement }) => {
    const details = await rail(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    expect(
      within(
        details.getByRole("region", { name: "Workspace actions" }),
      ).queryByRole("button", { name: "Resolve conflicts" }),
    ).toBeNull();
    expect(
      details.queryByRole("button", { name: "Reapply saved branches" }),
    ).toBeNull();
    expect(
      details.getByRole("button", { name: "Workspace settings" }),
    ).toBeEnabled();
    expect(
      details.getByRole("button", { name: "Refresh workspace" }),
    ).toBeEnabled();
    const toast = portal
      .getByText("Local workspace needs attention")
      .closest("section")!;
    expect(
      within(toast).queryByRole("button", { name: "Resolve conflicts" }),
    ).toBeNull();
  }}
/>

<Story
  name="Details links use repository navigation hover and keyboard focus"
  play={async ({ canvasElement }) => {
    const details = await rail(canvasElement);
    const navigation = details.getByRole("navigation", {
      name: "Stack workspace views",
    });
    const branch = within(navigation).getByRole("button", { name: "Branches" });
    expect(branch).not.toHaveAttribute("aria-current");
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      const swatch = canvasElement.ownerDocument.createElement("span");
      swatch.style.backgroundColor =
        "color-mix(in oklab, var(--muted-foreground) 10%, transparent)";
      canvasElement.append(swatch);
      const color = getComputedStyle(swatch).backgroundColor;
      swatch.remove();
      await pointer.hover(branch);
      await waitFor(() =>
        expect(getComputedStyle(branch).backgroundColor).toBe(color),
      );
      expect(parseFloat(getComputedStyle(branch).borderRadius)).toBeGreaterThan(
        0,
      );
      await userEvent.keyboard("{Tab}");
      branch.focus();
      expect(branch).toHaveFocus();
      await waitFor(() =>
        expect(getComputedStyle(branch).backgroundColor).toBe(color),
      );
      await userEvent.keyboard("{Enter}");
      await waitFor(() =>
        expect(branch).toHaveAttribute("aria-current", "page"),
      );
    }
  }}
/>
