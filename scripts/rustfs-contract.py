#!/usr/bin/env python3
"""Isolated, signed S3 conformance probes. No access to existing deployments.

Only --run starts a disposable container; --self-test tests the harness locally.
An image digest is mandatory. Test credentials and all storage are disposable.
"""

import argparse
import base64
import concurrent.futures
import datetime
import hashlib
import hmac
import http.client
import json
import os
from pathlib import Path
import socket
import subprocess
import threading
import time
import unittest
import urllib.parse
import uuid

IMAGE = "rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f"
ACCESS = "jj-store-disposable-test"
SECRET = "jj-store-disposable-test-secret-only"


def signed_headers(method, url, body, extra=None, now=None):
    parsed = urllib.parse.urlsplit(url)
    now = now or datetime.datetime.now(datetime.timezone.utc)
    date = now.strftime("%Y%m%d")
    stamp = now.strftime("%Y%m%dT%H%M%SZ")
    payload = hashlib.sha256(body).hexdigest()
    headers = {"host": parsed.netloc, "x-amz-date": stamp, "x-amz-content-sha256": payload}
    headers.update({k.lower(): " ".join(str(v).split()) for k, v in (extra or {}).items()})
    names = ";".join(sorted(headers))
    canonical_headers = "".join(f"{k}:{headers[k]}\n" for k in sorted(headers))
    query = "&".join(f"{urllib.parse.quote(k, safe='~')}={urllib.parse.quote(v, safe='~')}" for k, v in sorted(urllib.parse.parse_qsl(parsed.query, keep_blank_values=True)))
    canonical = "\n".join([method, parsed.path or "/", query, canonical_headers, names, payload])
    scope = f"{date}/us-east-1/s3/aws4_request"
    to_sign = "\n".join(["AWS4-HMAC-SHA256", stamp, scope, hashlib.sha256(canonical.encode()).hexdigest()])
    key = ("AWS4" + SECRET).encode()
    for value in [date, "us-east-1", "s3", "aws4_request"]:
        key = hmac.new(key, value.encode(), hashlib.sha256).digest()
    signature = hmac.new(key, to_sign.encode(), hashlib.sha256).hexdigest()
    headers["authorization"] = f"AWS4-HMAC-SHA256 Credential={ACCESS}/{scope}, SignedHeaders={names}, Signature={signature}"
    return headers


class S3:
    def __init__(self, endpoint):
        self.endpoint = endpoint

    def request(self, method, key="", body=b"", headers=None, drop_response=False):
        url = self.endpoint + "/jj-store-contract" + ("/" + urllib.parse.quote(key, safe="/~") if key else "")
        parsed = urllib.parse.urlsplit(url)
        conn = http.client.HTTPConnection(parsed.hostname, parsed.port, timeout=15)
        try:
            conn.request(method, parsed.path, body, signed_headers(method, url, body, headers))
            if drop_response:
                # Intentionally do not inspect the acknowledgement. The caller
                # cannot classify the write as rejected; it must read to resolve.
                time.sleep(0.2)
                return "Unknown", {}, b""
            response = conn.getresponse()
            return response.status, dict((k.lower(), v) for k, v in response.getheaders()), response.read()
        except (OSError, http.client.HTTPException):
            if method in {"PUT", "DELETE"}:
                return "Unknown", {}, b""
            raise
        finally:
            conn.close()


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def require_status(response, allowed):
    require(response[0] in allowed, f"HTTP {response[0]}, expected {allowed}: {response[2][:500]!r}")
    return response


class HarnessTests(unittest.TestCase):
    def test_signatures_bind_conditions_and_bytes(self):
        now = datetime.datetime(2026, 9, 26, tzinfo=datetime.timezone.utc)
        args = ("PUT", "http://localhost:9000/bucket/object", b"value")
        a = signed_headers(*args, {"If-Match": '"version"'}, now)
        b = signed_headers(*args, {"If-Match": '"other"'}, now)
        self.assertIn("if-match", a["authorization"])
        self.assertNotEqual(a["authorization"], b["authorization"])
        self.assertEqual(a, signed_headers(*args, {"If-Match": '"version"'}, now))

    def test_rejects_more_than_one_winner(self):
        with self.assertRaises(AssertionError):
            assert_race([(200, {}, b""), (200, {}, b"")])

    def test_unknown_is_not_a_precondition_failure(self):
        with self.assertRaises(AssertionError):
            assert_race([(200, {}, b""), ("Unknown", {}, b"")])


