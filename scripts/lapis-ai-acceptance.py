#!/usr/bin/env python3
"""Cold app/native-compute acceptance using isolated state and explicit resources."""
import argparse
import json
import os
from pathlib import Path
import select
import socket
import subprocess
import tempfile
import time
import urllib.request
import uuid

parser = argparse.ArgumentParser()
for name in ["server", "jj", "rustfs", "bundle", "model", "web"]:
    parser.add_argument("--" + name, required=True, type=Path)
parser.add_argument("--browser", action="store_true")
args = parser.parse_args()


def request(origin, path, body=None):
    headers = {"x-jj-store-protocol": "1", "Origin": origin,
               "Sec-Fetch-Site": "same-origin", "Sec-Fetch-Mode": "cors", "Sec-Fetch-Dest": "empty"}
    if body is not None:
        headers["Content-Type"] = "application/json"
    call = urllib.request.Request(origin + "/v1" + path,
        data=None if body is None else json.dumps(body).encode(), headers=headers)
    with urllib.request.urlopen(call, timeout=45) as response:
        return json.load(response)


def wait(origin, predicate, timeout=240):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        value = request(origin, "/local/compute")
        if (value.get("operation") or {}).get("state") in ("failed", "outcome_unknown"):
            raise AssertionError("classified compute failure: " + str(value["operation"].get("error")))
        if predicate(value):
            return value
        time.sleep(.1)
    raise AssertionError("compute state deadline")


def command(origin, action, retained=None):
    status = request(origin, "/local/compute")
    value = retained or {"version": 1, "request_id": uuid.uuid4().hex,
        "expected_revision": status["settings"]["revision"], "action": action}
    request(origin, "/local/compute", value)
    return value


with tempfile.TemporaryDirectory(prefix="jla-", dir="/private/tmp" if Path("/private/tmp").is_dir() else None) as temporary:
    root = Path(temporary)
    state = root / "installation"
    environment = {**os.environ, "JJ_STORE_AI_BUNDLE_DIR": str(args.bundle.resolve()),
        "JJ_STORE_AGENT_JJ_EXECUTABLE": str(args.jj.resolve()),
        "JJ_STORE_COMPUTE_IDENTITY": "55" * 32}
    for key in ["JJ_STORE_ACP_EXECUTABLE", "JJ_STORE_LOCAL_AGENT_EXECUTABLE",
                "JJ_STORE_AGENT_TOOLS_EXECUTABLE", "JJ_STORE_COMPUTE_EXECUTABLE",
                "JJ_STORE_COMPUTE_NATIVE_RUNTIME_DIR", "JJ_STORE_TOKEN", "JJ_STORE_OPERATOR_TOKEN"]:
        environment.pop(key, None)
    subprocess.run([str(args.server.resolve()), "init", "--local", "--state-dir", str(state),
        "--rustfs-executable", str(args.rustfs.resolve()), "--format=json"],
        cwd=root, env=environment, check=True, capture_output=True, timeout=90)
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        port = listener.getsockname()[1]
    origin = f"http://127.0.0.1:{port}"
    config = state / "server.toml"
    config.write_text("web_root = " + json.dumps(str(args.web.resolve())) + "\n" +
        config.read_text().replace("127.0.0.1:7443", f"127.0.0.1:{port}"))
    processes = []

    def start():
        error_path = root / f"service-{len(processes)}.stderr"
        errors = error_path.open("w")
        child = subprocess.Popen([str(args.server.resolve()), "--no-pager", "--format=json",
            "run", "--desktop-supervised", "--config", str(config)], cwd=root, env=environment,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, text=True)
        processes.append((child, errors))
        assert select.select([child.stdout], [], [], 60)[0], "service readiness deadline"
        line = child.stdout.readline()
        if not line:
            errors.flush()
            raise AssertionError("App failed before readiness: " + error_path.read_text()[-4000:])
        ready = json.loads(line)
        assert ready["address"] == f"127.0.0.1:{port}"
        return child

    def stop(child):
        child.stdin.close()
        assert child.wait(timeout=45) == 0, "graceful app/helper shutdown"
        child.stdout.close()

    try:
        child = start()
        before = request(origin, "/local/compute")
        assert before["supported"] and before["state"] == "off"
        assert not before["settings"]["sharing"]
        assert "token" not in before and "endpoint" not in before
        command(origin, {"action": "prepare"})
        prepared = wait(origin, lambda v: v["runtime_prepared"] and v["operation"]["state"] == "completed")
        identity = prepared["identity"]
        command(origin, {"action": "import", "path": str(args.model.resolve())})
        imported = wait(origin, lambda v: v["operation"]["state"] == "completed" and any(m["installed"] for m in v["models"]))
        model = next(m["id"] for m in imported["models"] if m["installed"])
        settings = {**imported["settings"], "model": model, "memory_gb": 4}
        command(origin, {"action": "configure", "settings": settings})
        wait(origin, lambda v: v["operation"]["state"] == "completed")
        start_request = command(origin, {"action": "start"})
        serving = wait(origin, lambda v: v["state"] == "serving" and v["operation"]["state"] == "completed")
        command(origin, {}, retained=start_request)
        assert request(origin, "/local/compute")["settings"]["revision"] == serving["settings"]["revision"], "duplicate start changed its intent"
        if args.browser:
            subprocess.run(["node", str(Path(__file__).with_name("lapis-ai-browser.mjs").resolve()), origin], check=True, timeout=90)
        command(origin, {"action": "stop"})
        wait(origin, lambda v: v["state"] == "off" and v["operation"]["state"] == "completed")
        models = state / "agent-execution/compute/installed-models.json"
        model_index = models.read_bytes()
        stop(child)
        child = start()
        restored = request(origin, "/local/compute")
        assert restored["state"] == "off" and restored["runtime_prepared"]
        assert restored["settings"]["model"] == model
        assert any(m["id"] == model and m["installed"] for m in restored["models"])
        command(origin, {"action": "configure", "settings": restored["settings"]})
        reconnected = wait(origin, lambda v: v["operation"]["state"] == "completed")
        assert reconnected["identity"] == identity, "compute identity changed on restart"
        assert models.read_bytes() == model_index, "model receipts changed without a model command"
        command(origin, {"action": "stop"})
        wait(origin, lambda v: v["state"] == "off")
        stop(child)
        print("Cold app Lapis compute: prepare/import/start/receipt/stop/restart/identity/cache passed", flush=True)
    finally:
        for child, errors in processes:
            if child.poll() is None:
                child.stdin.close()
                try:
                    child.wait(timeout=45)
                except subprocess.TimeoutExpired:
                    child.terminate()
                    child.wait(timeout=15)
            if not child.stdout.closed:
                child.stdout.close()
            errors.close()
