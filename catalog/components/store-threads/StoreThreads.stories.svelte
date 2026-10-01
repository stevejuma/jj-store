<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Threads",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Channel navigation creation and members"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await expect(
      await c.findByRole("button", { name: "Browse threads" }),
    ).toBeVisible();
    await userEvent.click(
      (await c.findAllByRole("button", { name: "general" }))[0]!,
    );
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: /View thread contributors/ }),
    );
    await expect(
      c.getByRole("complementary", { name: "Thread contributors" }),
    ).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Close thread panel" }),
    );
    await userEvent.click(c.getByRole("button", { name: "All threads" }));
    await userEvent.click(c.getByRole("button", { name: "New thread" }));
    await userEvent.type(
      await body.findByRole("textbox", { name: "Thread name" }),
      "planning",
    );
    await userEvent.click(body.getByRole("button", { name: "Create thread" }));
    await expect(
      await c.findByText("planning", { selector: "[data-ui-part=room-title]" }),
    ).toBeVisible();
    await expect(
      await c.findByRole("button", { name: "planning" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Shared composer replies and exact reactions"
  args={{ selected: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    const input = await c.findByRole("textbox");
    await userEvent.click(input);
    await userEvent.type(input, "A message{Enter}");
    await expect(
      await c.findByText("A message", { exact: true }),
    ).toBeVisible();
    const message =
      c.getByText("A message", { exact: true }).closest("[data-message-id]") ??
      c.getByText("A message", { exact: true }).closest("article");
    await userEvent.hover(message!);
    await waitFor(() =>
      expect(message).toHaveAttribute("data-actions-visible"),
    );
    await userEvent.click(
      within(message as HTMLElement).getByRole("button", {
        name: "Reply in thread",
      }),
    );
    const replies = c.getByRole("complementary", { name: "Thread replies" });
    const replyInput = within(replies).getByRole("textbox");
    await userEvent.click(replyInput);
    await userEvent.type(replyInput, "A reply{Enter}");
    await expect(
      await within(replies).findByText("A reply", { exact: true }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh threads" }));
    await expect(
      await within(replies).findByText("A reply", { exact: true }),
    ).toBeVisible();
    const reply =
      within(replies)
        .getByText("A reply", { exact: true })
        .closest("[data-message-id]") ??
      within(replies).getByText("A reply", { exact: true }).closest("article");
    await userEvent.hover(reply!);
    await waitFor(() => expect(reply).toHaveAttribute("data-actions-visible"));
    await userEvent.click(
      within(reply as HTMLElement).getByRole("button", {
        name: "Add reaction",
      }),
    );
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    await expect(
      await within(reply as HTMLElement).findByRole("button", {
        name: "Lapis",
      }),
    ).toBeVisible();
    await userEvent.click(
      within(reply as HTMLElement).getByRole("button", { name: "Lapis" }),
    );
    await waitFor(() =>
      expect(
        within(reply as HTMLElement).queryByRole("button", { name: "Lapis" }),
      ).toBeNull(),
    );
  }}
/>
<Story
  name="Unknown action keeps its receipt identity"
  args={{ selected: true, unknownFirst: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    const input = await c.findByRole("textbox");
    await userEvent.click(input);
    await userEvent.type(input, "Confirmed once{Enter}");
    await expect(
      (await c.findAllByText(/action outcome is unknown/))[0]!,
    ).toBeVisible();
    await userEvent.click(
      c.getAllByRole("button", { name: "Check action status" })[0]!,
    );
    await expect(
      await within(c.getByRole("log", { name: "general messages" })).findByText(
        "Confirmed once",
        { exact: true },
      ),
    ).toBeVisible();
    await waitFor(() =>
      expect(
        c.queryAllByRole("button", { name: "Check action status" }),
      ).toHaveLength(0),
    );
    const dismiss = c.queryByRole("button", { name: "Dismiss" });
    if (dismiss) await userEvent.click(dismiss);
  }}
/>
