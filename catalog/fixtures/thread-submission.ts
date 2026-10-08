import type { Model } from "../../web/src/lib/controller/thread-types.js";
import type { ThreadRequest } from "../../web/src/lib/controller/thread-types.js";
export function applyThreadSubmission(
  submission: Extract<
    ThreadRequest["command"],
    { type: "submit" }
  >["submission"],
  request: ThreadRequest,
  threads: Map<string, Model<"ThreadSummaryV1">>,
  history: Map<string, Model<"ThreadEventItemV1">[]>,
  addThread: (id: string, title: string) => void,
  addMessage: (
    thread: string,
    id: string,
    body: string | null,
    reply?: Model<"CollabObjectRef"> | null,
    previous?: Model<"CollabObjectRef"> | null,
  ) => Model<"CollabObjectRef">,
  ref: (
    kind: Model<"CollabObjectRef">["kind"],
    n: number,
  ) => Model<"CollabObjectRef">,
  next: () => number,
  current: () => number,
) {
  const batch = submission,
    references: Model<"ReactionTargetV1">[] = [...batch.references];
  for (const draft of batch.comments) {
    if (!threads.has(draft.thread_id)) {
      addThread(draft.thread_id, "Comment discussion");
      const thread = threads.get(draft.thread_id)!;
      threads.set(draft.thread_id, {
        ...thread,
        value: {
          ...thread.value,
          comment_source: {
            ...draft.source,
            part: draft.source.part ?? null,
            selection: draft.source.selection ?? null,
          },
          agent_message:
            draft.source.target.type === "human_message"
              ? null
              : draft.source.target,
        },
      });
    }
    const revision = addMessage(
      draft.thread_id,
      draft.message_id,
      draft.body,
      draft.reply_to ?? null,
      draft.expected_revision ?? null,
    );
    references.push({
      type: "human_message",
      thread_id: draft.thread_id,
      message_id: draft.message_id,
      revision,
    });
  }
  const message =
    batch.message_id && batch.text
      ? addMessage(
          batch.thread_id,
          batch.message_id,
          batch.text,
          batch.reply_to ?? null,
        )
      : null;
  const thread = threads.get(batch.thread_id)!,
    reference = ref("thread_link", next()),
    event = ref("thread_event", next());
  history.get(batch.thread_id)!.unshift({
    message: null,
    reference: event,
    value: {
      schema_version: 1,
      id: current().toString(16).padStart(32, "0"),
      thread_id: batch.thread_id,
      actor: thread.value.actor,
      accepted_at_millis: 1725000000000 + current(),
      previous: thread.value.event,
      event: { type: "link", version: reference },
    },
    link: {
      schema_version: 1,
      id: request.request_id,
      thread_id: batch.thread_id,
      actor: thread.value.actor,
      accepted_at_millis: 1725000000000 + current(),
      previous: null,
      active: true,
      target: {
        type: "comment_batch",
        request_id: request.request_id,
        message,
        reply_to: batch.reply_to ?? null,
        comments: references,
      },
    },
  });
  threads.set(batch.thread_id, {
    ...thread,
    reference: ref("thread_version", next()),
    value: { ...thread.value, event, previous: thread.reference },
  });
}
