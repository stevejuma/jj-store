<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "./StoreRepositoryApplication.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Review refinements",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Shell and graph alignment"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvasElement.querySelector(".lc-community-navigation__header"),
    ).toBeNull();
    const details = canvas.getByRole("button", { name: "Toggle details" });
    await expect(
      details.querySelector(
        '[data-ui-part="sidebar-toggle-icon"][data-side="right"]',
      ),
    ).not.toBeNull();
    const group = canvasElement.querySelector(
      '[data-ui-part="command-group"]',
    )!;
    const option = canvas.getByRole("option", { name: "main" });
    await expect(option.getBoundingClientRect().left).toBeCloseTo(
      group.getBoundingClientRect().left,
      0,
    );
    await expect(option.getBoundingClientRect().right).toBeCloseTo(
      group.getBoundingClientRect().right,
      0,
    );
    const heading = canvasElement.querySelector(
      '[data-ui-part="command-group-heading"]',
    )!;
    await expect(getComputedStyle(heading).position).toBe("sticky");
    await expect(getComputedStyle(heading).borderBottomWidth).toBe("1px");
    await expect(getComputedStyle(heading).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    const viewport = heading.closest<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    const top = heading.getBoundingClientRect().top;
    viewport.scrollTop = 120;
    await waitFor(() => expect(viewport.scrollTop).toBeGreaterThan(0));
    await expect(heading.getBoundingClientRect().top).toBeCloseTo(top, 0);
    viewport.scrollTop = 0;
    await expect(
      getComputedStyle(canvas.getByRole("option", { name: "retired" }))
        .borderBottomWidth,
    ).toBe("0px");
    const branchHeader = canvasElement.querySelector(
      '[data-ui-part="column-header"]',
    )!;
    await expect(getComputedStyle(branchHeader).backgroundColor).toBe(
      getComputedStyle(heading).backgroundColor,
    );
    await userEvent.click(details);
    const rail = canvas.getByRole("complementary", {
      name: "Repository details",
    });
    await expect(rail).toHaveAttribute("data-variant", "rail");
    await expect(getComputedStyle(rail).backgroundColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(getComputedStyle(rail).borderLeftColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(
      getComputedStyle(canvas.getByRole("option", { name: "main" }))
        .borderBottomWidth,
    ).toBe("1px");
    await userEvent.click(details);
    await userEvent.click(canvas.getByRole("option", { name: "main" }));
    await waitFor(() => {
      const avatars = Array.from(
        canvasElement.querySelectorAll(
          '.community-revision-graph-copy [data-ui-part="git-author"]',
        ),
      );
      expect(avatars.length).toBeGreaterThan(2);
      const xs = avatars.map((node) => node.getBoundingClientRect().left);
      expect(Math.max(...xs) - Math.min(...xs)).toBeLessThan(1);
    });
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Merge native storage and publication",
      }),
    );
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show files as list" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    const headers = canvasElement.querySelectorAll(
      '[data-ui-part="column-header"]',
    );
    await expect(headers).toHaveLength(3);
    for (const header of headers) {
      await expect(getComputedStyle(header).backgroundColor).toBe(
        getComputedStyle(heading).backgroundColor,
      );
      await expect(getComputedStyle(header).borderBottomWidth).toBe("1px");
    }
    await userEvent.click(canvas.getByRole("button", { name: "Op Log" }));
    const status = canvas.getByRole("status");
    await expect(getComputedStyle(status).borderBottomWidth).toBe("1px");
    await expect(getComputedStyle(status).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    const opHeader = canvasElement.querySelector(
      '[data-ui-part="column-header"]',
    )!;
    await expect(getComputedStyle(opHeader).backgroundColor).toBe(
      getComputedStyle(status).backgroundColor,
    );
    await userEvent.click(
      canvas.getByRole("button", {
        name: "describe: native object history",
      }),
    );
    const operationHeaders = canvasElement.querySelectorAll(
      '[data-ui-part="column-header"]',
    );
    await expect(operationHeaders).toHaveLength(2);
    for (const header of operationHeaders) {
      await expect(getComputedStyle(header).backgroundColor).toBe(
        getComputedStyle(status).backgroundColor,
      );
      await expect(getComputedStyle(header).borderBottomWidth).toBe("1px");
    }
    const breadcrumb = canvas.getByRole("list", {
      name: "Repository breadcrumb",
    });
    await expect(
      getComputedStyle(
        breadcrumb.querySelector('[data-ui-part="breadcrumb-page"]')!,
      ).color,
    ).toBe(getComputedStyle(status).color);
  }}
/>

<Story
  name="Repository directory navigation"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("table", { name: "Repository files" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Open directory src" }),
    );
    await expect(
      canvas.getByRole("table", { name: "Files in src" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Open file lib.rs" }),
    );
    await expect(
      canvasElement.querySelector('[data-ui-component="code-block"]'),
    ).toHaveTextContent("preserve_publication_history");
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByRole("table", { name: "Files in src" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Go to parent directory" }),
    );
    await expect(
      canvas.getByRole("table", { name: "Repository files" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Column titles replace body header"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    for (const label of ["Branches", "JJ Log", "Op Log"]) {
      await userEvent.click(canvas.getByRole("button", { name: label }));
      await expect(
        canvasElement.querySelector('[data-ui-part="toolbar"]'),
      ).toBeNull();
      await expect(
        canvasElement.querySelector('[data-ui-part="column-header"]'),
      ).toHaveTextContent(label);
    }
  }}
/>
