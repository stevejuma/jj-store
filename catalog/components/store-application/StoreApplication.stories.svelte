<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Shell",
    component: StoreApplication,
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: {
        story: { inline: false, iframeHeight: 700 },
        source: {
          code: '<script>\n  import { StoreApplication } from "./catalog/index.js";\n<\/script>\n<StoreApplication />',
        },
      },
    },
  });
</script>

<Story
  name="Default"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const bodyTitle = canvasElement.querySelector<HTMLElement>(
      ".store-application__title",
    )!;
    await expect(getComputedStyle(bodyTitle).position).toBe("absolute");
    await expect(bodyTitle.getBoundingClientRect().width).toBe(1);
    const mainBody = canvasElement.querySelector<HTMLElement>(
      '[data-ui-component="app-shell"][data-ui-part="body"]',
    )!;
    await expect(mainBody).toHaveAttribute("data-layout", "regions");
    await expect(getComputedStyle(mainBody).overflow).toBe("hidden");
    await expect(
      mainBody.querySelector(".ui-minimal-app-shell__body-scroll-area"),
    ).toBeNull();
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Expand left sidebar" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "JJ Log" }));
    await expect(
      canvas.getByText("jj-store · JJ Log", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Op Log" }));
    await expect(
      canvas.getByText("jj-store · Op Log", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByText("jj-store · JJ Log", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Forward" }));
    await expect(
      canvas.getByText("jj-store · Op Log", { exact: true }),
    ).toBeVisible();
  }}
/>
<Story
  name="Expanded"
  args={{ initiallyCollapsed: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Repositories" }));
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", { name: "native-demo" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Repositories" }));
    await userEvent.click(canvas.getByRole("button", { name: "native-demo" }));
    await expect(canvas.getByText("native-demo · Files")).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    await expect(
      canvas.getByRole("complementary", { name: "Repository details" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Close details" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("complementary", { name: "Repository details" }),
      ).not.toBeInTheDocument(),
    );
    const resize = canvas.getByRole("slider", {
      name: "Resize repository navigation sidebar",
    });
    const before = Number(resize.getAttribute("aria-valuenow"));
    resize.focus();
    await userEvent.keyboard("{ArrowRight}");
    await waitFor(() =>
      expect(Number(resize.getAttribute("aria-valuenow"))).toBeGreaterThan(
        before,
      ),
    );
  }}
/>
<Story
  name="Compact repository picker"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const trigger = canvas.getByRole("button", { name: "Repositories" });
    await userEvent.click(trigger);
    const body = within(canvasElement.ownerDocument.body);
    const search = await body.findByPlaceholderText("Search repositories");
    await userEvent.type(search, "native-demo");
    await expect(
      body.getByRole("option", { name: "native-demo" }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(trigger).toHaveFocus());
  }}
/>
<Story
  name="Mobile"
  args={{ displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Open navigation sidebar" }),
    );
    await expect(
      canvas.getByRole("navigation", { name: "Repository navigation" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "JJ Log" }));
    await expect(
      canvas.getByText("jj-store · JJ Log", { exact: true }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("navigation", { name: "Repository navigation" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Hover reveal"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const edge = canvas.getByRole("button", {
      name: "Preview repository navigation sidebar",
    });
    await userEvent.hover(edge);
    await waitFor(() => expect(edge).toHaveAttribute("aria-expanded", "true"));
    await expect(
      canvas.getByRole("button", { name: "native-demo" }),
    ).toBeVisible();
    await userEvent.unhover(edge);
    await userEvent.hover(
      canvas.getByText("jj-store · Branches", { exact: true }),
    );
    await waitFor(() => expect(edge).toHaveAttribute("aria-expanded", "false"));
  }}
/>
