<script module lang="ts">
  import { expectPersistentRepositoryLayout } from "./repository-layout-story-assertions.js";
  import { SOURCE_TEXT } from "../../fixtures/repository.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import LayoutPersistenceFixture from "../../fixtures/LayoutPersistenceFixture.svelte";
  import { IDS } from "../../fixtures/repository.js";
  import { expectSourceHighlight } from "./source-highlight-assertions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Connected application",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: {
        story: { inline: false, iframeHeight: 700 },
        source: {
          code: '<script>\n import { StoreRepositoryApplication, createInMemoryStoreController } from "./catalog/index.js";\n const controller = createInMemoryStoreController();\n<\/script>\n<StoreRepositoryApplication {controller} />',
        },
      },
    },
  });
</script>

<Story
  name="Persistent layout"
  play={async ({ canvasElement }) =>
    expectPersistentRepositoryLayout(canvasElement)}
>
  {#snippet template()}<LayoutPersistenceFixture />{/snippet}
</Story>

<Story
  name="Repository files"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(canvas.getByRole("button", { name: "jj-store" }));
    const fileSummary = within(
      canvasElement.querySelector<HTMLElement>(
        ".community-projects__file-browser-summary",
      )!,
    );
    await expect(
      fileSummary.getByRole("img", { name: "maya@example.test" }),
    ).toBeVisible();
    await expect(
      fileSummary.getByRole("button", { name: "View commit Working copy" }),
    ).toBeVisible();
    await expect(
      fileSummary.getByRole("button", { name: /Copy full Git revision/u }),
    ).toBeVisible();
    await expect(fileSummary.getByText(/seconds ago|just now/u)).toBeVisible();
    await expect(
      canvas.getByRole("list", { name: "Repository breadcrumb" }),
    ).toBeVisible();
    const landingPreview = canvas.getByRole("region", {
      name: "Repository document preview",
    });
    await expect(
      within(landingPreview).getByRole("tab", { name: "README" }),
    ).toBeVisible();
    const readmePreview = within(landingPreview).getByRole("region", {
      name: "README preview",
    });
    await expect(
      within(readmePreview).getByRole("heading", {
        name: "jj-store",
        level: 1,
      }),
    ).toBeVisible();
    // Shared safe Markdown omits executable HTML; Source below retains the bytes.
    await expect(readmePreview).not.toHaveTextContent(SOURCE_TEXT);
    await expect(
      readmePreview.querySelector(".mira-markdown-preview"),
    ).not.toBeNull();
    await expect(readmePreview.querySelector("script")).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Show repository file tree" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    const selectedMarkdownPreview = await canvas.findByRole("region", {
      name: "Markdown preview for README.md",
    });
    await expect(selectedMarkdownPreview).toHaveTextContent(
      "Native JJ snapshot",
    );
    await expect(selectedMarkdownPreview).not.toHaveTextContent(SOURCE_TEXT);
    await expect(selectedMarkdownPreview.querySelector("script")).toBeNull();
    await expect(
      canvas.getByRole("button", { name: "Preview" }),
    ).toHaveAttribute("aria-pressed", "true");
    await userEvent.click(canvas.getByRole("button", { name: "Source" }));
    await expectSourceHighlight(canvasElement, "markdown", "# jj-store");
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-ui-component="code-block"]'),
      ).toHaveTextContent("# jj-store"),
    );
    await expect(
      canvasElement.querySelector('[data-ui-component="code-block"]'),
    ).toHaveTextContent(SOURCE_TEXT);
    await expect(
      canvasElement.querySelector('[data-ui-component="code-block"] script'),
    ).toBeNull();
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByRole("table", { name: "Repository files" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository bookmark or change" }),
    );
    await userEvent.click(
      await body.findByRole("option", { name: "feature/native-objects" }),
    );
    await expect(
      canvas.getByRole("combobox", {
        name: "Repository bookmark or change",
      }),
    ).toHaveTextContent("feature/native-objects");
    await expect(
      canvas.getByRole("table", { name: "Repository files" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-ui-component="code-block"]'),
      ).toHaveTextContent("stream_native_objects"),
    );
    const rustSourceBlock = canvasElement.querySelector<HTMLElement>(
      '[data-ui-component="code-block"]',
    )!;
    await expect(rustSourceBlock).toHaveAttribute("data-language", "rust");
    await expect(
      within(
        canvas.getByRole("region", { name: "Source preview for src/lib.rs" }),
      ).getByText("src/lib.rs", { exact: true }),
    ).toBeVisible();
    await expectSourceHighlight(canvasElement, "rust", /pub|fn/);
    await expect(
      canvas.getByRole("region", { name: "Selected file metadata" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("status", { name: "File statistics" }),
    ).toHaveTextContent(/\d+ lines \(\d+ loc\) · \d+ Bytes/);
    await expect(rustSourceBlock).not.toHaveTextContent(
      "preserve_publication_history",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository bookmark or change" }),
    );
    await userEvent.click(await body.findByRole("tab", { name: "Changes" }));
    await userEvent.type(
      body.getByPlaceholderText("Find bookmark or change"),
      "po".repeat(16),
    );
    await expect(body.getAllByRole("option")).toHaveLength(2);
    await userEvent.click(
      await body.findByRole("option", {
        name: /34343434 · Preserve concurrent publication history/,
      }),
    );
    await expect(
      canvas.getByRole("combobox", {
        name: "Repository bookmark or change",
      }),
    ).toHaveTextContent(/· 34343434/);
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-ui-component="code-block"]'),
      ).toHaveTextContent("preserve_publication_history"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByRole("table", { name: "Repository files" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Conflicted snapshot"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository bookmark or change" }),
    );
    await userEvent.click(
      await body.findByRole("option", { name: /conflicted/ }),
    );
    await expect(body.getByText("Choose a bookmark target")).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "README.md" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", {
        name: `Open target ${IDS.right.slice(0, 8)}…${IDS.right.slice(-4)}`,
      }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Show repository file tree" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "assets/logo.bin" }),
    );
    await expect(
      canvas.getByText("Binary file", { exact: true }),
    ).toBeVisible();
  }}
