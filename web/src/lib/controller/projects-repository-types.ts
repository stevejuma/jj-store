import type { Model } from "../api/client.js";
import type { CodeSnapshot as Snapshot } from "./repository-profile.js";

export interface RepositoryDirectorySummary {
  code: Snapshot | null;
  work: import("./work-types.js").WorkSnapshot;
  review: import("./repository-read-types.js").ReviewSnapshot | null;
  description: string;
  contributor: Model<"GraphSignature"> | null;
  updatedAtMillis: number | bigint | null;
  commits: number;
  commitsPartial: boolean;
  threads: Model<"ReviewThreadSummary">[];
  reviewsPartial: boolean;
  tasks: Model<"WorkTaskSummary">[];
  tasksPartial: boolean;
}
