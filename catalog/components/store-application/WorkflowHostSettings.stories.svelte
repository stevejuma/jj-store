<script module lang="ts">
  import { page } from "vitest/browser";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ServerSettingsFixture.svelte";
  import {
    admission,
    selectPrincipal,
  } from "../../fixtures/workflow-host-settings-play.js";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Workflow hosts",
    component: Fixture,
    args: { section: "workflow" },
    tags: ["visual-pending"],
  });
  async function jsonEditor(canvas: ReturnType<typeof within>) {
    await userEvent.click(
      canvas.getByRole("button", { name: "Add local host" }),
    );
    const json = canvas.getByRole("tab", { name: "JSON" });
    await expect(
      json.closest(
        ".store-settings__section-heading, .store-settings-card__header",
      ),
    ).not.toBeNull();
    await userEvent.click(json);
    await expect(
      canvas.queryByRole("switch", { name: "Enable local host" }),
    ).toBeNull();
    return await canvas.findByRole("textbox", {
      name: "Host configuration JSON",
    });
  }
  const validHost = {
    version: 1,
    host: "builder",
    principal: "02".repeat(16),
    home: "/private/home",
    state_dir: "/private/state",
    jj_executable: "/usr/local/jj",
    allowed_owners: ["02".repeat(16)],
    sources: [{ repository: "03".repeat(16), workspace: "/private/code" }],
    commands: { build: { program: "/bin/sh" } },
  };
  async function replaceJson(editor: HTMLElement, text: string) {
    await userEvent.click(editor);
    await userEvent.keyboard("{Control>}a{/Control}");
    await userEvent.paste(text);
  }
</script>

