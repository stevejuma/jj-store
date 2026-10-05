import type { Model } from "../api/client.js";
export type WorkingFile = Model<"WorkingFile">;
export type WorkingFileSelection = Model<"WorkingFileSelection">;
export interface WorkingCopyTransport {
  read(path: string): Promise<WorkingFile>;
  save(selection: WorkingFileSelection, contents: string): Promise<WorkingFile>;
}
