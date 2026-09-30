<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreMutationCommitPickerFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Mutations/Commit picker",
    component: Fixture,
    tags: ["visual-pending"],
  });

  function center(element: Element) {
    const box = element.getBoundingClientRect();
    return box.top + box.height / 2;
  }
  async function inspectRow(row: Element) {
    const body = row.querySelector(".store-mutation-commit-target__body")!;
    const avatar = row.firstElementChild!;
    await expect(Math.abs(center(avatar) - center(body))).toBeLessThan(1);
    for (const chip of row.querySelectorAll(
      ".store-mutation-commit-target__ids .community-chip",
    )) {
      const box = chip.getBoundingClientRect();
      await expect(box.height).toBeLessThanOrEqual(16);
      await expect(
        parseFloat(getComputedStyle(chip).borderTopLeftRadius),
      ).toBeGreaterThanOrEqual(box.height / 2);
    }
  }
</script>

<Story
  name="Parent commit alignment"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    const trigger = canvas.getByRole("combobox", { name: "Parent commit" });
    const row = trigger.querySelector(".store-mutation-commit-target")!;
    await inspectRow(row);
    await expect(
      Math.abs(
        center(row.querySelector(".store-mutation-commit-target__body")!) -
          center(
            trigger.querySelector(".store-mutation-commit-picker__chevron")!,
          ),
      ),
    ).toBeLessThan(1);
    await userEvent.click(trigger);
    const options = await page.findAllByRole("option");
    await expect(options).toHaveLength(2);
    for (const option of options)
      await inspectRow(option.querySelector(".store-mutation-commit-target")!);
  }}
/>
