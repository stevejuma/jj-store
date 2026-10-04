<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
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
</script>

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
    await expect(getComputedStyle(content).paddingTop).toBe("0px");
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
    ).toBeLessThan(8);
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