/>
<Story
  name="Profile login footer"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Sign out" }),
    );
    await waitFor(() =>
      expect(
        body.queryByRole("menu", { name: "Profile menu" }),
      ).not.toBeInTheDocument(),
    );
    await waitFor(() =>
      expect(
        getComputedStyle(
          canvas.getByRole("button", { name: "Open profile menu for Guest" }),
        ).pointerEvents,
      ).not.toBe("none"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Guest" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Sign in" }),
    );
    await waitFor(() =>
      expect(
        canvas.getByRole("button", {
          name: "Open profile menu for Maya Chen",
        }),
      ).toBeVisible(),
    );
  }}
/>
<Story
  name="Mobile files"
  args={{ initialPage: "files", displayMode: "mobile" }}
/>

<Story
  name="Large virtualized repository"
  args={{ large: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await waitFor(() =>
      expect(
        canvasElement.querySelectorAll(".community-bookmark-card").length,
      ).toBeGreaterThan(0),
    );
    await expect(
      canvasElement.querySelectorAll(".community-bookmark-card").length,
    ).toBeLessThan(40);

    await userEvent.click(canvas.getByRole("tab", { name: "Commits" }));
    await waitFor(() =>
      expect(
        canvasElement.querySelectorAll("[data-revision-id]").length,
      ).toBeGreaterThan(0),
    );
    await expect(
      canvasElement.querySelectorAll("[data-revision-id]").length,
    ).toBeLessThan(80);
    await userEvent.click(
      canvasElement.querySelector<HTMLElement>("[data-revision-id]")!,
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelectorAll('[data-ui-part="file-row"]').length,
      ).toBeGreaterThan(0),
    );
    await expect(
      canvasElement.querySelectorAll('[data-ui-part="file-row"]').length,
    ).toBeLessThan(80);

    await userEvent.click(canvas.getByRole("button", { name: "jj-store" }));
    await waitFor(() =>
      expect(
        canvas.getByRole("table", { name: "Repository files" }),
      ).toBeVisible(),
    );
    await new Promise<void>((resolve) => setTimeout(resolve, 500));
    await expect(
      canvasElement.querySelectorAll('[data-ui-part="file-row"]').length,
    ).toBeLessThan(80);
    await expect(
      canvasElement.querySelectorAll("[data-row-index]").length,
    ).toBeLessThan(80);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository bookmark or change" }),
    );
    await waitFor(() =>
      expect(body.getAllByRole("option").length).toBeGreaterThan(0),
    );
    await new Promise<void>((resolve) => setTimeout(resolve, 500));
    await expect(body.getAllByRole("option").length).toBeLessThan(80);
  }}
/>
