import { ApiError, fullId, type Model } from "../api/client.js";
import { HttpLocalCodeDataSource } from "./http-local-code-data-source.js";

/** Exact local preparation pins live only for this data source. */
export class HttpLocalIndexDataSource extends HttpLocalCodeDataSource {
  readonly #indexPins = new Map<
    string,
    { operation: string; generation: string }
  >();
  private validateIndexTask(
    value: Model<"LocalIndexTask">,
    pin: { operation: string; generation: string },
    id?: string,
  ) {
    if (
      value.version !== 1 ||
      value.snapshot.version !== 1 ||
      value.snapshot.profile !== this.profile ||
      value.snapshot.repository !== this.repository ||
      value.snapshot.operation !== pin.operation ||
      String(value.snapshot.generation) !== pin.generation ||
      (id !== undefined && value.id !== id) ||
      !["running", "canceling", "ready", "failed", "canceled"].includes(
        value.status,
      )
    )
      throw new ApiError("snapshot_mismatch");
    fullId(value.id, 32);
    return value;
  }
  async prepareIndex(operation: string, signal?: AbortSignal) {
    const head = await this.snapshot();
    signal?.throwIfAborted();
    const pin = {
      operation: fullId(operation),
      generation: String(head.generation),
    };
    const task = this.validateIndexTask(
      await this.client.request<Model<"LocalIndexTask">>(
        "index/tasks",
        {
          operation: pin.operation,
        },
        signal,
      ),
      pin,
    );
    if (this.#indexPins.size >= 32)
      this.#indexPins.delete(this.#indexPins.keys().next().value!);
    this.#indexPins.set(task.id, pin);
    return task;
  }
  async indexTask(id: string, signal?: AbortSignal) {
    const pin = this.#indexPins.get(fullId(id, 32));
    if (!pin) throw new ApiError("task_reset");
    return this.validateIndexTask(
      await this.read<Model<"LocalIndexTask">>(
        `index/tasks/${id}`,
        undefined,
        signal,
      ),
      pin,
      id,
    );
  }
  async cancelIndexTask(id: string, signal?: AbortSignal) {
    const pin = this.#indexPins.get(fullId(id, 32));
    if (!pin) throw new ApiError("task_reset");
    return this.validateIndexTask(
      await this.client.request<Model<"LocalIndexTask">>(
        `index/tasks/${id}/cancel`,
        undefined,
        signal,
        "POST",
      ),
      pin,
      id,
    );
  }
  override close() {
    this.#indexPins.clear();
    super.close();
  }
}
