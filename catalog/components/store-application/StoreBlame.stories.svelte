<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreBlameFixture from "../../fixtures/StoreBlameFixture.svelte";
  import { expectSourceHighlight } from "./source-highlight-assertions.js";

  async function expectBlameHighlight(
    root: HTMLElement,
    language: string,
    path: string,
    contents: string,
  ) {
    const canvas = within(root);
    const view = canvas.getByRole("region", { name: `Blame for ${path}` });
    await waitFor(() => {
      const token = view.querySelector(".ui-code-token-keyword");
      expect(token).not.toBeNull();
      expect(getComputedStyle(token!).color).not.toBe(
        getComputedStyle(view.querySelector("code")!).color,
      );
    });
    expect(
      Array.from(
        view.querySelectorAll("code"),
        (line) => line.textContent,
      ).join("\n"),
    ).toBe(contents);
    expect(within(view).getAllByRole("status")[0]).toHaveAccessibleName(
      /Loading origin/,
    );
    await userEvent.click(canvas.getByRole("button", { name: "Source" }));
    await expectSourceHighlight(root, language, /pub|static/);
    await userEvent.click(canvas.getByRole("button", { name: "Blame" }));
    await waitFor(() =>
      expect(
        canvas
          .getByRole("region", { name: `Blame for ${path}` })
          .querySelector(".ui-code-token-keyword"),
      ).not.toBeNull(),
    );
  }
  const { Story } = defineMeta({
    title: "JJ Store/Repository/Blame",
    component: StoreBlameFixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte",
      },
      layout: "padded",
    },
  });
</script>

<Story
  name="Rust highlighting matches Source while origins load"
  args={{
    restored: true,
    sample: {
      path: "src/lib.rs",
      contents:
        'pub fn main() {\n    /* a comment\n       across lines */\n    println!("<not-markup>");\n}',
    },
  }}
  play={async ({ canvasElement, args }) => {
    await expectBlameHighlight(
      canvasElement,
      "rust",
      args.sample!.path,
      args.sample!.contents,
    );
  }}
/>
<Story
  name="Linux C highlighting matches Source while origins load"
  args={{
    restored: true,
    sample: {
      path: "drivers/edac/sample.c",
      contents:
        "static int sample(void) {\n    /* a comment\n       across lines */\n    return 42;\n}",
    },
  }}
  play={async ({ canvasElement, args }) => {
    await expectBlameHighlight(
      canvasElement,
      "c",
      args.sample!.path,
      args.sample!.contents,
    );
  }}
/>
<Story
  name="Extensionless README remains plain text"
  args={{
    restored: true,
    sample: {
      path: "README",
      contents: "Linux kernel\n============\nRead <documentation> before use.",
    },
  }}
  play={async ({ canvasElement, args }) => {
    const view = within(canvasElement).getByRole("region", {
      name: "Blame for README",
    });
    expect(
      Array.from(
        view.querySelectorAll("code"),
        (line) => line.textContent,
      ).join("\n"),
    ).toBe(args.sample!.contents);
    expect(view.querySelector('[class*="ui-code-token-"]')).toBeNull();
    expect(view.querySelector("documentation")).toBeNull();
  }}
/>

<Story
  name="Progress resumes and completed views are cached"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "0",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Blame" }));
    await waitFor(() =>
      expect(canvas.getByLabelText("Blame requests")).toHaveTextContent("1"),
    );
    const view = canvas.getByRole("region", {
      name: "Blame for src/sample.ts",
    });
    await expect(view).toHaveTextContent("const value1 = 1");
    await expect(within(view).getAllByRole("status")[0]).toHaveAccessibleName(
      /Loading origin/,
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve window" }),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Blame requests")).toHaveTextContent("2"),
    );
    await expect(within(view).getAllByRole("link")[0]).toHaveTextContent(
      "Correct line attribution",
    );
    await expect(view).not.toHaveTextContent("Body text must stay hidden");
    await userEvent.click(canvas.getByRole("button", { name: "Source" }));
    await waitFor(() =>
      expect(canvas.getByLabelText("Canceled requests")).toHaveTextContent("1"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await waitFor(() =>
      expect(canvas.getByLabelText("Blame requests")).toHaveTextContent("3"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve window" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Source" }));
    await userEvent.click(canvas.getByRole("button", { name: "Blame" }));
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "3",
    );
    const restored = canvas.getByRole("region", {
      name: "Blame for src/sample.ts",
    });
    const link = within(restored).getAllByRole("link")[0]!;
    await expect(link).toHaveAttribute(
      "href",
      expect.stringContaining("/log?"),
    );
    await expect(link).toHaveAttribute(
      "href",
      expect.stringContaining("commit=" + "56".repeat(20)),
    );
    await expect(link).not.toHaveAttribute(
      "href",
      expect.stringContaining("file_view"),
    );
    await userEvent.click(link);
    await expect(canvas.getByLabelText("Current route")).toHaveTextContent(
      "/log?",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "3",
    );
    await expect(canvas.getByRole("button", { name: "Blame" })).toHaveAttribute(
      "aria-pressed",
      "true",
    );
  }}
