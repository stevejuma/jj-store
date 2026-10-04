import {
  createColumnCanvasController,
  createLocalStorageColumnCanvasLayoutPersistence,
} from "@lapismd/design-core/shadcn/column-canvas";
import type { StorePage } from "../ui/community-navigation/model.js";
import { repositoryColumnCanvasStorageKey } from "../ui/layout-storage.js";

export const COLLABORATION_COMPACT_BREAKPOINT = 720;

export function createCollaborationCanvas(
  page: Extract<StorePage, "agents" | "reviews">,
  storage?: Storage,
) {
  return createColumnCanvasController({
    columns: {
      collection: {
        defaultWidth: 380,
        minWidth: 300,
        maxWidth: 620,
        pathLevel: 0,
        collapsible: true,
        resizable: true,
      },
      detail: {
        defaultWidth: 660,
        minWidth: 420,
        maxWidth: 1040,
        pathLevel: 1,
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
