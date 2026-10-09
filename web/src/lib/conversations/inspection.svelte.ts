import { WorkspaceBrowser } from "./workspace-browser.svelte.js";
import type { ConversationSession } from "./session.svelte.js";
import type { LocalClient } from "../api/local-client.js";
import { getContext, setContext, tick } from "svelte";
import type { AppShellController } from "@lapismd/design-core/shell";
import type { ChatBodyTool } from "../ui/store-agent-chat-body/model.js";
import type { ChatBodyComments } from "../ui/store-agent-chat-body/comments.svelte.js";
const key = Symbol("conversation-inspection");
export function provideConversationInspection(
  shell: AppShellController,
  inboxSession: () => string | null,
) {
  // Workspace inspection has its own layout identities. It must never resize or
  // open the existing repository/project rail or body details panel.
  const workspaceRail = shell.createSidebar("conversation-workspace", "right", {
    closed: true,
    width: 280,
    minWidth: 220,
    maxWidth: 440,
  });
  const workspaceInspector = shell.createSidebar("conversation-file", "right", {
    closed: true,
    width: 400,
    minWidth: 220,
    maxWidth: 720,
  });
  function ownsInbox(browser: WorkspaceBrowser | undefined) {
    const session = inboxSession();
    return (
      !!session &&
      !!browser?.bindings.some((binding) => binding.session === session)
    );
  }
  let current = $state<
    { tool: ChatBodyTool; state: ChatBodyComments } | undefined
  >();
  let workspace = $state<WorkspaceBrowser>();
  let file = $state<WorkspaceBrowser>();
  const browsers = new WeakMap<ConversationSession, WorkspaceBrowser>();
  let registered = $state<WorkspaceBrowser[]>([]);
  let opener: HTMLElement | undefined,
    wasOpen = false;
  const panel = {
    workspaceRail,
    workspaceInspector,
    get workspaceEnabled() {
      return inboxSession() !== null;
    },
    get inboxWorkspace() {
      return registered.find(ownsInbox);
    },
    toggleWorkspace() {
      const browser = panel.inboxWorkspace;
      if (!browser) return;
      if (panel.workspace === browser && !workspaceRail.closed) {
        workspaceRail.close();
        browser.railOpen = false;
        if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
      } else panel.showWorkspace(browser);
    },
    get workspace() {
      return ownsInbox(workspace) ? workspace : undefined;
    },
    get file() {
      return ownsInbox(file) ? file : undefined;
    },
    workspaceBrowser(client: LocalClient, session: ConversationSession) {
      let browser = browsers.get(session);
      if (!browser) {
        browser = new WorkspaceBrowser(client, session, () => {
          if (!ownsInbox(browser)) return;
          current = undefined;
          file = browser;
          workspaceInspector.open();
          void tick().then(() => {
            if (ownsInbox(browser) && shell.mobile.resolvedMode === "mobile")
              shell.mobile.show("right", "conversation-file");
          });
        });
        browsers.set(session, browser);
        registered = [...registered, browser];
      }
      return browser;
    },
    showWorkspace(browser: WorkspaceBrowser) {
      if (!ownsInbox(browser)) return;
      workspace = browser;
      browser.railOpen = true;
      workspaceRail.open();
      if (browser.fileOpen) {
        file = browser;
        workspaceInspector.open();
      }
      void browser.load();
      void tick().then(() => {
        if (ownsInbox(browser) && shell.mobile.resolvedMode === "mobile")
          shell.mobile.show("right", "conversation-workspace");
      });
    },
    hideWorkspace(browser: WorkspaceBrowser) {
      if (workspace === browser) {
        workspace = undefined;
        browser.railOpen = false;
      }
    },
    get current() {
      return current;
    },
    async open(
      tool: ChatBodyTool,
      state: ChatBodyComments,
      element: HTMLElement,
    ) {
      if (!current) wasOpen = !shell.right.closed;
      file = undefined;
      current = { tool, state };
      opener = element;
      shell.right.open();
      await tick();
      if (shell.mobile.resolvedMode === "mobile")
        shell.mobile.show("right", "right", element);
    },
    async close() {
      if (panel.file) {
        panel.file.fileOpen = false;
        file = undefined;
        workspaceInspector.close();
      } else {
        current = undefined;
        if (!wasOpen) shell.right.close();
      }
      if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
      await tick();
      if (opener?.isConnected) opener.focus({ preventScroll: true });
    },
  };
  setContext(key, panel);
  return panel;
}
export function useConversationInspection() {
  return getContext<
    ReturnType<typeof provideConversationInspection> | undefined
  >(key);
}
