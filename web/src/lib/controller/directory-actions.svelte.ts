import { ApiError, type Model } from "../api/client.js";
import type {
  DirectoryClient,
  DirectoryRequest as Request,
} from "../api/directory-client.js";
import type { DirectoryState } from "./directory-model.js";
type Receipt = Model<"DirectoryReceipt">;
type Emoji = Model<"LibraryEmoji">;
const copy = <T>(value: T): T => structuredClone($state.snapshot(value)) as T;
export function createDirectoryActions(options: {
  state: DirectoryState;
  client: DirectoryClient;
  image(value: string): Promise<string>;
  observe(): Promise<void>;
  epoch(): number;
}) {
  const { state, client, image, observe } = options;
  async function execute(request: Request): Promise<Receipt> {
    const ticket = options.epoch();
    state.busy = true;
    state.error = "";
    state.message = "";
    try {
      const receipt = await client.request<Receipt>("commands", request);
      if (receipt.request_id !== request.request_id)
        throw new ApiError("invalid_directory_receipt");
      if (ticket !== options.epoch()) return receipt;
      accept(receipt);
      return receipt;
    } catch (cause) {
      if (ticket !== options.epoch()) throw cause;
      if (
        cause instanceof ApiError &&
        [
          "directory_changed",
          "invalid_directory_input",
          "directory_limit",
          "directory_denied",
        ].includes(cause.code)
      ) {
        state.conflict =
          cause.code === "directory_changed" &&
          request.command.type === "save_profile";
        state.validation = state.conflict
          ? "Saved details changed. Refresh and review your retained draft."
          : "Check the fields and image selection.";
      } else {
        state.pending = copy(request);
        state.error = "";
      }
      throw cause;
    } finally {
      if (ticket === options.epoch()) state.busy = false;
    }
  }
  function accept(receipt: Receipt) {
    state.pending = null;
    state.conflict = false;
    state.validation = "";
    state.error = "";
    state.outcome = receipt.result.type;
    if (receipt.result.type === "profile") {
      state.profile = receipt.result.profile;
      state.draft = copy(receipt.result.profile.details);
      state.baseRevision = receipt.result.profile.revision;
      state.message = "Profile saved.";
    }
    if (receipt.result.type === "emoji") {
      state.emojiDraft = {
        shortcode: "",
        asset: "",
        filename: "",
        selected: undefined,
      };
      state.message = receipt.result.emoji.retired
        ? "Emoji removed from the library."
        : "Custom emoji saved.";
    }
    if (receipt.result.type === "media") {
      const asset = `blob:${receipt.result.asset}`;
      if (state.uploadTarget === "profile") state.draft.avatar = asset;
      else state.emojiDraft.asset = asset;
      void image(asset).catch(() => {});
    }
  }
  const request = (command: Request["command"]): Request => ({
    request_id: crypto.randomUUID().replaceAll("-", ""),
    command,
  });
  return {
    async save() {
      if (!state.profile || state.pending || state.busy || state.conflict)
        return;
      state.validation = "";
      await execute(
        request({
          type: "save_profile",
          expected: state.baseRevision,
          details: copy(state.draft),
        }),
      );
      await observe();
    },
    cancel() {
      if (state.profile && !state.pending && !state.busy) {
        state.draft = copy(state.profile.details);
        state.baseRevision = state.profile.revision;
        state.conflict = false;
        state.validation = "";
      }
    },
    review() {
      if (state.profile && !state.pending && !state.busy) {
        state.baseRevision = state.profile.revision;
        state.conflict = false;
      }
    },
    async upload(file: File, target: "profile" | "emoji" = "profile") {
      if (state.pending || state.busy)
        throw new Error("Check the original save first.");
      const ticket = options.epoch();
      state.uploadTarget = target;
      if (file.size > 2 * 1024 * 1024)
        throw new Error("Choose an image smaller than 2 MB.");
      const bytes = [...new Uint8Array(await file.arrayBuffer())];
      if (ticket !== options.epoch())
        throw new ApiError("directory_session_changed");
      const receipt = await execute(request({ type: "upload", bytes }));
      if (receipt.result.type !== "media")
        throw new Error("Invalid media receipt");
      return image(`blob:${receipt.result.asset}`);
    },
    async saveEmoji(shortcode: string, asset: string, existing?: Emoji) {
      if (state.pending || state.busy)
        throw new Error("Check the original save first.");
      await execute(
        request({
          type: "save_emoji",
          id: existing?.id ?? crypto.randomUUID().replaceAll("-", ""),
          expected: existing?.revision ?? null,
          shortcode,
          asset: asset.replace(/^blob:/, ""),
        }),
      );
      await observe();
    },
    async retire(emoji: Emoji) {
      if (state.pending || state.busy) return;
      await execute(
        request({
          type: "retire_emoji",
          id: emoji.id,
          expected: emoji.revision,
        }),
      );
      await observe();
    },
    async status() {
      if (!state.pending || state.busy) return;
      const ticket = options.epoch();
      const receipt = await client.request<Receipt | null>(
        `commands/${state.pending.request_id}`,
      );
      if (ticket !== options.epoch()) return;
      if (receipt) {
        if (receipt.request_id !== state.pending?.request_id)
          throw new ApiError("invalid_directory_receipt");
        accept(receipt);
        await observe();
      }
    },
    async retry() {
      if (state.pending) {
        await execute(copy(state.pending));
        await observe();
      }
    },
  };
}
