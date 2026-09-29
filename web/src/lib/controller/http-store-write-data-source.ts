import {
  Client,
  fullId,
  pretty,
  type Head,
  type Model,
  type Task,
} from "../api/client.js";
import type { StoreMutationRequest } from "./store-mutation-types.js";

const mutationInit = (request: StoreMutationRequest): RequestInit => ({
  method: "POST",
  headers: { "content-type": "application/json" },
  body: pretty(request),
});

export class HttpStoreWriteDataSource {
  constructor(private client: Client) {}

  uploadConflict(
    head: Head,
    commit: string,
    path: string,
    fingerprint: string,
    bytes: Uint8Array,
  ) {
    return this.client.request<Model<"ConflictUpload">>(
      "conflict-uploads",
      {
        epoch: head.epoch,
        position: head.published_seq,
        code_state_version: head.code_state_version,
        operation: fullId(head.heads[0]!),
        commit: fullId(commit),
        path,
        fingerprint: fullId(fingerprint),
      },
      {
        method: "POST",
        headers: { "content-type": "application/octet-stream" },
        body: Uint8Array.from(bytes).buffer,
      },
    );
  }

  previewMutation(request: StoreMutationRequest) {
    return this.client.request<Model<"Preview">>(
      "mutations/preview",
      {},
      mutationInit(request),
    );
  }

  applyMutation(request: StoreMutationRequest) {
    return this.client.request<Model<"Outcome">>(
      "mutations",
      {},
      mutationInit(request),
    );
  }

  mutationStatus(requestId: string) {
    return this.client.request<Model<"Status">>(
      `mutations/${fullId(requestId, 32)}`,
    );
  }

  prepareIndex(operation: string) {
    return this.client.request<Task>(
      "index/tasks",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ operation }),
      },
    );
  }

  indexTask(id: string) {
    return this.client.request<Task>(`index/tasks/${encodeURIComponent(id)}`);
  }
}
