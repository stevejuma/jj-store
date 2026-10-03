import type { Exact, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
import type { StoreLoadStatus } from "./contracts.js";

export type CaptureControl = Exact<
  NonNullable<
    operations["local_capture_control"]["requestBody"]
  >["content"]["application/json"]
>;
export type CaptureRescan = Exact<
  NonNullable<
    operations["local_capture_rescan"]["requestBody"]
  >["content"]["application/json"]
>;
export type CaptureImport = Model<"CaptureImportCommand">;
export type CaptureStatus = Model<"CaptureStatus">;
export type CaptureSourcePage = Model<"CaptureSourcePage">;
export type CaptureImportOutcome = Model<"CaptureImportOutcome">;
export type CaptureIntent =
  | { kind: "control"; command: CaptureControl; uncertain: boolean }
  | { kind: "import"; command: CaptureImport; uncertain: boolean };
export interface CaptureTransport {
  status(signal?: AbortSignal): Promise<Model<"CaptureStatus">>;
  sources(
    cursor?: string,
    signal?: AbortSignal,
  ): Promise<Model<"CaptureSourcePage">>;
  control(command: CaptureControl): Promise<Model<"CaptureControlOutcome">>;
  rescan(command: CaptureRescan): Promise<Model<"CaptureRescanOutcome">>;
  import(command: CaptureImport): Promise<Model<"CaptureImportOutcome">>;
  inspect(
    id: string,
    signal?: AbortSignal,
  ): Promise<Model<"CaptureImportStatus">>;
  close(): void;
}
export interface CaptureManagementState {
  value: Model<"CaptureStatus"> | null;
  status: StoreLoadStatus;
  sources: Model<"CaptureSourcePage"> | null;
  sourceState: StoreLoadStatus;
  busy: boolean;
  error: string;
  message: string;
  pending: CaptureIntent | null;
  controlOutcome: Model<"CaptureControlOutcome"> | null;
  importStatus: Model<"CaptureImportStatus"> | null;
  importRequestDraft: string;
}
export interface CaptureManagement {
  readonly supported: boolean;
  readonly state: CaptureManagementState;
  refresh(): Promise<void>;
  setActive(active: boolean): void;
  moreSources(): Promise<void>;
  control(action: "pause" | "resume"): Promise<void>;
  rescan(): Promise<void>;
  importHistory(selector: string): Promise<void>;
  inspectImport(id: string): Promise<void>;
  retry(): Promise<void>;
  destroy(): void;
}
export const captureProfiles = {
  "codex-cli-jsonl-v1": "Codex CLI",
  "codex-desktop-jsonl-v1": "Codex Desktop",
  "cursor-desktop-transcript-v1": "Cursor Desktop",
  "opencode-read-only-api-v1": "OpenCode API",
  "opencode-export-json-v1": "OpenCode export",
} as const;
export const emptyCaptureManagement = (): CaptureManagementState => ({
  value: null,
  status: "idle",
  sources: null,
  sourceState: "idle",
  busy: false,
  error: "",
  message: "",
  pending: null,
  controlOutcome: null,
  importStatus: null,
  importRequestDraft: "",
});

export const captureHistorySupported = (profile: string) =>
  profile === "codex-cli-jsonl-v1" || profile === "codex-desktop-jsonl-v1";
