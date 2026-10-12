<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
  import { IDS } from "../../fixtures/repository.js";
  import { version } from "../../../package.json";
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
    await canvas.findByRole(
      "heading",
      { name: "Local service", level: 1 },
      { timeout: 10000 },
    );
  }
  async function openRepositorySettings(
    canvasElement: HTMLElement,
    mobile = false,
  ) {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository settings" }),
    );
    await expect(
      await canvas.findByRole(
        "heading",
        { name: "Identity and snapshot", level: 2 },
        { timeout: 10000 },
      ),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("combobox", { name: "Settings repository" }),
    ).not.toBeInTheDocument();
    await expect(canvas.getByRole("tab", { name: "General" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      canvas.queryByRole("navigation", { name: "Repository views" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to repository" }),
    );
    await expect(
      await canvas.findByRole("tab", { name: "Overview" }),
    ).toHaveAttribute("aria-selected", "true");
  }
</script>

<Story
  name="Repository header opens scoped settings"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) =>
    await openRepositorySettings(canvasElement)}
/>
<Story
  name="Mobile repository header opens scoped settings"
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
      await c.findByRole(
        "heading",
        { name: "Service status" },
        { timeout: 10000 },
      ),
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
    for (const name of ["Appearance", "Local service", "Agents", "Compute"]) {
      await userEvent.click(sections.getByRole("button", { name }));
      const title = c.getByRole("heading", { name, level: 1 });
      const content = [
        ...canvasElement.querySelectorAll<HTMLElement>(".store-settings"),
      ].find((element) => element.checkVisibility())!;
      await expect(getComputedStyle(content).paddingTop).toBe("16px");
      const navigation = c.queryByRole("navigation", {
        name: "Settings page navigation",
      });
      const scroll = content.closest("[data-slot='scroll-area']")!;
      await expect(scroll).not.toContainElement(title);
      {
        await expect(scroll).not.toContainElement(navigation);
        const tab = within(navigation!).getAllByRole("tab")[0];
        await expect(tab.querySelector("svg")).not.toBeNull();
        await expect(tab.getBoundingClientRect().left).toBeGreaterThanOrEqual(
          content.getBoundingClientRect().left + 15,
        );
        await expect(getComputedStyle(tab).fontWeight).toBe("600");
      }
    }
  }}
/>

<Story
  name="Global Settings contains only installation sections"
  play={async ({ canvasElement }) => {
    await openSettings(canvasElement);
    const c = within(canvasElement),
      navigation = c.getByRole("navigation", { name: "Settings navigation" }),
      nav = within(navigation);
    const personal = within(nav.getByRole("region", { name: "Personal" }));
    for (const name of ["Profile", "Appearance", "Custom emoji"])
      await expect(personal.getByRole("button", { name })).toBeVisible();
    const app = within(nav.getByRole("region", { name: "App" }));
    for (const name of ["Local service", "Agents", "Compute"])
      await expect(app.getByRole("button", { name })).toBeVisible();
    const footer = nav.getByText(`v${version}`, { exact: true });
    await expect(footer).toBeVisible();
    await expect(footer.getBoundingClientRect().top).toBeGreaterThan(
      app.getByRole("button", { name: "Compute" }).getBoundingClientRect()
        .bottom,
    );
    await expect(
      navigation.getBoundingClientRect().bottom -
        footer.getBoundingClientRect().bottom,
    ).toBeLessThanOrEqual(20);
    for (const name of ["Repository", "Search", "Storage", "Sync"])
      await expect(nav.queryByRole("button", { name })).not.toBeInTheDocument();
    await expect(
      c.queryByRole("combobox", { name: "Settings repository" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Compact health toolbar"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository settings" }),
    );
    await userEvent.click(await canvas.findByRole("tab", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    const navigation = canvas.getByRole("navigation", {
      name: "Repository settings navigation",
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
    const content = [
      ...canvasElement.querySelectorAll<HTMLElement>(".store-settings"),
    ].find((element) => element.checkVisibility())!;
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
    await expect(canvas.getByRole("tab", { name: "Theme" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    const card = canvas.getByRole("region", { name: "Theme" });
    const header = card.querySelector(".store-settings-card__header")!;
    await expect(header).toContainElement(
      canvas.getByRole("heading", { name: "Theme" }),
    );
    await expect(getComputedStyle(header).borderBottomWidth).toBe("1px");
    await expect(card.querySelector(".appearance-card")).not.toBeNull();
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
      await c.findByRole(
        "heading",
        { name: "Service status" },
        { timeout: 10000 },
      ),
    ).toBeVisible();
    await expect(
      c.queryByRole("heading", { name: "Agent transcript capture" }),
    ).toBeNull();
  }}
/>
