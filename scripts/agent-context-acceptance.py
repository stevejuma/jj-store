#!/usr/bin/env python3
"""Isolated native metadata/CLI and cold built-UI acceptance with a fixture ACP."""
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
args = parser.parse_args()
checkout = Path(__file__).resolve().parent.parent


def run(binary, arguments, cwd):
    result = subprocess.run([str(binary.resolve()), '--no-pager', '--color=never', *arguments],
                            cwd=cwd, capture_output=True, text=True, timeout=90)
    if result.returncode:
        raise RuntimeError('CLI failed: ' + arguments[0] + '\n' + result.stderr)
    return json.loads(result.stdout)


def request(origin, path, body=None):
    headers = {'x-jj-store-protocol': '1', 'Origin': origin, 'Sec-Fetch-Site': 'same-origin',
               'Sec-Fetch-Mode': 'cors', 'Sec-Fetch-Dest': 'empty'}
    if body is not None:
        headers['Content-Type'] = 'application/json'
    call = urllib.request.Request(origin + '/v1' + path,
        data=None if body is None else json.dumps(body).encode(), headers=headers)
    with urllib.request.urlopen(call, timeout=120) as response:
        return json.load(response)


def wait(origin, predicate, timeout=240):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        value = request(origin, '/local/agents')
        if predicate(value):
            return value
        time.sleep(.2)
    raise RuntimeError('context acceptance deadline')


