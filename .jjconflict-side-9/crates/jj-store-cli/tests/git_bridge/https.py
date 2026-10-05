"""Disposable authenticated smart Git HTTPS fixture; no user credentials."""
import base64
import http.server
import os
import ssl
import subprocess
import sys
import urllib.parse

root, certificate, key = sys.argv[1:]
credential = base64.b64encode(b"git-fixture:fixture-only").decode()

class Handler(http.server.BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def exchange(self):
        if self.headers.get("Authorization") != "Basic " + credential:
            self.send_response(401)
            self.send_header("WWW-Authenticate", 'Basic realm="fixture"')
            self.send_header("Content-Length", "0")
            self.end_headers()
            return
        parsed = urllib.parse.urlsplit(self.path)
        if not parsed.path.startswith("/source.git/") or ".." in parsed.path:
            self.send_error(404)
            return
        length = int(self.headers.get("Content-Length", "0"))
        if length > 64 * 1024 * 1024:
            self.send_error(413)
            return
        environment = dict(os.environ, GIT_PROJECT_ROOT=root,
            GIT_HTTP_EXPORT_ALL="1", REMOTE_USER="git-fixture",
            PATH_INFO=parsed.path, REQUEST_METHOD=self.command,
            QUERY_STRING=parsed.query, CONTENT_TYPE=self.headers.get("Content-Type", ""),
            CONTENT_LENGTH=str(length))
        result = subprocess.run(["git", "http-backend"], input=self.rfile.read(length),
            env=environment, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30)
        headers, body = result.stdout.split(b"\r\n\r\n", 1)
        status = 200
        selected = []
        for line in headers.decode().splitlines():
            name, value = line.split(":", 1)
            if name.lower() == "status":
                status = int(value.strip().split()[0])
            else:
                selected.append((name, value.strip()))
        self.send_response(status)
        for name, value in selected:
            self.send_header(name, value)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    do_GET = exchange
    do_POST = exchange

server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(certificate, key)
server.socket = context.wrap_socket(server.socket, server_side=True)
print(server.server_port, flush=True)
server.serve_forever()
