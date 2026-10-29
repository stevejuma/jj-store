import {
  ConversationClient,
  promptDeliveryObservations,
} from "@lapismd/jj-conversations";
type Command = { request_id: string; text: string; counter: bigint };
type Receipt = { request_id: string; sequence: bigint; delivery: string };
const client = new ConversationClient<Command, Receipt>(
  "vault",
  {
    submit: async (command) => ({
      request_id: command.request_id,
      sequence: 1n,
      delivery: "queued",
    }),
    lookup: async (id) => ({
      request_id: id,
      sequence: 1n,
      delivery: "unknown",
    }),
  },
  (value) => `{"request_id":"${value.request_id}","counter":${value.counter}}`,
);
const typed: Promise<Receipt> = client.send({
  request_id: "request",
  text: "Hello",
  counter: 9007199254740995n,
});
void typed;
const sequence: bigint = promptDeliveryObservations([
  {
    sequence: 9007199254740995n,
    payload: { type: "started", requestId: "request" },
  },
]).get("request")!.sequence;
// @ts-expect-error Native sequence widths must not silently narrow to number.
const narrowed: number = sequence;
void narrowed;
