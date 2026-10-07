import { createThreadTransport } from "./thread-transport.js";
import { fixtureJsonResponse as json } from "./agent-execution-session.js";
import { targetKey } from "../../web/src/lib/collaboration/thread-presentation.js";
import type { Model } from "../../web/src/lib/controller/thread-types.js";

export function seedReplayDrafts(drafts: Record<string, string>) {
  const session = "44".repeat(16);
  drafts[`${session}:open-${session}/assistant`] =
    "Is there a license in the repo?";
  drafts[`${session}:${"55".repeat(16)}/user`] =
    "Keep the existing prompt reply";
}

export function agentDiscussionFixture(
  enabled: boolean,
  unknownFirst = false,
  writeUnavailable = false,
) {
  const { transport } = createThreadTransport({ unknownFirst });
  const targets = ["user", "assistant"].map((role, i) => ({
    row_id: `${"55".repeat(16)}/${role}`,
    target: {
      type: "agent_message" as const,
      session_id: "99".repeat(16),
      message_id: (i ? "bb" : "aa").repeat(16),
      event_id: (i ? "dd" : "cc").repeat(16),
      batch: { kind: "event_batch" as const, id: "ee".repeat(32) },
      content: { kind: "content" as const, id: (i ? "22" : "11").repeat(32) },
    },
  }));
  return async (path: string, init?: RequestInit) => {
    if (path.includes("/feedback")) {
      if (!enabled) return json({ messages: [] });
      const list = await transport.list();
      const threads = list.items.type === "threads" ? list.items.threads : [];
      const messages: Model<"AgentExecutionFeedbackMessage">[] = [];
      for (const row of targets) {
        const ids = threads
          .filter(
            (t) =>
              t.value.agent_message &&
              targetKey(t.value.agent_message) === targetKey(row.target),
          )
          .map((t) => t.value.thread_id);
        const replies: Model<"AgentExecutionFeedbackReply">[] = [];
        for (const id of ids) {
          const page = await transport.events(id);
          if (page.items.type === "events")
            for (const event of page.items.events) {
              if (event.message && event.value.event.type === "message")
                replies.push({
                  thread_id: id,
                  message: event.message,
                  revision: event.value.event.revision,
                });
            }
        }
        messages.push({ ...row, threads: ids, replies });
      }
      return json({
        messages,
        ...(writeUnavailable
          ? {
              write_unavailable:
                "Replies and reactions are unavailable for this code repository until collaboration replication is qualified.",
            }
          : {}),
      });
    }
    if (!path.startsWith("/v1/repos/")) return undefined;
    const body = init?.body ? JSON.parse(String(init.body)) : undefined;
    const query = new URL(path, "http://fixture");
    if (path.includes("/reaction-assets/")) {
      const asset = await transport.asset(query.pathname.split("/").at(-1)!);
      return new Response(new Uint8Array(asset.bytes).buffer, {
        headers: { "content-type": asset.type },
      });
    }
    if (path.includes("/reaction-emojis"))
      return json(await transport.emojis());
    if (path.includes("/reactions/query"))
      return json(await transport.reactions(body.target));
    const kind = path.includes("/reactions/") ? "reactions" : "threads";
    if (path.includes("/commands/"))
      return json(
        await transport.status(kind, query.pathname.split("/").at(-1)!),
      );
    if (path.endsWith("/commands"))
      return json(await transport.command(kind, body));
    const id = query.pathname.split("/")[5];
    if (path.includes("/timeline")) return json(await transport.events(id));
    if (id) return json(await transport.thread(id));
    return json(await transport.list());
  };
}
