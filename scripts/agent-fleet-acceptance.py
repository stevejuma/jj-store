#!/usr/bin/env python3
"""Disposable real-RustFS and installed Codex fleet acceptance (M24)."""
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
import uuid

parser = argparse.ArgumentParser()
for name in ['server', 'jj', 'rustfs', 'acp', 'web']:
    parser.add_argument('--' + name, required=True, type=Path)
args = parser.parse_args()
environment = dict(os.environ, JJ_STORE_ACP_EXECUTABLE=str(args.acp.resolve()),
                   JJ_STORE_AGENT_JJ_EXECUTABLE=str(args.jj.resolve()))
for key in ['JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_ACCESS_KEY',
            'JJ_STORE_S3_SECRET_KEY', 'JJ_STORE_S3_SESSION_TOKEN']:
    environment.pop(key, None)


def run(binary, arguments, cwd):
    result = subprocess.run([str(binary.resolve()), '--no-pager', '--color=never', *arguments],
                            cwd=cwd, env=environment, capture_output=True, text=True, timeout=90)
    if result.returncode:
        raise RuntimeError('classified CLI failure: ' + arguments[0])
    return result.stdout.strip()


def request(origin, path, body=None):
    headers = {'x-jj-store-protocol': '1', 'Origin': origin, 'Sec-Fetch-Site': 'same-origin',
               'Sec-Fetch-Mode': 'cors', 'Sec-Fetch-Dest': 'empty'}
    if body is not None:
        headers['Content-Type'] = 'application/json'
    call = urllib.request.Request(origin + '/v1' + path,
                                  data=None if body is None else json.dumps(body).encode(),
                                  headers=headers)
    with urllib.request.urlopen(call, timeout=90) as response:
        return json.load(response)


def wait(origin, predicate, timeout=240):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        value = request(origin, '/local/agents')
        if predicate(value):
            return value
        if any(a['state'] == 'error' for a in value['live_agents']):
            raise RuntimeError('worker unavailable during acceptance')
        time.sleep(.2)
    raise RuntimeError('fleet acceptance deadline')


