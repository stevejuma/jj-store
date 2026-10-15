import { describe, expect, it } from "vitest";
import { linkContent, linkSite } from "../../src/lib/links/site-links.js";
import { remarkStandaloneLinks } from "../../src/lib/ui/community-markdown-content/syntax-external-links.js";
import {
  applyCommunityMarkdownSyntax,
  type CommunityMarkdownHastNode,
} from "../../src/lib/ui/community-markdown-content/syntax-hast.js";

describe("local website identity", () => {
  it.each([
    ["github.com", "GitHub"],
    ["www.gitlab.com", "GitLab"],
    ["forgejo.org", "Forgejo"],
    ["code.forgejo.org", "Forgejo"],
    ["codeberg.org", "Codeberg"],
    ["bitbucket.org", "Bitbucket"],
    ["m.facebook.com", "Facebook"],
    ["www.facebook.com", "Facebook"],
    ["twitter.com", "X"],
    ["x.com", "X"],
    ["docs.google.com", "Google"],
    ["google.com", "Google"],
    ["team.slack.com", "Slack"],
    ["slack.com", "Slack"],
    ["linear.app", "Linear"],
    ["notion.so", "Notion"],
    ["figma.com", "Figma"],
    ["youtu.be", "YouTube"],
    ["discord.gg", "Discord"],
    ["reddit.com", "Reddit"],
    ["stackoverflow.com", "Stack Overflow"],
  ])("recognises %s without contacting it", (host, name) => {
    expect(linkSite(`https://${host}/path`)?.name).toBe(name);
    expect(linkSite(`https://${host}/path`)?.path).toBeTruthy();
  });
  it.each([
    "github.com.evil.test",
    "evilgithub.com",
    "slack.com.evil.test",
    "team.evil.slack.com",
    "self-hosted.test",
    "example.com",
    "xn--gthub-3sa.com",
  ])("does not infer identity for %s", (host) => {
    expect(linkSite(`https://${host}/`)).toBeUndefined();
  });
  it("rejects credentials and unsafe protocols", () => {
    for (const href of [
      "javascript:alert(1)",
      "data:text/html,hi",
      "file:///test",
      "https://user:secret@github.com/a/b",
      "/relative",
    ]) {
      expect(linkSite(href)).toBeUndefined();
      expect(linkContent(href)).toBeUndefined();
    }
  });
});

