import { page } from "vitest/browser";
import { expect, userEvent, waitFor, within } from "storybook/test";
export async function pendingReaction(
  { canvasElement }: { canvasElement: HTMLElement },
  reply = false,
) {
  const c = within(canvasElement),
    body = within(document.body);
  const mainLog = await c.findByRole("log", { name: "general messages" });
  await within(mainLog).findByText("Welcome to the channel");
  const message = reply
    ? (await c.findByText("A retained reply")).closest("article")!
    : within(mainLog).getByText("Welcome to the channel").closest("article")!;
  await page
    .elementLocator(message.querySelector(".chat-text-block") ?? message)
    .hover();
  await waitFor(() =>
    expect(
      within(message as HTMLElement).getAllByRole("button", {
        name: /More message actions|Add reaction/,
      })[0]!,
    ).toBeVisible(),
  );
  await userEvent.click(
    within(message).getByRole("button", {
      name: /^Add reaction(?: to|$)/,
    }),
  );
  await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
  await waitFor(() =>
    expect(
      message.querySelector(
        '[data-ui-component="ai-chat-message-metadata"] [role="status"]',
      ),
    ).toHaveTextContent("Awaiting reaction confirmation"),
  );
  const status = within(message).getByRole("status");
  const metadataRow = status.closest(".community-message__metadata");
  await expect(metadataRow).not.toBeNull();
  await expect(
    status.querySelector('[data-ui-component="spinner"]'),
  ).not.toBeNull();
  if (!reply) {
    const replies = within(message).getByRole("button", { name: "1 reply" });
    await expect(metadataRow).toContainElement(replies);
    await expect(
      replies.compareDocumentPosition(status) &
        Node.DOCUMENT_POSITION_FOLLOWING,
    ).toBeTruthy();
    const time = metadataRow!.querySelector('[data-ui-part="timestamp"]')!;
    await expect(
      Math.abs(
        time.getBoundingClientRect().top - status.getBoundingClientRect().top,
      ),
    ).toBeLessThan(8);
  }
  await expect(
    canvasElement.querySelector('.store-threads > [data-ui-component="alert"]'),
  ).toBeNull();
  await expect(message).toHaveTextContent(
    reply ? "A retained reply" : "Welcome to the channel",
  );
  if (reply)
    await expect(
      c
        .getByRole("log", { name: "general messages" })
        .querySelector(
          '[data-ui-component="ai-chat-message-metadata"] [role="status"]',
        ),
    ).toBeNull();
  await userEvent.click(
    within(message).getByRole("button", {
      name: "Check action status",
    }),
  );
  await waitFor(() =>
    expect(
      within(message).queryByText(/Awaiting reaction confirmation/),
    ).toBeNull(),
  );
  await expect(
    await within(message).findByRole("button", {
      name: /1 reaction with Lapis/,
    }),
  ).toBeVisible();
}
