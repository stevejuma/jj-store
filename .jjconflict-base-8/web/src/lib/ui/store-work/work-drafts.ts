import type { WorkController } from "../../controller/work-types.js";
import type { WorkDraft } from "../../controller/work-types.js";
export function workDraft(
  work: WorkController,
  key: string,
  values: Partial<WorkDraft> = {},
): WorkDraft {
  if (!work.state.drafts) work.state.drafts = {};
  const drafts = work.state.drafts;
  if (!drafts[key])
    drafts[key] = {
      title: "",
      body: "",
      repository: "",
      parent: "",
      status: "open",
      reply: null,
      edit: null,
      ...values,
    };
  return drafts[key]!;
}
