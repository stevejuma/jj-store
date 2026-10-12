<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page } from "vitest/browser";
  import StoreServerSwitcher from "../../../web/src/lib/ui/store-connections/StoreServerSwitcher.svelte";
  import Fixture from "../../fixtures/ServerConnectionsFixture.svelte";
  const { Story } = defineMeta({
    title: "Store/Server connections",
    component: StoreServerSwitcher,
    tags: ["autodocs", "visual-pending"],
    parameters: {
      docs: { source: { code: "<StoreServerSwitcher compact />" } },
    },
  });
</script>

<Story
  asChild
  name="Expanded"
  args={{ compact: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getAllByRole("button", { name: /^Select .* server$/ }),
    ).toHaveLength(2);
    await expect(
      canvas.queryByRole("button", { name: "Select Research server" }),
    ).not.toBeInTheDocument();
    const height = canvas
      .getByRole("button", { name: "Select Local server" })
      .getBoundingClientRect().height;
    await expect(
      canvas.getByRole("button", { name: "Add server" }).getBoundingClientRect()
        .height,
    ).toBe(height);
    await expect(
      canvas
        .getByRole("button", { name: "More servers" })
        .getBoundingClientRect().height,
    ).toBe(height);
    await userEvent.click(
      canvas.getByRole("button", { name: "Select Build host server" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Select Build host server" }),
    ).toHaveAttribute("aria-pressed", "true");
    await expect(canvas.getByText("Selected Build host")).toBeVisible();
  }}>{#snippet children()}<Fixture />{/snippet}</Story
>
<Story
  asChild
  name="Compact keyboard"
  args={{ compact: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const more = canvas.getByRole("button", { name: "More servers" });
    more.focus();
    await userEvent.keyboard("{Enter}");
    const research = within(document.body).getByRole("menuitemradio", {
      name: "Research",
    });
    research.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      canvas.getByRole("button", { name: "Select Research server" }),
    ).toHaveAttribute("aria-pressed", "true");
    await expect(more).toHaveFocus();
    await userEvent.click(
      canvas.getByRole("button", { name: "Manage selected server" }),
    );
    await expect(
      within(document.body).getByRole("dialog", { name: "Server connection" }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
  }}>{#snippet children()}<Fixture compact />{/snippet}</Story
>
<Story
  asChild
  name="Connection dialog"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Add server" }));
    const page = within(document.body);
    const description = page.getByText(
      "Connect to an authenticated JJ Store server. Its token stays in this application session.",
    );
    await expect(description.closest('[role="region"]')).toHaveAccessibleName(
      "Connection details",
    );
    await expect(
      page
        .getByRole("dialog", { name: "Add server" })
        .querySelector('[data-slot="dialog-description"]'),
    ).toBeNull();
    const dialog = page.getByRole("dialog", { name: "Add server" });
    const header = dialog.querySelector(
      '[data-ui-part="dialog-frame-header"]',
    )!;
    const card = page.getByRole("region", { name: "Connection details" });
    await expect(
      card.getBoundingClientRect().top - header.getBoundingClientRect().bottom,
    ).toBe(4);
    await expect(getComputedStyle(header).paddingBottom).toBe("4px");
    const viewport = dialog.querySelector(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await expect(
      card.getBoundingClientRect().top - viewport.getBoundingClientRect().top,
    ).toBeGreaterThan(1);
    await expect(
      page.getByRole("button", { name: /^Connect$/ }),
    ).toBeDisabled();
    await userEvent.type(page.getByRole("textbox", { name: "Name" }), "Worker");
    await userEvent.type(
      page.getByRole("textbox", { name: "Server address" }),
      "https://worker.example",
    );
    await userEvent.type(
      page.getByLabelText("Access token"),
      "session-only-credential",
    );
    await userEvent.click(page.getByRole("button", { name: /^Connect$/ }));
    await expect(canvas.getByText("Connected Worker")).toBeVisible();
  }}>{#snippet children()}<Fixture />{/snippet}</Story
>
<Story
  asChild
  name="Mobile"
  play={async ({ canvasElement }) => {
    await page.viewport(390, 844);
    try {
      await userEvent.click(
        within(canvasElement).getByRole("button", { name: "Add server" }),
      );
      const dialog = within(document.body).getByRole("dialog", {
        name: "Add server",
      });
      const input = within(dialog).getByRole("textbox", { name: "Name" });
      await expect(input).toBeVisible();
      const label = within(dialog).getByText(
        "A display name for this sidebar.",
      );
      await expect(input.getBoundingClientRect().top).toBeGreaterThanOrEqual(
        label.getBoundingClientRect().bottom,
      );
      await expect(dialog.scrollWidth).toBeLessThanOrEqual(dialog.clientWidth);
      await userEvent.keyboard("{Escape}");
    } finally {
      await page.viewport(1280, 800);
    }
  }}
  >{#snippet children()}<div style="width:320px">
      <Fixture />
    </div>{/snippet}</Story
>
<Story
  asChild
  name="Local only"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getAllByRole("button", { name: /^Select .* server$/ }),
    ).toHaveLength(1);
    await expect(
      canvas.queryByRole("button", { name: "More servers" }),
    ).not.toBeInTheDocument();
  }}>{#snippet children()}<Fixture remoteCount={0} />{/snippet}</Story
>
<Story
  asChild
  name="Two servers without overflow"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getAllByRole("button", { name: /^Select .* server$/ }),
    ).toHaveLength(2);
    await expect(
      canvas.queryByRole("button", { name: "More servers" }),
    ).not.toBeInTheDocument();
  }}>{#snippet children()}<Fixture remoteCount={1} />{/snippet}</Story
>
<Story
  asChild
  name="Overflow selection and management"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(canvas.getByRole("button", { name: "More servers" }));
    await expect(body.getAllByRole("menuitemradio")).toHaveLength(3);
    await userEvent.click(body.getByRole("menuitemradio", { name: "Archive" }));
    await expect(
      canvas.getByRole("button", { name: "Select Archive server" }),
    ).toHaveAttribute("aria-pressed", "true");
    await expect(
      canvas.getAllByRole("button", { name: /^Select .* server$/ }),
    ).toHaveLength(2);
    await userEvent.click(
      canvas.getByRole("button", { name: "Manage Archive connection" }),
    );
    await expect(body.getByRole("textbox", { name: "Name" })).toHaveValue(
      "Archive",
    );
    await expect(
      body.getByRole("textbox", { name: "Server address" }),
    ).toBeDisabled();
    await userEvent.click(
      body.getByRole("button", { name: "Close server connection" }),
    );
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Select Local server" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Select Local server" }),
    ).toHaveAttribute("aria-pressed", "true");
  }}>{#snippet children()}<Fixture remoteCount={4} />{/snippet}</Story
>
<Story
  asChild
  name="Selected long server name in dark theme"
  globals={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const selected = canvas.getByRole("button", {
      name: "Select Shared compute server with a very long display name server",
    });
    await expect(selected).toHaveAttribute("aria-pressed", "true");
    await expect(selected.scrollWidth).toBeLessThanOrEqual(
      selected.clientWidth,
    );
  }}
  >{#snippet children()}<Fixture
      remoteCount={4}
      selectedServer={"1".repeat(32)}
    />{/snippet}</Story
>
<Story
  asChild
  name="Busy dialog"
  play={async () => {
    const dialog = within(document.body).getByRole("dialog", {
      name: "Add server",
    });
    await expect(
      within(dialog).getByRole("textbox", { name: "Name" }),
    ).toBeDisabled();
    await expect(
      within(dialog).getByRole("button", { name: /Connect$/ }),
    ).toBeDisabled();
    await expect(
      within(dialog).getByRole("button", { name: "Close server connection" }),
    ).toBeDisabled();
  }}>{#snippet children()}<Fixture dialogOpen busy />{/snippet}</Story
>
<Story
  asChild
  name="Overflow sign in preserves dialog focus"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "More servers" }));
    await userEvent.click(
      body.getByRole("menuitemradio", { name: "Research" }),
    );
    const dialog = await body.findByRole("dialog", {
      name: "Server connection",
    });
    await waitFor(() =>
      expect(dialog.contains(document.activeElement)).toBe(true),
    );
    await expect(body.getByRole("textbox", { name: "Name" })).toHaveValue(
      "Research",
    );
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        canvas.getByRole("button", { name: "More servers" }),
      ).toHaveFocus(),
    );
  }}>{#snippet children()}<Fixture requireSignIn />{/snippet}</Story
>
<Story
  asChild
  name="Overflow obeys pending command guard"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "More servers" }));
    await userEvent.click(
      body.getByRole("menuitemradio", { name: "Research" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Select Local server" }),
    ).toHaveAttribute("aria-pressed", "true");
    await expect(
      canvas.queryByRole("button", { name: "Select Research server" }),
    ).not.toBeInTheDocument();
  }}>{#snippet children()}<Fixture blockSwitching />{/snippet}</Story
>
<Story
  asChild
  name="Failed authentication retains draft and clears token"
  play={async ({ canvasElement }) => {
    const body = within(document.body);
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(
      within(canvasElement).getByRole("button", { name: "Add server" }),
    );
    await userEvent.type(body.getByRole("textbox", { name: "Name" }), "Worker");
    await userEvent.type(
      body.getByRole("textbox", { name: "Server address" }),
      "https://worker.example",
    );
    await userEvent.type(body.getByLabelText("Access token"), "fixture-token");
    await userEvent.click(body.getByRole("button", { name: "Connect" }));
    await expect(body.getByRole("alert")).toHaveTextContent(
      "authentication failed",
    );
    await expect(body.getByRole("textbox", { name: "Name" })).toHaveValue(
      "Worker",
    );
    await expect(body.getByLabelText("Access token")).toHaveValue("");
  }}>{#snippet children()}<Fixture connectFails />{/snippet}</Story
>
