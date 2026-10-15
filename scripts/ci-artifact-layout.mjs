// Payload destinations are repository-owned, never taken from a receipt.
export const payloads = {
  "ui-tools": {
    role: "ui",
    root: "target/debug/examples",
    files: ["browser_preview", "docs_reference"],
  },
  "ui-fixtures": {
    role: "ui",
    root: ".scratch",
    files: ["web-responses.json"],
  },
  "ui-web": { role: "ui", root: "web/dist" },
  "ui-catalog": { role: "ui", root: "storybook-static" },
  "tests-heavy": {
    role: "build",
    root: ".release",
    files: ["tests-heavy.tar.zst", "tests-heavy.tar.zst.json"],
  },
  "linux-development-tools": {
    role: "build",
    root: "target/debug",
    files: [
      "jj-store",
      "jj-store-jj",
      "examples/docs_reference",
      "examples/browser_preview",
    ],
  },
  "native-browser-fixtures": {
    role: "build",
    root: ".scratch",
    files: ["web-responses.json"],
  },
  "linux-storage-runtime": {
    role: "runtime",
    root: "runtime/embedded-rustfs/target/debug",
    files: ["jj-store-rustfs"],
  },
  "web-bundle": { role: "assets", root: "web/dist" },
  "storybook-bundle": { role: "assets", root: "storybook-static" },
};
export const uiLanePayloads = {
  storybook: [],
  browser: ["ui-tools", "ui-fixtures", "ui-web"],
  help: ["ui-tools", "ui-fixtures", "ui-web"],
  catalog: ["ui-catalog"],
};
export const lanePayloads = {
  standard: [],
  contracts: [],
  native: [],
  external: [],
  assets: ["linux-development-tools"],
  backend: ["tests-heavy"],
  embedded: ["linux-storage-runtime"],
  storybook: [],
  catalog: ["storybook-bundle"],
  browser: ["web-bundle", "native-browser-fixtures", "linux-development-tools"],
  help: ["web-bundle", "native-browser-fixtures", "linux-development-tools"],
  cold: ["web-bundle", "linux-development-tools", "linux-storage-runtime"],
  "cold-profile": [
    "web-bundle",
    "linux-development-tools",
    "linux-storage-runtime",
  ],
  "git-cycle": [
    "web-bundle",
    "linux-development-tools",
    "linux-storage-runtime",
    "tests-heavy",
  ],
};
