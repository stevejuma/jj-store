<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/DiscussionSheetFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Discussion sheets",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Multiple Split drafts use one Focus sheet"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const openFirst = c.getByRole("button", { name: "Open discussion 1" });
    await userEvent.click(openFirst);
    await userEvent.type(
      c.getByRole("textbox", { name: "Draft 1" }),
      "First draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Open discussion 2" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Draft 2" }),
      "Second draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Focus layout" }));
    const sheet = await body.findByRole("region", { name: "Discussion 2" });
    await expect(
      document.querySelectorAll('[data-ui-part="surface-layer-surface"]'),
    ).toHaveLength(1);
    await expect(within(sheet).getByRole("textbox")).toHaveValue(
      "Second draft",
    );
    const main = canvasElement.querySelector('[data-ui-part="main"]')!;
    const rail = canvasElement.querySelector(
      '[data-mobile-panel-id="repository-details"]',
    )!;
    await waitFor(() => {
      expect(
        sheet.getBoundingClientRect().left - main.getBoundingClientRect().left,
      ).toBeCloseTo(112, 0);
      expect(sheet.getBoundingClientRect().right).toBeCloseTo(
        rail.getBoundingClientRect().right,
        0,
      );
    });
    await userEvent.keyboard("{Escape}");
    const previous = await body.findByRole("region", { name: "Discussion 1" });
    await expect(
      document.querySelectorAll('[data-ui-part="surface-layer-surface"]'),
    ).toHaveLength(1);
    await expect(within(previous).getByRole("textbox")).toHaveValue(
      "First draft",
    );
    const scrim = document.querySelector<HTMLElement>(
      '[data-ui-part="surface-layer-scrim"]',
    )!;
    await userEvent.click(scrim);
    await waitFor(() =>
      expect(body.queryByRole("region", { name: "Discussion 1" })).toBeNull(),
    );
    await waitFor(() => expect(openFirst).toHaveFocus());
    await userEvent.click(c.getByRole("button", { name: "Split layout" }));
    await userEvent.click(openFirst);
    await expect(c.getByRole("textbox", { name: "Draft 1" })).toHaveValue(
      "First draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Open discussion 2" }));
    await expect(c.getByRole("textbox", { name: "Draft 2" })).toHaveValue(
      "Second draft",
    );
  }}
/>

<Story
  name="Mobile Focus uses the root sheet"
  args={{ mobile: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "Focus layout" }));
    const opener = c.getByRole("button", { name: "Open discussion 1" });
    await userEvent.click(opener);
    const sheet = await body.findByRole("region", { name: "Discussion 1" });
    const layer = sheet.closest('[data-ui-part="surface-layer"]')!;
    await expect(layer).toHaveAttribute("data-mobile-placement", "root");
    const root = layer.parentElement!;
    await waitFor(() => {
      expect(
        sheet.getBoundingClientRect().left - root.getBoundingClientRect().left,
      ).toBeCloseTo(8, 0);
      expect(
        root.getBoundingClientRect().right -
          sheet.getBoundingClientRect().right,
      ).toBeCloseTo(8, 0);
    });
    await userEvent.type(within(sheet).getByRole("textbox"), "Mobile draft");
    await userEvent.click(
      within(sheet).getByRole("button", { name: "Close Discussion 1" }),
    );
    await waitFor(() => expect(opener).toHaveFocus());
    await userEvent.click(opener);
    await expect(
      within(
        await body.findByRole("region", { name: "Discussion 1" }),
      ).getByRole("textbox"),
    ).toHaveValue("Mobile draft");
  }}
/>
