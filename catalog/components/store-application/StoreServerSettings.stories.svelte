<script module lang="ts">
  import { joinedTable } from "./server-settings-story-actions.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import ServerSettingsFixture from "../../fixtures/ServerSettingsFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-settings/StoreSettingsPage.svelte",
      },
    },
    title: "JJ Store/Settings/Server configuration",
    component: ServerSettingsFixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story
  name="Global service configuration cards"
  args={{ section: "service" }}
  play={joinedTable}
/>
<Story
  name="Global protection configuration cards"
  args={{ section: "protection" }}
  play={joinedTable}
/>
<Story
  name="Shared sync configuration cards"
  args={{ section: "sync" }}
  play={joinedTable}
/>

<Story
  name="Service limits"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = await c.findByRole("spinbutton", {
      name: "Concurrent requests",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "3");
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeEnabled();
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a configuration reload."),
    ).toBeVisible();
    const field = input.closest('[data-slot="field"]')!;
    await expect(field.getBoundingClientRect().left).toBeLessThan(
      input.getBoundingClientRect().left,
    );
    await expect(input.closest(".store-settings-card")).not.toBeNull();
  }}
/>
<Story
  name="Installation protection"
  args={{ section: "protection" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = await c.findByRole("textbox", {
      name: "Protected bookmarks",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "main\nrelease");
    await expect(input).toHaveValue("main\nrelease");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a configuration reload."),
    ).toBeVisible();
  }}
/>
<Story
  name="Upstream configuration"
  args={{ section: "sync" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Add upstream" }),
    );
    await userEvent.type(
      c.getByRole("textbox", { name: "Upstream origin" }),
      "https://upstream.example/",
    );
    await expect(
      c.getByRole("spinbutton", { name: "Origins per batch" }),
    ).toHaveValue(128);
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a configuration reload."),
    ).toBeVisible();
  }}
/>
<Story
  name="Unknown response retains draft"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = await c.findByRole("spinbutton", {
      name: "Request timeout (seconds)",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "18");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(input).toHaveValue(18);
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Refresh settings" }));
    await userEvent.click(
      c.getByRole("button", { name: "Use latest saved settings" }),
    );
    await expect(input).toHaveValue(30);
  }}
/>
<Story
  name="Unavailable"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByText(
        "Configuration editing requires a local service with administrator access.",
      ),
    ).toBeVisible();
    await expect(c.queryByRole("button", { name: "Save settings" })).toBeNull();
  }}
/>

<Story
  name="Repository sync uses grouped cards and one shared save footer"
  args={{ section: "sync", embedded: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const add = await c.findByRole("button", { name: "Add upstream" });
    const status = c.getByRole("region", { name: "Current status" });
    const upstream = c.getByRole("region", { name: "Upstream configuration" });
    await expect(status).toContainElement(c.getByText("Credential reference"));
    await expect(upstream).toContainElement(add);
    await expect(add.closest(".store-settings-footer")).not.toBeNull();
    await userEvent.click(add);
    const input = c.getByRole("textbox", { name: "Upstream origin" });
    await userEvent.type(input, "https://peer.example.test");
    await expect(c.getByRole("region", { name: "Sync limits" })).toBeVisible();
    const save = c.getByRole("button", { name: "Save settings" });
    const cancel = c.getByRole("button", { name: "Cancel changes" });
    await expect(save.parentElement!.lastElementChild).toBe(save);
    await expect(save.closest("form")).toContainElement(input);
    await expect(
      save.closest("form")!.querySelectorAll('button[type="submit"]'),
    ).toHaveLength(1);
    await expect(canvasElement.querySelector("dl button")).toBeNull();
    await expect(cancel.getBoundingClientRect().right).toBeLessThanOrEqual(
      save.getBoundingClientRect().left,
    );
    await userEvent.click(cancel);
    await expect(c.getByRole("button", { name: "Add upstream" })).toBeVisible();
    await expect(
      c.queryByRole("textbox", { name: "Upstream origin" }),
    ).toBeNull();
  }}
/>

<Story
  name="Repository sync units retain numeric saves and invalid drafts"
  args={{ section: "sync", embedded: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      await c.findByRole("button", { name: "Add upstream" }),
    );
    await userEvent.type(
      c.getByRole("textbox", { name: "Upstream origin" }),
      "https://peer.example.test",
    );
    await userEvent.click(
      c.getByRole("button", { name: "Polling interval unit" }),
    );
    await userEvent.click(page.getByRole("option", { name: "sec" }));
    const polling = c.getByRole("spinbutton", { name: "Polling interval" });
    await waitFor(() =>
      expect(getComputedStyle(polling).pointerEvents).toBe("auto"),
    );
    await expect(polling).toHaveValue(5);
    await userEvent.clear(polling);
    await userEvent.type(polling, "2.5");
    await expect(c.getByText("Exact value: 2,500 ms")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a configuration reload."),
    ).toBeVisible();
    await userEvent.clear(polling);
    await userEvent.type(polling, "0.0001");
    await expect(polling).toHaveValue(0.0001);
    await expect(polling).toHaveAttribute("aria-invalid", "true");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(polling).toHaveValue(0.0001);
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(polling).toHaveValue(2.5);
    await userEvent.click(
      c.getByRole("button", { name: "Reset Polling interval to default" }),
    );
    await expect(polling).toHaveValue(5);
  }}
