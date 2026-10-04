import type { Model } from "../api/client.js";
export type HistoryQueryInfo = Model<"QueryInfo">;
export interface HistoryQueryController {
  readonly draft: string;
  readonly email: string;
  readonly error: string;
  readonly pending: boolean;
  readonly info: HistoryQueryInfo | null;
  setDraft(value: string): void;
  setEmail(value: string): void;
  /** Load the session identity hint before initial or explicit history reads. */
  prepare(): Promise<void>;
  run(): Promise<void>;
  clear(): void;
  dispose(): void;
}