<Story
  name="Save inactive admission"
  play={async ({ canvasElement }) => {
    const { canvas, input } = await admission(canvasElement);
    await expect(input.getBoundingClientRect().width).toBeGreaterThan(200);
    await selectPrincipal(input);
    await userEvent.click(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    );
    await expect(
      await within(document.body).findByText(
        "Settings saved. Reload saved settings to apply changes.",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByText("Saved changes awaiting reload"),
    ).toBeVisible();
    await expect(canvas.getByText("0 principals")).toBeVisible();
  }}
/>
<Story
  name="Validation uses toast"
  play={async ({ canvasElement }) => {
    const { canvas } = await admission(canvasElement);
    const editor = await jsonEditor(canvas);
    await replaceJson(editor, "null");
    await userEvent.click(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    );
    const message = await within(document.body).findByText(
      "Host configuration must be an object. Your text is retained.",
    );
    await expect(message.closest("[data-notification-id]")).not.toBeNull();
    await expect(editor).toHaveTextContent("null");
    await expect(canvasElement.querySelector("form")).not.toContainElement(
      message,
    );
  }}
/>
<Story
  name="Malformed host configuration stays editable"
  play={async ({ canvasElement }) => {
    const { canvas } = await admission(canvasElement);
    const text = await jsonEditor(canvas);
    await replaceJson(text, "null");
    await userEvent.click(canvas.getByRole("tab", { name: "Form" }));
    const message = await within(document.body).findByText(
      "Host configuration must be an object. Your text is retained.",
    );
    await expect(message.closest("[data-notification-id]")).not.toBeNull();
    await expect(text).toHaveTextContent("null");
    await expect(canvas.getByRole("tab", { name: "JSON" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Cancel" }));
    await expect(
      canvas.getByRole("button", { name: "Add local host" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Unknown save retains original draft"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const { canvas, input } = await admission(canvasElement);
    await selectPrincipal(input);
    await userEvent.click(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    );
    const message = await within(document.body).findByText(
      "Save outcome unknown. Your original draft is retained; refresh to inspect saved settings.",
    );
    await expect(message.closest("[data-notification-id]")).not.toBeNull();
    await expect(input).toHaveTextContent("1 principals selected");
    await expect(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Loading layout"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("status", {
        name: "Loading workflow host settings",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Administrator required"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByText(
        "Workflow host settings require local administrator access.",
      ),
    ).toBeVisible();
  }}
/>

<Story
  name="Mobile host fields"
  play={async ({ canvasElement }) => {
    await page.viewport(390, 844);
    try {
      const { canvas } = await admission(canvasElement);
      await userEvent.click(
        canvas.getByRole("button", { name: "Add local host" }),
      );
      await expect(
        canvas.getByRole("textbox", { name: "Host name" }),
      ).toBeVisible();
      await expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(
        window.innerWidth,
      );
    } finally {
      await page.viewport(1280, 800);
    }
  }}
/>

<Story
  name="One shared reload dialog and focus restoration"
  play={async ({ canvasElement }) => {
    const { canvas, input } = await admission(canvasElement);
    await selectPrincipal(input);
    await userEvent.click(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    );
    const trigger = canvas.getByRole("button", {
      name: "Reload saved settings",
    });
    await userEvent.click(trigger);
    const dialog = await within(document.body).findByRole("dialog", {
      name: "Reload saved settings",
    });
    await waitFor(() => expect(dialog).toBeVisible());
    await expect(document.querySelectorAll('[role="dialog"]')).toHaveLength(1);
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Reload saved settings" }),
    );
    await waitFor(() =>
      expect(within(dialog).getByText("Applied")).toBeVisible(),
    );
    await userEvent.click(
      within(dialog).getByRole("button", {
        name: "Close configuration reload",
      }),
    );
    await waitFor(() => expect(trigger).toHaveFocus());
    await waitFor(() =>
      expect(canvas.getByText("Current", { exact: true })).toBeVisible(),
    );
  }}
/>

<Story
  name="Environment form derives repository context"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const addHost = await canvas.findByRole("button", {
      name: "Add local host",
    });
    await waitFor(() =>
      expect(getComputedStyle(addHost).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(addHost);
    await expect(
      canvas.getByRole("textbox", { name: "Host name" }),
    ).toHaveValue("local");
    for (const name of [
      "Metadata home",
      "Execution principal",
      "Source 1 repository",
    ])
      await expect(canvas.queryByRole("combobox", { name })).toBeNull();
    await expect(
      canvas.queryByRole("textbox", { name: "Recovery state directory" }),
    ).toBeNull();
    await expect(
      canvas.getByRole("button", {
        name: "Advanced runner settings",
      }),
    ).toHaveAttribute("aria-expanded", "false");
    const advanced = canvas
      .getByRole("button", { name: "Advanced runner settings" })
      .closest(".store-settings-disclosure")!.parentElement!;
    await expect(getComputedStyle(advanced).borderTopWidth).toBe("1px");
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Allowed agents" }),
    );
    const agent = await within(document.body).findByRole("option", {
      name: /Demo agent/,
    });
    await waitFor(() => expect(agent).toBeVisible());
    await expect(
      within(agent).getByRole("img", { name: /Demo agent/ }),
    ).toHaveTextContent("🤖");
    await userEvent.click(agent);
    await expect(
      within(agent).getByRole("img", { name: /Demo agent/ }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Advanced runner settings",
      }),
    );
    await expect(
      canvas.getByRole("textbox", { name: "Recovery state directory" }),
    ).toHaveValue("");
    await expect(
      canvas.getByRole("spinbutton", { name: "Polling interval" }),
    ).toHaveValue(1000);
    const gauge = canvas.getByRole("slider", {
      name: "Polling interval slider",
    });
    gauge.focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(
      canvas.getByRole("spinbutton", { name: "Polling interval" }),
    ).toHaveValue(1100);
    const status = canvas
      .getByRole("region", { name: "Workflow hosts" })
      .getBoundingClientRect();
    const environment = canvas
      .getByRole("region", { name: "Local execution host" })
      .getBoundingClientRect();
    await expect(environment.top - status.bottom).toBeGreaterThanOrEqual(16);
    await userEvent.click(canvas.getByRole("tab", { name: "JSON" }));
    const json = canvas.getByRole("textbox", {
      name: "Host configuration JSON",
    });
    await expect(json).toHaveTextContent('"version": 2');
    await expect(json).toHaveTextContent('"poll_millis": 1100');
    await expect(json).not.toHaveTextContent('"sources"');
    await expect(json).not.toHaveTextContent('"principal"');
    await expect(json).not.toHaveTextContent('"home"');
  }}
/>
<Story
  name="Header editor modes share one saved configuration"
  play={async ({ canvasElement }) => {
    const { canvas } = await admission(canvasElement);
    const editor = await jsonEditor(canvas);
    await replaceJson(editor, JSON.stringify(validHost, null, 2));
    await expect(editor.querySelector(".cm-line span")).not.toBeNull();
    await userEvent.click(canvas.getByRole("tab", { name: "Form" }));
    await expect(
      canvas.queryByRole("textbox", { name: "Host configuration JSON" }),
    ).toBeNull();
    const name = canvas.getByRole("textbox", { name: "Host name" });
    await expect(name).toHaveValue("builder");
    await userEvent.clear(name);
    await userEvent.type(name, "changed");
    await userEvent.click(canvas.getByRole("tab", { name: "JSON" }));
    const current = canvas.getByRole("textbox", {
      name: "Host configuration JSON",
    });
    await expect(current).toHaveTextContent('"host": "changed"');
    await replaceJson(
      current,
      JSON.stringify({ ...validHost, host: "saved-json" }, null, 2),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    );
    await expect(
      await within(document.body).findByText(
        "Settings saved. Reload saved settings to apply changes.",
      ),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Form" }));
    await expect(
      canvas.getByRole("textbox", { name: "Host name" }),
    ).toHaveValue("saved-json");
  }}
/>

<Story
  name="Profile lookup failure uses a toast and retains choices"
  args={{ lookupFailure: "profiles" }}
  play={async ({ canvasElement }) => {
    const { input } = await admission(canvasElement);
    await userEvent.click(input);
    await waitFor(() =>
      expect(
        within(document.body).getByRole("option", { name: /Steve/ }),
      ).toBeVisible(),
    );
    const message = await within(document.body).findByText(
      "Principal profiles could not be loaded. Your selection is retained.",
    );
    await expect(message.closest("[data-notification-id]")).not.toBeNull();
    await expect(canvasElement.querySelector("form")).not.toContainElement(
      message,
    );
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Forgejo environment is configurable with the existing form"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const addHost = await canvas.findByRole("button", {
      name: "Add local host",
    });
    await waitFor(() =>
      expect(getComputedStyle(addHost).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(addHost);
    await userEvent.click(
      await canvas.findByRole("switch", { name: "GitHub workflow executor" }),
    );
    const program = canvas.getByRole("textbox", {
      name: "Forgejo runner executable",
    });
    await userEvent.type(program, "/tools/forgejo-runner");
    await userEvent.type(
      canvas.getByRole("textbox", { name: "Docker executable" }),
      "/tools/docker",
    );
    await userEvent.type(
      canvas.getByRole("textbox", { name: "Docker endpoint" }),
      "unix:///tmp/docker.sock",
    );
    await expect(
      canvas.getByRole("textbox", { name: "Job image" }),
    ).toHaveValue("node:20-bullseye");
    await userEvent.click(
      canvas.getByRole("button", { name: "Save workflow settings" }),
    );
    await expect(
      await within(document.body).findByText(
        "Settings saved. Reload saved settings to apply changes.",
      ),
    ).toBeVisible();
    await expect(program).toHaveValue("/tools/forgejo-runner");
  }}
/>
