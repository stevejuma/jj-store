#!/usr/bin/env python3
"""Cold real-RustFS / installed-runner acceptance; every file is disposable."""
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
import urllib.error
import urllib.request
import uuid

parser = argparse.ArgumentParser()
for name in ['server', 'jj', 'rustfs', 'acp']:
    parser.add_argument('--' + name, required=True, type=Path)
parser.add_argument('--web', type=Path)
parser.add_argument('--browser', action='store_true')
args = parser.parse_args()
environment = dict(os.environ, JJ_STORE_ACP_EXECUTABLE=str(args.acp.resolve()),
                   JJ_STORE_AGENT_JJ_EXECUTABLE=str(args.jj.resolve()))
for key in ['JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_ACCESS_KEY', 'JJ_STORE_S3_SECRET_KEY', 'JJ_STORE_S3_SESSION_TOKEN']:
    environment.pop(key, None)

def run(binary, arguments, cwd):
    result = subprocess.run([str(binary.resolve()), '--no-pager', '--color=never', *arguments],
                            cwd=cwd, env=environment, capture_output=True, text=True, timeout=90)
    if result.returncode:
        raise RuntimeError('classified CLI failure: ' + arguments[0])
    return result.stdout.strip()

def request(origin, path, body=None):
    data = None if body is None else json.dumps(body).encode()
    headers = {
        'x-jj-store-protocol': '1', 'Origin': origin, 'Sec-Fetch-Site': 'same-origin',
        'Sec-Fetch-Mode': 'cors', 'Sec-Fetch-Dest': 'empty',
    }
    if body is not None:
        headers['Content-Type'] = 'application/json'
    call = urllib.request.Request(origin + '/v1' + path, data=data, headers=headers)
    with urllib.request.urlopen(call, timeout=35) as response:
        return json.load(response)

def wait(origin, predicate, timeout=240):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        value = request(origin, '/local/agents')
        if predicate(value):
            return value
        time.sleep(.2)
    raise RuntimeError('agent runtime gate exceeded its bound')

def command(origin, session, action):
    request_id = uuid.uuid4().hex
    result = request(origin, '/local/agents/command', dict(version=1, request_id=request_id,
        session_id=session, action=action))
    return request_id, result

