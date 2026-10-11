import type { RepositorySettingsSection } from "../../controller/repository-settings-navigation.js";

type Row =
  | "fact"
  | "input"
  | "numeric"
  | "toggle"
  | "textarea"
  | "disclosure"
  | "runtime"
  | "preview";
export type SettingsSkeletonCard = {
  header?: boolean;
  rows: Row[];
  actions?: number;
  badge?: boolean;
};
const facts = (count: number): Row[] =>
  Array.from({ length: count }, () => "fact");
export function settingsSkeletonCards(
  section: RepositorySettingsSection,
  tab = "",
  local = false,
): SettingsSkeletonCard[] {
  switch (section) {
    case "general":
      return [{ rows: facts(local ? 9 : 10), badge: true }];
    case "folders":
      return [{ rows: local ? facts(4) : ["fact", "input"], actions: 2 }];
    case "advanced":
      return local
        ? [{ rows: facts(6) }]
        : [
            { rows: facts(2), actions: 1 },
            { rows: facts(7) },
            { rows: facts(6) },
          ];
    case "search":
      if (tab === "health") return [{ rows: facts(6) }];
      return [
        { rows: facts(3) },
        {
          rows:
            tab === "scopes"
              ? ["toggle", "textarea", "textarea", "textarea", "textarea"]
              : tab === "advanced"
                ? Array.from({ length: 7 }, () => "numeric")
                : ["toggle", "numeric"],
          actions: 2,
        },
        { rows: [], actions: 3 },
      ];
    case "storage":
      return tab === "maintenance"
        ? [{ rows: facts(1), actions: 2 }, { rows: facts(4) }]
        : [{ rows: facts(3) }, { rows: facts(5) }];
    case "sync":
      if (tab === "configuration")
        return [
          { rows: facts(3) },
          { rows: ["input", "input", "input", "toggle"], actions: 1 },
        ];
      return [{ rows: facts(tab === "progress" ? 10 : 5), actions: 2 }];
  }
}
export const settingsSkeletonHeading = (
  section: RepositorySettingsSection,
  tab = "",
) =>
  section === "advanced" ||
  (section === "search" && tab !== "health") ||
  (section === "sync" && tab === "configuration");
