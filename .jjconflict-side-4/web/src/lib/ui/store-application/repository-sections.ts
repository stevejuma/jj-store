import type { StorePage } from "../community-navigation/model.js";

export const repositorySections: readonly { page: StorePage; label: string }[] =
  [
    { page: "files", label: "Overview" },
    { page: "branches", label: "Branches" },
    { page: "log", label: "Commits" },
    { page: "oplog", label: "Op Log" },
    { page: "agents", label: "Agents" },
    { page: "reviews", label: "Reviews" },
    { page: "projects", label: "Projects" },
  ];

export const repositoryRailSections = repositorySections
  .filter((section) => section.page !== "files")
  .map((section) => ({
    ...section,
    label: section.page === "log" ? "JJ Log" : section.label,
  }));
