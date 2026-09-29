const { existsSync } = require("node:fs");
const path = require("node:path");

const localPackages = new Map([
  ["@lapismd/design-core", { protocol: "file:", path: "../design-core" }],
  [
    "@lapismd/lapis-community",
    { protocol: "file:", path: "../lapis-community" },
  ],
]);

function createHooks({
  root = path.resolve(__dirname, ".."),
  fileExists = existsSync,
} = {}) {
  const availableLocalPackages = new Map(
    [...localPackages].flatMap(([packageName, localPackage]) => {
      const packagePath = path.resolve(root, localPackage.path);
      return fileExists(packagePath)
        ? [[packageName, `${localPackage.protocol}${packagePath}`]]
        : [];
    }),
  );

  return {
    hooks: {
      readPackage(packageManifest) {
        for (const field of [
          "dependencies",
          "devDependencies",
          "optionalDependencies",
        ]) {
          const dependencies = packageManifest[field];
          if (!dependencies) continue;
          for (const [packageName, localPackage] of availableLocalPackages) {
            if (dependencies[packageName]) {
              dependencies[packageName] = localPackage;
            }
          }
        }
        return packageManifest;
      },
    },
  };
}

module.exports = { createHooks };
