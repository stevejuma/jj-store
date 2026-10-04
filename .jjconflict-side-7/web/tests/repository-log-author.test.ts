import assert from "node:assert/strict";
import test from "node:test";
import { repositoryLogGravatarUrl } from "../src/lib/projects/repository-log-author.js";

test("Git author email produces Community-compatible Gravatar URLs", () => {
  const url = repositoryLogGravatarUrl("  Maya@Example.TEST ");
  assert.match(
    url ?? "",
    /^https:\/\/www\.gravatar\.com\/avatar\/[0-9a-f]{32}\?d=identicon&s=80$/u,
  );
  assert.equal(url, repositoryLogGravatarUrl("maya@example.test"));
  assert.equal(repositoryLogGravatarUrl(""), undefined);
  assert.equal(repositoryLogGravatarUrl("not-an-email"), undefined);
  assert.equal(
    repositoryLogGravatarUrl("npub1repositoryauthor@nostr.invalid"),
    undefined,
  );
});
