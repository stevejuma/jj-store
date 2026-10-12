import type { GlobalSettingsSection } from "../../controller/global-settings-navigation.js";
import type { SettingsSkeletonCard as Card } from "./settings-skeleton.js";
const facts = (n: number): Card => ({
  rows: Array.from({ length: n }, () => "fact"),
});
const footer = (actions = 2): Card => ({ header: false, rows: [], actions });
export function globalSettingsSkeletonCards(
  section: GlobalSettingsSection,
  tab: string,
): Card[] {
  switch (section) {
    case "profile":
      return [{ rows: ["preview", "input", "textarea", "input"], actions: 2 }];
    case "custom-emoji":
      return [{ rows: ["preview", "input"], actions: 1 }, facts(3)];
    case "appearance":
      return tab === "preferences"
        ? [
            { rows: ["input", "input", "input", "preview"] },
            { rows: ["input", "preview"] },
            { rows: ["input", "preview"] },
          ]
        : [{ rows: ["input", "input", "toggle", "toggle"] }];
    case "service":
      if (tab === "overview")
        return [facts(5), { rows: ["fact", "fact"], actions: 3 }];
      if (tab === "limits")
        return [
          facts(1),
          { rows: ["numeric", "numeric", "numeric", "numeric"] },
          footer(),
        ];
      if (tab === "protection")
        return [facts(1), { rows: ["textarea", "textarea"] }, footer()];
      if (tab === "launch") return [facts(5), facts(3), facts(4)];
      return [facts(3), facts(4), facts(3), facts(2)];
    case "agents":
      if (tab === "runtimes")
        return [
          { rows: ["toggle", "toggle", "disclosure"] },
          { rows: ["runtime", "runtime", "runtime"], actions: 1 },
          { rows: [], actions: 3 },
          { rows: ["input", "input", "input", "disclosure"], actions: 1 },
        ];
      if (tab === "configuration")
        return [
          facts(2),
          {
            rows: [
              "toggle",
              "numeric-slider",
              "numeric-slider",
              "numeric-slider",
              "numeric-slider",
            ],
          },
          { rows: ["disclosure", "disclosure", "disclosure"] },
          footer(),
        ];
      if (tab === "imports")
        return [{ rows: ["input", "input", "input"], actions: 2 }];
      if (tab === "sources")
        return [{ rows: ["input", "fact", "fact", "fact"], actions: 2 }];
      if (tab === "diagnostics") return [facts(5), facts(4)];
      return [{ rows: [], actions: 2 }, facts(4)];
    case "compute":
      if (tab === "resources")
        return [facts(3), { rows: ["numeric"] }, footer(1)];
      if (tab === "sharing")
        return [
          { rows: ["fact", "toggle", "input"], actions: 2 },
          { rows: ["input", "textarea", "disclosure"], actions: 1 },
          footer(1),
        ];
      return [
        { rows: ["fact", "fact", "input", "input"], actions: 3 },
        { rows: ["input"], actions: 1 },
        footer(1),
      ];
  }
}
