"""Owned PTY/pager acceptance. No user settings, credentials, or services."""
import errno
import json
import os
from pathlib import Path
import pty
import select
import subprocess
import sys
import tempfile
import time

binary = sys.argv[1]

def run_tty(args, environment):
    master, slave = pty.openpty()
    process = subprocess.Popen([binary, *args], stdin=subprocess.DEVNULL, stdout=slave,
                               stderr=slave, env=environment)
    os.close(slave)
    output = bytearray()
    deadline = time.monotonic() + 10
    try:
        while time.monotonic() < deadline:
            if select.select([master], [], [], 0.1)[0]:
                try:
                    part = os.read(master, 65536)
                except OSError as error:
                    if error.errno == errno.EIO:
                        break
                    raise
                if not part:
                    break
                output.extend(part)
        else:
            raise AssertionError("CLI or pager exceeded deadline")
        assert process.wait(timeout=2) == 0, output.decode(errors="replace")
    finally:
        os.close(master)
        if process.poll() is None:
            process.kill()
            process.wait()
    return output.decode()

with tempfile.TemporaryDirectory(prefix="jj-store-pager-") as directory:
    root = Path(directory)
    pager = root / "pager.py"
    marker = root / "paged"
    pager.write_text("from pathlib import Path\nimport sys\nPath(sys.argv[1]).write_text('paged')\nsys.stdout.write(sys.stdin.read())\n")
    env = os.environ.copy()
    env.pop("JJ_STORE_TOKEN", None)
    env.pop("NO_COLOR", None)
    env["JJ_CONFIG"] = ""
    env["JJ_STORE_PAGER"] = f"{sys.executable} {pager} {marker}"
    output = run_tty(["store", "version"], env)
    assert marker.exists() and "Client:" in output
    marker.unlink()
    run_tty(["--no-pager", "store", "version"], env)
    assert not marker.exists()
    output = run_tty(["store", "version", "--format=json", "--color=always"], env)
    assert not marker.exists() and "\x1b[" not in output
    assert json.loads(output)["client"]["upstream_jj"] == "0.45.1"
    output = run_tty(["store", "attach", "--help"], env)
    assert marker.exists() and "\x1b[" in output
    marker.unlink()
    env["NO_COLOR"] = "1"
    output = run_tty(["store", "attach", "--help"], env)
    assert "\x1b[" not in output
    output = run_tty(["store", "attach", "--help", "--color=always"], env)
    assert "\x1b[" in output
    env["JJ_STORE_PAGER"] = "cat"
    marker.unlink()
    run_tty(["store", "version"], env)
    assert not marker.exists()
    env["JJ_STORE_PAGER"] = "jj-store-nonexistent-test-pager"
    output = run_tty(["store", "version"], env)
    assert "Client:" in output and "Warning:" in output and "Hint:" in output and "--no-pager" in output, output
print("PTY: text/help paging, no-pager, JSON, auto/forced color and NO_COLOR passed")
