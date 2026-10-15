import type { Model, Exact } from "../../api/client.js";
import type { operations } from "../../api/generated.js";
export type ForgeRequest = Exact<
  NonNullable<
    operations["local_forge"]["requestBody"]
  >["content"]["application/json"]
>;
export type ForgeAction = ForgeRequest["action"];
export type ForgePrAction = Model<"ForgePrAction">;
export type ForgeResponse = Model<"ForgeResponse">;
export type ForgeConnection = Model<"ForgeConnection">;
export type ForgePlan = Model<"ForgePlan">;
export type ForgePr = Model<"ForgePullRequest">;
export type ForgeCheck = Model<"ForgeCheck">;
export type ForgeTransport = (request: ForgeRequest) => Promise<ForgeResponse>;
