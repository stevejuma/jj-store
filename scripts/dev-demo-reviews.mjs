import { createHash } from "node:crypto";
import { seedAgentReviews } from "./dev-demo-agent-reviews.mjs";

function requestId(label) {
  return createHash("sha256")
    .update(`jj-store/dev-demo-review/v1/${label}`)
    .digest("hex")
    .slice(0, 32);
}

function body(text) {
  return {
    schema_version: 1,
    format: "safe_markdown",
    text,
    sanitizer_version: "review-markdown-v1",
  };
}

function anchor(repositoryId, target, createdContext = {}) {
  return {
    schema_version: 1,
    repository_id: repositoryId,
    target,
    created_context: {
      publication_position: null,
      code_state_version: null,
      agent_root: null,
      ...createdContext,
    },
    sensitivity: "repository",
  };
}

export async function seedReviews({ origin, repositoryId, references }) {
  const base = `${origin}v1/repos/${repositoryId}/`;
  const headers = {
    Origin: new URL(origin).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const api = async (path, options = {}) => {
    const response = await fetch(`${base}${path}`, {
      ...options,
      headers: { ...headers, ...options.headers },
    });
    if (!response.ok) {
      throw new Error(
        `development collaboration seed failed: ${response.status} ${await response.text()}`,
      );
    }
    return response.json();
  };
  let request = 0;
  const apply = async (label, command) => {
    const outcome = await api("reviews/commands", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        version: 1,
        request_id: requestId(`${String(request++).padStart(2, "0")}-${label}`),
        command,
      }),
    });
    if (outcome.status !== "committed") {
      throw new Error(`development review seed outcome was ${outcome.status}`);
    }
    return outcome.receipt.result;
  };
  const create = (label, target, text, context) =>
    apply(label, {
      command: "create_thread",
      anchor: anchor(repositoryId, target, context),
      body: body(text),
    });

  const repository = await create(
    "repository",
    { target: "repository" },
    "## Repository readiness\n\nThe sample keeps immutable collaboration data across restarts.",
  );
  const repositoryReply = await apply("repository-reply", {
    command: "append_reply",
    thread_id: repository.thread_id,
    expected_workflow_version: 1,
    parent_comment_id: repository.comment_id,
    body: body(
      "Confirmed after a cold native read. I added the recovery evidence.",
    ),
  });
  await apply("repository-edit", {
    command: "edit_comment",
    comment_id: repositoryReply.comment_id,
    expected_revision_id: repositoryReply.comment_revision.id,
    replacement_body: body(
      "Confirmed after a cold native read. The recovery evidence now covers agent and review roots.",
    ),
  });

  const operation = await create(
    "operation",
    { target: "operation", value: { operation_id: references.operationId } },
    "This operation captures the representative branching and conflict history.",
  );
  await apply("operation-resolve", {
    command: "resolve_thread",
    thread_id: operation.thread_id,
    expected_thread_version: operation.thread_snapshot.id,
    reason: "Operation shape verified",
  });

  const revision = await create(
    "revision",
    {
      target: "revision",
      value: {
        operation_id: references.operationId,
        commit_id: references.commitId,
        change_id: references.changeId,
      },
    },
    "The browser commit should keep the native commit and change identities visible.",
  );
  const obsolete = await apply("revision-reply", {
    command: "append_reply",
    thread_id: revision.thread_id,
    expected_workflow_version: 1,
    parent_comment_id: revision.comment_id,
    body: body("This note is superseded by the exact tree check."),
  });
  const tombstoned = await apply("revision-tombstone", {
    command: "tombstone_comment",
    comment_id: obsolete.comment_id,
    expected_revision_id: obsolete.comment_revision.id,
    reason: "Superseded by exact tree evidence",
  });
  const resolvedRevision = await apply("revision-resolve", {
    command: "resolve_thread",
    thread_id: revision.thread_id,
    expected_thread_version: tombstoned.thread_snapshot.id,
    reason: "Revision checked",
  });
  await apply("revision-reopen", {
    command: "reopen_thread",
    thread_id: revision.thread_id,
    expected_thread_version: resolvedRevision.thread_snapshot.id,
    reason: "Keeping one revision discussion open for the sample",
  });

  const tree = await api(
    `commits/${references.commitId}/tree?at_op=${references.operationId}&path=src&limit=50`,
  );
  const file = tree.data.entries
    .find((entry) => entry.name === "lib.rs")
    ?.terms.find((term) => term?.type === "file");
  if (!file)
    throw new Error("development review seed could not resolve src/lib.rs");
  const sourceResponse = await fetch(
    `${base}files/${file.id}?at_op=${references.operationId}`,
    { headers },
  );
  if (!sourceResponse.ok)
    throw new Error("development review seed could not read src/lib.rs");
  const source = await sourceResponse.text();
  const selected = "supports_native_history";
  const selectedIndex = source.indexOf(selected);
  if (selectedIndex < 0)
    throw new Error("development review seed could not select src/lib.rs text");
  const start = Buffer.byteLength(source.slice(0, selectedIndex));
  const end = start + Buffer.byteLength(selected);
  const selection = {
    encoding: "utf8",
    start_byte: String(start),
    end_byte: String(end),
    selected_sha256: createHash("sha256").update(selected).digest("hex"),
  };
  await create(
    "code-text",
    {
      target: "code_text",
      value: {
        operation_id: references.operationId,
        commit_id: references.commitId,
        change_id: references.changeId,
        path: "src/lib.rs",
        file_id: file.id,
        value_kind: "file",
        text: selection,
        diff_context: null,
      },
    },
    "Can this exact UTF-8 selection stay pinned while the working copy advances?",
  );
  const artifact = await create(
    "code-artifact",
    {
      target: "code_artifact",
      value: {
        operation_id: references.operationId,
        commit_id: references.commitId,
        change_id: references.changeId,
        path: "src/lib.rs",
        object: { kind: "file", id: file.id },
        value_kind: "file",
      },
    },
    "The complete file object is the immutable review artifact.",
  );
  await apply("code-artifact-resolve", {
    command: "resolve_thread",
    thread_id: artifact.thread_id,
    expected_thread_version: artifact.thread_snapshot.id,
    reason: "Artifact identity verified",
  });

  await seedAgentReviews({
    api,
    apply,
    create,
    repositoryId,
    requestId,
  });
}