with tempfile.TemporaryDirectory(prefix='jj-context-', dir='/private/tmp') as temporary:
    root = Path(temporary)
    state = root / 'state'
    acp = root / 'fixture-acp'
    acp.write_text((checkout / 'crates/jj-store-cli/tests/fixtures/agent_control.py').read_text())
    acp.chmod(0o700)
    binary_dir = root / 'bundled tools'
    binary_dir.mkdir()
    binary = binary_dir / 'jj'
    os.link(args.jj.resolve(), binary)
    environment = dict(os.environ, JJ_STORE_ACP_EXECUTABLE=str(acp),
                       JJ_STORE_AGENT_JJ_EXECUTABLE=str(binary))
    for key in ['JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_ACCESS_KEY',
                'JJ_STORE_S3_SECRET_KEY', 'JJ_STORE_S3_SESSION_TOKEN']:
        environment.pop(key, None)
    run(args.server, ['init', '--local', '--state-dir', str(state),
                     '--rustfs-executable', str(args.rustfs.resolve()), '--format=json'], root)
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        port = listener.getsockname()[1]
    origin = 'http://127.0.0.1:' + str(port)
    config = state / 'server.toml'
    config.write_text('web_root = ' + json.dumps(str(args.web.resolve())) + '\n' +
                      config.read_text().replace('127.0.0.1:7443', '127.0.0.1:' + str(port)))
    with (root / 'server.stderr').open('w+') as errors:
        def start():
            child = subprocess.Popen([str(args.server.resolve()), '--no-pager', '--format=json',
                'run', '--config', str(config)], cwd=root, env=environment,
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors, text=True)
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
            current = wait(origin, lambda v: v['runtime'] == 'ready')
            setup = dict(id='context-fixture', name='Context fixture', avatar='🤖',
                description='Fixture acceptance', instructions='## Standing instructions\n\nKeep **Markdown**.',
                runner='codex', preferences=dict(model=None, effort=None, mode='ask'))
            request(origin, '/local/agents/setups', dict(version=1, request_id=uuid.uuid4().hex,
                expected_revision=current['revision'], setup=setup, remove=None))
            send = dict(version=1, request_id=uuid.uuid4().hex, conversation_id=uuid.uuid4().hex,
                origin=dict(scope='personal', home=None, thread=None, agent_target=None,
                    context=json.dumps(dict(task_content=dict(title='Context fixture', body='**Original context**')))),
                agent_ids=[setup['id']], attachments=[], text='First authored message', max_duration_ms=5000)

            def turn():
                outcome = request(origin, '/local/agents/chat', send)
                assert all(not a['error'] for a in outcome['agents']), outcome
                session_id = outcome['agents'][0]['session_id']
                child_request = outcome['agents'][0]['request_id']
                wait(origin, lambda v: any(s['id'] == session_id and s['state'] == 'idle'
                    and s['last_sequence'] == s['published_sequence'] for s in v['sessions']))
                events = request(origin, '/local/agents/events?session=' + session_id)['events']
                frame = next(e['payload'] for e in events if e['payload']['type'] == 'prompt'
                    and e['payload']['requestId'] == child_request)
                assert frame['context'] == frame['sentContext']['wireContext']
                return session_id, frame

            session, first = turn()
            assert '## Standing instructions' in first['context']
            assert '**Original context**' in first['context'] and '```json' not in first['context']
            status = request(origin, '/local/agents')
            home = Path(status['sessions'][0]['workspace']) / 'coordination'
            assert home.joinpath('.jj').exists()
            projects = run(binary, ['-R', str(home), 'project', 'list', '--format=json'], root)
            project = projects['projects'][0]
            request_id, task_id = uuid.uuid4().hex, uuid.uuid4().hex
            create = ['-R', str(home), 'task', 'create', '--request-id', request_id,
                '--id', task_id, '--project', project['id'], '--expected-project-version', str(project['version']),
                '--title', 'Native coordination task', '--body', '**Native Markdown**', '--format=json']
            receipt = run(binary, create, root)
            assert run(binary, create, root) == receipt
            assert run(binary, ['-R', str(home), 'task', 'status', request_id, '--format=json'], root) == receipt
            task = run(binary, ['-R', str(home), 'task', 'show', task_id, '--format=json'], root)['tasks'][0]
            assert task['body'] == '**Native Markdown**'
            run(binary, ['-R', str(home), 'task', 'comment', task_id, '--request-id', uuid.uuid4().hex,
                '--expected-task-version', str(task['version']), '--body', 'Bound native task reply', '--format=json'], root)
            assert run(binary, ['-R', str(home), 'task', 'show', task_id, '--format=json'], root)['comments'][0]['body'] == 'Bound native task reply'
            task = run(binary, ['-R', str(home), 'task', 'show', task_id, '--format=json'], root)['tasks'][0]
            project = run(binary, ['-R', str(home), 'project', 'show', project['id'], '--format=json'], root)['projects'][0]
            run(binary, ['-R', str(home), 'task', 'edit', task_id, '--request-id', uuid.uuid4().hex,
                '--expected-version', str(task['version']), '--expected-project-version', str(project['version']),
                '--title', task['title'], '--body', task['body'], '--state', 'in-progress', '--format=json'], root)
            assert run(binary, ['-R', str(home), 'task', 'show', task_id, '--format=json'], root)['tasks'][0]['state'] == 'in_progress'
            print('native coordination CLI reads, task/comment writes and exact receipts passed', flush=True)
            send.update(request_id=uuid.uuid4().hex, text='Second authored message')
            assert turn()[1]['context'] == ''
            send.update(request_id=uuid.uuid4().hex, text='Changed context message')
            send['origin']['context'] = json.dumps(dict(task_content=dict(title='Context fixture', body='**Updated context**')))
            changed = turn()[1]['context']
            assert '**Updated context**' in changed and 'Standing instructions' not in changed
            subprocess.run(['node', str(checkout / 'scripts/agent-context-browser.mjs'), origin, session], check=True, timeout=90)
            before = request(origin, '/local/agents/events?session=' + session)['events']
            server.send_signal(signal.SIGTERM)
            server.wait(timeout=40)
            assert server.returncode == 0
            server = start()
            wait(origin, lambda v: v['runtime'] == 'ready')
            assert request(origin, '/local/agents/events?session=' + session)['events'] == before
            request(origin, '/local/agents/chat', send)  # Exact duplicate never prompts.
            assert request(origin, '/local/agents/events?session=' + session)['events'] == before
            send.update(request_id=uuid.uuid4().hex, text='Restored authored message')
            assert turn()[1]['context'] == ''
            assert run(binary, ['-R', str(home), 'task', 'status', request_id, '--format=json'], root) == receipt
            subprocess.run(['node', str(checkout / 'scripts/agent-context-browser.mjs'), origin, session], check=True, timeout=90)
            print('cold native restart, loaded-session deltas, retained task receipt and built UI passed', flush=True)
            pointer = home / '.jj/repo'
            metadata = (pointer.parent / pointer.read_text().strip()).resolve() if pointer.is_file() else pointer
            connection = metadata / 'store-connection.json'
            original = connection.read_bytes()
            before = request(origin, '/local/agents/events?session=' + session)['events']
            try:
                altered = json.loads(original)
                altered['repository'] = uuid.uuid4().hex
                connection.write_text(json.dumps(altered))
                send.update(request_id=uuid.uuid4().hex, text='Binding must be verified before Send')
                try:
                    outcome = request(origin, '/local/agents/chat', send)
                except urllib.error.HTTPError as error:
                    assert error.code == 503, error.code
                    # Preparation failure is retained even when the initial HTTP
                    # response reports unavailability. Exact retry only reads it.
                    outcome = request(origin, '/local/agents/chat', send)
                assert all(a['state'] == 'failed' and a['error'] for a in outcome['agents']), outcome
                assert request(origin, '/local/agents/events?session=' + session)['events'] == before
            finally:
                connection.write_bytes(original)
            assert request(origin, '/local/agents/chat', send) == outcome
            assert request(origin, '/local/agents/events?session=' + session)['events'] == before
            send.update(request_id=uuid.uuid4().hex, text='Explicit Send after binding recovery')
            assert turn()[1]['context'] == ''
            print('mismatched metadata binding refuses prompting, retains failed request and recovers on explicit Send', flush=True)
        finally:
            if server.poll() is None:
                server.send_signal(signal.SIGTERM)
                server.wait(timeout=40)
            assert server.returncode == 0, 'native shutdown failed'
    assert not (state / 'native.sock').exists()
    print('isolated native service and storage stopped; fixture directory removed', flush=True)
