<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AppearanceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Appearance",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  const saved = (canvas: HTMLElement) =>
    JSON.parse(
      canvas.querySelector<HTMLElement>(".appearance-fixture")!.dataset.saved!,
    );
</script>

<Story
  name="Original theme comparison and paired modes"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: /Theme style,/ }));
    await userEvent.click(
      c.getByRole("button", { name: "Use Original theme" }),
    );
    await waitFor(() =>
      expect(document.documentElement.dataset.storeTheme).toBe("original"),
    );
    await expect(
      getComputedStyle(document.documentElement)
        .getPropertyValue("--background")
        .trim(),
    ).toBe("oklch(100% 0 0)");
    await expect(
      getComputedStyle(document.documentElement)
        .getPropertyValue("--popover")
        .trim(),
    ).toBe("oklch(100% 0 0)");
    await expect(saved(canvasElement)).toMatchObject({
      theme: "original",
      accent: "#3b82f6",
    });
    await userEvent.click(c.getByRole("radio", { name: "Dark" }));
    await waitFor(() =>
      expect(document.documentElement.dataset.storeTheme).toBe("original-dark"),
    );
    await expect(
      getComputedStyle(document.documentElement)
        .getPropertyValue("--background")
        .trim(),
    ).toBe("oklch(12.9% 0.042 264.695)");
    await expect(
      getComputedStyle(document.documentElement)
        .getPropertyValue("--popover")
        .trim(),
    ).toBe("oklch(20.8% 0.042 265.755)");
    await userEvent.click(c.getByRole("radio", { name: "Light" }));
    await waitFor(() =>
      expect(document.documentElement.dataset.storeTheme).toBe("original"),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Use Github Light Default theme" }),
    );
    await expect(
      c.getByRole("button", { name: "Use Blue accent" }),
    ).toHaveAttribute("aria-pressed", "true");
    const originalPreview = canvasElement.querySelector(
      '[data-store-theme="original"]',
    )!;
    await expect(
      getComputedStyle(originalPreview).getPropertyValue("--background").trim(),
    ).toBe("oklch(100% 0 0)");
  }}
/>

<Story
  name="All controls and local links"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByRole("heading", { name: "Appearance" })).toBeVisible();
    await expect(
      c.getAllByRole("heading", { name: "Appearance", level: 1 }),
    ).toHaveLength(1);
    await expect(
      c.queryByText("Choose how jj-store looks and feels."),
    ).toBeNull();
    await expect(
      c.getByRole("navigation", { name: "Settings page navigation" })
        .firstElementChild,
    ).toBe(c.getByRole("heading", { name: "Appearance", level: 1 }));
    await expect(c.queryByRole("button", { name: "Back to app" })).toBeNull();
    await userEvent.click(c.getByRole("button", { name: /Theme style,/ }));
    await expect(
      c.getAllByRole("button", { name: /Use .* theme/ }),
    ).toHaveLength(19);
    await userEvent.click(c.getByRole("radio", { name: "Dark" }));
    await expect(
      c.getAllByRole("button", { name: /Use .* theme/ }),
    ).toHaveLength(44);
    await userEvent.click(c.getByRole("button", { name: "Use Dracula theme" }));
    await expect(
      c.getByRole("button", { name: "Use Blue accent" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("switch", { name: "Prominent active tab" }),
    ).toBeNull();
    const expectCircularAccent = async (name: string) => {
      const button = c.getByRole("button", { name: `Use ${name} accent` });
      await expect(button).toHaveAttribute("aria-pressed", "true");
      const swatch = button.getBoundingClientRect();
      const ring = button.querySelector("span")!.getBoundingClientRect();
      await expect(ring.width).toBeGreaterThan(0);
      await expect(Math.abs(swatch.width - swatch.height)).toBeLessThan(0.1);
      await expect(Math.abs(ring.width - ring.height)).toBeLessThan(0.1);
      await expect(
        Math.abs(ring.x + ring.width / 2 - swatch.x - swatch.width / 2),
      ).toBeLessThan(0.1);
      await expect(
        Math.abs(ring.y + ring.height / 2 - swatch.y - swatch.height / 2),
      ).toBeLessThan(0.1);
    };
    await expectCircularAccent("Blue");
    await userEvent.click(
      c.getByRole("button", { name: "Use Neutral accent" }),
    );
    await expectCircularAccent("Neutral");
    await expect(
      c.getByRole("button", { name: "Use Blue accent" }),
    ).toHaveAttribute("aria-pressed", "false");
    await userEvent.click(c.getByRole("button", { name: "Use Green accent" }));
    await expectCircularAccent("Green");
    await userEvent.click(c.getByRole("switch", { name: "Glass background" }));
    const slider = c.getByRole("slider", { name: "Glass opacity" });
    await userEvent.click(slider);
    await userEvent.keyboard("{End}");
    await expect(slider).toHaveAttribute("aria-valuenow", "90");
    const reset = c.getByRole("button", { name: "Reset glass opacity" });
    await expect(reset.querySelector("svg")).not.toBeNull();
    await expect(
      reset.querySelector("svg")!.getBoundingClientRect().width,
    ).toBeGreaterThanOrEqual(16);
    const modes = c.getAllByRole("radio").slice(0, 3);
    await expect(modes[0].getBoundingClientRect().top).toBe(
      modes[1].getBoundingClientRect().top,
    );
    await userEvent.click(reset);
    await expect(slider).toHaveAttribute("aria-valuenow", "65");
    await userEvent.click(c.getByRole("radio", { name: "Larger" }));
    await userEvent.click(c.getByRole("radio", { name: "Spacious" }));
    await userEvent.click(c.getByRole("radio", { name: "Rich" }));
    await userEvent.click(c.getByRole("radio", { name: "Focus" }));
    await expect(saved(canvasElement)).toMatchObject({
      mode: "dark",
      theme: "dracula",
      accent: "#22c55e",
      glass: true,
      opacity: 65,
      fontSize: "larger",
      density: "spacious",
      links: "rich",
      threads: "focus",
    });
    const link = c.getByRole("link", { name: /Existing product updates/ });
    await expect(link).toHaveAttribute("data-layout", "rich");
    await expect(
      c.getByText("https://example.com/code", { exact: true }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("radio", { name: "Light" }));
    await expect(
      c.getAllByRole("button", { name: /Use .* theme/ }),
    ).toHaveLength(20);
    await userEvent.click(c.getByRole("button", { name: "Use Buzz theme" }));
    await expect(
      c.getByRole("switch", { name: "Prominent active tab" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Use Blue accent" }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("switch", { name: "Prominent active tab" }),
    );
    await expect(saved(canvasElement).prominent).toBe(true);
  }}
/>
<Story
  name="Preview and keyboard selection"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const before = saved(canvasElement);
    await userEvent.hover(c.getByRole("radio", { name: "Smaller" }));
    await waitFor(() =>
      expect(document.documentElement.dataset.storeFont).toBe("smaller"),
    );
    await expect(saved(canvasElement)).toEqual(before);
    await userEvent.unhover(c.getByRole("radio", { name: "Smaller" }));
    await waitFor(() =>
      expect(document.documentElement.dataset.storeFont).toBe("default"),
    );
    c.getByRole("radio", { name: "Default" }).focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(c.getByRole("radio", { name: "Larger" })).toHaveAttribute(
      "aria-checked",
      "true",
    );
    await expect(saved(canvasElement).fontSize).toBe("larger");
  }}
/>
<Story
  name="Dark paired appearance"
  args={{ preferences: { mode: "dark", theme: "catppuccin-mocha" } }}
/>
