<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MailIcon from "@lucide/svelte/icons/mail";
  let { kind, count }: { kind: "session" | "thread"; count: number } = $props();
</script>

<Empty.Root class="lc-community-inbox__empty" role="status"
  ><Empty.Header
    ><Empty.Media variant="icon"><MailIcon aria-hidden="true" /></Empty.Media
    ><Empty.Title
      >{count
        ? "No matching conversations"
        : `No ${kind === "thread" ? "Threads" : "sessions"} yet`}</Empty.Title
    ><Empty.Description
      >{count
        ? "Try changing your search or filters."
        : kind === "thread"
          ? "Create a Thread to begin a conversation."
          : "Start a session to begin a chat with this agent."}</Empty.Description
    ></Empty.Header
  ></Empty.Root
>
