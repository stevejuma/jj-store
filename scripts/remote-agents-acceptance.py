#!/usr/bin/env python3
"""Isolated production gateway/agent acceptance with real RustFS, synthetic credentials."""
import argparse
import base64
import json
import os
from pathlib import Path
import re
import select
import shutil
import signal
import socket
import subprocess
import tempfile
import time
import urllib.error
import urllib.request
import uuid


def identity():
    return uuid.uuid4().hex


def free_port():
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        return listener.getsockname()[1]


def request(origin, path, body=None, token=None, handle=None, expected=200):
    headers = {'x-jj-store-protocol': '1', 'Origin': origin,
               'Sec-Fetch-Site': 'same-origin', 'Sec-Fetch-Mode': 'cors',
               'Sec-Fetch-Dest': 'empty'}
    if token:
        headers['Authorization'] = 'Bearer ' + token
    if handle:
        headers['x-jj-store-connection-session'] = handle
    if body is not None:
        headers['Content-Type'] = 'application/json'
    call = urllib.request.Request(origin + '/v1/' + path, headers=headers,
                                  data=None if body is None else json.dumps(body).encode())
    try:
        response = urllib.request.urlopen(call, timeout=40)
    except urllib.error.HTTPError as error:
        response = error
    with response:
        data = response.read()
        assert response.status == expected, (path, response.status, data[:200])
        if 'application/json' in response.headers.get('Content-Type', ''):
            return json.loads(data)
        return data


def eventually(read, predicate, timeout=240):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        value = read()
        if predicate(value):
            return value
        time.sleep(.2)
    raise AssertionError('bounded acceptance deadline')


class Installation:
    def __init__(self, args, root, label, env, origin=None):
        self.args, self.root, self.env = args, root, env
        self.state = root / label
        self.origin = origin or 'http://127.0.0.1:' + str(free_port())
        self.cli(args.server, ['init', '--local', '--state-dir', str(self.state),
                              '--rustfs-executable', str(args.rustfs)])
        self.config = self.state / 'server.toml'
        text = self.config.read_text().replace('http://127.0.0.1:7443', self.origin)
        text = text.replace('127.0.0.1:7443', self.origin.removeprefix('http://'))
        self.config.write_text('web_root = ' + json.dumps(str(args.web)) + '\n' + text)
        self.errors = (root / (label + '.stderr')).open('w+')
        self.process = None

    def cli(self, binary, argv, extra=None):
        result = subprocess.run([str(binary), '--no-pager', '--color=never', *argv],
                                cwd=self.root, env=dict(self.env, **(extra or {})),
                                capture_output=True, text=True, timeout=120)
        assert result.returncode == 0, ('CLI failed', argv[0], result.stderr[:500])
        return result.stdout.strip()

    def start(self):
        self.process = subprocess.Popen([str(self.args.server), '--no-pager', '--format=json',
                                         'run', '--config', str(self.config)], cwd=self.root,
                                        env=self.env, stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=self.errors, text=True)
        assert select.select([self.process.stdout], [], [], 60)[0], 'server readiness deadline'
        line = self.process.stdout.readline()
        if not line:
            self.errors.flush()
            self.errors.seek(0)
            raise AssertionError(self.errors.read()[-2000:])
        assert json.loads(line)['address'] == self.origin.removeprefix('http://')

    def stop(self):
        if self.process and self.process.poll() is None:
            self.process.send_signal(signal.SIGTERM)
            try:
                self.process.wait(timeout=40)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=5)
                raise AssertionError('graceful shutdown deadline')
            assert self.process.returncode == 0, 'server shutdown failed'
        assert not (self.state / 'native.sock').exists()


