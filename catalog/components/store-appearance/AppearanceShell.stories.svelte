<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AppearanceShellFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Appearance/Shell",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
  async function chrome({ canvasElement }: { canvasElement: HTMLElement }) {
    const shell = canvasElement.querySelector<HTMLElement>(
      ".community-app-shell",
    )!;
    const header = within(canvasElement).getByRole("navigation", {
      name: "Application navigation",
    });
    await waitFor(() => {
      expect(getComputedStyle(header).backgroundImage).toBe("none");
      expect(getComputedStyle(header).backgroundColor).toBe("rgba(0, 0, 0, 0)");
      expect(getComputedStyle(shell).backgroundImage).toContain(
        "linear-gradient",
      );
    });
    // The body shell and transparent left rail reveal the same full-height image.
    const surface = shell.querySelector<HTMLElement>(
      ".community-app-shell__surface",
    )!;
    await expect(getComputedStyle(surface).backgroundImage).toBe("none");
    await expect(getComputedStyle(surface).backgroundColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
  }
  async function picker({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement),
      p = within(canvasElement.ownerDocument.body);
    const sample = canvasElement.querySelector(".appearance-shell-fixture")!;
    const rail = c.getByRole("complementary", {
      name: "Repository navigation sidebar",
    });
    const collapse = c.queryByRole("button", { name: "Collapse left sidebar" });
    if (collapse) await userEvent.click(collapse);
    await waitFor(() =>
      expect(rail).toHaveAttribute("data-state", "collapsed"),
    );
    const triggers = Array.from(
      rail.querySelectorAll<HTMLButtonElement>(
        ".lc-community-navigation__compact-section-trigger",
      ),
    );
    await expect(triggers.length).toBeGreaterThan(0);
    for (const trigger of triggers) {
      const label = trigger.getAttribute("aria-label")!;
      await userEvent.click(trigger);
      const input = await p.findByRole("combobox", {
        name: `${label} navigation`,
      });
      const popup = input.closest<HTMLElement>(
        ".lc-community-navigation__section-picker",
      )!;
      const command = popup.querySelector(
        '[data-ui-component="command-view"]',
      )!;
      await waitFor(() => {
        expect(
          popup.getBoundingClientRect().left -
            rail.getBoundingClientRect().right,
        ).toBeGreaterThanOrEqual(7.9);
        expect(getComputedStyle(popup).backgroundColor).toBe(
          getComputedStyle(sample).backgroundColor,
        );
        expect(getComputedStyle(command).backgroundColor).toBe(
          getComputedStyle(sample).backgroundColor,
        );
      });
      await expect(popup.getBoundingClientRect().right).toBeLessThanOrEqual(
        window.innerWidth - 7.9,
      );
      await userEvent.keyboard("{Escape}");
      await waitFor(() => expect(trigger).toHaveFocus());
    }
  }
  async function navigationHover({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const c = within(canvasElement);
    const collapse = c.queryByRole("button", { name: "Collapse left sidebar" });
    if (collapse) await userEvent.click(collapse);
    const rail = c.getByRole("complementary", {
      name: "Repository navigation sidebar",
    });
    await waitFor(() =>
      expect(rail).toHaveAttribute("data-state", "collapsed"),
    );
    const reference = rail.querySelector<HTMLButtonElement>(
      ".lc-community-navigation__item:not([data-active]), .lc-community-navigation__compact-section-trigger:not([data-active])",
    );
    const header = c.getByRole("navigation", {
      name: "Application navigation",
    });
    const search = within(rail).getByRole("button", { name: /^Search / });
    // A native browser pointer proves the actual CSS hover, including theme paint.
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const sample = canvasElement.querySelector(".navigation-hover-sample")!;
    const expected = getComputedStyle(sample).backgroundColor;
    const foreground = getComputedStyle(sample).color;
    if (reference) {
      await pointer.hover(reference);
      await waitFor(() =>
        expect(getComputedStyle(reference).backgroundColor).toBe(expected),
      );
    }
    const main = c.getByRole("main");
    async function verify(button: HTMLElement) {
      await pointer.hover(button);
      await waitFor(() => {
        expect(getComputedStyle(button).backgroundColor).toBe(expected);
        expect(getComputedStyle(button).color).toBe(foreground);
      });
      await pointer.hover(main);
      await userEvent.keyboard("{Tab}");
      button.focus();
      await waitFor(() => {
        expect(button).toHaveFocus();
        expect(getComputedStyle(button).backgroundColor).toBe(expected);
      });
      button.blur();
    }
    const branches = c.queryByRole("tab", { name: "Branches" });
    if (branches) await userEvent.click(branches);
    for (const button of header.querySelectorAll<HTMLButtonElement>(
      "button:not(:disabled)",
    ))
      await verify(button);
    await verify(search);
    const menuTrigger = within(header).queryByRole("button", {
      name: "Application menu",
    });
    if (menuTrigger && !menuTrigger.hasAttribute("disabled")) {
      await userEvent.click(menuTrigger);
      const menu = await within(canvasElement.ownerDocument.body).findByRole(
        "menu",
      );
      await pointer.hover(menu);
      await waitFor(() => {
        expect(menuTrigger).toHaveAttribute("data-state", "open");
        expect(menuTrigger.matches(":hover")).toBe(false);
        expect(getComputedStyle(menuTrigger).backgroundColor).toBe(expected);
      });
      await userEvent.keyboard("{Escape}");
      menuTrigger.blur();
      await pointer.hover(main);
      await waitFor(() => {
        expect(menuTrigger).toHaveAttribute("data-state", "closed");
        expect(getComputedStyle(menuTrigger).backgroundColor).toBe(
          "rgba(0, 0, 0, 0)",
        );
      });
    }
    if (branches) {
      await userEvent.click(c.getByRole("button", { name: /^Back$/ }));
      await verify(c.getByRole("button", { name: /^Forward$/ }));
    }
    await pointer.hover(main);
  }
</script>

<Story name="Buzz uses one continuous shell gradient" play={chrome} />
<Story
  name="Buzz dark uses one continuous shell gradient"
  args={{ theme: "buzz-dark", repository: true }}
  play={chrome}
/>
<Story
  name="Buzz navigation picker has app surface and rail clearance"
  play={picker}
/>
<Story
  name="Dark glass navigation picker retains an opaque app surface"
  args={{ theme: "buzz-dark", glass: true, repository: true }}
  play={picker}
/>
<Story
  name="Neutral navigation picker follows its app surface"
  args={{ theme: "github-light-default" }}
  play={picker}
/>
<Story
  name="Original navigation picker follows its app surface"
  args={{ theme: "original", repository: true }}
  play={picker}
/>
<Story
  name="Repository header and search hover match the Buzz sidebar"
  args={{ repository: true }}
  play={navigationHover}
/>
<Story
  name="Projects header and search hover match the Buzz sidebar"
  play={navigationHover}
/>
<Story
  name="Repository header and search hover match the dark sidebar"
  args={{ theme: "buzz-dark", repository: true }}
  play={navigationHover}
/>
<Story
  name="Repository header and search hover match the neutral sidebar"
  args={{ theme: "github-light-default", repository: true }}
  play={navigationHover}
/>
