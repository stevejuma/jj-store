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
    await expect(options).toHaveLength(3);
    for (const option of options)
      await inspectRow(option.querySelector(".store-mutation-commit-target")!);
  }}
/>

<Story
  name="Aligned identifiers and bottom time"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    const reference = canvas
      .getByTestId("copy-id-reference")
      .querySelector(".community-chip")!;
    const referenceSize = parseFloat(getComputedStyle(reference).fontSize);
    const trigger = canvas.getByRole("combobox", { name: "Parent commit" });
    async function inspectMetadata(row: Element) {
      const primary = row.querySelector(
        ".store-mutation-commit-target__primary",
      )!;
      const time = row.querySelector("time")!;
      await expect(time.getBoundingClientRect().top).toBeGreaterThanOrEqual(
        primary.getBoundingClientRect().bottom,
      );
      await expect(
        Math.abs(
          time.getBoundingClientRect().right -
            row.getBoundingClientRect().right,
        ),
      ).toBeLessThan(1);
      for (const chip of row.querySelectorAll(
        ".store-mutation-commit-target__ids .community-chip",
      )) {
        const size = parseFloat(getComputedStyle(chip).fontSize);
        await expect(size).toBeGreaterThan(referenceSize);
        await expect(size - referenceSize).toBeLessThanOrEqual(2);
      }
    }
    await inspectMetadata(
      trigger.querySelector(".store-mutation-commit-target")!,
    );
    await userEvent.click(trigger);
    const options = await page.findAllByRole("option");
    const positions = new Map<string, number>();
    for (const option of options) {
      const row = option.querySelector(".store-mutation-commit-target")!;
      await inspectMetadata(row);
      for (const chip of row.querySelectorAll(
        ".store-mutation-commit-target__ids .community-chip",
      )) {
        const name = chip.textContent!.trim();
        const left = chip.getBoundingClientRect().left;
        if (positions.has(name))
          await expect(Math.abs(left - positions.get(name)!)).toBeLessThan(1);
        else positions.set(name, left);
      }
    }
    await expect(positions.size).toBe(2);
    await userEvent.click(options[1]!);
    await expect(trigger).toHaveTextContent("A much longer title");
    await inspectMetadata(
      trigger.querySelector(".store-mutation-commit-target")!,
    );
  }}
/>
