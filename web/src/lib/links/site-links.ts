import { localLink } from "../appearance/local-link.js";
import { siteIcons } from "./site-icons.js";

type Site = keyof typeof siteIcons;
const hosts: Readonly<Record<string, Site>> = {
  "github.com": "github",
  "gitlab.com": "gitlab",
  "forgejo.org": "forgejo",
  "code.forgejo.org": "forgejo",
  "codeberg.org": "codeberg",
  "bitbucket.org": "bitbucket",
  "facebook.com": "facebook",
  "m.facebook.com": "facebook",
  "fb.com": "facebook",
  "x.com": "x",
  "twitter.com": "x",
  "mobile.twitter.com": "x",
  "google.com": "google",
  "docs.google.com": "google",
  "drive.google.com": "google",
  "mail.google.com": "google",
  "calendar.google.com": "google",
  "meet.google.com": "google",
  "slack.com": "slack",
  "linear.app": "linear",
  "notion.so": "notion",
  "notion.site": "notion",
  "figma.com": "figma",
  "youtube.com": "youtube",
  "m.youtube.com": "youtube",
  "youtu.be": "youtube",
  "discord.com": "discord",
  "discord.gg": "discord",
  "reddit.com": "reddit",
  "old.reddit.com": "reddit",
  "stackoverflow.com": "stackoverflow",
};

/** Local identity only: never contact a site or infer self-hosted forge identity. */
export function linkSite(href: string) {
  const link = localLink(href, "");
  if (!link) return undefined;
  const host = link.host.replace(/^www\./, "");
  const site =
    hosts[host] ??
    (/^[a-z0-9-]+\.slack\.com$/.test(host) ? "slack" : undefined);
  return site ? siteIcons[site] : undefined;
}

/** Recognised content addresses eligible for a standalone, local-only card. */
export function linkContent(href: string): string | undefined {
  const link = localLink(href, "");
  if (!link) return;
  const url = new URL(link.href);
  const host = url.hostname.replace(/^www\./, "");
  const path = url.pathname.replace(/\/$/, "");
  if (host === "github.com") {
    const resource = /^\/([^/]+)\/([^/]+)(?:\/(pull|issues)\/(\d+))?$/.exec(
      path,
    );
    // Product routes such as /settings/profile are not repositories.
    if (
      resource &&
      ![
        "settings",
        "orgs",
        "users",
        "search",
        "features",
        "marketplace",
        "topics",
        "collections",
        "sponsors",
        "login",
        "signup",
      ].includes(resource[1]!)
    ) {
      const [, owner, repo, kind, number] = resource;
      return `${owner}/${repo}${kind ? ` ${kind === "pull" ? "PR" : "issue"} #${number}` : ""}`;
    }
  }
  if (host === "linear.app") {
    const issue = /^\/[^/]+\/issue\/([a-z][a-z0-9]*-\d+)(?:\/[^/]+)?$/i.exec(
      path,
    );
    if (issue) return `Linear ${issue[1]!.toUpperCase()}`;
  }
  if (host === "docs.google.com") {
    const doc =
      /^\/(document|spreadsheets|presentation)\/d\/[^/]+(?:\/(?:edit|view|preview|copy|present))?$/.exec(
        path,
      );
    if (doc)
      return {
        document: "Google Docs document",
        spreadsheets: "Google Sheets spreadsheet",
        presentation: "Google Slides presentation",
      }[doc[1]!];
  }
  if (host === "drive.google.com") {
    if (/^\/(?:drive\/(?:u\/\d+\/)?)?folders\/[^/]+$/.test(path))
      return "Google Drive folder";
    if (
      /^\/file\/d\/[^/]+(?:\/(?:view|edit|preview))?$/.test(path) ||
      (path === "/open" && url.searchParams.get("id"))
    )
      return "Google Drive file";
  }
}
