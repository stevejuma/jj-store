<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    waitFor,
    within,
  } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Collaboration",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: true },
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: { story: { inline: false, iframeHeight: 700 } },
    },
  });
</script>

<Story
  name="Agent response activity"
  args={{ initialPage: "agents" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await waitFor(() =>
      expect(
        canvas.getByRole("list", { name: "Agent sessions" }),
      ).toBeVisible(),
    );
    const sessionCard = canvas.getByRole("button", { name: /Codex ended/u });
    const avatar = sessionCard.querySelector(
      '[data-ui-component="community-member-avatar"]',
    );
    if (!avatar) throw new Error("Agent session card avatar was not rendered");
    await expect(avatar).toBeVisible();
    await expect(sessionCard).toHaveTextContent("Started");
    await expect(sessionCard).toHaveTextContent("5m");
    await userEvent.click(sessionCard);
    await expect(
      canvas.getByRole("log", { name: "Agent conversation activity" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: /agent root/u }),
    ).toBeVisible();
    await expect(
      canvas.getByText(
        "Update the native parser and keep its review anchors exact.",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByText("Updated the parser and verified the focused contract."),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Show details for apply_patch" }),
    );
    await expect(
      canvas.getByRole("region", { name: "Changed file src/lib.rs" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: /tool call ID/u }),
    ).toBeVisible();
    const resize = canvas.getByRole("separator", {
      name: "Resize Agent sessions and Agent response columns",
    });
    const width = resize.getAttribute("aria-valuenow");
    await fireEvent.keyDown(resize, { key: "ArrowRight" });
    await waitFor(() =>
      expect(resize).not.toHaveAttribute("aria-valuenow", width),
    );
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Revision collaboration context"
  args={{ initialPage: "log" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Merge native storage and publication",
      }),
    );
    await expect(
      canvas.getByRole("button", { name: "Close Agent context column" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Close Review discussion column" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("link", { name: /Open related change/u }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    await expect(
      canvas.getByText("src/lib.rs", {
        selector: ".store-collaboration__detail-title",
      }),
    ).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Agent activity loading"
  args={{ initialPage: "agents", collaborationState: "loading" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("status", { name: "Loading agent sessions" }),
    ).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Review discussion empty"
  args={{ initialPage: "reviews", collaborationState: "empty" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("No review threads")).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Agent activity unavailable"
  args={{ initialPage: "agents", collaborationState: "failed" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("Agent activity unavailable")).toBeVisible();
    await expect(
      canvas.getAllByText("The collaboration snapshot could not be loaded.")[0],
    ).toBeVisible();
    await expect(canvas.getByText("No agent sessions")).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Review discussion actions"
  args={{ initialPage: "reviews" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await waitFor(() =>
      expect(
        canvas.getByRole("list", { name: "Review threads" }),
      ).toBeVisible(),
    );
    await userEvent.click(
      canvas.getAllByRole("button", { name: /Revision open/u })[0]!,
    );
    await expect(
      canvas.getByRole("list", { name: "Review comments" }),
    ).toBeVisible();
    await expect(
      canvas.getByText(
        "Can we preserve the exact operation when this retry races?",
      ),
    ).toBeVisible();
    const editor = canvas.getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(editor);
    await userEvent.type(editor, "The pinned root remains exact.");
    await userEvent.click(canvas.getByRole("button", { name: "Reply" }));
    await expect(
      canvas.getByText("The pinned root remains exact."),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Resolve" }));
    await expect(
      canvas.getAllByText("resolved", { exact: true }).at(-1),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("textbox", { name: "Markdown editor" }),
    ).not.toBeInTheDocument();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Review outcome unknown"
  args={{ initialPage: "reviews", reviewOutcome: "unknown" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getAllByRole("button", { name: /Revision open/u })[0]!,
    );
    await userEvent.click(canvas.getByRole("button", { name: "Resolve" }));
    await expect(canvas.getByText("Review outcome unknown")).toBeVisible();
    await expect(canvas.getByText(/outcome is unknown/u)).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Check status" }));
    await expect(canvas.getByText("Review updated")).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Check status" }),
    ).not.toBeInTheDocument();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>
