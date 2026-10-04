#!/usr/bin/env python3
"""Run the pinned Git-backed upstream extension example in an isolated workspace.
This qualifies CliRunner, not the production jj-store backend.
"""
import os
from pathlib import Path
import subprocess
import tempfile

binary = Path(__file__).resolve().parents[1] / "target/debug/examples/upstream_custom_backend"
with tempfile.TemporaryDirectory(prefix="jj-store-cli-probe-") as tmp:
    root = Path(tmp)
    config = root / "isolated.toml"
    config.write_text('[user]\nname="jj-store probe"\nemail="probe@example.invalid"\n[signing]\nbehavior="drop"\n[ui]\npaginate="never"\n')
    workspace = root / "workspace"
    workspace.mkdir()
    environment = {**os.environ, "JJ_CONFIG": str(config)}
    commands = [["--help"], ["init-jit"], ["describe", "-m", "extension probe"], ["log", "-r", "all()"], ["op", "log"], ["evolog"]]
    for args in commands:
        result = subprocess.run([str(binary), "--no-pager", *args], cwd=workspace, env=environment, capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError(f"{args}: {result.stderr}")
    print(f"upstream extension smoke: {len(commands)} commands passed (Git-backed reference only)")
