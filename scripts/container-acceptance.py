#!/usr/bin/env python3
"""Disposable Linux container, TLS and external RustFS qualification."""
import base64
import importlib.util
import json
import os
from pathlib import Path
import secrets
import socket
import ssl
import subprocess
import sys
import tempfile
import time
import urllib.request
import urllib.error
import uuid

image = sys.argv[1]
name = "jj-store-image-" + uuid.uuid4().hex[:12]
volume = name + "-state"
storage = name + "-storage"
storage_volume = name + "-objects"
helper = "python:3.12-slim-bookworm"
spec = importlib.util.spec_from_file_location("rustfs_contract", Path(__file__).with_name("rustfs-contract.py"))
contract = importlib.util.module_from_spec(spec)
spec.loader.exec_module(contract)

def docker(*args, input=None, check=True):
    result = subprocess.run(["docker", *args], input=input, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, text=True)
    if check and result.returncode:
        raise RuntimeError("Docker fixture action failed: " + result.stderr[-2000:])
    return result

def private_files(files):
    program = """import json,sys,os,base64
from pathlib import Path
root=Path('/state');root.mkdir(exist_ok=True)
os.chown(root,65532,65532);root.chmod(0o700)
for name,data in json.load(sys.stdin).items():
 p=root/name;p.write_bytes(base64.b64decode(data));os.chown(p,65532,65532);p.chmod(0o600)
"""
    docker("run", "--rm", "-i", "--mount", f"type=volume,src={volume},dst=/state",
           helper, "python3", "-c", program,
           input=json.dumps({k: base64.b64encode(v).decode() for k, v in files.items()}))

