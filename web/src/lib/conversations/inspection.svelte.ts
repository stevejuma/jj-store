import { WorkspaceBrowser } from "./workspace-browser.svelte.js";
import type { ConversationSession } from "./session.svelte.js";
import type { LocalClient } from "../api/local-client.js";
import { repositoryDetailsPanelId } from "../ui/layout-storage.js";
import { getContext, setContext, tick } from "svelte";
import type { AppShellController } from "@lapismd/design-core/shell";
import type { ChatBodyTool } from "../ui/store-agent-chat-body/model.js";
import type { ChatBodyComments } from "../ui/store-agent-chat-body/comments.svelte.js";
const key = Symbol("conversation-inspection");
export function provideConversationInspection(shell: AppShellController) {
  let current = $state<
    { tool: ChatBodyTool; state: ChatBodyComments } | undefined
  >();
  let workspace = $state<WorkspaceBrowser>();
  let file = $state<WorkspaceBrowser>();
  const browsers = new WeakMap<ConversationSession, WorkspaceBrowser>();
  let opener: HTMLElement | undefined,
    wasOpen = false;
  const panel = {
    get workspace() {
      return workspace;
    },
    get file() {
      return file;
    },
    workspaceBrowser(client: LocalClient, session: ConversationSession) {
      let browser = browsers.get(session);
      if (!browser) {
        browser = new WorkspaceBrowser(client, session, () => {
          if (!current && !file) wasOpen = !shell.right.closed;
          current = undefined;
          file = browser;
          shell.right.open();
          void tick().then(() => {
            if (shell.mobile.resolvedMode === "mobile")
              shell.mobile.show("right", "right");
          });
        });
        browsers.set(session, browser);
      }
      return browser;
    },
    showWorkspace(browser: WorkspaceBrowser) {
      workspace = browser;
      browser.railOpen = true;
      shell.getPanel(repositoryDetailsPanelId)?.open();
      void browser.load();
      void tick().then(() => {
        if (shell.mobile.resolvedMode === "mobile")
          shell.mobile.show("right", repositoryDetailsPanelId);
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
      if (!current && !file) wasOpen = !shell.right.closed;
      file = undefined;
      current = { tool, state };
      opener = element;
      shell.right.open();
      await tick();
      if (shell.mobile.resolvedMode === "mobile")
        shell.mobile.show("right", "right", element);
    },
    async close() {
      current = undefined;
      if (file) file.fileOpen = false;
      file = undefined;
      if (!wasOpen) shell.right.close();
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
