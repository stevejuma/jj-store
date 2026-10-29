#!/usr/bin/env python3
"""Disposable native/local durable-comment and cold built-browser acceptance.
Uses a deterministic ACP fixture; this does not qualify real model compliance.
"""
import argparse
import json
import os
from pathlib import Path
import select
import signal
import socket
import subprocess
import tempfile
import time
import urllib.request
import urllib.error
import uuid

parser = argparse.ArgumentParser()
for name in ['server', 'jj', 'rustfs', 'web']:
    parser.add_argument('--' + name, required=True, type=Path)
parser.add_argument("--profile", choices=["native", "local_jj_git", "both"], default="both")
args = parser.parse_args()
checkout = Path(__file__).resolve().parent.parent

def request(origin, path, body=None):
    headers = {'x-jj-store-protocol': '1', 'Origin': origin, 'Sec-Fetch-Site': 'same-origin',
               'Sec-Fetch-Mode': 'cors', 'Sec-Fetch-Dest': 'empty'}
    if body is not None:
        headers['Content-Type'] = 'application/json'
    call = urllib.request.Request(origin + '/v1' + path,
        data=None if body is None else json.dumps(body).encode(), headers=headers)
    try:
        with urllib.request.urlopen(call, timeout=120) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        raise RuntimeError(path + ": " + error.read().decode()) from error

def wait(origin, predicate, timeout=120):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        value = request(origin, '/local/agents')
        if predicate(value):
            return value
        time.sleep(.2)
    raise RuntimeError('conversation acceptance deadline')

