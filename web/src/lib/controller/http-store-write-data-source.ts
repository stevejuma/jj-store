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
  readonly #codeTasks = new Set<string>();
  constructor(private client: Client) {}
  async prepareCodeIndex(operation: string, signal?: AbortSignal) {
    const task = await this.client.request<Task>(
      "code-index/tasks",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ operation: fullId(operation) }),
        signal,
      },
    );
    if (this.#codeTasks.size >= 64)
      this.#codeTasks.delete(this.#codeTasks.values().next().value!);
    this.#codeTasks.add(fullId(task.id, 32));
    return task;
  }

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

  prepareKindIndex(
    operation: string,
    kind: Model<"KindPreparationKind">,
    signal?: AbortSignal,
  ) {
    return this.client.request<Model<"KindPreparationTask">>(
      "index/kinds/tasks",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ operation: fullId(operation), kind }),
        signal,
      },
    );
  }
  kindIndexTask(id: string, signal?: AbortSignal) {
    return this.client.request<Model<"KindPreparationTask">>(
      `index/kinds/tasks/${encodeURIComponent(id)}`,
      {},
      { signal },
    );
  }
  cancelKindIndexTask(id: string, signal?: AbortSignal) {
    return this.client.request<Model<"KindPreparationTask">>(
      `index/kinds/tasks/${encodeURIComponent(id)}/cancel`,
      {},
      { method: "POST", signal },
    );
  }
  prepareIndex(operation: string, signal?: AbortSignal) {
    return this.client.request<Task>(
      "index/tasks",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ operation }),
        signal,
      },
    );
  }

  indexTask(id: string, signal?: AbortSignal) {
    return this.client.request<Task>(
      `${this.#codeTasks.has(id) ? "code-index" : "index"}/tasks/${encodeURIComponent(id)}`,
      {},
      { signal },
    );
  }
  cancelIndexTask(id: string, signal?: AbortSignal) {
    return this.client.request<Task>(
      `${this.#codeTasks.has(id) ? "code-index" : "index"}/tasks/${encodeURIComponent(id)}/cancel`,
      {},
      { method: "POST", signal },
    );
  }
}
