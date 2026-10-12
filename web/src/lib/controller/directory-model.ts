import type { Model } from "../api/client.js";
import type { DirectoryRequest } from "../api/directory-client.js";
type Profile = Model<"UserProfile">;
type Emoji = Model<"LibraryEmoji">;
type Request = DirectoryRequest;
export const emptyProfile = () => ({
  display_name: "",
  description: "",
  email: null as string | null,
  avatar: null as string | null,
});
export const initialDirectoryState = () => ({
  profile: null as Profile | null,
  draft: emptyProfile(),
  emojis: [] as Emoji[],
  loading: false,
  busy: false,
  error: "",
  validation: "",
  message: "",
  conflict: false,
  emojiDraft: {
    shortcode: "",
    asset: "",
    filename: "",
    selected: undefined as Emoji | undefined,
  },
  uploadTarget: "profile" as "profile" | "emoji",
  outcome: "",
  pending: null as Request | null,
  baseRevision: null as string | null,
  urls: {} as Record<string, string>,
  profiles: {} as Record<string, Model<"PublicProfile">>,
});
export type DirectoryState = ReturnType<typeof initialDirectoryState>;
