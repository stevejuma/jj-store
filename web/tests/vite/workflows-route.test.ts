import { expect, test } from "vitest";
import {
  readWorkflowsHash,
  workflowsRoute,
  workflowsRouteToHash,
} from "../../src/lib/controller/workflows-route-codec.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../../src/lib/controller/store-route-codec.js";
const home = "41".repeat(16),
  run = "51".repeat(16),
  attempt = "52".repeat(16);
test("workflow URLs preserve filters, exact run, attempt and full uint64 publication", () => {
  const selection = {
    tab: "history" as const,
    q: "Parser main",
    state: "completed" as const,
    trigger: "event" as const,
    author: "22".repeat(16),
    bookmark: "feature/parse λ",
    scope: home,
    source: "42".repeat(16),
    workflow: "11".repeat(16),
    home,
    run,
    at: "18446744073709551615",
    attempt,
  };
  const hash = workflowsRouteToHash(workflowsRoute(selection))!;
  const decoded = storeRouteFromHash(hash);
  expect(decoded.kind).toBe("route");
  if (decoded.kind === "route") {
    expect(decoded.route.workflow).toEqual(selection);
    expect(storeRouteToHash(decoded.route)).toBe(hash);
  }
  expect(workflowsRouteToHash(workflowsRoute())).toBe("#/workflows");
});
test("workflow routes refuse malformed, duplicated and incompatible selections", () => {
  for (const query of [
    "tab=wrong",
    "tab=history&run=" + run,
    "home=" + home,
    "tab=history&run=" + run + "&home=" + home + "&at=18446744073709551616",
    "tab=history&attempt=" + attempt,
    "tab=history&state=wrong",
    "q=%00",
    "q=" + encodeURIComponent("😀".repeat(129)),
    "tab=history&tab=history",
    "workflow=" + run,
    "unknown=1",
    "tab=history&author=invalid",
    "tab=history&author=" + home + "&author=" + home,
    "tab=history&trigger=push",
    "tab=history&bookmark=",
    "tab=history&bookmark=%00",
    "tab=history&bookmark=" + encodeURIComponent("😀".repeat(129)),
    "trigger=manual",
    "author=" + home,
    "bookmark=main",
  ])
    expect(readWorkflowsHash("#/workflows?" + query)?.kind).toBe("invalid");
});
test("global Settings routes still take precedence over a retained workflow view", () => {
  expect(
    storeRouteToHash({
      ...workflowsRoute({ tab: "history", home, run }),
      globalSettings: { section: "appearance" },
    }),
  ).toContain("#/settings");
});
