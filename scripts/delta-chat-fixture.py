"""Project a local Delta JSON export into a sanitized, replay-free UI fixture."""
import hashlib
import json
import pathlib
import sys

source_path, destination_path = map(pathlib.Path, sys.argv[1:])
source = json.loads(source_path.read_text())
original = source["buffer"].encode("utf-8")


def clean(value):
    if isinstance(value, str):
        return value.replace("/Users/stevejuma", "/workspace/demo").replace(
            "stevejuma", "demo-user"
        )
    if isinstance(value, list):
        return [clean(item) for item in value]
    if isinstance(value, dict):
        return {key: clean(item) for key, item in value.items()}
    return value


def offset(anchor):
    value = anchor["$resolved_offset"]
    return len(clean(original[:value].decode("utf-8")).encode("utf-8"))


def span(value):
    return {"start": offset(value["start"]), "end": offset(value["end"])}


records = []
for index, message in enumerate(source["messages"]):
    kind = message["value"]["$variant"]
    value = message["value"].get("$value", {})
    entry = {"id": f"delta-{index}", "kind": kind, "start": offset(message["key"])}
    if kind in ("User", "ComposerDraft"):
        entry["end"] = offset(value["enclosure"]["content_end"])
    elif kind in ("ToolUse", "ExternalEdits", "AgentMessage"):
        entry["layout"] = {
            key: span(ranges) for key, ranges in value["layout"].items()
        }
        entry["end"] = entry["layout"]["range"]["end"]
    elif kind == "Thinking":
        entry["end"] = offset(value["range_end"])
        entry["titleEnd"] = offset(value["title_end"])
        entry["contentEnd"] = offset(value["content_end"])
    if kind in ("User", "AssistantTurn"):
        entry["createdAt"] = value.get("created_at")
    if kind == "ToolUse":
        entry.update(
            name=value["name"],
            input=clean(value["input"]),
            output=clean(value["output"]),
            status=value["status"]["$variant"],
        )
    if kind == "AgentMessage":
        entry["title"] = clean(value["sender_title"])
    if kind != "ComposerDraft":
        records.append(entry)

result = {
    "version": 1,
    "source": {
        "application": "Delta 0.1.0",
        "title": source["name"],
        "captureDate": "2026-10-08",
        "method": "delta thread data (UI copy unavailable during capture)",
        "sha256": hashlib.sha256(source_path.read_bytes()).hexdigest(),
        "offsetEncoding": "utf-8-bytes",
        "scope": "main thread only; no live replay or child-thread import",
    },
    "buffer": clean(source["buffer"]),
    "records": records,
    "draftStart": offset(next(m["key"] for m in source["messages"] if m["value"]["$variant"] == "ComposerDraft")),
}
destination_path.parent.mkdir(parents=True, exist_ok=True)
destination_path.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
print(f"Wrote {destination_path}: {len(records)} records, {len(result['buffer'])} characters")
