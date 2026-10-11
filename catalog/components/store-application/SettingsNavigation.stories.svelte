<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
  import { IDS } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Navigation",
    component: StoreApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
  });
  async function openSettings(canvasElement: HTMLElement) {
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
  }
  async function openRepositorySettings(
    canvasElement: HTMLElement,
    mobile = false,
  ) {
    const canvas = within(canvasElement);
    const window = canvasElement.ownerDocument.defaultView!;
    const beforeHash = window.location.hash;
    const beforeHistory = window.history.length;
    await userEvent.click(
      canvas.getByRole("button", {
        name: mobile ? "Open details sidebar" : "Toggle details",
      }),
    );
    const rail = await canvas.findByRole("navigation", {
      name: "Repository views",
    });
    const settings = within(rail).getByRole("button", {
      name: "Repository settings",
    });
    await expect(settings).toHaveTextContent("Settings");
    settings.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      await canvas.findByRole(
        "heading",
        { name: "Repository", level: 1 },
        { timeout: 10000 },
      ),
    ).toBeVisible();
    await expect(
      canvas.getByRole("combobox", { name: "Settings repository" }),
    ).toHaveTextContent("jj-store");
    await expect(
      canvas.getByRole("button", {
        name: `Copy full Selected operation: ${IDS.opLeft}`,
      }),
    ).toBeVisible();
    await expect(canvas.getByRole("tab", { name: "General" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(window.location.hash).toBe(beforeHash);
    await expect(window.history.length).toBe(beforeHistory);
    if (mobile)
      await userEvent.click(
        canvas.getByRole("button", { name: "Open settings sidebar" }),
      );
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to jj-store" }),
    );
    await expect(
      await canvas.findByText("jj-store · Overview", { exact: true }),
    ).toBeVisible();
    await expect(window.location.hash).toBe(beforeHash);
    await expect(window.history.length).toBe(beforeHistory);
  }
</script>

<Story
  name="Repository rail opens selected repository settings"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) =>
    await openRepositorySettings(canvasElement)}
/>
<Story
  name="Mobile repository rail opens selected repository settings"
  args={{ initialPage: "files", displayMode: "mobile" }}
  play={async ({ canvasElement }) =>
    await openRepositorySettings(canvasElement, true)}
/>

<Story
  name="Appearance typography matches other Settings sections"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const c = within(canvasElement);
    const typography = (element: Element) => {
      const style = getComputedStyle(element);
      return {
        family: style.fontFamily,
        size: style.fontSize,
        weight: style.fontWeight,
        color: style.color,
      };
    };
    const heading = typography(
      c.getByRole("heading", { name: "Service status" }),
    );
    const label = typography(c.getByText("Access mode", { exact: true }));
    await userEvent.click(c.getByRole("button", { name: "Appearance" }));
    const theme = c.getByRole("heading", { name: "Theme" });
    await expect(typography(theme)).toEqual(heading);
    await expect(getComputedStyle(theme).marginLeft).toBe("0px");
    await expect(
      typography(c.getByText("Color mode", { exact: true })),
    ).toEqual(label);
    const description = c.getByText(
      "Follow your system or choose a light or dark appearance.",
    );
    await expect(getComputedStyle(description).fontSize).toBe(label.size);
    await expect(getComputedStyle(description).fontWeight).toBe("400");
  }}
/>

<Story
  name="Consistent title and body spacing across Settings pages"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const c = within(canvasElement);
    const sections = within(
      c.getByRole("navigation", { name: "Settings navigation" }),
    );
    for (const name of [
      "Appearance",
      "Local service",
      "Agents",
      "Compute",
      "Repository",
      "Search",
      "Storage",
      "Sync",
    ]) {
      await userEvent.click(sections.getByRole("button", { name }));
      const title = c.getByRole("heading", { name, level: 1 });
      const content = canvasElement.querySelector(".store-settings")!;
      await expect(getComputedStyle(content).paddingTop).toBe("16px");
      const navigation = c.getByRole("navigation", {
        name: "Settings page navigation",
      });
      const firstTab = within(navigation).queryAllByRole("tab")[0];
      if (firstTab)
        await expect(
          firstTab.getBoundingClientRect().left -
            title.getBoundingClientRect().right,
        ).toBeGreaterThanOrEqual(20);
    }
  }}
/>

<Story
  name="Repository context"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Repository" }));
    const picker = canvas.getByRole("combobox", {
      name: "Settings repository",
    });
    await expect(picker).toHaveTextContent("jj-store");
    await expect(picker).toHaveAttribute("data-variant", "ghost");
    await expect(getComputedStyle(picker).borderTopWidth).toBe("0px");
    await expect(picker.getBoundingClientRect().width).toBeLessThanOrEqual(176);
    await expect(
      getComputedStyle(picker.querySelector("span")!).textOverflow,
    ).toBe("ellipsis");
    await userEvent.click(picker);
    const body = within(canvasElement.ownerDocument.body);
    await userEvent.type(
      body.getByRole("combobox", { name: "Select settings repository" }),
      "native-demo",
    );
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(picker).toHaveTextContent("native-demo");
    await expect(
      canvas.getByRole("heading", { name: "Repository", level: 1 }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", {
        name: "Copy full Repository ID: native-demo",
      }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to jj-store" }),
    );
    await expect(
      await canvas.findByText("native-demo · Overview", { exact: true }),
    ).toBeVisible();
  }}
/>

<Story
  name="Compact health toolbar"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    const navigation = canvas.getByRole("navigation", {
      name: "Settings page navigation",
    });
    const refresh = within(navigation).getByRole("button", {
      name: "Refresh status",
    });
    await expect(refresh).toHaveTextContent("");
    refresh.focus();
    await waitFor(() =>
      expect(
        canvasElement.ownerDocument.querySelector(
          '[data-slot="tooltip-content"]',
        ),
      ).toHaveTextContent("Refresh status"),
    );
    const content = canvasElement.querySelector(".store-settings")!;
    await expect(getComputedStyle(content).paddingTop).toBe("16px");
  }}
/>

<Story
  name="Compact appearance"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Appearance" }));
    await expect(
      canvas.queryByText("Choose how jj-store looks and feels."),
    ).toBeNull();
    const navigation = canvas.getByRole("navigation", {
      name: "Settings page navigation",
    });
    const theme = canvas.getByRole("heading", { name: "Theme" });
    await expect(
      theme.getBoundingClientRect().top -
        navigation.getBoundingClientRect().bottom,
    ).toBeGreaterThanOrEqual(16);
    await expect(
      theme.getBoundingClientRect().top -
        navigation.getBoundingClientRect().bottom,
    ).toBeLessThan(24);
  }}
/>

<Story
  name="Dedicated installation Agents section"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const c = within(canvasElement);
    await expect(
      c.queryByRole("heading", { name: "Agent transcript capture" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: /^Agents$/ }));
    await expect(
      c.getByRole("heading", { name: "Agents", level: 1 }),
    ).toBeVisible();
    await expect(
      c.queryByRole("combobox", { name: "Settings repository" }),
    ).toBeNull();
    await expect(c.getByRole("tab", { name: "Runtimes" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await expect(
      c.getByText("Capture management is not available for this connection."),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: /^Local service$/ }));
    await expect(
      c.getByRole("heading", { name: "Service status" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("heading", { name: "Agent transcript capture" }),
    ).toBeNull();
  }}
/>
