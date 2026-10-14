import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  appearanceStorageKey,
  defaultAppearance,
} from "../../src/lib/appearance/preferences.js";
import { api, repo, operation } from "./native-browser-fixture.js";
import { agentDetail } from "../../../catalog/fixtures/collaboration.js";
import {
  agentPromptMarkup,
  inertPromptMarkup,
} from "../../../catalog/fixtures/agent-prompt-markup.js";

async function transcript(
  page: Page,
  content: string,
  fallback = false,
  conversationView: "classic" | "document" = "classic",
) {
  await api(page, { access: "local_trusted" });
  await page.addInitScript(
    ({ key, appearance }) => {
      localStorage.setItem(key, JSON.stringify(appearance));
      let copied = "";
      Object.defineProperty(navigator, "clipboard", {
        configurable: true,
        value: {
          writeText: async (value: string) => {
            copied = value;
          },
          readText: async () => copied,
        },
      });
    },
    {
      key: appearanceStorageKey,
      appearance: { ...defaultAppearance, conversationView },
    },
  );
  const detail = structuredClone(agentDetail);
  detail.snapshot.repo_id = repo;
  detail.code = [];
  const first = detail.activity.find((a) => a.type === "message")!;
  if (first.type !== "message") throw Error("message fixture");
  detail.activity = fallback
    ? []
    : [
        { ...first, content: { media_type: "text", content } },
        {
          ...first,
          sequence: 2,
          message_id: "99".repeat(16),
          event_id: "98".repeat(16),
          content: {
            media_type: "text",
            content: "Following message remains separate.",
          },
        },
      ];
  await page.route(`**/v1/repos/${repo}/agent/**`, async (route) => {
    const path = new URL(route.request().url()).pathname;
    const value = path.endsWith("/sessions")
      ? {
          version: 1,
          snapshot: detail.snapshot,
          sessions: [detail.session],
          next_cursor: null,
        }
      : path.includes("/transcripts/")
        ? {
            version: 1,
            snapshot: detail.snapshot,
            chunks: [
              { sequence: 0, media_type: "text", content },
              {
                sequence: 1,
                media_type: "text",
                content: "Following message remains separate.",
              },
            ],
            next_cursor: null,
          }
        : path.endsWith("/associations")
          ? {
              version: 1,
              snapshot: detail.snapshot,
              relations: [],
              next_cursor: null,
            }
          : detail;
    await route.fulfill({ json: value });
  });
  await page.route(`**/v1/repos/${repo}/reviews/threads?*`, async (route) => {
    await route.fulfill({
      json: {
        version: 1,
        snapshot: detail.snapshot,
        threads: [],
        next_cursor: null,
      },
    });
  });
  await page.goto(
    `/#/repos/${repo}/agents?at_op=${operation}&session=${detail.session.session_id}`,
  );
  return page.locator(".store-agent-context");
}

