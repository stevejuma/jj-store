import { promptDeliveryObservations as observations } from "@lapismd/jj-conversations";
import type { AgentEvent } from "../controller/agent-execution-types.js";
export {
  authoredPrompt,
  promptDelivery,
  promptDeliveries,
} from "@lapismd/jj-conversations";

/** Existing UI sequence arithmetic stays at its original display boundary. */
export function promptDeliveryObservations(events: readonly AgentEvent[]) {
  return new Map(
    [...observations(events)].map(([id, value]) => [
      id,
      {
        state: value.state,
        sequence: Number(value.sequence),
      },
    ]),
  );
}
