# Conversation metadata API

JST-API-001 adds LocalTrusted Admin GET/POST /v1/local/agents/metadata and GET
/v1/local/agents/metadata/job?request=… . Every operation independently checks
repository admission, including duplicate commands and job inspection. A query
names a managed execution session, or a captured session with its repository.
Known aliases select one typed conversation target. Reads never create an owner,
publish a suggestion, dispatch an agent or initiate metadata inference.

Version-1 commands retain request_id, query and expected metadata revision.
Actions are rename, reset_title, refresh, regenerate_title, cancel and publish.
The capture_titles action accepts no title text: it copies already committed,
sanitized ACP suggestions through the same publication authority. Job responses
retain original commands for explicit recovery after browser/host restart.

Repository-selected values and exact source snapshots are separate from private
job state. Results carry immutable snapshots, provider/model, prompt version,
time and partial-coverage state. Session and recorded-directory responses add
optional conversation_title/discovery_text projections without changing captured
agent identity. Missing optional settings/metadata fields retain legacy bytes.

One installation serializes inference with bounded admission and retained
receipts. Jobs journal frozen source/model inputs before any process side
effect. Metadata input is capped at 128 KiB, completion output at 16 KiB and the
helper at 120 seconds (125-second parent deadline). Only visible authored text,
delivered comments and concise tool headers are included; system/reasoning/tool
bodies are excluded. Sources are independently resolved through native/local
AgentStore.

The fixed companion performs one tool-free provider completion outside ACP. It
creates no coding workspace, MCP/tool session or ordinary transcript turn, never
opens interactive authentication and never starts compute. Transport,
authentication and endpoint retries are disabled for metadata. Interrupted or
unknown jobs retain their original request and cannot replay inference. Explicit
receipt recovery retries only a retained publication request.

Native publication is manifest/CAS selected; local publication is companion/ref
selected. Native origin code repositories without a WorkStore home retain their
write refusal. No cross-repository permission or multi-agent atomicity is added.

The 128-KiB transcript limit applies after decoding. Helper wire admission
allows six-fold JSON escape expansion plus 1024 bytes of framing, without
increasing the transcript budget. Cancel may stop local waiting for an unknown
inference only when no publication request exists; receipts and original
identity remain.
