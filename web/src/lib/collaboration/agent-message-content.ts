import type { Model } from "../api/client.js";

export function agentMessageContent(
  content: Model<"AgentContent">,
  integration: Model<"IntegrationId">,
): string {
  if (content.media_type !== "json") return content.content;
  try {
    const value: unknown = JSON.parse(content.content);
    if (
      integration === "open_code" &&
      Array.isArray(value) &&
      value.length > 0 &&
      value.every(
        (part) =>
          part !== null &&
          typeof part === "object" &&
          !Array.isArray(part) &&
          part.type === "text" &&
          typeof part.text === "string",
      )
    ) {
      return value.map((part) => part.text).join("\n\n");
    }
    return `\`\`\`json\n${JSON.stringify(value, null, 2)}\n\`\`\``;
  } catch {
    return `\`\`\`text\n${content.content}\n\`\`\``;
  }
}
