<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import Fixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Inline PR links",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
  async function verify({ canvasElement }: { canvasElement: HTMLElement }) {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const pr = await canvas.findByRole("region", {
      name: "Pull request for feature/api",
    });
    const link = within(pr).getByRole("link", { name: "#41" });
    await expect(link.closest("li")).not.toBeNull();
    await expect(
      link.querySelector('[data-link-site="GitHub"]'),
    ).not.toBeNull();
    await expect(link).not.toHaveAttribute("data-layout");
    await expect(link.getBoundingClientRect().height).toBeLessThan(28);
    await expect(pr.querySelectorAll(".store-link-card")).toHaveLength(0);
    await userEvent.click(
      within(pr).getByRole("button", { name: "Float PR details" }),
    );
    const floating = await page.findByRole("dialog", {
      name: "PR #42 · feature/api",
    });
    await expect(within(floating).getByRole("link", { name: "#41" })).toBe(
      link,
    );
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await expect(within(pr).getByRole("link", { name: "#41" })).toBe(link);
  }
</script>

<Story
  name="Stack navigation stays inline in PR details"
  args={{ colorMode: "light" }}
  play={verify}
/>
<Story
  name="Stack navigation in dark PR details"
  globals={{ colorMode: "dark" }}
  args={{ colorMode: "dark" }}
  play={verify}
/>