with tempfile.TemporaryDirectory(prefix='jse-', dir='/private/tmp') as temporary:
    root = Path(temporary)
    state = root / 'state'
    run(args.server, ['init', '--local', '--state-dir', str(state), '--rustfs-executable', str(args.rustfs.resolve()), '--format=json'], root)
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        port = listener.getsockname()[1]
    origin = 'http://127.0.0.1:' + str(port)
    config = state / 'server.toml'
    text = config.read_text().replace('127.0.0.1:7443', '127.0.0.1:' + str(port))
    if args.web:
        text = 'web_root = ' + json.dumps(str(args.web.resolve())) + '\n' + text
    config.write_text(text)
    diagnostics = root / 'server.stderr'
    with diagnostics.open('w') as errors:
        server = subprocess.Popen([str(args.server.resolve()), '--no-pager', '--format=json', 'run', '--config', str(config)],
            cwd=root, env=environment, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, text=True)
        try:
            if not select.select([server.stdout], [], [], 60)[0]:
                raise RuntimeError('native server readiness deadline')
            ready = json.loads(server.stdout.readline())
            assert ready['address'] == '127.0.0.1:' + str(port)
            source = root / 'source'
            created = json.loads(run(args.jj, ['store', '--format=json', 'init', str(source), '--name', 'agents/native',
                '--cache', str(root / 'cache'), '--state-dir', str(root / 'client-state'), '--workspace', 'native-source',
                '--unix-socket', str(state / 'native.sock')], root))
            repository = created['repository']
            source.joinpath('source.txt').write_text('base\n')
            run(args.jj, ['status'], source)
            source.joinpath('source.txt').write_text('dirty source\n')
            request(origin, '/repos/' + repository + '/local/workspaces/link', dict(version=1, expected_revision=0, path=str(source)))
            wait(origin, lambda v: v['runtime'] == 'ready')
            print('native real-RustFS server and exact workspace selection ready', flush=True)
            for runner in ['codex', 'cursor']:
                targets = request(origin, '/local/agents/targets?repository=' + repository)
                target = next(t for t in targets if t['profile'] == 'native')
                launched = request(origin, '/local/agents/launch', dict(version=1, request_id=uuid.uuid4().hex,
                    target_id=target['id'], repository=repository, operation=target['operation'], commit=target['commit'],
                    runner=runner, setup_id=None, context=None))
                session = launched['session_id']
                def view(value):
                    return next(s for s in value['sessions'] if s['id'] == session)
                status = wait(origin, lambda v: view(v)['state'] in ['idle', 'interrupted'])
                assert view(status)['state'] == 'idle', 'native adapter failed to open'
                mode = 'read-only' if runner == 'codex' else 'agent'
                command(origin, session, dict(action='configure', preferences=dict(model=None, effort=None, mode=mode)))
                wait(origin, lambda v: view(v)['capabilities'].get('modeReady') is True)
                request_id, _ = command(origin, session, dict(action='prompt', max_duration_ms=180000,
                    text='Create agent-acceptance.txt in this isolated workspace with exactly ACP integration accepted followed by a newline. Use a file editing tool, change no other file, do not commit, then reply briefly.'))
                deadline = time.monotonic() + 210
                after = 0
                permissions = 0
                while time.monotonic() < deadline:
                    page = request(origin, '/local/agents/events?session=' + session + '&after=' + str(after))
                    for event in page['events']:
                        after = event['sequence']
                        payload = event['payload']
                        if payload.get('type') == 'permission':
                            option = next(o for o in payload['request']['options'] if o['kind'] == 'allow_once')
                            command(origin, session, dict(action='permission', permission_id=payload['permissionId'], option_id=option['optionId']))
                            permissions += 1
                    outcome = request(origin, '/local/agents/outcome?request=' + request_id)
                    if outcome['state'] not in ['dispatching', 'queued', 'started']:
                        assert outcome['state'] == 'completed', 'native agent turn did not complete'
                        break
                    time.sleep(.2)
                else:
                    raise RuntimeError('native turn deadline')
                status = wait(origin, lambda v: view(v)['state'] == 'idle' and view(v)['published_sequence'] == view(v)['last_sequence'])
                workspace = Path(view(status)['workspace'])
                assert workspace.joinpath('agent-acceptance.txt').read_text() == 'ACP integration accepted\n'
                assert source.joinpath('source.txt').read_text() == 'dirty source\n'
                assert not source.joinpath('agent-acceptance.txt').exists()
                command(origin, session, dict(action='end'))
                wait(origin, lambda v: view(v)['state'] == 'ended')
                time.sleep(.5)
                print(runner + ': native isolated edit, source preservation, AgentStore publication; ' + str(permissions) + ' permission decisions', flush=True)
            if args.browser:
                assert args.web, 'browser qualification needs a built web root'
                subprocess.run(['node', str(Path(__file__).with_name('agent-execution-browser.mjs').resolve()), origin, repository], env=environment, check=True, timeout=300)
            before = request(origin, '/local/agents')
            server.send_signal(signal.SIGTERM)
            server.wait(timeout=40)
            assert server.returncode == 0
            server = subprocess.Popen([str(args.server.resolve()), '--no-pager', '--format=json', 'run', '--config', str(config)],
                cwd=root, env=environment, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, text=True)
            assert select.select([server.stdout], [], [], 60)[0]
            assert json.loads(server.stdout.readline())['address'] == '127.0.0.1:' + str(port)
            after_restart = request(origin, '/local/agents')
            assert [(s['id'], s['last_sequence']) for s in after_restart['sessions']] == [(s['id'], s['last_sequence']) for s in before['sessions']]
            assert all(s['state'] in ['ended', 'interrupted'] for s in after_restart['sessions'])
            wait(origin, lambda v: v['runtime'] == 'ready')
            selected = after_restart['sessions'][0]
            session = selected['id']
            old_events = request(origin, '/local/agents/events?session=' + session)['events']
            old_prompts = sum(e['payload'].get('type') == 'prompt' for e in old_events)
            action = 'resume' if selected['capabilities'].get('initialize', {}).get('loadSession') else 'continue_with_context'
            command(origin, session, dict(action=action))
            wait(origin, lambda v: next(s for s in v['sessions'] if s['id'] == session)['state'] in ['idle', 'interrupted'])
            retained = request(origin, '/local/agents/events?session=' + session)['events']
            assert sum(e['payload'].get('type') == 'prompt' for e in retained) == old_prompts
            command(origin, session, dict(action='end'))
            wait(origin, lambda v: next(s for s in v['sessions'] if s['id'] == session)['state'] == 'ended')
            print('native cold restart retained sessions and exact history; explicit ' + action + ' sent no prompt', flush=True)
        finally:
            if server.poll() is None:
                server.send_signal(signal.SIGTERM)
                try:
                    server.wait(timeout=40)
                except subprocess.TimeoutExpired:
                    server.kill()
                    server.wait(timeout=5)
                    raise RuntimeError('native server graceful shutdown deadline')
            if server.returncode != 0:
                print(diagnostics.read_text(), flush=True)
                raise RuntimeError('native server did not shut down cleanly')
    assert not (state / 'native.sock').exists()
    print('native real-RustFS foreground shutdown released the RPC socket', flush=True)