def assert_race(results):
    winners = [i for i, r in enumerate(results) if r[0] == 200]
    require(len(winners) == 1, f"expected exactly one winner, got statuses {[r[0] for r in results]}")
    require(all(r[0] in {200, 412} for r in results), f"unexpected race response: {[r[0] for r in results]}")
    return winners[0]


def race(s3, key, condition, contenders):
    barrier = threading.Barrier(contenders)
    def send(i):
        body = f"candidate-{i}".encode()
        barrier.wait(timeout=20)
        return s3.request("PUT", key, body, condition)
    with concurrent.futures.ThreadPoolExecutor(max_workers=contenders) as pool:
        results = list(pool.map(send, range(contenders)))
    winner = assert_race(results)
    read = require_status(s3.request("GET", key), {200})
    require(read[2] == f"candidate-{winner}".encode(), "read-after-write returned another candidate")
    require(read[1]["etag"] == results[winner][1]["etag"], "acknowledged token differs from read token")


def run_contract(image, rounds, contenders, output, rust_client=False):
    require("@sha256:" in image, "use an immutable image digest")
    name = "jj-store-m0-" + uuid.uuid4().hex[:12]
    volume = name + "-data"
    report = {"schema": 1, "image": image, "topology": "single container, one data volume, independent HTTP client connections", "rounds": rounds, "contenders": contenders, "started_at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "tests": [], "limitations": ["No multi-node/load-balancer qualification", "Discarded acknowledgement simulates an unknown client outcome, not a faulty RustFS network", "Production token and semantic-policy integration remains M3/M5; test principals explicitly permitted"]}
    def docker(*args):
        return subprocess.check_output(["docker", *args], text=True, stderr=subprocess.STDOUT).strip()
    def record(test, action):
        start = time.monotonic()
        try:
            action()
            row = {"name": test, "result": "pass", "failures": 0}
        except Exception as error:
            row = {"name": test, "result": "fail", "failures": 1, "error": str(error)}
        row["seconds"] = round(time.monotonic() - start, 3)
        report["tests"].append(row)
        print(json.dumps(row), flush=True)
    try:
        docker("volume", "create", volume)
        docker("run", "--detach", "--name", name, "--publish", "127.0.0.1::9000", "--mount", f"type=volume,src={volume},dst=/data", "--env", f"RUSTFS_ACCESS_KEY={ACCESS}", "--env", f"RUSTFS_SECRET_KEY={SECRET}", "--env", "RUSTFS_VOLUMES=/data", "--env", "RUSTFS_ADDRESS=0.0.0.0:9000", "--env", "RUSTFS_CONSOLE_ENABLE=false", image)
        port = docker("port", name, "9000/tcp").rsplit(":", 1)[1]
        s3 = S3(f"http://127.0.0.1:{port}")
        ready = False
        for _ in range(100):
            try:
                response = s3.request("PUT")
                if response[0] in {200, 409}: ready = True; break
            except (OSError, http.client.HTTPException): pass
            time.sleep(0.2)
        require(ready, "RustFS did not accept signed bucket create: " + docker("logs", "--tail", "25", name))
        report["image_id"] = docker("inspect", name, "--format", "{{.Image}}")
        def creates():
            for i in range(rounds): race(s3, f"create/{i}", {"If-None-Match": "*"}, contenders)
        record("concurrent_create_exactly_one_winner", creates)
        def swaps():
            for i in range(rounds):
                key = f"cas/{i}"
                initial = require_status(s3.request("PUT", key, b"initial", {"If-None-Match": "*"}), {200})
                race(s3, key, {"If-Match": initial[1]["etag"]}, contenders)
        record("concurrent_cas_exactly_one_winner", swaps)
        def stale():
            first = require_status(s3.request("PUT", "stale", b"first", {"If-None-Match": "*"}), {200})
            second = require_status(s3.request("PUT", "stale", b"second", {"If-Match": first[1]["etag"]}), {200})
            require_status(s3.request("PUT", "stale", b"wrong", {"If-Match": first[1]["etag"]}), {412})
            require_status(s3.request("PUT", "stale", b"overwrite", {"If-None-Match": "*"}), {412})
            current = require_status(s3.request("GET", "stale"), {200})
            require(current[2] == b"second" and current[1]["etag"] == second[1]["etag"], "stale mutation changed data")
        record("stale_tokens_do_not_overwrite", stale)
        def reads():
            body = bytes(range(256)) * 32768
            checksum = base64.b64encode(hashlib.sha256(body).digest()).decode()
            uploaded = require_status(s3.request("PUT", "large", body, {"If-None-Match":"*", "x-amz-checksum-sha256":checksum}), {200})
            head = require_status(s3.request("HEAD", "large"), {200})
            require(int(head[1]["content-length"]) == len(body), "HEAD size mismatch")
            require(head[1]["etag"] == uploaded[1]["etag"], "HEAD token mismatch")
            partial = require_status(s3.request("GET", "large", headers={"Range":"bytes=123-1048576"}), {206})
            require(partial[2] == body[123:1048577], "range bytes mismatch")
            require(partial[1].get("content-range") == f"bytes 123-1048576/{len(body)}", "range metadata mismatch")
            invalid = base64.b64encode(bytes(32)).decode()
            require_status(s3.request("PUT", "bad-checksum", b"value", {"If-None-Match":"*", "x-amz-checksum-sha256":invalid}), {400})
            require_status(s3.request("GET", "bad-checksum"), {404})
        record("size_range_checksum_and_partial_rejection", reads)
        def truncated_upload():
            body = b"must-not-publish-partial-content" * 65536
            url = s3.endpoint + "/jj-store-contract/truncated"
            parsed = urllib.parse.urlsplit(url)
            headers = signed_headers("PUT", url, body, {"If-None-Match": "*"})
            headers["content-length"] = str(len(body))
            headers["connection"] = "close"
            request = f"PUT {parsed.path} HTTP/1.1\r\n" + "".join(f"{k}: {v}\r\n" for k, v in headers.items()) + "\r\n"
            with socket.create_connection((parsed.hostname, parsed.port), timeout=10) as conn:
                conn.sendall(request.encode() + body[:1024])
                conn.shutdown(socket.SHUT_WR)
                while conn.recv(4096):
                    pass
            require_status(s3.request("GET", "truncated"), {404})
        record("interrupted_upload_is_not_published", truncated_upload)
        def unknown():
            outcome = s3.request("PUT", "lost-ack", b"durable", {"If-None-Match":"*"}, drop_response=True)
            require(outcome[0] == "Unknown", "discarded acknowledgement classified as known")
            require(require_status(s3.request("GET", "lost-ack"), {200})[2] == b"durable", "cannot resolve discarded acknowledgement")
            require_status(s3.request("PUT", "lost-ack", b"duplicate", {"If-None-Match":"*"}), {412})
        record("discarded_acknowledgement_resolves_by_read", unknown)
        def restart():
            token = require_status(s3.request("PUT", "restart", b"durable-before-kill", {"If-None-Match":"*"}), {200})[1]["etag"]
            docker("kill", name)
            docker("start", name)
            # Docker may allocate a fresh ephemeral host port on start.
            new_port = docker("port", name, "9000/tcp").rsplit(":", 1)[1]
            s3.endpoint = f"http://127.0.0.1:{new_port}"
            for _ in range(100):
                try:
                    response = s3.request("GET", "restart")
                    if response[0] == 200: break
                except (OSError, http.client.HTTPException): pass
                time.sleep(0.2)
            else: raise AssertionError("restart did not recover object")
            require(response[2] == b"durable-before-kill" and response[1]["etag"] == token, "acknowledged object/token changed after process kill")
            require_status(s3.request("PUT", "restart", b"after-restart", {"If-Match":token}), {200})
            require_status(s3.request("PUT", "restart", b"old-writer", {"If-Match":token}), {412})
        record("process_kill_restart_preserves_data_and_fences_stale_token", restart)
        def conditional_delete():
            a = require_status(s3.request("PUT", "conditional-delete", b"a", {"If-None-Match":"*"}), {200})
            b = require_status(s3.request("PUT", "conditional-delete", b"b", {"If-Match":a[1]["etag"]}), {200})
            require_status(s3.request("DELETE", "conditional-delete", headers={"If-Match":a[1]["etag"]}), {412})
            require(require_status(s3.request("GET", "conditional-delete"), {200})[2] == b"b", "stale delete removed new incarnation")
            require_status(s3.request("DELETE", "conditional-delete", headers={"If-Match":b[1]["etag"]}), {204})
        record("conditional_delete_cannot_remove_new_incarnation", conditional_delete)
        if rust_client:
            def compiled_client():
                env = dict(os.environ, JST_TEST_ENDPOINT=s3.endpoint, JST_TEST_ACCESS_KEY=ACCESS,
                           JST_TEST_SECRET_KEY=SECRET, DEVELOPER_DIR="/Library/Developer/CommandLineTools")
                command = ["cargo", "test", "-p", "jj-store-jj", "--test", "rustfs", "--locked", "--", "--ignored", "--nocapture"]
                result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=180)
                report["rust_client"] = {"command": command, "exit_code": result.returncode,
                                         "output": (result.stdout + result.stderr)[-8000:]}
                require(result.returncode == 0, "compiled Rust client failed: " + report["rust_client"]["output"])
            record("compiled_rust_native_transport", compiled_client)
            def cold_publication():
                docker("kill", name)
                docker("start", name)
                new_port = docker("port", name, "9000/tcp").rsplit(":", 1)[1]
                s3.endpoint = f"http://127.0.0.1:{new_port}"
                for _ in range(100):
                    try:
                        if s3.request("GET", "restart")[0] == 200: break
                    except (OSError, http.client.HTTPException): pass
                    time.sleep(0.2)
                else: raise AssertionError("publication restart failed readiness")
                env = dict(os.environ, JST_TEST_ENDPOINT=s3.endpoint, JST_TEST_ACCESS_KEY=ACCESS,
                           JST_TEST_SECRET_KEY=SECRET, JST_TEST_RECOVER_ONLY="1",
                           DEVELOPER_DIR="/Library/Developer/CommandLineTools")
                command = ["cargo", "test", "-p", "jj-store-jj", "--test", "rustfs", "--locked",
                           "real_rustfs_manifest_publication_and_cold_recovery", "--", "--ignored", "--nocapture"]
                result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=180)
                report["cold_publication"] = {"command": command, "exit_code": result.returncode,
                                               "output": (result.stdout + result.stderr)[-8000:]}
                require(result.returncode == 0, "cold publication recovery failed: " + report["cold_publication"]["output"])
            record("native_publication_survives_process_kill_and_empty_caches", cold_publication)
    except Exception as error:
        report["startup_error"] = str(error)
    finally:
        # Only the uniquely named resources created by this invocation.
        for args in [("rm", "--force", name), ("volume", "rm", volume)]:
            result = subprocess.run(["docker", *args], capture_output=True, text=True)
            if result.returncode and "No such" not in result.stderr: report.setdefault("cleanup_errors", []).append(result.stderr.strip())
        report["failures"] = sum(t["failures"] for t in report["tests"]) + int("startup_error" in report) + len(report.get("cleanup_errors", []))
        report["result"] = "pass" if report["failures"] == 0 and len(report["tests"]) == (10 if rust_client else 8) else "fail"
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(report, indent=2) + "\n")
        print(f"report={output} result={report['result']} failures={report['failures']}", flush=True)
    return 0 if report["result"] == "pass" else 1


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--run", action="store_true")
    parser.add_argument("--rust-client", action="store_true", help="Also qualify the compiled native object transport")
    parser.add_argument("--image", default=IMAGE)
    parser.add_argument("--rounds", type=int, default=20)
    parser.add_argument("--contenders", type=int, default=16)
    parser.add_argument("--output", type=Path, default=Path(".scratch/rustfs-contract.json"))
    options = parser.parse_args()
    if options.self_test:
        unittest.main(argv=[__file__])
    else:
        require(1 <= options.rounds <= 1000 and 2 <= options.contenders <= 64, "invalid concurrency limits")
        raise SystemExit(run_contract(options.image, options.rounds, options.contenders, options.output, options.rust_client))
