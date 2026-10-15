import type { StoreController } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";

export function workingCopyActions(
  current: () => StoreDataSource,
): Pick<StoreController["actions"], "openWorkingFile" | "saveWorkingFile"> {
  return {
    async openWorkingFile(path) {
      const source = current();
      if (!source.workingCopy)
        throw new Error("Working copy editing is unavailable.");
      const file = await source.workingCopy.read(path);
      if (source !== current())
        throw new DOMException("Session changed", "AbortError");
      return file;
    },
    async saveWorkingFile(selection, contents) {
      const source = current();
      if (!source.workingCopy)
        throw new Error("Working copy editing is unavailable.");
      const file = await source.workingCopy.save(selection, contents);
      if (source !== current())
        throw new DOMException("Session changed", "AbortError");
      return file;
    },
  };
}
