import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { managedCargo, cargoSelection } from "./managed-cargo.mjs";
import { storageContext } from "./build-storage-inventory.mjs";
import { execute } from "./validation/process.mjs";

/** Quote the exact executable selected by managed Cargo for Playwright's shell. */
export function previewCommand(path) {
  return "'" + path.replaceAll("'", "'\\''") + "'";
}

/** Compilation finishes before Playwright starts its unchanged readiness deadline. */
export async function runBrowserTests(kind, args = [], options = {}) {
  if (!["app", "docs"].includes(kind)) throw Error("Unknown browser suite");
  const root = options.root ?? resolve(import.meta.dirname, "..");
  const environment = options.environment ?? process.env;
  const prepare =
    options.prepare ??
    (async () => {
      const build = [
        "build",
        "--quiet",
        "--locked",
        "-p",
        "jj-store-http",
        "--example",
        "browser_preview",
      ];
      const selection = await cargoSelection(
        await storageContext(root),
        build,
        environment,
      );
      await managedCargo(build, root);
      return join(
        selection.path,
        "debug",
        "examples",
        "browser_preview" + (process.platform === "win32" ? ".exe" : ""),
      );
    });
  const executable = await prepare();
  await (options.execute ?? execute)(
    "playwright",
    [
      "test",
      "--config",
      kind === "docs"
        ? "web/playwright.docs.config.ts"
        : "web/playwright.config.ts",
      ...args,
    ],
    {
      cwd: root,
      env: { ...environment, JJ_STORE_BROWSER_PREVIEW_EXECUTABLE: executable },
    },
  );
}
if (
  process.argv[1] &&
  pathToFileURL(resolve(process.argv[1])).href === import.meta.url
)
  await runBrowserTests(process.argv[2], process.argv.slice(3));
