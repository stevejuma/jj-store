import { ApiError, type Model } from "../api/client.js";
import type {
  DirectoryClient,
  DirectoryRequest as Request,
} from "../api/directory-client.js";
import type { DirectoryState } from "./directory-model.js";
import { emptyEmojiDraft } from "./directory-model.js";
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
  function clearEmojiSource() {
    if (state.emojiDraft.preview) URL.revokeObjectURL(state.emojiDraft.preview);
    delete state.emojiDraft.preview;
    delete state.emojiDraft.source;
  }
  async function continueEmojiSave() {
    const original = state.emojiSave;
    if (!original || original.command.type !== "save_emoji") return;
    const asset = state.emojiDraft.asset || original.command.asset;
    if (!asset) return;
    state.emojiSave = null;
    await execute({
      ...original,
      command: { ...original.command, asset: asset.replace(/^blob:/, "") },
    });
  }
  async function execute(request: Request): Promise<Receipt> {
    const ticket = options.epoch();
    state.busy = true;
    state.error = "";
    state.message = "";
    try {
      const receipt =
        request.command.type === "upload_image"
          ? await client.request<Receipt>(
              `image-uploads/${request.request_id}`,
              undefined,
              new Uint8Array(request.command.bytes.map(Number)),
            )
          : await client.request<Receipt>("commands", request);
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
        state.validation =
          request.command.type === "upload" ||
          request.command.type === "upload_image"
            ? ""
            : state.conflict
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
      if (
        !receipt.result.emoji.retired ||
        state.emojiDraft.selected?.id === receipt.result.emoji.id
      ) {
        clearEmojiSource();
        state.emojiDraft = emptyEmojiDraft();
      }
      state.message = receipt.result.emoji.retired
        ? "Emoji removed from the library."
        : "Custom emoji saved.";
    }
    if (receipt.result.type === "media") {
      const asset = `blob:${receipt.result.asset}`;
      if (state.uploadTarget === "profile") state.draft.avatar = asset;
      else {
        state.emojiDraft.asset = asset;
        delete state.emojiDraft.source;
      }
      void image(asset).catch(() => {});
    }
  }
  const request = (command: Request["command"]): Request => ({
    request_id: crypto.randomUUID().replaceAll("-", ""),
    command,
  });
  const actions = {
    selectEmoji(file: File) {
      if (state.pending || state.busy)
        throw new Error("Check the original save first.");
      if (file.size > 20 * 1024 * 1024)
        throw new Error("Choose an image smaller than 20 MB.");
      clearEmojiSource();
      state.emojiDraft.source = file;
      state.emojiDraft.preview = URL.createObjectURL(file);
      state.emojiDraft.asset = "";
    },
    cancelEmoji() {
      if (state.pending || state.busy) return;
      clearEmojiSource();
      state.emojiDraft = emptyEmojiDraft();
    },
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
    async upload(
      file: File,
      target: "profile" | "emoji" = "profile",
    ): Promise<string> {
      if (state.pending || state.busy)
        throw new Error("Check the original save first.");
      const ticket = options.epoch();
      state.busy = true;
      state.validation = "";
      try {
        const limit = 20;
        if (file.size > limit * 1024 * 1024)
          throw new Error(`Choose an image smaller than ${limit} MB.`);
        const bytes = [...new Uint8Array(await file.arrayBuffer())];
        if (ticket !== options.epoch())
          throw new ApiError("directory_session_changed");
        state.uploadTarget = target;
        const receipt = await execute(
          request({
            type: "upload_image",
            bytes,
          }),
        );
        if (receipt.result.type !== "media")
          throw new Error("Invalid media receipt");
        return await image(`blob:${receipt.result.asset}`);
      } catch (cause) {
        if (
          cause instanceof ApiError &&
          cause.code === "invalid_directory_input"
        )
          throw new Error(
            "The image could not be read. Choose a valid PNG, JPEG, GIF, or WebP image.",
          );
        if (cause instanceof ApiError && cause.code === "directory_limit")
          throw new Error(
            "Choose an image up to 20 MB, 8192 pixels per side and 16 megapixels.",
          );
        throw cause;
      } finally {
        if (ticket === options.epoch()) state.busy = false;
      }
    },
    async saveEmoji(shortcode: string, asset: string, existing?: Emoji) {
      if (state.pending || state.busy)
        throw new Error("Check the original save first.");
      shortcode = shortcode
        .trim()
        .replace(/^:+|:+$/g, "")
        .toLowerCase();
      if (!/^[a-z0-9_-]{1,64}$/.test(shortcode))
        throw new Error(
          "Use 1–64 lowercase letters, numbers, underscores or hyphens.",
        );
      state.emojiSave = request({
        type: "save_emoji",
        id: existing?.id ?? crypto.randomUUID().replaceAll("-", ""),
        expected: existing?.revision ?? null,
        shortcode,
        asset: asset.replace(/^blob:/, ""),
      });
      try {
        if (state.emojiDraft.source)
          await actions.upload(state.emojiDraft.source, "emoji");
        await continueEmojiSave();
      } catch (cause) {
        if (!state.pending) state.emojiSave = null;
        throw cause;
      }
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
        await continueEmojiSave();
        await observe();
      }
    },
    async retry() {
      if (state.pending) {
        await execute(copy(state.pending));
        await continueEmojiSave();
        await observe();
      }
    },
  };
  return actions;
}
