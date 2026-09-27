import { createColumnCanvasController } from "@lapismd/design-core/shadcn/column-canvas";
/** Community column geometry; the Branches page prepends its bookmark path level. */
export function createRepositoryLogCanvas(branches = false) {
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
      operation: {
        defaultWidth: 360,
        minWidth: 280,
        maxWidth: 720,
        pathLevel: 1,
        closeable: true,
        resizable: true,
      },
      diff: {
        defaultWidth: 560,
        minWidth: 360,
        maxWidth: 980,
        pathLevel: branches ? 3 : 2,
        closeable: true,
        resizable: true,
      },
    },
  });
}