def qualify(args, root):
    environment = dict(os.environ, JJ_STORE_AGENT_PRIVACY_CACHE=str(root / 'privacy-cache'))
    for name in list(environment):
        if name.startswith(('JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_',
                            'JJ_STORE_ACP_', 'JJ_STORE_AGENT_JJ_', 'JJ_STORE_REPLICATION_TOKEN_')):
            environment.pop(name)
    if args.privacy_cache:
        shutil.copytree(args.privacy_cache, root / 'privacy-cache')
    acp = root / 'fixture-agent'
    shutil.copyfile(Path(__file__).resolve().parent.parent /
                    'crates/jj-store-cli/tests/fixtures/agent_control.py', acp)
    acp.chmod(0o700)
    remote = Installation(args, root, 'remote', environment)
    local = Installation(args, root, 'local', environment)
    try:
        generate = lambda: json.loads(remote.cli(args.server, ['token', 'generate', '--format=json']))
        creator, operator, admin, user, outsider = [generate() for _ in range(5)]
        principal, other_principal = identity(), identity()
        for credential, permission in [(creator, 'create-repository'), (operator, 'manage-identities')]:
            remote.cli(args.server, ['operator', 'grant', '--policy', str(remote.state / 'operators.json'),
                       '--endpoint', remote.origin, '--insecure-loopback', '--principal', identity(),
                       '--permission', permission], {'JJ_STORE_OPERATOR_TOKEN': credential['credential']})
        text = remote.config.read_text()
        text = re.sub(r'\[access\].*?(?=\n\[)', '[access]\nmode = "authenticated"\n' +
                      'installation = ' + json.dumps(str(remote.state / 'installation.json')) + '\n' +
                      'catalog = ' + json.dumps(str(remote.state / 'repositories.json')) + '\n', text,
                      flags=re.S)
        text += '\n[remote_agents]\nacp_executable = ' + json.dumps(str(acp)) + '\n'
        text += 'jj_executable = ' + json.dumps(str(args.jj)) + '\n'
        text += ('[[remote_agents.agents]]\nprincipals = ["' + principal + '"]\n'
                 '[remote_agents.agents.setup]\nid = "fixture"\nname = "Remote fixture"\n'
                 'avatar = "🤖"\ndescription = "Deterministic acceptance"\n'
                 'instructions = "Private operator instruction"\nrunner = "codex"\n'
                 '[remote_agents.agents.setup.preferences]\nmode = "ask"\n'
                 '[remote_agents.agents.setup.options]\ninherit_defaults = false\n')
        remote.config.write_text(text)
        remote.start()
        local.start()
        repos = []
        for label in ['home', 'code']:
            created = json.loads(remote.cli(args.jj, ['store', 'init', str(root / label), '--name', label,
                '--endpoint', remote.origin, '--insecure-loopback', '--cache', str(root / (label + '-cache')),
                '--state-dir', str(root / (label + '-client')), '--format=json'],
                {'JJ_STORE_OPERATOR_TOKEN': creator['credential'], 'JJ_STORE_TOKEN': admin['credential']}))
            repos.append(created['repository'])
        home, code = repos
        source = root / 'code/source.txt'
        source.write_text('committed source\n')
        remote.cli(args.jj, ['-R', str(root / 'code'), 'status'], {'JJ_STORE_TOKEN': admin['credential']})
        source.write_text('dirty source preserved\n')
        request(remote.origin, 'repos/' + home + '/work/commands',
                dict(version=1, request_id=identity(), command=dict(command='create_project', id=identity(),
                     title='Remote acceptance home', body='')), admin['credential'])
        for credential, actor in [(user, principal), (outsider, other_principal)]:
            request(remote.origin, 'directory/identities', dict(request_id=identity(), principal=actor,
                    credential=credential['credential'], expires_at=None), operator['credential'])
            for repo in repos:
                remote.cli(args.server, ['token', 'grant', '--endpoint', remote.origin, '--insecure-loopback',
                           '--repo-id', repo, '--principal', actor, '--permission', 'write',
                           '--request', str(root / (actor + repo + '.json'))],
                           {'JJ_STORE_TOKEN': admin['credential'], 'JJ_STORE_GRANT_TOKEN': credential['credential']})
        connected = request(local.origin, 'local/connections/connect', dict(id=None, name='Build host',
                            origin=remote.origin, token=user['credential']))
        prefix = 'local/connections/' + connected['connection']['id'] + '/api/'
        gateway = lambda path, body=None, expected=200: request(local.origin, prefix + path, body,
                                                               handle=connected['handle'], expected=expected)
        assert connected['principal'] == principal
        assert len(gateway('repositories')['repositories']) == 2
        status = gateway('agents')
        assert [a['id'] for a in status['setups']] == ['fixture']
        assert status['setups'][0]['instructions'] == ''
        request(remote.origin, 'agents', token=outsider['credential'], expected=403)
        gateway('local/agents', expected=400)
        image = base64.b64decode('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGP4z8DwHwAFAAH/iZk9HQAAAABJRU5ErkJggg==')
        media = gateway('directory/commands', dict(request_id=identity(), command=dict(type='upload', bytes=list(image))))
        asset = media['result']['asset']
        gateway('directory/commands', dict(request_id=identity(), command=dict(type='save_profile', expected=None,
                details=dict(display_name='Remote collaborator', description='Remote profile', email=None, avatar=asset))))
        image = gateway('directory/media/' + asset.removeprefix('blob:'))
        assert image.startswith(b'\x89PNG')
        assert image == request(remote.origin, 'directory/media/' + asset.removeprefix('blob:'), token=user['credential'])
        assert request(local.origin, 'directory/profile')['details']['display_name'] != 'Remote collaborator'
        print('PASS real gateway: native discovery, allowlists, profile and binary avatar isolation', flush=True)
        browser_input = dict(local=local.origin, remote=remote.origin, token=user['credential'],
                             connection=connected['connection'], avatar=asset, image=list(image))
        subprocess.run(['node', str(Path(__file__).with_name('remote-agents-browser.mjs'))],
                       input=json.dumps(browser_input), env=environment, text=True, check=True, timeout=180)
        command = dict(version=1, request_id=identity(), conversation_id=identity(),
                       origin=dict(scope='general', home=home, thread=None, agent_target=None, context=''),
                       agent_ids=['fixture'], attachments=[dict(repository=code, target_id=None,
                       operation=None, commit=None)], text='acceptance work', comments=None, max_duration_ms=10000)
        request(remote.origin, 'agents/chat', command, outsider['credential'], expected=403)
        def dispatch():
            try:
                return gateway('agents/chat', command)
            except AssertionError as failure:
                if (failure.args[0][1] != 409 or
                        json.loads(failure.args[0][2]).get('code') != 'agent_execution_busy'):
                    raise
                return None
        sent = eventually(dispatch, lambda v: v is not None, timeout=300)
        outcome = sent['agents'][0]
        assert outcome['state'] != 'failed', outcome
        session, original = outcome['session_id'], outcome['request_id']
        eventually(lambda: gateway('agents/outcome?request=' + original),
                   lambda v: v['state'] == 'completed')
        def published(value):
            s = next(s for s in value['sessions'] if s['id'] == session)
            assert s['workspace'] == ''
            return s['state'] == 'idle' and s['published_sequence'] == s['last_sequence']
        eventually(lambda: gateway('agents'), published)
        events = gateway('agents/events?session=' + session)['events']
        assert sum(e['payload'].get('type') == 'started' for e in events) == 1
        assert source.read_text() == 'dirty source preserved\n'
        assert not (root / 'code/agent-edit.txt').exists()
        remote.stop()
        remote.start()
        replay = gateway('agents/chat', command)
        assert replay['agents'][0]['request_id'] == original
        assert replay['agents'][0]['state'] == 'completed'
        after = gateway('agents/events?session=' + session)['events']
        assert sum(e['payload'].get('type') == 'started' for e in after) == 1
        assert gateway('repos/' + home + '/agent/sessions')['sessions']
        print('PASS real remote restart: published history, original receipt and no prompt replay', flush=True)
        remote.cli(args.server, ['token', 'revoke', '--endpoint', remote.origin, '--insecure-loopback',
                   '--repo-id', code, '--token-id', user['token_id'], '--request', str(root / 'revoke-code.json')],
                   {'JJ_STORE_TOKEN': admin['credential']})
        gateway('agents/chat', dict(command, request_id=identity()), expected=403)
        print('PASS native code grant revocation independently blocks new agent work', flush=True)
        request(remote.origin, 'directory/identity-revoke',
                dict(request_id=identity(), token=user['token_id']), operator['credential'])
        gateway('agents/events?session=' + session, expected=403)
        gateway('agents/chat', command, expected=403)
        remote.stop()
        replacement = Installation(args, root, 'replacement', environment, origin=remote.origin)
        try:
            replacement.start()
            gateway('agents/chat', command, expected=409)
            gateway('bootstrap', expected=403)
            print('PASS endpoint replacement: installation fence rejects work and invalidates session', flush=True)
        finally:
            replacement.stop()
            replacement.errors.close()
        local.stop()
        local.start()
        saved = request(local.origin, 'local/connections')
        assert saved == [connected['connection']]
        gateway('bootstrap', expected=403)
        for path in [local.state / 'connections/connections.json', remote.state / 'remote-agents/scopes.json']:
            assert user['credential'] not in path.read_text()
        print('PASS revocation and local restart: authority denied, hints retained, credentials absent', flush=True)
    finally:
        try:
            local.stop()
        finally:
            remote.stop()
            local.errors.close()
            remote.errors.close()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['server', 'jj', 'rustfs', 'web']:
        parser.add_argument('--' + name, type=Path, required=True)
    parser.add_argument('--privacy-cache', type=Path)
    args = parser.parse_args()
    for name in ['server', 'jj', 'rustfs', 'web']:
        setattr(args, name, getattr(args, name).resolve(strict=True))
    with tempfile.TemporaryDirectory(prefix='jra-', dir='/tmp') as temporary:
        qualify(args, Path(temporary))
    print('PASS isolated roots and processes cleaned up', flush=True)
