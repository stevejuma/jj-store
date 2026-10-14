import { expect, test, vi } from "vitest";
import { workingCopyActions } from "../../src/lib/controller/working-copy-actions.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type {
  WorkingFile,
  WorkingFileSelection,
} from "../../src/lib/controller/working-copy-types.js";

test("working-copy actions preserve the exact selection and refuse late results from another session", async () => {
  const file = {} as WorkingFile;
  const selection = {} as WorkingFileSelection;
  let release!: (file: WorkingFile) => void;
  const read = vi.fn(
    () =>
      new Promise<WorkingFile>((done) => {
        release = done;
      }),
  );
  const save = vi.fn(async () => file);
  let source = { workingCopy: { read, save } } as unknown as StoreDataSource;
  const actions = workingCopyActions(() => source);
  await expect(
    actions.saveWorkingFile!(selection, "exact bytes\n"),
  ).resolves.toBe(file);
  expect(save).toHaveBeenCalledWith(selection, "exact bytes\n");
  const pending = actions.openWorkingFile!("draft.md");
  source = {} as StoreDataSource;
  release(file);
  await expect(pending).rejects.toMatchObject({ name: "AbortError" });
  expect(read).toHaveBeenCalledWith("draft.md");
  await expect(actions.openWorkingFile!("draft.md")).rejects.toThrow(
    "unavailable",
  );
  await expect(
    actions.saveWorkingFile!(selection, "new bytes"),
  ).rejects.toThrow("unavailable");
  expect(save).toHaveBeenCalledTimes(1);
});
