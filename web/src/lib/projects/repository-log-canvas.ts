import {
  createColumnCanvasController,
  createLocalStorageColumnCanvasLayoutPersistence,
} from "@lapismd/design-core/shadcn/column-canvas";
import type { StorePage } from "../ui/community-navigation/model.js";
import { repositoryColumnCanvasStorageKey } from "../ui/layout-storage.js";

/** Community column geometry; the Branches page prepends its bookmark path level. */
export function createRepositoryLogCanvas(
  page: StorePage = "log",
  storage?: Storage,
) {
  const branches = page === "branches";
  return createColumnCanvasController({
    columns: {
      branches: {
        defaultWidth: 380,
        minWidth: 300,
        maxWidth: 640,
        pathLevel: 0,
        collapsible: true,
        resizable: true,
      },
      revisions: {
        defaultWidth: 480,
        minWidth: 360,
        maxWidth: 720,
        pathLevel: branches ? 1 : 0,
        collapsible: true,
        resizable: true,
      },
      diff: {
        defaultWidth: 560,
        minWidth: 360,
        maxWidth: 980,
        pathLevel: branches ? 2 : 1,
        closeable: true,
        resizable: true,
      },
      agent: {
        defaultWidth: 660,
        minWidth: 420,
        maxWidth: 1040,
        pathLevel: branches ? 2 : 1,
        closeable: true,
        resizable: true,
      },
      review: {
        defaultWidth: 620,
        minWidth: 400,
        maxWidth: 960,
        pathLevel: branches ? 2 : 1,
        closeable: true,
        resizable: true,
      },
    },
    ...(storage
      ? {
          persistence: createLocalStorageColumnCanvasLayoutPersistence(
            repositoryColumnCanvasStorageKey(page),
            storage,
          ),
        }
      : {}),
  });
}
