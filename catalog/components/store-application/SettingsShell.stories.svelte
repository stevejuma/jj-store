<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { playMobileSettings } from "./settings-shell-play.js";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Shell",
    component: StoreApplication,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Settings"
  args={{ accessMode: "local_trusted", initiallyCollapsed: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const beforeHash = canvasElement.ownerDocument.defaultView?.location.hash;
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await expect(
      await canvas.findByRole("heading", {
        name: "Local service",
        level: 1,
      }),
    ).toBeVisible();
    const globalNav = canvas.getByRole("navigation", {
      name: "Settings navigation",
    });
    await expect(
      within(globalNav).queryByRole("button", { name: "Repository" }),
    ).toBeNull();
    await expect(
      within(globalNav).queryByRole("button", { name: "Search" }),
    ).toBeNull();
    await userEvent.click(
      within(globalNav).getByRole("button", { name: "Local service" }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    await expect(
      canvas.queryByRole("button", { name: "Toggle details" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to jj-store" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository settings" }),
    );
    await expect(
      await canvas.findByRole("tab", { name: "General" }),
    ).toHaveAttribute("aria-selected", "true");
    await expect(canvas.getByText("admin")).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: /^Copy full Repository ID:/ }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await expect(canvas.getByText("jj-store-native-v1")).toBeVisible();
    await userEvent.click(await canvas.findByRole("tab", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(canvas.getByText("Fresh")).toBeVisible();
    for (const label of [
      "Repository position",
      "Durable position",
      "Indexed position",
    ]) {
      const row = canvas
        .getByText(label, { exact: true })
        .closest(".store-readonly-row")!;
      await expect(
        within(row as HTMLElement).getByText("18", { exact: true }),
      ).toBeVisible();
    }
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to repository" }),
    );
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
    await expect(canvasElement.ownerDocument.defaultView?.location.hash).toBe(
      beforeHash,
    );
  }}
/>
<Story
  name="Settings collapsed"
  args={{ accessMode: "local_trusted", initiallyCollapsed: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    const sidebar = await canvas.findByLabelText("Settings sidebar");
    const navigation = within(sidebar).getByRole("navigation", {
      name: "Settings navigation",
    });
    await expect(navigation).not.toHaveAttribute("data-compact");
    await userEvent.click(
      canvas.getByRole("button", { name: "Collapse left sidebar" }),
    );
    await waitFor(() =>
      expect(sidebar).toHaveAttribute("data-state", "collapsed"),
    );
    await expect(navigation).toHaveAttribute("data-compact", "true");
    for (const label of [
      "Back to jj-store",
      "Local service",
      "Profile",
      "Appearance",
      "Agents",
    ]) {
      const item = within(navigation).getByRole("button", { name: label });
      await expect(item).toBeVisible();
      await expect(item).toHaveAttribute("title", label);
      await expect(item.querySelector("svg")).toBeVisible();
      await expect(item.querySelector("span")).not.toBeVisible();
    }
  }}
/>
<Story
  name="Settings degraded search"
  args={{
    accessMode: "local_trusted",
    initiallyCollapsed: false,
    settingsSearch: "degraded",
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository settings" }),
    );
    await userEvent.click(await canvas.findByRole("tab", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(canvas.getByText("Degraded")).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Prepare graph index" }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Prepare graph index" }),
    );
    await expect(
      within(canvasElement.ownerDocument.body).getByText(
        "Graph index preparation completed.",
      ),
    ).toBeVisible();
    await userEvent.click(await canvas.findByRole("tab", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(canvas.getByText("Degraded")).toBeVisible();
  }}
/>
<Story
  name="Settings unavailable search"
  args={{
    accessMode: "local_trusted",
    initiallyCollapsed: false,
    settingsSearch: "unavailable",
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository settings" }),
    );
    await canvas.findByRole("tab", { name: "General" });
    await expect(canvas.queryByRole("tab", { name: "Search" })).toBeNull();
    await expect(canvas.queryByRole("tab", { name: "Health" })).toBeNull();
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Prepare graph index" }),
    );
    await expect(
      within(canvasElement.ownerDocument.body).getByText(
        "Graph index preparation completed.",
      ),
    ).toBeVisible();
  }}
/>
<Story
  name="Settings status failure"
  args={{
    accessMode: "local_trusted",
    initiallyCollapsed: false,
    settingsSearch: "failed",
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository settings" }),
    );
    await userEvent.click(await canvas.findByRole("tab", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(
      within(canvasElement.ownerDocument.body).getByText(
        "Search status could not be loaded.",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Refresh status" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Settings mobile"
  args={{ accessMode: "local_trusted", displayMode: "mobile" }}
  play={playMobileSettings}
/>