/>
<Story
  name="Virtualized groups retain origins and scroll horizontally"
  args={{ restored: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve window" }),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Blame requests")).toHaveTextContent("2"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve window" }),
    );
    const view = canvas.getByRole("region", {
      name: "Blame for src/sample.ts",
    });
    await waitFor(() =>
      expect(view.querySelector(".ui-code-token-keyword")).not.toBeNull(),
    );
    await expect(
      getComputedStyle(view.querySelector(".ui-code-token-keyword")!).color,
    ).not.toBe(getComputedStyle(view.querySelector("code")!).color);
    const link = within(view).getAllByRole("link")[0]!;
    await expect(getComputedStyle(link).textOverflow).toBe("ellipsis");
    await expect(link.scrollWidth).toBeGreaterThan(link.clientWidth);
    await expect(view.scrollWidth).toBeGreaterThan(view.clientWidth);
    view.scrollLeft = 200;
    await expect(view.scrollLeft).toBe(200);
    view.scrollTop = 7000;
    view.dispatchEvent(new Event("scroll"));
    await waitFor(() =>
      expect(view.querySelector('[data-line="250"]')).not.toBeNull(),
    );
    await expect(view.querySelectorAll("[data-line]").length).toBeLessThan(250);
    await expect(
      view.querySelector('[data-continued="true"] a'),
    ).not.toBeNull();
    await waitFor(() => {
      const bounds = view.getBoundingClientRect();
      const visible = within(view)
        .getAllByRole("link")
        .some((link) => {
          const box = link.getBoundingClientRect();
          return box.top >= bounds.top && box.bottom <= bounds.bottom;
        });
      expect(visible).toBe(true);
    });
    const panel = view.closest<HTMLElement>(".fixture-panel")!;
    panel.style.width = "420px";
    await waitFor(() => expect(view.clientWidth).toBeLessThanOrEqual(420));
    const gutter = view.querySelector<HTMLElement>(".blame-group__gutter")!;
    await waitFor(() =>
      expect(gutter.getBoundingClientRect().width).toBeLessThan(
        view.clientWidth / 2,
      ),
    );
    view.scrollTop = view.scrollHeight;
    view.dispatchEvent(new Event("scroll"));
    await waitFor(() => expect(view).toHaveTextContent("Origin unavailable"));
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "2",
    );
  }}
/>
<Story
  name="Unavailable live capability retains source"
  args={{ enabled: false, restored: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByRole("button", { name: "Blame" })).toBeDisabled();
    await expect(
      canvas.getByText(
        "Blame is not available on this server yet. Source remains available.",
        { selector: "p" },
      ),
    ).toBeVisible();
    await expect(
      canvas.getByRole("region", { name: "Blame for src/sample.ts" }),
    ).toHaveTextContent("const value1 = 1");
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "0",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Source" }));
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "0",
    );
  }}
/>
<Story
  name="Terminal errors retain source and use a shared toast"
  args={{ restored: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(canvas.getByRole("button", { name: "Fail window" }));
    await expect(
      await body.findByText("Fixture annotation failed"),
    ).toBeVisible();
    const view = canvas.getByRole("region", {
      name: "Blame for src/sample.ts",
    });
    await expect(view).toHaveTextContent("Origin unavailable");
    await expect(view).not.toHaveTextContent("Fixture annotation failed");
    await expect(within(view).queryAllByRole("status")).toHaveLength(0);
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "1",
    );
  }}
/>
<Story
  name="Empty file has no invented line"
  args={{ empty: true, restored: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("This file has no lines to attribute."),
    ).toBeVisible();
    await expect(canvas.queryByLabelText("Line 1")).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve window" }),
    );
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "1",
    );
  }}
/>

<Story
  name="Binary files do not request Blame"
  args={{ contentKind: "binary", restored: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("Binary file", { exact: true }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Blame" }),
    ).not.toBeInTheDocument();
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "0",
    );
  }}
/>
<Story
  name="Oversized files do not request Blame"
  args={{ contentKind: "large", restored: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("src/sample.ts is too large to preview."),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Blame" }),
    ).not.toBeInTheDocument();
    await expect(canvas.getByLabelText("Blame requests")).toHaveTextContent(
      "0",
    );
  }}
/>
