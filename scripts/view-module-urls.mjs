import { dirname, resolve, relative } from "node:path";

/**
 * Give retry imports exact compiler-owned entry URLs, including hashed builds.
 * @param {string} root
 * @returns {import("vite").Plugin}
 */
export function viewModuleUrls(root) {
  let building = false;
  let servingRoot = root;
  return {
    name: "view-module-urls",
    enforce: "pre",
    configResolved(config) {
      building = config.command === "build";
      servingRoot = config.root ?? root;
    },
    resolveFileUrl({ relativePath }) {
      return `new URL(${JSON.stringify(relativePath)}, import.meta.url).href`;
    },
    transform(source, id) {
      const filename = id.split("?")[0];
      if (!filename.endsWith("/store-application/application-views.ts")) return;
      return source.replace(
        /viewModuleURL\(\s*("[^"]+")\s*,?\s*\)/g,
        /** @param {string} _ @param {string} literal */
        (_, literal) => {
          const target = resolve(dirname(filename), JSON.parse(literal));
          if (!target.startsWith(root + "/") || !target.endsWith(".svelte"))
            throw Error("Unexpected view module path");
          if (!building)
            return JSON.stringify("/" + relative(servingRoot, target));
          const reference = this.emitFile({ type: "chunk", id: target });
          return `import.meta.ROLLUP_FILE_URL_${reference}`;
        },
      );
    },
  };
}
