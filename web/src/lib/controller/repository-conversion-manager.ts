import {
  ApiError,
  decode,
  fullId,
  pretty,
  type Exact,
  type Model,
} from "../api/client.js";
import type { operations } from "../api/generated.js";
import { LocalClient } from "../api/local-client.js";
export type ConversionIntent = Exact<
  operations["local_repository_conversion_preview"]["requestBody"]["content"]["application/json"]
>;
type Start = Exact<
  operations["local_repository_conversion_start"]["requestBody"]["content"]["application/json"]
>;
const key = "jj-store.repository-conversion.v1",
  completedKey = `${key}.completed`;
/** Retain exact public intent before dispatch. The installation owns archives and recovery. */
export class RepositoryConversionManager {
  readonly client: LocalClient;
  pending: Start | null = null;
  hasReport = false;
  report: Model<"ConversionReport"> | null = null;
  constructor(
    fetcher: typeof fetch | undefined,
    private storage: Pick<Storage, "getItem" | "setItem" | "removeItem">,
  ) {
    this.client = new LocalClient("", fetcher);
    this.refresh();
  }
  private read(key: string): Start | null {
    const raw = this.storage.getItem(key);
    if (!raw) return null;
    const value = decode(raw) as Start;
    const intent = value.intent;
    if (
      !intent ||
      intent.version !== 1 ||
      typeof intent.name !== "string" ||
      !intent.name.trim() ||
      !["native", "local_jj_git"].includes(intent.source_profile) ||
      !["native", "local_jj_git"].includes(intent.destination_profile) ||
      intent.source_profile === intent.destination_profile ||
      (intent.destination_profile === "native"
        ? intent.path !== null
        : typeof intent.path !== "string")
    )
      throw new ApiError("invalid_conversion_intent");
    fullId(value.request_id, 32);
    fullId(intent.source, 32);
    fullId(intent.operation);
    fullId(value.prepared_source, 64);
    return value;
  }
  refresh() {
    this.pending = this.read(key);
    this.hasReport = Boolean(this.storage.getItem(completedKey));
  }
  close() {
    this.client.close();
  }
  async preview(intent: ConversionIntent) {
    return this.client.request<Model<"ConversionPreview">>(
      "repositories/conversion/preview",
      intent,
    );
  }
  private accept(value: Model<"ConversionOutcome">, request: Start) {
    if (value.status !== "committed") {
      if (value.request_id !== request.request_id)
        throw new ApiError("conversion_identity_mismatch");
      return value;
    }
    const report = value.report;
    if (
      report.version !== 1 ||
      report.request_id !== request.request_id ||
      report.source !== request.intent.source ||
      report.destination === report.source ||
      report.name !== request.intent.name ||
      report.destination_profile !== request.intent.destination_profile ||
      report.archive_digest !== request.prepared_source ||
      report.code_mappings.operations[request.intent.operation] !==
        report.operation
    )
      throw new ApiError("conversion_identity_mismatch");
    fullId(report.destination, 32);
    fullId(report.operation);
    // Retain a small report lookup, before clearing a pending request.
    this.storage.setItem(completedKey, pretty(request));
    if (this.pending?.request_id === request.request_id) {
      this.storage.removeItem(key);
      this.pending = null;
    }
    this.report = report;
    this.hasReport = true;
    return value;
  }
  async start(intent: ConversionIntent, preparedSource: string) {
    this.refresh();
    if (!this.pending) {
      this.pending = {
        request_id: crypto.randomUUID().replaceAll("-", ""),
        intent: decode(pretty(intent)) as ConversionIntent,
        prepared_source: preparedSource,
      };
      this.storage.setItem(key, pretty(this.pending));
    }
    const request = this.pending;
    try {
      return this.accept(
        await this.client.request<Model<"ConversionOutcome">>(
          "repositories/conversion",
          request,
        ),
        request,
      );
    } catch (cause) {
      // Stale is emitted only before a new request is accepted; final publication conflicts retain intent.
      if (cause instanceof ApiError && cause.code === "stale_local_revision") {
        this.storage.removeItem(key);
        this.pending = null;
      }
      throw cause;
    }
  }
  async status() {
    this.refresh();
    const request = this.pending ?? this.read(completedKey);
    if (!request) return null;
    return this.accept(
      await this.client.request<Model<"ConversionOutcome">>(
        `repositories/conversion/requests/${request.request_id}`,
      ),
      request,
    );
  }
}