with tempfile.TemporaryDirectory(prefix='jj-conversation-', dir='/private/tmp') as temporary:
    root = Path(temporary)
    state = root / 'state'
    acp = root / 'fixture-acp'
    original = (checkout / 'crates/jj-store-cli/tests/fixtures/agent_control.py').read_text()
    marker = "        emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=f'Fixture in {workspace}')))"
    replacement = '''        import re
        refs = re.findall(r'- Discussion: ([a-z0-9-]+); comment: ([a-z0-9-]+);', frame.get('context', ''))
        groups = {}
        for discussion, comment in refs:
            groups.setdefault(discussion, []).append(comment)
        answer = 'Fixture source for Unicode 🦀 comments.'
        if groups:
            answer = 'Ordinary introduction.\\n' + '\\nBetween answers.\\n'.join('<comment-reply discussion="' + d + '" comments="' + ' '.join(cs) + '">\\nAnswered 🦀.\\n</comment-reply>' for d, cs in groups.items())
        middle = len(answer) // 2
        for chunk in [answer[:middle], answer[middle:]]:
            emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=chunk)))'''
    assert marker in original
    acp.write_text(original.replace(marker, replacement))
    acp.chmod(0o700)
    environment = dict(os.environ, JJ_STORE_ACP_EXECUTABLE=str(acp), JJ_STORE_AGENT_JJ_EXECUTABLE=str(args.jj.resolve()))
    for key in ['JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_ACCESS_KEY', 'JJ_STORE_S3_SECRET_KEY', 'JJ_STORE_S3_SESSION_TOKEN']:
        environment.pop(key, None)
    def run(binary, arguments, cwd=root):
        result = subprocess.run([str(binary.resolve()), '--no-pager', '--color=never', *arguments], cwd=cwd, env=environment, capture_output=True, text=True, timeout=90)
        if result.returncode:
            raise RuntimeError('CLI failed: ' + arguments[0] + '\n' + result.stderr)
        return result.stdout.strip()
    run(args.server, ['init', '--local', '--state-dir', str(state), '--rustfs-executable', str(args.rustfs.resolve()), '--format=json'])
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        port = listener.getsockname()[1]
    origin = 'http://127.0.0.1:' + str(port)
    config = state / 'server.toml'
    config.write_text('web_root = ' + json.dumps(str(args.web.resolve())) + '\n' + config.read_text().replace('127.0.0.1:7443', '127.0.0.1:' + str(port)))
    with (root / 'server.stderr').open('w+') as errors:
        def start():
            child = subprocess.Popen([str(args.server.resolve()), '--no-pager', '--format=json', 'run', '--config', str(config)], cwd=root, env=environment, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, text=True)
            if not select.select([child.stdout], [], [], 60)[0]:
                child.send_signal(signal.SIGTERM)
                child.wait(timeout=40)
                raise RuntimeError('native startup deadline')
            line = child.stdout.readline()
            if not line:
                errors.seek(0)
                raise RuntimeError('native startup failed: ' + errors.read())
            assert json.loads(line)['address'] == '127.0.0.1:' + str(port)
            return child
        server = start()
        try:
            wait(origin, lambda v: v['runtime'] == 'ready')
            native = root / 'native'
            created = json.loads(run(args.jj, ['store', '--format=json', 'init', str(native), '--name', 'conversation/native', '--cache', str(root / 'cache'), '--state-dir', str(root / 'client'), '--workspace', 'source', '--unix-socket', str(state / 'native.sock')]))
            native.joinpath('source.txt').write_text('Native source\n')
            run(args.jj, ['status'], native)
            request(origin, '/repos/' + created['repository'] + '/local/workspaces/link', dict(version=1, expected_revision=0, path=str(native)))
            local = root / 'local'
            run(args.jj, ['git', 'init', '--colocate', str(local)])
            local.joinpath('source.txt').write_text('Local source\n')
            run(args.jj, ['status'], local)
            bindings = request(origin, '/local/repositories')
            registered = request(origin, '/local/repositories', dict(version=1, request_id=uuid.uuid4().hex, expected_revision=bindings['revision'], path=str(local), name='Conversation local'))
            assert registered['status'] == 'committed'
            sessions = []
            for profile, repository in [('native', created['repository']), ('local_jj_git', registered['binding']['repository'])]:
                if args.profile not in ['both', profile]:
                    continue
                print('Preparing ' + profile, flush=True)
                current = request(origin, '/local/agents')
                setup = dict(id='comments-' + profile, name='Comments ' + profile, avatar='🤖', description='Controlled acceptance', instructions='Test comment answers.', runner='codex', preferences=dict(model=None, effort=None, mode='ask'))
                request(origin, '/local/agents/setups', dict(version=1, request_id=uuid.uuid4().hex, expected_revision=current['revision'], setup=setup, remove=None))
                targets = request(origin, '/local/agents/targets?repository=' + repository)
                target = next(t for t in targets if t['profile'] == profile)
                launched = request(origin, '/local/agents/launch', dict(version=1, request_id=uuid.uuid4().hex, target_id=target['id'], repository=target['repository'], operation=target['operation'], commit=target['commit'], runner='codex', setup_id=setup['id'], context=None))
                session = launched['session_id']
                wait(origin, lambda v: any(s['id'] == session and s['state'] == 'idle' for s in v['sessions']))
                request(origin, '/local/agents/command', dict(version=1, request_id=uuid.uuid4().hex, session_id=session, action=dict(action='prompt', text='Start', max_duration_ms=5000)))
                wait(origin, lambda v: any(s['id'] == session and s['state'] == 'idle' and s['published_sequence'] == s['last_sequence'] for s in v['sessions']))
                subprocess.run(['node', str(checkout / 'scripts/conversation-browser.mjs'), origin, session, setup['id'], profile, 'send'], check=True, timeout=150)
                sessions.append((session, setup['id'], profile))
                # Standalone per-session execution deliberately permits one runtime.
                # End participation before qualifying the next storage profile.
                request(origin, '/local/agents/command', dict(version=1, request_id=uuid.uuid4().hex, session_id=session, action=dict(action='end')))
                wait(origin, lambda v: any(s['id'] == session and not s['connected'] for s in v['sessions']))
            before = {s: request(origin, '/local/agents/feedback?session=' + s) for s, _, _ in sessions}
            server.send_signal(signal.SIGTERM)
            server.wait(timeout=40)
            assert server.returncode == 0
            server = start()
            wait(origin, lambda v: v['runtime'] == 'ready')
            for session, setup, profile in sessions:
                assert request(origin, '/local/agents/feedback?session=' + session) == before[session]
                subprocess.run(['node', str(checkout / 'scripts/conversation-browser.mjs'), origin, session, setup, profile, 'retained'], check=True, timeout=120)
            print('Native and local cold restart retained exact comment/reply feedback and browser navigation.', flush=True)
        finally:
            if server.poll() is None:
                server.send_signal(signal.SIGTERM)
                server.wait(timeout=40)
