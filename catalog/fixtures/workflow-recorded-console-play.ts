import { expect, userEvent, within, waitFor } from "storybook/test";
export const verifyRecordedConsole = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const canvas = within(canvasElement);
  await waitFor(async () => {
    await expect(
      await canvas.findByRole("region", { name: "Step console" }),
    ).toBeVisible();
  });
  const step = await canvas.findByRole("button", {
    name: "Step build · completed",
  });
  await expect(step).toHaveAttribute("aria-expanded", "false");
  const console = canvas.getByRole("region", { name: "Step console" });
  await expect(getComputedStyle(console).backgroundColor).toBe(
    "rgb(31, 33, 43)",
  );
  await expect(
    console.querySelectorAll(".workflow-console-step[data-step]"),
  ).toHaveLength(1);
  const gear = canvas.getByRole("button", { name: "Log settings" });
  await expect(gear.getBoundingClientRect().right).toBeLessThanOrEqual(
    console.getBoundingClientRect().right,
  );
  await expect(
    canvas.getByRole("combobox", { name: "Run attempt" }),
  ).toHaveTextContent(/Run attempt #1 \(.+ ago\)/);
  await userEvent.click(canvas.getByRole("button", { name: "Run output" }));
  await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
    "12 tests passed",
  );
};