describe("standalone content cards", () => {
  it.each([
    ["https://github.com/team/repo", "team/repo"],
    [
      "https://www.github.com/team/repo/pull/42?diff=split#review",
      "team/repo PR #42",
    ],
    ["https://github.com/team/repo/issues/7", "team/repo issue #7"],
    ["https://linear.app/team/issue/APP-12/fix", "Linear APP-12"],
    ["https://docs.google.com/document/d/abc/edit", "Google Docs document"],
    [
      "https://docs.google.com/spreadsheets/d/abc/edit#gid=1",
      "Google Sheets spreadsheet",
    ],
    [
      "https://docs.google.com/presentation/d/abc/view",
      "Google Slides presentation",
    ],
    ["https://drive.google.com/drive/u/0/folders/abc", "Google Drive folder"],
    ["https://drive.google.com/file/d/abc/view", "Google Drive file"],
    ["https://drive.google.com/open?id=abc", "Google Drive file"],
  ])("classifies %s using only its address", (href, label) => {
    expect(linkContent(href)).toBe(label);
  });
  it.each([
    "https://github.com/team",
    "https://github.com/settings/profile",
    "https://github.com/team/repo/actions",
    "https://github.com/team/repo/pull/nope",
    "https://linear.app/team",
    "https://docs.google.com/",
    "https://drive.google.com/open",
    "https://facebook.com/post",
    "https://example.com",
  ])("keeps unsupported content inline: %s", (href) => {
    expect(linkContent(href)).toBeUndefined();
  });
  it("records only bare/autolink top-level paragraphs and clears stale eligibility", () => {
    const href = "https://github.com/team/repo/pull/42";
    const paragraphs = new Map<number, string>();
    for (const authored of [
      href,
      `<${href}>`,
      `[#42](${href})`,
      `[${href}](${href})`,
    ]) {
      const position = {
        start: { offset: 0 },
        end: { offset: authored.length },
      };
      const link = { type: "link", url: href, position };
      const paragraph = { type: "paragraph", position, children: [link] };
      const transform = remarkStandaloneLinks({
        source: () => authored,
        paragraphs,
      });
      transform({ type: "root", children: [paragraph] });
      expect(paragraphs.size).toBe(authored.startsWith("[") ? 0 : 1);
      for (const type of ["listItem", "blockquote", "tableCell", "heading"]) {
        transform({
          type: "root",
          children: [{ type, children: [paragraph] }],
        });
        expect(paragraphs.size).toBe(0);
      }
      transform({
        type: "root",
        children: [{ ...paragraph, children: [link, { type: "text" }] }],
      });
      expect(paragraphs.size).toBe(0);
    }
  });
  it("preserves formatted children and replaces eligible paragraphs as blocks", () => {
    const href = "https://github.com/team/repo/pull/42";
    const formatted: CommunityMarkdownHastNode = {
      type: "element",
      tagName: "strong",
      children: [{ type: "text", value: "#42" }],
    };
    const anchor = (): CommunityMarkdownHastNode => ({
      type: "element",
      tagName: "a",
      properties: { href },
      children: [structuredClone(formatted)],
    });
    const tree: CommunityMarkdownHastNode = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "p",
          position: { start: { offset: 0 } },
          children: [anchor()],
        },
        {
          type: "element",
          tagName: "p",
          position: { start: { offset: 50 } },
          children: [anchor()],
        },
        { type: "element", tagName: "code", children: [anchor()] },
      ],
    };
    applyCommunityMarkdownSyntax(
      tree,
      {
        members: () => [],
        rooms: () => [],
        emojis: () => [],
        mentionPubkeys: () => [],
      },
      (value) => [{ kind: "text", value }],
      new Map([[0, href]]),
    );
    expect(tree.children?.[0]?.tagName).toBe("communitylinkpreview");
    expect(tree.children?.[1]?.tagName).toBe("p");
    expect(tree.children?.[1]?.children?.[0]?.tagName).toBe(
      "communityexternallink",
    );
    expect(tree.children?.[1]?.children?.[0]?.children).toEqual([formatted]);
    expect(tree.children?.[2]?.children?.[0]?.tagName).toBe("a");
  });
  it("leaves file, mail, native and image links with their existing renderer", () => {
    const anchors: CommunityMarkdownHastNode[] = [
      {
        type: "element",
        tagName: "pathlink",
        properties: { href: "/repo/file.ts" },
      },
      {
        type: "element",
        tagName: "a",
        properties: { href: "mailto:team@example.com" },
      },
      { type: "element", tagName: "a", properties: { href: "jj://repo/test" } },
      {
        type: "element",
        tagName: "a",
        properties: { href: "https://github.com/team/repo" },
        children: [
          {
            type: "element",
            tagName: "img",
            properties: { src: "/local.png", alt: "Preview" },
          },
        ],
      },
    ];
    const before = structuredClone(anchors);
    const tree = { type: "root", children: anchors };
    applyCommunityMarkdownSyntax(
      tree,
      {
        members: () => [],
        rooms: () => [],
        emojis: () => [],
        mentionPubkeys: () => [],
      },
      (value) => [{ kind: "text", value }],
    );
    expect(tree.children).toEqual(before);
  });
  it("gives recognised app entities priority over external website styling", () => {
    const href = `https://store.test/#/repos/${"a".repeat(32)}/log`;
    const tree: CommunityMarkdownHastNode = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "a",
          properties: { href },
          children: [{ type: "text", value: "Repository log" }],
        },
      ],
    };
    applyCommunityMarkdownSyntax(
      tree,
      {
        members: () => [],
        rooms: () => [],
        emojis: () => [],
        mentionPubkeys: () => [],
      },
      (value) => [{ kind: "text", value }],
    );
    expect(tree.children?.[0]?.tagName).toBe("communityentity");
    expect(tree.children?.[0]?.properties?.href).toBe(
      `#/repos/${"a".repeat(32)}/log`,
    );
  });
});
