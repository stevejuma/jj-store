import type { DirectoryManagement } from "./directory.svelte.js";
import type { ThreadRequest } from "./thread-types.js";

/** Prepare exact library snapshots before retaining the repository command. */
export function prepareThreadEmoji(
  request: ThreadRequest,
  profiles: DirectoryManagement | undefined,
): Promise<void> | undefined {
  if (!profiles?.available()) return;
  const command = request.command;
  if (command.type === "message" && command.body)
    return profiles.snapshots(command.body).then((values) => {
      command.emojis = values;
    });
  if (command.type === "submit") {
    const submission = command.submission;
    return profiles
      .snapshots(
        [submission.text ?? "", ...submission.comments.map((c) => c.body)].join(
          "\n",
        ),
      )
      .then((values) => {
        submission.emojis = values;
      });
  }
}
