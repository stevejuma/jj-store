import type { WorkspaceFileSource } from "./workspace-types.js";
import { middleFoldKey } from "../ui/store-agent-chat-body/middle-fold.js";
import { tick } from "svelte";
import { SvelteMap } from "svelte/reactivity";
import { bytesToHex } from "@noble/hashes/utils.js";
import { sha256 } from "@noble/hashes/sha2.js";
import type { ChatBodyBlock } from "../ui/store-agent-chat-body/model.js";
import { ChatBodyComments } from "../ui/store-agent-chat-body/comments.svelte.js";
import type {
  ChatBodyProps,
  ChatPromptSubmission,
} from "../ui/store-agent-chat-body/model.js";
import type { Model } from "../api/client.js";
import type { ConversationProjection } from "./agent-adapter.js";
import { validAnchor } from "../ui/store-agent-chat-body/anchors.js";
import { exactSource } from "./source-map.js";

/** Owned by an application controller, never a renderer or browser storage. */
export class ConversationSession {
  projection = $state<ConversationProjection>({
    items: [],
    sources: new Map(),
    threads: [],
    targets: new Map(),
  });
  host = $state<Partial<ChatBodyProps>>({});
  readonly workspaceSources = new SvelteMap<
    string,
    { block: ChatBodyBlock; source: WorkspaceFileSource; reveal: () => void }
  >();
  initialized = false;
  editing = $state<{
    id: string;
    text: string;
    revision: Model<"CollabObjectRef">;
    thread: Model<"CollabObjectRef">;
  }>();
  readonly comments = new ChatBodyComments(() => ({
    ...this.host,
    items: this.projection.items,
    externalBlocks: [...this.workspaceSources.values()].map((v) => v.block),
    onNavigateSource: async (thread) => {
      const file = this.workspaceSources.get(thread.anchor.blockId);
      if (file) {
        file.reveal();
        this.comments.disclosures[`file:${file.block.id}`] = true;
        this.comments.disclosures[middleFoldKey(file.block)] = true;
        await tick();
        await tick();
        this.comments.readers.get(file.block.id)?.(thread.anchor, true);
      } else await this.host.onNavigateSource?.(thread);
    },
  }));
  adopt(projection: ConversationProjection) {
    this.projection = projection;
    // A complete discussion read may delete/tombstone a published revision.
    // Missing pages or unavailable reads are not evidence of deletion.
    for (const thread of this.comments.threads) {
      if (!projection.authoritativeDiscussions?.has(thread.id)) continue;
      const incoming = projection.threads.find((t) => t.id === thread.id);
      thread.comments = thread.comments.filter(
        (c) =>
          !c.revision ||
          c.status === "draft" ||
          c.status === "sending" ||
          incoming?.comments.some(
            (n) =>
              (n.messageId ?? n.id) === (c.messageId ?? c.id) &&
              n.revision?.id === c.revision?.id,
          ),
      );
    }
    this.comments.threads = this.comments.threads.filter(
      (t) => t.comments.length || this.comments.active?.threadId === t.id,
    );
    for (const incoming of projection.threads) {
      const thread = this.comments.threads.find((t) => t.id === incoming.id);
      if (!thread) {
        this.comments.threads.push(incoming);
        continue;
      }
      thread.revision = incoming.revision;
      thread.source = incoming.source;
      // Keep the exact anchor of pending drafts if their source has changed.
      if (
        !thread.comments.some(
          (c) => c.status === "draft" || c.status === "sending",
        )
      )
        thread.anchor = incoming.anchor;
      for (const next of incoming.comments) {
        const existing = thread.comments.find(
          (c) =>
            c.id === next.id ||
            (c.messageId === next.messageId &&
              c.revision?.id === next.revision?.id) ||
            ((c.status === "sending" || !c.revision) &&
              (c.messageId ?? c.id) === next.messageId &&
              c.text === next.text),
        );
        if (existing) {
          if (existing.status !== "draft")
            Object.assign(existing, { ...next, id: existing.id });
        } else thread.comments.push(next);
      }
    }
  }
  prepare(
    submission: ChatPromptSubmission,
    parent: { id: string; revision: Model<"CollabObjectRef"> },
    ordinary = false,
  ): NonNullable<
    import("../controller/agent-chat-types.js").ChatSend["comments"]
  > {
    const blocks = this.projection.items.flatMap((i) => [
      ...i.blocks,
      ...(i.tools ?? []).flatMap((t) => t.blocks),
    ]);
    return {
      thread_id: parent.id,
      expected_thread: parent.revision,
      message_id: ordinary && submission.text.trim() ? submission.id : null,
      text: ordinary && submission.text.trim() ? submission.text : null,
      references: [],
      workspace_comments: submission.comments
        .filter((e) => this.workspaceSources.has(e.anchor.blockId))
        .map((entry) => {
          const file = this.workspaceSources.get(entry.anchor.blockId)!;
          if (!validAnchor(file.block, entry.anchor))
            throw new Error(
              "The workspace selection changed. Your draft is retained.",
            );
          const bytes = new TextEncoder();
          // Avoid an invalid UTF-16 split before converting exact editor offsets.
          const quote = file.block.text.slice(
            entry.anchor.from,
            entry.anchor.to,
          );
          if (
            new TextDecoder("utf-8", { fatal: true }).decode(
              bytes.encode(quote),
            ) !== quote
          )
            throw new Error("Select complete characters.");
          return {
            id: entry.commentId,
            body: entry.text,
            previous: null,
            edit: false,
            source: {
              ...file.source,
              selection:
                entry.anchor.from === entry.anchor.to
                  ? null
                  : {
                      encoding: "utf8" as const,
                      start_byte: String(
                        bytes.encode(
                          file.block.text.slice(0, entry.anchor.from),
                        ).length,
                      ),
                      end_byte: String(
                        bytes.encode(file.block.text.slice(0, entry.anchor.to))
                          .length,
                      ),
                      selected_sha256: bytesToHex(sha256(bytes.encode(quote))),
                    },
            },
          };
        }),
      comments: submission.comments
        .filter((e) => !this.workspaceSources.has(e.anchor.blockId))
        .map((entry) => {
          const block = blocks.find((b) => b.id === entry.anchor.blockId),
            source = this.projection.sources.get(entry.anchor.blockId);
          const thread = this.comments.threads.find(
            (t) => t.id === entry.threadId,
          );
          if (!block || !source || !thread || !validAnchor(block, entry.anchor))
            throw new Error(
              "The exact published source is unavailable. Your draft is retained.",
            );
          const selector =
            thread.source ?? exactSource(block, entry.anchor, source);
          return {
            thread_id: entry.threadId,
            expected_thread: thread.revision ?? null,
            source: selector,
            message_id:
              thread.comments.find((c) => c.id === entry.commentId)
                ?.messageId ?? entry.commentId,
            expected_revision:
              thread.comments.find((c) => c.id === entry.commentId)?.revision ??
              null,
            reply_to: null,
            body: entry.text,
          };
        }),
    };
  }
}
export class ConversationSessions {
  destination = $state<{
    repository: string;
    source: Model<"ConversationSourceV1">;
    itemId?: string;
  }>();
  #sessions = new Map<string, ConversationSession>();
  get(key: string) {
    let session = this.#sessions.get(key);
    if (!session) {
      session = new ConversationSession();
      this.#sessions.set(key, session);
    }
    return session;
  }
  clear() {
    this.#sessions.clear();
  }
}