for (const conversationView of ["classic", "document"] as const)
  for (const fallback of [false, true]) {
    test.describe(`${conversationView} ${fallback ? "touch mobile" : "desktop"}`, () => {
      test.use({ isMobile: fallback, hasTouch: fallback });
      test(`cold ${fallback ? "mobile recorded chunks" : "desktop activity"} preserve nested prompt Markdown and exact copy`, async ({
        page,
      }) => {
        await page.setViewportSize(
          fallback
            ? { width: 390, height: 844 }
            : { width: 1600, height: 1000 },
        );
        const context = await transcript(
          page,
          agentPromptMarkup,
          fallback,
          conversationView,
        );
        const parent = context.getByRole("button", {
          name: '<harness:context source="cli > desktop">',
          exact: true,
        });
        await expect(parent).toHaveAttribute("aria-expanded", "false");
        const row = parent.locator("xpath=ancestor::*[@data-message-id][1]");
        const initialHeight = (await row.boundingBox())!.height;
        await expect(
          context.getByText("Between", { exact: false }),
        ).toBeVisible();
        await expect(
          context.getByText("After", { exact: false }),
        ).toBeVisible();
        await parent.focus();
        await page.keyboard.press("Enter");
        await expect(parent).toHaveAttribute("aria-expanded", "true");
        await expect(
          context.getByRole("heading", { name: /Session notes/ }),
        ).toBeVisible();
        await expect(
          context.getByRole("link", { name: "safe links" }),
        ).toHaveAttribute("href", "https://example.com/");
        await expect(
          context.locator('[data-prompt-element^="<arbitrary_limits "] button'),
        ).toHaveCount(0);
        const leaf = context.locator(
          '[data-prompt-element^="<arbitrary_limits "]',
        );
        const childTag = leaf.locator(".store-prompt-tag").first();
        const offsets = await childTag.evaluate((element) => {
          const parent = element.closest(
            ".store-prompt-element__body",
          )!.parentElement!;
          const parentTag = parent.querySelector("button .store-prompt-tag")!;
          const siblingTag = parent.querySelector(
            '[data-prompt-element="<environment_context>"] button .store-prompt-tag',
          )!;
          return {
            indent:
              element.getBoundingClientRect().left -
              parentTag.getBoundingClientRect().left,
            siblingDifference:
              element.getBoundingClientRect().left -
              siblingTag.getBoundingClientRect().left,
          };
        });
        expect(offsets.indent).toBeGreaterThanOrEqual(16);
        expect(offsets.siblingDifference).toBe(0);
        expect(
          await leaf.evaluate(
            (element) => getComputedStyle(element).borderTopWidth,
          ),
        ).toBe("0px");
        const attribute = parent.locator('[data-xml-token="attribute"]');
        expect(
          await attribute.evaluate(
            (element) => getComputedStyle(element).color,
          ),
        ).not.toBe(
          await parent
            .locator('[data-xml-token="string"]')
            .evaluate((element) => getComputedStyle(element).color),
        );
        await expect(
          context.getByText("cargo test", { exact: true }),
        ).toBeVisible();
        await context
          .getByRole("button", { name: "<environment_context>", exact: true })
          .click();
        await expect(context.locator("pre")).toHaveText(
          "<literal><keep_this /></literal>\n",
        );
        await expect(
          context.getByRole("button", { name: "<literal>", exact: true }),
        ).toHaveCount(0);
        await expect
          .poll(async () => (await row.boundingBox())!.height)
          .toBeGreaterThan(initialHeight + 100);
        const next = context.getByText("Following message remains separate.", {
          exact: true,
        });
        await expect
          .poll(async () => {
            const first = (await row.boundingBox())!,
              second = (await next.boundingBox())!;
            return second.y - (first.y + first.height);
          })
          .toBeGreaterThanOrEqual(-1);
        await expect(
          context.getByRole("button", {
            name: "<another-tool.context>",
            exact: true,
          }),
        ).toHaveCount(0);
        await expect(
          context.getByText("Markdown", { exact: true }),
        ).toBeVisible();
        const message = parent.locator(
          "xpath=ancestor::*[@data-message-id][1]",
        );
        await message.hover();
        await message
          .getByRole("button", { name: "More message actions", exact: true })
          .click();
        await page.getByRole("menuitem", { name: "Copy", exact: true }).click();
        await page
          .getByRole("menuitem", { name: "Copy message", exact: true })
          .click();
        expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(
          agentPromptMarkup,
        );
        await parent.focus();
        await page.keyboard.press("Space");
        await expect(parent).toHaveAttribute("aria-expanded", "false");
        await expect(
          context.getByText("cargo test", { exact: true }),
        ).toBeHidden();
        await expect(
          context.getByText("<example />", { exact: true }),
        ).toBeVisible();
        expect(
          await page.evaluate(
            () => document.documentElement.scrollWidth <= window.innerWidth,
          ),
        ).toBe(true);
        // The mobile shell replaces its main landmark with the response sidebar.
        // Audit this change's transcript surface; document landmarks are separate.
        expect(
          (
            await new AxeBuilder({ page })
              .include(".store-agent-context")
              .analyze()
          ).violations,
        ).toEqual([]);
      });
    });
  }

test("cold prompt markup attributes are inert and malformed sections remain visible", async ({
  page,
}) => {
  const leaks: string[] = [];
  page.on("request", (request) => {
    if (request.url().includes("example.invalid")) leaks.push(request.url());
  });
  const malformed =
    "\n\n<unfinished>\n\n**Retained body**\n\n<wrong></other>\n\n    <indented />\n";
  const context = await transcript(page, inertPromptMarkup + malformed);
  await context
    .getByRole("button", { name: '<payload onclick="alert(1)">', exact: true })
    .click();
  await expect(
    context.getByRole("link", { name: "reference" }),
  ).toHaveAttribute("href", "https://example.com/");
  await expect(context.locator('[data-prompt-element^="<image "]')).toHaveText(
    '<image src="https://example.invalid/leak" />',
  );
  await expect(
    context.getByRole("button", { name: "<script>", exact: true }),
  ).toHaveCount(0);
  await expect(
    context.getByText("window.promptExecuted = true", { exact: true }),
  ).toBeVisible();
  await expect(
    context.locator(
      ".community-markdown-content script, .community-markdown-content img, .community-markdown-content [onclick]",
    ),
  ).toHaveCount(0);
  expect(await page.evaluate(() => "promptExecuted" in window)).toBe(false);
  expect(leaks).toEqual([]);
  await expect(
    context.getByText("Retained body", { exact: true }),
  ).toBeVisible();
  await expect(context).toContainText("<unfinished>");
  await expect(context).toContainText("<wrong></other>");
  await expect(context.locator("pre")).toHaveText("<indented />\n");
  await expect(
    context.getByRole("button", { name: "<unfinished>", exact: true }),
  ).toHaveCount(0);
});
