import { ApplicationEventHub } from "./application-event-hub.js";
import { validApplicationCursor } from "./application-events.js";

const hub = new ApplicationEventHub();
// Keep the worker entry compatible with the web project's DOM type library.
const worker = self as unknown as {
  onconnect: (event: MessageEvent) => void;
};
worker.onconnect = (event) => {
  const port = event.ports[0]!;
  let unsubscribe: (() => void) | undefined;
  const close = () => {
    unsubscribe?.();
    unsubscribe = undefined;
    port.close();
  };
  port.onmessage = ({ data }) => {
    if (data?.type === "unsubscribe") return close();
    if (
      unsubscribe ||
      data?.type !== "subscribe" ||
      !(
        data.token === null ||
        (typeof data.token === "string" && data.token.length <= 4096)
      ) ||
      !(
        data.after === undefined ||
        (typeof data.after === "string" && validApplicationCursor(data.after))
      )
    ) {
      port.postMessage({ type: "end", error: "invalid_application_event" });
      close();
      return;
    }
    unsubscribe = hub.subscribe(data.token, data.after, {
      deliver: (value, id) => port.postMessage({ type: "event", value, id }),
      end: (error) => {
        port.postMessage({ type: "end", error });
        close();
      },
    });
  };
  port.onmessageerror = close;
  port.addEventListener("close", close);
  port.start();
};
