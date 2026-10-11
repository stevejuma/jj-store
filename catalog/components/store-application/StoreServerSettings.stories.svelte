<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import ServerSettingsFixture from "../../fixtures/ServerSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Server configuration",
    component: ServerSettingsFixture,
    tags: ["visual-pending"],
  });
  async function joinedTable({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const c = within(canvasElement);
    const save = await c.findByRole("button", { name: "Save settings" });
    if (c.queryByRole("button", { name: "Add upstream" }))
      await userEvent.click(c.getByRole("button", { name: "Add upstream" }));
    const form = save.closest("form")!;
    const rows = form.querySelectorAll(".store-settings__rows");
    await expect(rows).toHaveLength(1);
    await expect(rows[0]).toContainElement(c.getByText("Saved and running"));
    await expect(rows[0]).toContainElement(save);
    for (const action of form.querySelectorAll(".store-settings__action-row")) {
      await expect(action.parentElement).toBe(rows[0]);
      const style = getComputedStyle(action);
      await expect(style.borderTopWidth).toBe("1px");
      await expect(style.borderRightWidth).toBe("0px");
      await expect(style.borderBottomWidth).toBe("0px");
      await expect(style.borderLeftWidth).toBe("0px");
      await expect(style.borderRadius).toBe("0px");
    }
    if (c.queryByRole("textbox", { name: "Upstream origin" })) {
      await expect(rows[0]).toContainElement(
        c.getByText("Credential reference"),
      );
      await expect(rows[0]).toContainElement(
        c.getByRole("textbox", { name: "Upstream origin" }),
      );
    }
  }
</script>

<Story
  name="Joined service configuration table"
  args={{ section: "service" }}
  play={joinedTable}
/>
<Story
  name="Joined protection configuration table"
  args={{ section: "protection" }}
  play={joinedTable}
/>
<Story
  name="Joined sync configuration table"
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
      c.getByText("Saved changes need a server restart."),
    ).toBeVisible();
    const field = input.closest('[data-slot="field"]')!;
    await expect(field.getBoundingClientRect().left).toBeLessThan(
      input.getBoundingClientRect().left,
    );
    await expect(input.closest(".store-settings__rows")).not.toBeNull();
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
      c.getByText("Saved changes need a server restart."),
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
      c.getByText("Saved changes need a server restart."),
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
      c.getByText("Saved changes need a server restart."),
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
