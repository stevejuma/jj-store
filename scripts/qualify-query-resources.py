#!/usr/bin/env python3
"""Run one release fixture process and retain explicit, bounded qualification data."""
import argparse
import datetime
import json
import hashlib
import pathlib
import platform
import resource
import subprocess
import time


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    parser.add_argument("--commits", type=int, default=50000)
    parser.add_argument("--scenario", choices=["graphs", "content"], default="graphs")
    args = parser.parse_args()
    if not 100 <= args.commits <= 50000:
        parser.error("commit count must be 100..=50000")
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    started = time.monotonic()
    command = [str(args.binary.resolve())] + ([str(args.commits)] if args.scenario == "graphs" else [])
    try:
        child = subprocess.run(command, text=True, capture_output=True, timeout=180)
        status = child.returncode
        stdout, stderr = child.stdout, child.stderr
    except subprocess.TimeoutExpired as exc:
        status = -1
        stdout = ""
        stderr = f"fixture process exceeded 180 seconds: {exc}"
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    peak = int(usage.ru_maxrss * (1 if platform.system() == "Darwin" else 1024))
    cpu = usage.ru_utime + usage.ru_stime - before.ru_utime - before.ru_stime
    payload = None
    if status == 0:
        payload = json.loads(stdout)
    scenarios = (payload or {}).get("scenarios", [])
    runs = [run for case in scenarios for run in case["runs"]]
    expected = {"cold_log": min(500, args.commits), "warm_log": min(500, args.commits), "warm_filter": 0, "operations": 2}
    passed = (
        status == 0
        and len(scenarios) == 2
        and {case["shape"] for case in scenarios} == {"linear", "wide_heads"}
        and all(case["commits"] == args.commits and case["negative_admission_cases"] == 2 for case in scenarios)
        and len(runs) == 8
        and all(len(case["runs"]) == 4 and {run["name"] for run in case["runs"]} == set(expected) for case in scenarios)
        and all(run["result"] == "pass" and run["rows"] == expected[run["name"]] for run in runs)
        and peak <= 1024 * 1024 * 1024
        and cpu <= 60
    )
    if args.scenario == "content":
        expected_content = {"dense_tree_at_limit", "dense_tree_over_limit", "merge_at_limit", "merge_over_limit", "cumulative_nodes_over_limit", "stream_blob_at_limit", "verified_tail_range", "blob_over_limit"}
        runs = (payload or {}).get("runs", [])
        passed = (status == 0 and len(runs) == 8
                  and {run["name"] for run in runs} == expected_content
                  and all(run["result"] == "pass" for run in runs)
                  and peak <= 1024 * 1024 * 1024 and cpu <= 60)
    with args.binary.open("rb") as binary:
        binary_hash = hashlib.file_digest(binary, "sha256").hexdigest()
    report = {
        "recorded_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "host": {"system": platform.system(), "release": platform.release(), "machine": platform.machine()},
        "command": command,
        "binary_sha256": binary_hash,
        "cargo_lock_sha256": hashlib.sha256(pathlib.Path("Cargo.lock").read_bytes()).hexdigest(),
        "scenario": args.scenario,
        "scope": "Encoded immutable fixture storage (large content uses disk spools) and production native query executor; no service authorization, network, RustFS or browser cost. Whole-process memory includes fixture generation and encoded source objects.",
        "ceilings": {"peak_process_bytes": 1024 * 1024 * 1024, "cpu_seconds": 60, "wall_seconds": 180},
        "process": {"exit_code": status, "wall_seconds": time.monotonic() - started, "cpu_seconds": cpu, "peak_rss_bytes": peak},
        "native": payload,
        "stderr": stderr[-8000:],
        "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"passed": passed, **report["process"], "output": str(args.output)}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
