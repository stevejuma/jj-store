import type { Model, Snapshot } from "../api/client.js";

export interface RepositoryDirectorySummary {
  code: Snapshot | null;
  work: Model<"WorkReadSnapshot">;
  review: Model<"ReviewReadSnapshot">;
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
