import { ApiError, decode, type Model } from "./client.js";
import { validateLocalHead } from "../controller/repository-profile.js";
/** Local freshness has no durable replay ID; reconnect compares retained selections. */
export class LocalEventParser {
  #buffer = "";
  constructor(
    private repository: string,
    private deliver: (value: Model<"LocalFreshnessEvent">) => void,
  ) {}
  push(text: string) {
    this.#buffer += text;
    if (this.#buffer.length > 128 * 1024)
      throw new ApiError("event_frame_limit");
    while (true) {
      const match = /\r?\n\r?\n/u.exec(this.#buffer);
      if (!match) return;
      const frame = this.#buffer.slice(0, match.index);
      this.#buffer = this.#buffer.slice(match.index + match[0].length);
      let name = "";
      const lines: string[] = [];
      for (const line of frame.split(/\r?\n/u)) {
        if (line.startsWith("id:")) throw new ApiError("invalid_local_event");
        if (line.startsWith("event:")) name = line.slice(6).trim();
        if (line.startsWith("data:"))
          lines.push(line.slice(5).replace(/^ /u, ""));
      }
      if (!lines.length) continue;
      const value = decode(lines.join("\n")) as Model<"LocalFreshnessEvent">;
      if (
        !value ||
        !["ready", "changed"].includes(value.type) ||
        name !== (value.type === "ready" ? "ready" : "repository-changed")
      )
        throw new ApiError("invalid_local_event");
      validateLocalHead(value.snapshot, this.repository);
      this.deliver(value);
    }
  }
}