/>

<Story
  name="Repository sync loading retains the same configuration draft during refresh"
  args={{ section: "sync", embedded: true, loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByRole("status", { name: "Loading sync configuration" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    await userEvent.click(
      await c.findByRole("button", { name: "Add upstream" }),
    );
    const input = c.getByRole("textbox", { name: "Upstream origin" });
    await userEvent.clear(input);
    await userEvent.type(input, "https://retained.example");
    await userEvent.click(
      c.getByRole("button", { name: "Start background refresh" }),
    );
    await expect(input).toBeVisible();
    await expect(input).toHaveValue("https://retained.example");
    await expect(
      c.queryByRole("status", { name: "Loading sync configuration" }),
    ).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    await expect(input).toHaveValue("https://retained.example");
  }}
/>

<Story
  name="Global units preserve canonical service values and invalid drafts"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const save = await c.findByRole("button", { name: "Save settings" });
    await expect(save).toBeDisabled();
    await userEvent.click(
      c.getByRole("button", { name: "Maximum blob size (bytes) unit" }),
    );
    await userEvent.click(page.getByRole("option", { name: /^MiB$/ }));
    const input = c.getByRole("spinbutton", {
      name: "Maximum blob size (bytes)",
    });
    await waitFor(() =>
      expect(getComputedStyle(input).pointerEvents).toBe("auto"),
    );
    await expect(input).toHaveValue(32);
    await expect(save).toBeDisabled();
    await userEvent.clear(input);
    await userEvent.type(input, "16");
    await userEvent.click(save);
    await expect(c.getByText("Running: 33,554,432")).toBeVisible();
    await expect(
      c.getByText("Saved changes need a configuration reload."),
    ).toBeVisible();
    await userEvent.clear(input);
    await userEvent.type(input, "0.0000001");
    await expect(input).toHaveAttribute("aria-invalid", "true");
    await userEvent.click(save);
    await expect(input).toHaveValue(0.0000001);
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(input).toHaveValue(16);
    await expect(
      c.queryByRole("button", { name: /Reset Maximum blob/ }),
    ).toBeNull();
  }}
/>
<Story
  name="Global configuration skeleton matches numeric cards"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const loading = await c.findByRole("status", {
      name: "Loading service configuration",
    });
    await expect(
      loading.querySelectorAll(
        '[data-kind="numeric"].store-settings-skeleton__row',
      ),
    ).toHaveLength(4);
    await expect(c.queryByRole("spinbutton")).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    await expect(
      await c.findByRole("spinbutton", { name: "Concurrent requests" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Launch inventory groups paths and retains exact copy values"
  args={{ inventorySection: "service" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const files = await c.findByRole("region", { name: "Installation files" });
    await expect(
      c.getByRole("region", { name: "Server and network" }),
    ).toBeVisible();
    await expect(
      c.getByRole("region", { name: "Identity and access" }),
    ).toBeVisible();
    const copy = within(files).getByRole("button", {
      name: "Copy full State directory: /private/example/local",
    });
    await expect(
      copy
        .closest(".store-readonly-row")!
        .querySelector(".store-readonly-row__icon svg"),
    ).not.toBeNull();
    const original = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      await userEvent.click(copy);
      await expect(write).toHaveBeenCalledWith("/private/example/local");
      await expect(copy).toHaveAttribute("data-copied", "true");
    } finally {
      if (original) Object.defineProperty(navigator, "clipboard", original);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
    await expect(copy).toHaveAttribute(
      "aria-label",
      "Copy full State directory: /private/example/local",
    );
    await expect(
      within(files).getByText("State directory copied"),
    ).toBeInTheDocument();
    await expect(c.queryByRole("button", { name: /Copy full Tls/ })).toBeNull();
  }}
/>
<Story
  name="Launch inventory omits workflow host rows owned by Workflows"
  args={{ inventorySection: "service" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByRole("region", { name: "Server and network" }),
    ).toBeVisible();
    await expect(c.queryAllByText(/^Workflow hosts?(?: \/|\[|$)/)).toHaveLength(
      0,
    );
    await expect(
      c.getByRole("region", { name: "Installation files" }),
    ).toBeVisible();
    await expect(
      c.getByRole("region", { name: "Identity and access" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Storage inventory separates connection paths credentials and backup"
  args={{ inventorySection: "storage" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    for (const name of [
      "Storage connection",
      "Runtime and data",
      "Storage identity and credentials",
      "Backup inventory",
    ])
      await expect(await c.findByRole("region", { name })).toBeVisible();
    const runtime = c.getByRole("region", { name: "Runtime and data" });
    await expect(
      within(runtime).getByRole("button", {
        name: "Copy full Volume: /private/example/rustfs",
      }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: /Copy full Backup/ }),
    ).toBeNull();
  }}
/>
