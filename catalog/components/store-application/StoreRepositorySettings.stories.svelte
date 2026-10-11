<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositorySettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Repository",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Native categories are scoped with no picker"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("heading", { name: "Identity and snapshot" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("navigation", { name: "Settings page navigation" }),
    ).not.toBeInTheDocument();
    await expect(
      canvasElement.querySelector(".store-settings__page-title"),
    ).toBeNull();
    for (const name of [
      "General",
      "Local folders",
      "Search",
      "Storage",
      "Sync",
      "Advanced",
    ])
      await expect(c.getByRole("tab", { name })).toBeVisible();
    await expect(
      c.queryByRole("combobox", { name: "Settings repository" }),
    ).not.toBeInTheDocument();
    await expect(c.getByText("Server Hosted", { exact: true })).toHaveAttribute(
      "data-chip-tone",
      "accent",
    );
  }}
/>
<Story
  name="Local JJ omits unsupported management"
  args={{ local: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByText("Local JJ", { exact: true })).toHaveAttribute(
      "data-chip-tone",
      "success",
    );
    for (const name of ["Search", "Storage", "Sync", "Protection"])
      await expect(c.queryByRole("tab", { name })).not.toBeInTheDocument();
    await expect(c.queryByText("Unavailable")).not.toBeInTheDocument();
  }}
/>
<Story
  name="Reader search link resolves to Health"
  args={{ reader: true, section: "search", tab: "scopes" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByRole("tab", { name: "Health" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      c.getByRole("status", { name: "Settings route" }),
    ).toHaveTextContent("settings:search:health");
    await expect(
      c.queryByRole("tab", { name: "Configuration" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Native folder draft survives categories"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Local folders" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Folder path" }),
      "/retained/draft",
    );
    await userEvent.click(c.getByRole("tab", { name: "General" }));
    await userEvent.click(c.getByRole("tab", { name: "Local folders" }));
    await expect(c.getByRole("textbox", { name: "Folder path" })).toHaveValue(
      "/retained/draft",
    );
  }}
/>
<Story
  name="Unsupported category returns to General"
  args={{ local: true, section: "storage" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByRole("tab", { name: "General" }),
    ).toHaveAttribute("aria-selected", "true");
    await expect(
      c.getByRole("status", { name: "Settings route" }),
    ).toHaveTextContent("settings:general:");
  }}
/>

<Story
  name="Advanced uses compact cards with complete capability values"
  args={{ section: "advanced" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("heading", { name: "Repository capabilities", level: 1 }),
    ).toBeVisible();
    for (const name of [
      "Graph index",
      "Capabilities",
      "Read and event limits",
    ]) {
      const card = c.getByRole("region", { name });
      await expect(card).toHaveAttribute("data-slot", "card");
      await expect(card.querySelector("h2")).toBeVisible();
    }
    const value = c
      .getByText("Typed mutations")
      .closest(".store-readonly-row")!
      .querySelector(".store-readonly-row__value")!;
    await expect(getComputedStyle(value).whiteSpace).toBe("normal");
    await expect(
      canvasElement.querySelector('[data-presentation="line"]'),
    ).toBeNull();
    await expect(
      c.queryByRole("button", { name: /Copy full Typed mutations/ }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Secondary line tabs retain primary pills and keyboard routing"
  args={{ section: "storage" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const navigation = c.getByRole("navigation", {
      name: "Settings page navigation",
    });
    const overview = within(navigation).getByRole("tab", { name: "Overview" });
    await expect(overview.querySelector("svg")).toBeVisible();
    await expect(overview.closest('[data-slot="tabs-list"]')).toHaveAttribute(
      "data-variant",
      "line",
    );
    await expect(
      c.getByRole("tab", { name: "Storage" }).closest("[data-presentation]"),
    ).toHaveAttribute("data-presentation", "pill");
    const indicator = getComputedStyle(overview, "::after");
    await expect(indicator.opacity).toBe("1");
    await expect(parseFloat(indicator.height)).toBeGreaterThan(0);
    overview.focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(
      c.getByRole("status", { name: "Settings route" }),
    ).toHaveTextContent("settings:storage:maintenance");
  }}
/>
<Story
  name="Narrow folder layout retains full input and bottom actions"
  args={{ section: "folders", narrow: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = c.getByRole("textbox", { name: "Folder path" });
    await userEvent.type(input, "/a/very/long/repository/folder/draft");
    await expect(input).toHaveValue("/a/very/long/repository/folder/draft");
    const card = c.getByRole("region", { name: "Local folders" });
    await expect(card.getBoundingClientRect().width).toBeLessThanOrEqual(390);
    await userEvent.click(c.getByRole("button", { name: "Cancel" }));
    await expect(input).toHaveValue("");
  }}
/>

<Story
  name="Discovery retains Back and replaces placeholders with repository categories"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("status", { name: "Loading repository settings" }),
    ).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Back to repository" }),
    ).toBeEnabled();
    await expect(c.queryAllByRole("tab")).toHaveLength(0);
    await userEvent.click(c.getByRole("button", { name: "Finish discovery" }));
    await expect(
      c.getByRole("heading", { name: "Identity and snapshot" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("status", { name: "Loading repository settings" }),
    ).not.toBeInTheDocument();
    await expect(c.getByRole("tab", { name: "General" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
  }}
/>

<Story
  name="Reader Search health skeleton stays within its card"
  args={{ reader: true, section: "search", tab: "health", healthLoading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const loading = c.getByRole("status", { name: "Loading search health" });
    await expect(loading).toBeVisible();
    await expect(loading.closest('[data-slot="card"]')).not.toBeNull();
    await expect(loading.querySelector('[data-slot="card"]')).toBeNull();
    await expect(
      c.queryByRole("button", { name: "Save settings" }),
    ).not.toBeInTheDocument();
  }}
/>
