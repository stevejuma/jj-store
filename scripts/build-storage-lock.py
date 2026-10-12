"""POSIX build leases and guarded Cargo cache removal; never follows symlinks."""
import contextlib
import fcntl
import json
import os
import pathlib
import shutil
import signal
import subprocess
import sys
import time

LOCK_NAMES = {".cargo-lock", ".cargo-build-lock", ".cargo-artifact-lock"}
PROTECTED = {".jj", ".git", ".local", "dev-demo", "workspace-archives", "workspace-retirement", "server.toml", "runtime.json", "fixture.json", "installation.json", "credentials.json"}
CACHE_SIGNATURE = "Signature: 8a477f597d28d172789f06886806bc55"


def scan(path):
    result = {"bytes": 0, "artifactBytes": 0, "modified": 0, "unsafe": [], "categories": {}}
    if not os.path.exists(path):
        return result
    for directory, names, files in os.walk(path, followlinks=False):
        for name in names + files:
            item = os.path.join(directory, name)
            try:
                info = os.lstat(item)
            except FileNotFoundError:
                continue
            if os.path.islink(item) or name in PROTECTED:
                result["unsafe"].append(os.path.relpath(item, path))
            if name in files:
                size = info.st_blocks * 512
                result["bytes"] += size
                if name not in LOCK_NAMES | {"CACHEDIR.TAG"}:
                    result["artifactBytes"] += size
                if name not in LOCK_NAMES:
                    result["modified"] = max(result["modified"], info.st_mtime * 1000)
                parts = pathlib.Path(os.path.relpath(item, path)).parts
                category = next((p for p in parts if p in {"deps", "incremental", "build", "bundle", "resources"}), "other")
                result["categories"][category] = result["categories"].get(category, 0) + size
    return result


def consumers(path):
    # lsof includes running executables, cwd, mapped libraries and open compiler files.
    command = ["lsof", "-nP", "-Fpn", "+D", path]
    result = subprocess.run(command, capture_output=True, text=True, timeout=60)
    if result.returncode not in (0, 1) or result.stderr.strip():
        raise RuntimeError("Cannot prove target idle: " + result.stderr.strip())
    pids = {int(line[1:]) for line in result.stdout.splitlines() if line.startswith("p") and int(line[1:]) != os.getpid()}
    processes = subprocess.run(["ps", "-axo", "pid=,comm="], capture_output=True, text=True, check=True, timeout=10)
    for line in processes.stdout.splitlines():
        fields = line.strip().split(None, 1)
        if len(fields) == 2 and fields[1].startswith(path + os.sep):
            pids.add(int(fields[0]))
    return sorted(pids)


@contextlib.contextmanager
def lease(path, exclusive, wait=False):
    os.makedirs(os.path.dirname(path), mode=0o700, exist_ok=True)
    flags = os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW
    with os.fdopen(os.open(path, flags, 0o600), "r+") as handle:
        deadline = time.monotonic() + 30
        while True:
            try:
                fcntl.flock(handle, (fcntl.LOCK_EX if exclusive else fcntl.LOCK_SH) | fcntl.LOCK_NB)
                break
            except BlockingIOError:
                if not wait or time.monotonic() >= deadline:
                    raise
                time.sleep(0.1)
        yield handle.fileno()


@contextlib.contextmanager
def cargo_locks(path):
    # Keep Cargo's locks in place even when removing all other cache contents.
    with contextlib.ExitStack() as stack:
        for directory, _, files in os.walk(path):
            for name in sorted(set(files) & LOCK_NAMES):
                stack.enter_context(lease(os.path.join(directory, name), True))
        yield


def remove_cache(path, keep):
    for entry in os.scandir(path):
        if entry.path in keep:
            continue
        if entry.is_dir(follow_symlinks=False):
            remove_cache(entry.path, keep)
            if not os.listdir(entry.path):
                os.rmdir(entry.path)
        else:
            os.unlink(entry.path)


def clean(path, manifest, kind):
    if os.path.realpath(path) != path or not os.path.isdir(path):
        raise RuntimeError("Target must be an existing canonical directory")
    inventory = scan(path)
    if inventory["unsafe"]:
        raise RuntimeError("Unsafe cache entries: " + ", ".join(inventory["unsafe"][:5]))
    tag = os.path.join(path, "CACHEDIR.TAG")
    with open(tag, encoding="utf8") as stream:
        if not stream.read().startswith(CACHE_SIGNATURE):
            raise RuntimeError("Target has no valid Cargo cache marker")
    with cargo_locks(path):
        active = consumers(path)
        if active:
            raise RuntimeError("Target in use by PIDs " + ", ".join(map(str, active)))
    if kind == "workspace":
        # Cargo dry-run does not list every executable it removes. Delegate the
        # entire package selection to Cargo, retaining the managed exclusive
        # lease while Cargo acquires its own native cache locks.
        with cargo_locks(path):
            if consumers(path):
                raise RuntimeError("Target became active")
            if scan(path)["unsafe"]:
                raise RuntimeError("Cache acquired unsafe entries")
        result = subprocess.run(["cargo", "clean", "--workspace", "--locked", "--offline", "--manifest-path", manifest, "--target-dir", path], stdout=sys.stderr)
        if result.returncode:
            raise RuntimeError("Cargo workspace cleanup failed")
    else:
        with cargo_locks(path):
            if consumers(path):
                raise RuntimeError("Target became active")
            if scan(path)["unsafe"]:
                raise RuntimeError("Cache acquired unsafe entries")
            keep = {os.path.join(d, n) for d, _, fs in os.walk(path) for n in fs if n in LOCK_NAMES} | {tag}
            remove_cache(path, keep)
    print(json.dumps({"beforeBytes": inventory["bytes"], "afterBytes": scan(path)["bytes"]}))


def main():
    action, *args = sys.argv[1:]
    if action == "inspect":
        print(json.dumps(scan(args[0])))
    elif action == "clean":
        clean(*args)
    elif action == "active":
        with cargo_locks(args[0]):
            print(json.dumps(consumers(args[0]) if os.path.exists(args[0]) else []))
    elif action == "lock":
        path, mode, *command = args
        with lease(path, mode.startswith("exclusive"), mode.endswith("-wait")) as descriptor:
            child = subprocess.Popen(command, pass_fds=(descriptor,), start_new_session=True)
            def forward(number, _):
                if child.poll() is None:
                    os.killpg(child.pid, number)
            signal.signal(signal.SIGTERM, forward)
            signal.signal(signal.SIGINT, forward)
            try:
                sys.exit(child.wait())
            except KeyboardInterrupt:
                child.wait()
                sys.exit(130)
    else:
        raise RuntimeError("Unknown build-storage helper operation")


if __name__ == "__main__":
    try:
        main()
    except (OSError, RuntimeError, subprocess.TimeoutExpired) as error:
        print(str(error), file=sys.stderr)
        sys.exit(75)
