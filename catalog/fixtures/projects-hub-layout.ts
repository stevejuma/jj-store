import { expect, userEvent, within } from "storybook/test";

export async function expectCommunityDisclosures(detail: HTMLElement) {
  for (const name of ["Description", "Subtasks", "Activity"]) {
    const trigger = within(detail).getByRole("button", { name });
    const label = trigger.querySelector("span")!;
    const chevron = trigger.querySelector("svg")!;
    await expect(getComputedStyle(trigger).fontWeight).toBe("500");
    await expect(getComputedStyle(trigger).paddingInlineStart).toBe("24px");
    await expect(
      chevron.getBoundingClientRect().left -
        label.getBoundingClientRect().right,
    ).toBeLessThan(12);
    await userEvent.click(trigger);
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(trigger);
  }
}

export async function expectTaskTitleRow(detail: HTMLElement) {
  const header = detail.querySelector("header")!;
  const refresh = within(header).getByRole("button", {
    name: "Refresh projects",
  });
  await expect(detail.firstElementChild).toBe(header);
  await expect(refresh.getBoundingClientRect().right).toBeCloseTo(
    header.getBoundingClientRect().right,
    0,
  );
  const title = within(header).getByRole("heading");
  await expect(refresh.getBoundingClientRect().top).toBeCloseTo(
    title.getBoundingClientRect().top,
    0,
  );
}

export async function expectReviewRowUsesWidth(
  row: HTMLElement,
  heading: HTMLElement,
) {
  const bounds = row.getBoundingClientRect();
  await expect(bounds.left).toBeCloseTo(
    heading.getBoundingClientRect().left,
    0,
  );
  const fields = [...row.children]
    .map((field) => field.getBoundingClientRect())
    .filter((bounds) => bounds.width > 0);
  await expect(bounds.right - fields.at(-1)!.right).toBeLessThan(12);
  for (let index = 1; index < fields.length; index++) {
    await expect(
      fields[index].left - fields[index - 1].right,
    ).toBeGreaterThanOrEqual(7);
  }
}