with tempfile.TemporaryDirectory(prefix="jj-store-container-") as temp:
    temp = Path(temp)
    try:
        metadata = json.loads(docker("image", "inspect", image).stdout)[0]
        assert metadata["Config"]["User"] == "65532:65532"
        assert metadata["Config"]["Entrypoint"] == ["/usr/local/bin/jj-store"]
        with socket.socket() as reserve:
            reserve.bind(("127.0.0.1", 0))
            port = reserve.getsockname()[1]
        docker("volume", "create", volume)
        docker("volume", "create", storage_volume)
        private_files({})
        mount = ["--mount", f"type=volume,src={volume},dst=/var/lib/jj-store"]
        docker("run", "--rm", *mount, image, "registry", "init", "--state-dir", "/var/lib/jj-store")
        credential = "jst1_" + secrets.token_hex(16) + "_" + secrets.token_hex(32)
        os.environ["JJ_STORE_OPERATOR_TOKEN"] = credential
        docker("run", "--rm", *mount, "--env", "JJ_STORE_OPERATOR_TOKEN", image,
               "operator", "init", "--policy", "/var/lib/jj-store/operators.json",
               "--endpoint", f"https://127.0.0.1:{port}/", "--principal", "11" * 16)
        subprocess.run(["openssl", "req", "-x509", "-newkey", "rsa:2048", "-nodes",
                        "-days", "1", "-subj", "/CN=localhost", "-addext",
                        "subjectAltName=IP:127.0.0.1,DNS:localhost", "-keyout", str(temp / "key.pem"),
                        "-out", str(temp / "cert.pem")], check=True, stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL)
        docker("run", "--detach", "--name", storage, "--publish", "127.0.0.1::9000",
               "--mount", f"type=volume,src={storage_volume},dst=/data",
               "--env", f"RUSTFS_ACCESS_KEY={contract.ACCESS}", "--env", f"RUSTFS_SECRET_KEY={contract.SECRET}",
               "--env", "RUSTFS_VOLUMES=/data", "--env", "RUSTFS_ADDRESS=0.0.0.0:9000",
               "--env", "RUSTFS_CONSOLE_ENABLE=false", contract.IMAGE)
        storage_port = docker("port", storage, "9000/tcp").stdout.strip().rsplit(":", 1)[1]
        s3 = contract.S3(f"http://127.0.0.1:{storage_port}")
        for attempt in range(120):
            try:
                if s3.request("PUT")[0] in (200, 409):
                    break
            except OSError:
                pass
            time.sleep(0.5)
        else:
            raise RuntimeError("disposable RustFS did not become ready")
        config = f'''format = 2
bind = "0.0.0.0:{port}"
public_url = "https://127.0.0.1:{port}/"
state_dir = "/var/lib/jj-store/runtime"
operator_policy = "/var/lib/jj-store/operators.json"
web_root = "/usr/share/jj-store/web"
[tls]
certificate = "/var/lib/jj-store/cert.pem"
private_key = "/var/lib/jj-store/key.pem"
[access]
mode = "authenticated"
installation = "/var/lib/jj-store/installation.json"
catalog = "/var/lib/jj-store/repositories.json"
[storage]
mode = "external-s3"
endpoint = "http://127.0.0.1:{storage_port}/"
bucket = "jj-store-contract"
region = "us-east-1"
insecure_loopback = true
[storage.credentials]
source = "environment"
access_key = "JJ_STORE_S3_ACCESS_KEY"
secret_key = "JJ_STORE_S3_SECRET_KEY"
'''
        bad = config.replace('[tls]\ncertificate = "/var/lib/jj-store/cert.pem"\nprivate_key = "/var/lib/jj-store/key.pem"\n', '')
        private_files({"server.toml": config.encode(), "invalid.toml": bad.encode(),
                       "cert.pem": (temp / "cert.pem").read_bytes(), "key.pem": (temp / "key.pem").read_bytes()})
        assert docker("run", "--rm", *mount, image, "config", "check", "--config",
                      "/var/lib/jj-store/invalid.toml", check=False).returncode != 0
        os.environ["JJ_STORE_S3_ACCESS_KEY"] = contract.ACCESS
        os.environ["JJ_STORE_S3_SECRET_KEY"] = contract.SECRET
        docker("run", "--detach", "--name", name, "--network", "host", "--read-only",
               "--tmpfs", "/tmp:rw,noexec,nosuid,size=64m", "--cap-drop", "ALL",
               "--security-opt", "no-new-privileges", *mount,
               "--env", "JJ_STORE_S3_ACCESS_KEY", "--env", "JJ_STORE_S3_SECRET_KEY", image)
        context = ssl.create_default_context(cafile=str(temp / "cert.pem"))
        origin = f"https://127.0.0.1:{port}"
        def get(path):
            with urllib.request.urlopen(origin + path, context=context, timeout=5) as response:
                return response.status, response.read()
        def ready():
            for attempt in range(120):
                try:
                    if get("/")[0] == 200: return
                except (OSError, urllib.error.URLError): pass
                time.sleep(0.5)
            raise RuntimeError("authenticated container did not become ready")
        ready()
        assert get("/help/")[0] == 200
        try:
            get("/v1/repositories")
            raise AssertionError("credential-free API request unexpectedly succeeded")
        except urllib.error.HTTPError as error:
            assert error.code in (401, 403)
        docker("stop", "--time", "30", name)
        assert json.loads(docker("inspect", name).stdout)[0]["State"]["ExitCode"] == 0
        docker("start", name)
        ready()
        docker("stop", "--time", "30", storage)
        assert json.loads(docker("inspect", name).stdout)[0]["State"]["Running"]
        docker("start", storage)
        ready()
        print("Non-root/read-only container, authenticated TLS, UI/Help, restart and storage outage smoke passed.")
    finally:
        for container in [name, storage]: docker("rm", "--force", container, check=False)
        for item in [volume, storage_volume]: docker("volume", "rm", item, check=False)
        for variable in ["JJ_STORE_OPERATOR_TOKEN", "JJ_STORE_S3_ACCESS_KEY", "JJ_STORE_S3_SECRET_KEY"]:
            os.environ.pop(variable, None)