with tempfile.TemporaryDirectory(prefix='jsf-', dir='/private/tmp') as temporary:
    root = Path(temporary)
    state = root / 'state'
    run(args.server, ['init', '--local', '--state-dir', str(state),
                     '--rustfs-executable', str(args.rustfs.resolve()), '--format=json'], root)
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        port = listener.getsockname()[1]
    origin = 'http://127.0.0.1:' + str(port)
    config = state / 'server.toml'
    config.write_text('web_root = ' + json.dumps(str(args.web.resolve())) + '\n' +
                      config.read_text().replace('127.0.0.1:7443', '127.0.0.1:' + str(port)))
    with (root / 'server.stderr').open('w') as errors:
        def start():
            child = subprocess.Popen([str(args.server.resolve()), '--no-pager', '--format=json',
                                      'run', '--config', str(config)], cwd=root, env=environment,
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, text=True)
            if not select.select([child.stdout], [], [], 60)[0]:
                child.send_signal(signal.SIGTERM)
                child.wait(timeout=40)
                raise RuntimeError('native readiness deadline')
            assert json.loads(child.stdout.readline())['address'] == '127.0.0.1:' + str(port)
            return child
        server = start()
        try:
            sources, repositories = [], []
            for index in range(2):
                source = root / ('source-' + str(index))
                created = json.loads(run(args.jj, ['store', '--format=json', 'init', str(source),
                    '--name', 'fleet/native-' + str(index), '--cache', str(root / ('cache-' + str(index))),
                    '--state-dir', str(root / ('client-' + str(index))),
                    '--workspace', 'native-source-' + str(index), '--unix-socket', str(state / 'native.sock')], root))
                source.joinpath('source.txt').write_text('base\n')
                run(args.jj, ['status'], source)
                source.joinpath('source.txt').write_text('dirty source\n')
                sources.append(source)
                repositories.append(created['repository'])
            wait(origin, lambda v: v['runtime'] == 'ready')
            print('native real-RustFS host ready; two registered repos have no selected workspace', flush=True)
            assert all(not request(origin, '/local/agents/targets?repository=' + repo)
                       for repo in repositories), 'registered repos must have no selected workspace'
            before_catalog = request(origin, '/local/agents')
            catalog = request(origin, '/local/agents/catalog?runner=codex')
            assert catalog['runner'] == 'codex'
            assert catalog['facts']['models']['availableModels'], 'installed models must be selectable before a first chat'
            after_catalog = request(origin, '/local/agents')
            assert before_catalog['revision'] == after_catalog['revision']
            assert not after_catalog['setups'] and not after_catalog['sessions'] and not after_catalog['conversations']
            print('installed model discovery before the first agent/chat passed without a prompt or workspace', flush=True)
            current = after_catalog
            setup = dict(id='native-fleet', name='Native fleet helper', avatar='🤖',
                         description='Acceptance helper', instructions='Preserve source folders.',
                         runner='codex', preferences=dict(model=None, effort=None, mode='read-only'))
            request(origin, '/local/agents/setups', dict(version=1, request_id=uuid.uuid4().hex,
                expected_revision=current['revision'], setup=setup, remove=None))
            request(origin, '/local/agents/control', dict(version=1, request_id=uuid.uuid4().hex,
                agent_id=setup['id'], start=True))
            warm = wait(origin, lambda v: any(a['state'] == 'idle' for a in v['live_agents']))
            assert warm['sessions'] == []
            send = dict(version=1, request_id=uuid.uuid4().hex, conversation_id=uuid.uuid4().hex,
                        origin=dict(scope='personal', home=None, thread=None, agent_target=None,
                                    context='General native chat'), agent_ids=[setup['id']], attachments=[],
                        text='Reply Ready. Do not access or modify files.', max_duration_ms=180000)
            outcome = request(origin, '/local/agents/chat', send)
            assert all(not a['error'] for a in outcome['agents']), 'native general preparation failed'
            first = wait(origin, lambda v: len(v['sessions']) == 1 and
                         v['sessions'][0]['state'] == 'idle' and v['sessions'][0]['last_sequence'] > 3)
            session = first['sessions'][0]['id']
            home = first['conversations'][0]['home']
            assert first['conversations'][0]['discussion_thread']
            assert home not in repositories
            print('native personal home and warm general turn passed', flush=True)
            send['request_id'] = uuid.uuid4().hex
            send['attachments'] = [dict(repository=repo, target_id=None, operation=None, commit=None)
                                   for repo in repositories]
            send['text'] = ('Create fleet-acceptance.txt containing exactly Fleet accepted followed by a newline '
                            'in each of the two attached repositories/<repository-id> directories. '
                            'Use your editing tool. Do not edit other files or commit. Reply briefly when done.')
            result = request(origin, '/local/agents/chat', send)
            assert result['agents'][0]['session_id'] == session
            assert all(not a['error'] for a in result['agents']), 'native attachment preparation failed'
            child = result['agents'][0]['request_id']
            deadline, after, decisions = time.monotonic() + 240, 0, 0
            while time.monotonic() < deadline:
                for event in request(origin, '/local/agents/events?session=' + session + '&after=' + str(after))['events']:
                    after = event['sequence']
                    if event['payload'].get('type') == 'permission':
                        decisions += 1
                        payload = event['payload']
                        option = next(o for o in payload['request']['options'] if o['kind'] == 'allow_once')
                        request(origin, '/local/agents/command', dict(version=1, request_id=uuid.uuid4().hex,
                            session_id=session, action=dict(action='permission', permission_id=payload['permissionId'],
                                                            option_id=option['optionId'])))
                outcome = request(origin, '/local/agents/outcome?request=' + child)
                if outcome['state'] not in ['queued', 'started', 'dispatching']:
                    assert outcome['state'] == 'completed', 'native edit turn did not complete'
                    break
                time.sleep(.2)
            else:
                raise RuntimeError('native edit deadline')
            completed = wait(origin, lambda v: all(s['state'] == 'idle' and
                s['last_sequence'] == s['published_sequence'] for s in v['sessions']))
            bindings = completed['sessions'][0]['repository_bindings']
            assert len(bindings) == 2 and all(b['profile'] == 'native' for b in bindings)
            for binding in bindings:
                assert Path(binding['workspace']).joinpath('fleet-acceptance.txt').read_text() == 'Fleet accepted\n'
                assert binding['commit'] != binding['base_commit']
            for source in sources:
                assert source.joinpath('source.txt').read_text() == 'dirty source\n'
                assert not source.joinpath('fleet-acceptance.txt').exists()
            before = completed['sessions'][0]['last_sequence']
            request(origin, '/local/agents/chat', send)
            assert request(origin, '/local/agents')['sessions'][0]['last_sequence'] == before
            print('two registered native repos materialized; isolated edits/snapshots and canonical history passed; '
                  + str(decisions) + ' permission decisions', flush=True)
            subprocess.run(['node', str(Path(__file__).with_name('agent-fleet-browser.mjs').resolve()),
                            origin, repositories[0]], env=environment, check=True, timeout=420)
            before_restart = wait(origin, lambda v: all(s['published_sequence'] == s['last_sequence']
                                                       for s in v['sessions']))
            server.send_signal(signal.SIGTERM)
            server.wait(timeout=40)
            assert server.returncode == 0
            server = start()
            after_restart = request(origin, '/local/agents')
            assert [(s['id'], s['last_sequence']) for s in after_restart['sessions']] == [
                (s['id'], s['last_sequence']) for s in before_restart['sessions']]
            assert all(not s['connected'] for s in after_restart['sessions'])
            assert after_restart['conversations'] == before_restart['conversations']
            request(origin, '/local/agents/chat', send)
            assert [(s['id'], s['last_sequence']) for s in request(origin, '/local/agents')['sessions']] == [
                (s['id'], s['last_sequence']) for s in before_restart['sessions']]
            print('cold host restart retained history and duplicate Send never replayed', flush=True)
        finally:
            if server.poll() is None:
                server.send_signal(signal.SIGTERM)
                server.wait(timeout=40)
            assert server.returncode == 0, 'native service shutdown failed'
    assert not (state / 'native.sock').exists()
    print('native fleet foreground shutdown released the RPC socket', flush=True)
