import type { Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { NativeReviewContext } from "./native-activity-types.js";
import { readAnchoredAgentContext } from "./native-agent-context.js";
import { readCommitActivity } from "./native-activity-reader.js";
import {
  readCommitInspection,
  readOperationInspection,
  readRevision,
  type Comparison,
} from "./production-store-loaders.js";
import { assertPinned, assertSameSnapshot } from "./production-snapshot.js";
import { presentChangedFiles, presentPatch } from "./native-presentation.js";

export async function readNativeReviewContext(
  source: StoreDataSource,
  anchor: Model<"ArtifactAnchorV1">,
): Promise<NativeReviewContext> {
  const result: NativeReviewContext = {
    commits: [],
    files: [],
    patches: {},
    agents: { sessions: [], error: "" },
    codeError: "",
  };
  if (anchor.repository_id !== source.repository)
    throw new Error("Review repository mismatch.");
  const target = anchor.target;
  if (
    target.target === "agent_text" ||
    target.target === "agent_artifact" ||
    target.target === "agent_session"
  ) {
    result.agents = await readAnchoredAgentContext(source, anchor);
    return result;
  }
  if (target.target === "repository") return result;
  if (
    target.target === "portable_code_text" ||
    target.target === "portable_code_artifact"
  ) {
    result.codeError =
      "This source requires the retained portable-file reader.";
    return result;
  }
  try {
    const operation = target.value.operation_id;
    let comparisons: Map<string, Comparison>;
    if (target.target === "operation") {
      const inspection = await readOperationInspection(source, operation);
      if (inspection.parents.length)
        throw new Error(
          "This operation has multiple parents. Select a comparison in Op Log.",
        );
      if (inspection.unavailableComparisons)
        throw new Error(
          "Some native operation comparisons are unavailable. Open Op Log for the recorded change groups.",
        );
      if (inspection.continuation)
        throw new Error(
          "This operation comparison exceeds the review detail bound. Open Op Log for the complete comparison.",
        );
      result.files = inspection.detail.files;
      comparisons = inspection.comparisons;
      const ids = [...new Set([...comparisons.values()].map((c) => c.to))];
      if (ids.length > 20)
        throw new Error(
          "This operation has more than 20 revisions. Open Op Log for the complete comparison.",
        );
      for (const id of ids)
        result.commits.push(
          await readCommitActivity(
            source,
            operation,
            await readRevision(source, operation, id),
          ),
        );
    } else {
      const commit = target.value.commit_id;
      const revision = await readRevision(source, operation, commit);
      result.commits.push(
        await readCommitActivity(source, operation, revision),
      );
      const context =
        target.target === "code_text" ? target.value.diff_context : null;
      if (context) {
        const comparison = {
          from: context.before_commit_id,
          to: context.after_commit_id,
          beforeOperation: context.before_operation_id,
          afterOperation: context.after_operation_id,
        };
        const page = await source.diffPaths(
          context.after_operation_id,
          comparison.from,
          comparison.to,
        );
        assertPinned(page.snapshot, source, context.after_operation_id);
        if (page.next_cursor)
          throw new Error(
            "This explicit comparison exceeds the review detail bound. Open the commit comparison for all files.",
          );
        result.files = presentChangedFiles(page.data.changes);
        comparisons = new Map(
          result.files.map((file) => [file.path, comparison]),
        );
      } else {
        const inspection = await readCommitInspection(
          source,
          operation,
          commit,
        );
        result.files = inspection.detail.files;
        comparisons = inspection.comparisons;
        let continuation = inspection.continuation;
        const cursors = new Set<string>();
        while (continuation?.kind === "commit") {
          if (cursors.has(continuation.cursor))
            throw new Error("Repeated review file cursor.");
          cursors.add(continuation.cursor);
          const page = await source.diffPaths(
            operation,
            continuation.from,
            continuation.to,
            continuation.cursor,
          );
          assertPinned(page.snapshot, source, operation);
          assertSameSnapshot(page.snapshot, continuation.snapshot);
          result.files.push(...presentChangedFiles(page.data.changes));
          if (result.files.length > 1000 || cursors.size > 100)
            throw new Error("Review files exceed the browser read bound.");
          for (const file of page.data.changes)
            comparisons.set(file.path, {
              from: continuation.from,
              to: continuation.to,
              beforeOperation: operation,
              afterOperation: operation,
            });
          continuation = page.next_cursor
            ? { ...continuation, cursor: page.next_cursor }
            : null;
        }
      }
    }
    if (result.files.length > 100)
      throw new Error(
        "More than 100 files. Open the native comparison for the complete diff.",
      );
    for (const file of result.files) {
      const comparison = comparisons.get(file.path);
      if (comparison) {
        const diff = await source.diff(
          comparison.afterOperation,
          comparison.from,
          comparison.to,
          file.path,
        );
        assertPinned(diff.snapshot, source, comparison.afterOperation);
        const change = diff.data.changes.find(
          (change) => change.path === file.path,
        );
        if (change) {
          Object.assign(file, presentChangedFiles([change])[0]);
          const patch = presentPatch(change);
          if (patch !== undefined) result.patches[file.path] = patch;
        }
      }
    }
  } catch (cause) {
    result.codeError =
      cause instanceof Error
        ? cause.message
        : "Review code context unavailable";
    result.files = [];
    result.patches = {};
  }
  return result;
}
