import { expect, userEvent, waitFor, within } from "storybook/test";
import { reader } from "./acceptance-helpers.js";

export async function readPanelResize({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await reader(canvasElement);
  await userEvent.click(
    c.getByRole("button", { name: "Expand Read a file, ran a command" }),
  );
  const opener = c.getByRole("button", {
    name: "Show details for Read example.ts",
  });
  await userEvent.click(opener);
  const pane = c.getByRole("complementary", { name: "Read tool details" });
  const handle = c.getByRole("slider", { name: "Resize read tool details" });
  const width = () => pane.getBoundingClientRect().width;
  const initial = width();
  const main = canvasElement.querySelector<HTMLElement>(
    '[data-ui-part="main"]',
  )!;
  expect(getComputedStyle(main).borderBottomWidth).toBe("1px");
  expect(main.getBoundingClientRect().bottom).toBeLessThanOrEqual(
    canvasElement.ownerDocument.documentElement.clientHeight,
  );
  expect(pane.getBoundingClientRect().top).toBe(
    main.getBoundingClientRect().top,
  );
  expect(pane.getBoundingClientRect().bottom).toBe(
    main.getBoundingClientRect().bottom,
  );
  expect(pane.getBoundingClientRect().left).toBe(
    main.getBoundingClientRect().right,
  );
  handle.focus();
  await userEvent.keyboard("{ArrowLeft}");
  await waitFor(() => expect(width()).toBeCloseTo(initial + 16, 0));
  await expect(handle).toHaveFocus();
  await userEvent.keyboard("{ArrowRight}");
  await waitFor(() => expect(width()).toBeCloseTo(initial, 0));
  await userEvent.keyboard("{Home}");
  await waitFor(() => expect(width()).toBe(280));
  await userEvent.keyboard("{End}");
  await waitFor(() => expect(width()).toBe(720));
  await userEvent.keyboard("{ArrowLeft}");
  expect(width()).toBe(720);
  await userEvent.keyboard("{Home}");
  await waitFor(() => expect(width()).toBe(280));
  await userEvent.keyboard("{ArrowLeft}{ArrowLeft}");
  const resized = width();
  await userEvent.click(
    c.getByRole("button", { name: "Close read tool details" }),
  );
  await expect(opener).toHaveFocus();
  expect(
    c.queryByRole("slider", { name: "Resize read tool details" }),
  ).toBeNull();
  await userEvent.click(opener);
  await waitFor(() =>
    expect(
      c
        .getByRole("complementary", { name: "Read tool details" })
        .getBoundingClientRect().width,
    ).toBe(resized),
  );
  expect(c.queryByText("Captured read")).toBeNull();
  expect(
    getComputedStyle(canvasElement.querySelector(".chat-body-fixture-header")!)
      .borderBottomWidth,
  ).toBe("0px");
}
