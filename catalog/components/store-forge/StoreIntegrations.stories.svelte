<script lang="ts" module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import IntegrationsFixture from "../../fixtures/IntegrationsFixture.svelte";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Integrations",
    component: IntegrationsFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.settings();
    },
  });
</script>

<Story
  name="GitHub account setup in global settings"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Add GitHub account" }),
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "Add Personal Access Token",
      }),
    );
    const input = canvas.getByLabelText("Personal Access Token", {
      exact: true,
    });
    await userEvent.type(input, "fixture-token");
    await expect(input).toHaveAttribute("type", "password");
    await userEvent.click(canvas.getByRole("button", { name: "Show token" }));
    await expect(input).toHaveAttribute("type", "text");
    await userEvent.click(canvas.getByRole("button", { name: "Add account" }));
    await expect(
      await canvas.findByText("octocat", { exact: true }),
    ).toBeVisible();
    await expect(input).not.toBeInTheDocument();
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "Credentials saved in your OS credential store.",
      ),
    ).toBeVisible();
  }}
/>
<Story
  name="Connected account and forget"
  args={{ connected: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Forget stevejuma pat account",
      }),
    );
    await expect(
      await canvas.findByRole("button", { name: "Add GitHub account" }),
    ).toBeEnabled();
    await expect(
      canvas.queryByText("stevejuma", { exact: true }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="GitHub device authorization"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Add GitHub account" }),
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "Authorize GitHub Account",
      }),
    );
    await expect(await canvas.findByText("ABCD-1234")).toBeVisible();
    await expect(
      canvas.getByRole("link", { name: /Open GitHub activation/ }),
    ).toHaveAttribute("href", "https://github.com/login/device");
    await userEvent.click(canvas.getByRole("button", { name: "Cancel" }));
    await expect(canvas.queryByText("ABCD-1234")).not.toBeInTheDocument();
  }}
/>
<Story
  name="Token failure preserves draft and shows toast"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Add GitHub account" }),
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitem", {
        name: "Add Personal Access Token",
      }),
    );
    await userEvent.type(
      canvas.getByLabelText("Personal Access Token", { exact: true }),
      "bad-token",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Add account" }));
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "Integration action failed",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByLabelText("Personal Access Token", { exact: true }),
    ).toHaveValue("bad-token");
  }}
/>
<Story
  name="Device authorization unavailable without client ID"
  args={{ oauth: false }}
/>
<Story
  name="Accounts in dark mode"
  args={{ connected: true, colorMode: "dark" }}
  globals={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByText("stevejuma", { exact: true });
    await expect(document.documentElement).toHaveClass("dark");
  }}
/>

<Story
  name="Enterprise account form in a narrow window"
  args={{ connected: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const browser =
      import.meta.env.MODE === "test"
        ? (await import("vitest/browser")).page
        : undefined;
    try {
      await browser?.viewport(640, 640);
      await userEvent.click(
        await canvas.findByRole("button", {
          name: "Add another GitHub account",
        }),
      );
      await userEvent.click(
        within(canvasElement.ownerDocument.body).getByRole("menuitem", {
          name: "Add GitHub Enterprise Account",
        }),
      );
      await expect(canvas.getByLabelText("API Base URL")).toBeVisible();
      const token = canvas.getByLabelText("Personal Access Token", {
        exact: true,
      });
      await expect(token).toHaveAttribute("type", "password");
      token.scrollIntoView({ block: "nearest", inline: "nearest" });
      await expect(token.getBoundingClientRect().width).toBeLessThanOrEqual(
        640,
      );
      await userEvent.click(canvas.getByRole("button", { name: /^Cancel$/ }));
    } finally {
      await browser?.viewport(1280, 800);
    }
  }}
/>
