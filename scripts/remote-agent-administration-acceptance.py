#!/usr/bin/env python3
"""Cold operator UI and installed-provider acceptance on disposable real RustFS servers."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import time

spec = importlib.util.spec_from_file_location('remote_acceptance', Path(__file__).with_name('remote-agents-acceptance.py'))
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
request, identity, eventually = base.request, base.identity, base.eventually


def qualify(args, root):
    environment = dict(os.environ, JJ_STORE_AGENT_PRIVACY_CACHE=str(root / 'privacy-cache'))
    for name in list(environment):
        if name.startswith(('JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_',
                            'JJ_STORE_ACP_', 'JJ_STORE_AGENT_JJ_', 'JJ_STORE_REPLICATION_TOKEN_')):
            environment.pop(name)
    if args.privacy_cache:
        shutil.copytree(args.privacy_cache, root / 'privacy-cache')
    if args.tooling_bin:
        environment['PATH'] = str(args.tooling_bin) + os.pathsep + environment.get('PATH', '')
    assert shutil.which('codex-acp', path=environment.get('PATH')), 'installed codex-acp required; supply --tooling-bin'
    remote = base.Installation(args, root, 'remote', environment)
    local = base.Installation(args, root, 'local', environment)
    try:
        generate = lambda: json.loads(remote.cli(args.server, ['token', 'generate', '--format=json']))
        creator, identities, operator, admin, user, outsider = [generate() for _ in range(6)]
        actor, identity_actor, user_actor, outsider_actor = [identity() for _ in range(4)]
        for token, principal, permission in [(creator, identity(), 'create-repository'),
                                             (identities, identity_actor, 'manage-identities'),
                                             (operator, actor, 'manage-agents')]:
            remote.cli(args.server, ['operator', 'grant', '--policy', str(remote.state / 'operators.json'),
                       '--endpoint', remote.origin, '--insecure-loopback', '--principal', principal,
                       '--permission', permission], {'JJ_STORE_OPERATOR_TOKEN': token['credential']})
        text = remote.config.read_text()
        text = re.sub(r'\[access\].*?(?=\n\[)', '[access]\nmode = "authenticated"\n' +
                      'installation = ' + json.dumps(str(remote.state / 'installation.json')) + '\n' +
                      'catalog = ' + json.dumps(str(remote.state / 'repositories.json')) + '\n', text, flags=re.S)
        text += '\n[remote_agents]\nacp_executable = ' + json.dumps(str(args.acp)) + '\n'
        text += 'jj_executable = ' + json.dumps(str(args.jj)) + '\n'
        text += ('[[remote_agents.agents]]\nprincipals = ["' + actor + '"]\n'
                 '[remote_agents.agents.setup]\nid = "builder"\nname = "Builder"\n'
                 'avatar = "🤖"\ndescription = "Installed Codex acceptance"\n'
                 'instructions = "Work only in the isolated admitted workspace. Preserve source files."\nrunner = "codex"\n'
                 '[remote_agents.agents.setup.preferences]\nmode = "read-only"\n'
                 '[remote_agents.agents.setup.options]\ninherit_defaults = false\n')
        remote.config.write_text(text)
        remote.start()
        local.start()
        repositories = []
        for label in ['home', 'code']:
            created = json.loads(remote.cli(args.jj, ['store', 'init', str(root / label), '--name', label,
                '--endpoint', remote.origin, '--insecure-loopback', '--cache', str(root / (label + '-cache')),
                '--state-dir', str(root / (label + '-client')), '--format=json'],
                {'JJ_STORE_OPERATOR_TOKEN': creator['credential'], 'JJ_STORE_TOKEN': admin['credential']}))
            repositories.append(created['repository'])
        home, code = repositories
        source = root / 'code/source.txt'
        source.write_text('committed source\n')
        remote.cli(args.jj, ['-R', str(root / 'code'), 'status'], {'JJ_STORE_TOKEN': admin['credential']})
        source.write_text('dirty source preserved\n')
        request(remote.origin, 'repos/' + home + '/work/commands', dict(version=1, request_id=identity(),
                command=dict(command='create_project', id=identity(), title='Remote home', body='')), admin['credential'])
        for token, principal in [(operator, actor), (identities, identity_actor), (outsider, outsider_actor)]:
            request(remote.origin, 'directory/identities', dict(request_id=identity(), principal=principal,
                    credential=token['credential'], expires_at=None), identities['credential'])
        for token, principal in [(operator, actor), (identities, identity_actor)]:
            for repo in repositories:
                remote.cli(args.server, ['token', 'grant', '--endpoint', remote.origin, '--insecure-loopback',
                    '--repo-id', repo, '--principal', principal, '--permission', 'admin',
                    '--request', str(root / (principal + repo + '.json'))],
                    {'JJ_STORE_TOKEN': admin['credential'], 'JJ_STORE_GRANT_TOKEN': token['credential']})
        request(remote.origin, 'operator/agents', token=identities['credential'], expected=403)
        request(remote.origin, 'operator/agents', token=admin['credential'], expected=403)
        request(remote.origin, 'directory/identities', token=operator['credential'], expected=403)
        request(remote.origin, 'operator/access', token=user['credential'], expected=403)
        assert request(remote.origin, 'operator/access', token=operator['credential']) == dict(agents=True, identities=False)
        assert request(remote.origin, 'operator/access', token=identities['credential']) == dict(agents=False, identities=True)
        before = request(remote.origin, 'operator/agents', token=operator['credential'])
        connected = request(local.origin, 'local/connections/connect', dict(id=None, name='Acceptance host',
                            origin=remote.origin, token=operator['credential']))
        subprocess.run(['node', str(Path(__file__).with_name('remote-agent-administration-browser.mjs'))],
                       input=json.dumps(dict(local=local.origin, remote=remote.origin, connection=connected['connection'],
                           operator=operator['credential'], identities=identities['credential'], user=user['credential'],
                           principal=user_actor)), env=environment, text=True, check=True, timeout=300)
        saved = request(remote.origin, 'operator/agents', token=operator['credential'])
        assert saved['restart_required'] and saved['effective'] == before['effective']
        assert user_actor in saved['desired']['agents'][0]['principals']
        request(remote.origin, 'operator/agents', dict(expected_revision=before['revision'], configuration=before['desired']),
                operator['credential'], expected=409)
        assert remote.config.read_text().split('[remote_agents]')[0] != ''
        for repo in repositories:
            snapshot = request(remote.origin, 'repos/' + repo + '/agent-access', token=operator['credential'])
            intent = dict(request_id=identity(), expected=snapshot, principal=outsider_actor, credential=outsider['credential'])
            request(remote.origin, 'repos/' + repo + '/agent-access', intent, user['credential'], expected=403)
            request(remote.origin, 'repos/' + repo + '/agent-access', dict(intent, principal=user_actor), operator['credential'], expected=403)
            status = request(remote.origin, 'repos/' + repo + '/agent-access/status', intent, operator['credential'])
            assert not status['published']
            committed = request(remote.origin, 'repos/' + repo + '/agent-access', intent, operator['credential'])
            receipt = request(remote.origin, 'repos/' + repo + '/agent-access/status', intent, operator['credential'])
            assert committed == receipt == dict(request_id=intent['request_id'], published=True)
        request(remote.origin, 'agents', token=user['credential'], expected=403)
        reload_status = request(remote.origin, 'configuration/reload', token=operator['credential'])
        assert reload_status['supported'] == ['remote_agents']
        reload_command = dict(version=1, request_id=identity(), expected_revision=reload_status['saved_revision'], domains=['remote_agents'])
        server_pid = remote.process.pid
        request(remote.origin, 'configuration/reload', reload_command, operator['credential'])
        result = eventually(lambda: request(remote.origin, 'configuration/reload/outcome', reload_command, operator['credential']), lambda v: v['state'] != 'running')
        assert result['state'] == 'applied', result
        assert remote.process.pid == server_pid and remote.process.poll() is None
        assert request(remote.origin, 'configuration/reload', reload_command, operator['credential']) == result
        effective = request(remote.origin, 'operator/agents', token=operator['credential'])
        assert not effective['restart_required'] and effective['effective'] == saved['desired']
        request(remote.origin, 'agents', token=outsider['credential'], expected=403)
        fresh = request(local.origin, 'local/connections/connect', dict(id=connected['connection']['id'], name='Acceptance host',
                        origin=remote.origin, token=user['credential']))
        prefix = 'local/connections/' + fresh['connection']['id'] + '/api/'
        gateway = lambda path, body=None, expected=200: request(local.origin, prefix + path, body,
                                                               handle=fresh['handle'], expected=expected)
        assert len(gateway('repositories')['repositories']) == 2
        gateway('operator/agents', expected=403)
        command = dict(version=1, request_id=identity(), conversation_id=identity(),
                       origin=dict(scope='general', home=home, thread=None, agent_target=None, context=''),
                       agent_ids=['builder'], attachments=[dict(repository=code, target_id=None, operation=None, commit=None)],
                       text='In the isolated attached repository, create remote-acceptance.txt with exactly Remote Codex accepted followed by a newline. Use a file editing tool, change no other files, do not commit. Reply briefly.',
                       comments=None, max_duration_ms=180000)
        def dispatch():
            try:
                return gateway('agents/chat', command)
            except AssertionError as failure:
                if failure.args[0][1] != 409 or json.loads(failure.args[0][2]).get('code') != 'agent_execution_busy':
                    raise
                return None
        sent = eventually(dispatch, lambda v: v is not None, timeout=300)
        outcome = sent['agents'][0]
        if outcome['state'] == 'failed':
            status = gateway('agents')
            print('Preparation status', json.dumps([dict(id=s['id'], state=s['state'], error=s.get('last_error'))
                  for s in status['sessions']]), flush=True)
            events = gateway('agents/events?session=' + outcome['session_id'])['events']
            print('Preparation events', json.dumps([dict(type=e['payload'].get('type'),
                  configuration=e['payload'].get('configuration'), error=e['payload'].get('error'))
                  for e in events]), flush=True)
            diagnostic = remote.errors.name and Path(remote.errors.name).read_text()
            for token in [creator, identities, operator, admin, user, outsider]:
                diagnostic = diagnostic.replace(token['credential'], '[redacted]')
            print('Remote stderr', diagnostic[-8000:], flush=True)
        assert outcome['state'] != 'failed', outcome
        session, original = outcome['session_id'], outcome['request_id']
        handled = set()
        def completed():
            events = gateway('agents/events?session=' + session)['events']
            for event in events:
                payload = event['payload']
                if payload.get('type') == 'permission' and payload['permissionId'] not in handled:
                    permission = payload['request']
                    # Only this bounded, disposable-workspace edit is accepted.
                    option = next(o for o in permission['options'] if o['kind'] == 'allow_once')
                    gateway('agents/command', dict(version=1, request_id=identity(), session_id=session,
                        action=dict(action='permission', permission_id=payload['permissionId'], option_id=option['optionId'])))
                    handled.add(payload['permissionId'])
            result = gateway('agents/outcome?request=' + original)
            if result['state'] not in ['dispatching', 'queued', 'started']:
                assert result['state'] == 'completed', result
                return True
            return False
        eventually(completed, bool, timeout=240)
        def published(value):
            s = next(s for s in value['sessions'] if s['id'] == session)
            assert s['workspace'] == ''
            return s['state'] == 'idle' and s['published_sequence'] == s['last_sequence']
        eventually(lambda: gateway('agents'), published)
        assert source.read_text() == 'dirty source preserved\n'
        assert not (root / 'code/remote-acceptance.txt').exists()
        files = list((remote.state / 'remote-agents').rglob('remote-acceptance.txt'))
        assert len(files) == 1 and files[0].read_text() == 'Remote Codex accepted\n'
        assert gateway('repos/' + home + '/agent/sessions')['sessions']
        remote.stop()
        remote.start()
        replay = gateway('agents/chat', command)
        assert replay['agents'][0]['request_id'] == original and replay['agents'][0]['state'] == 'completed'
        events = gateway('agents/events?session=' + session)['events']
        assert sum(e['payload'].get('type') == 'started' for e in events) == 1
        sharing = request(remote.origin, 'operator/agents', token=operator['credential'])
        for agent in sharing['desired']['agents']:
            agent['principals'] = [p for p in agent['principals'] if p != user_actor]
        request(remote.origin, 'operator/agents', dict(expected_revision=sharing['revision'], configuration=sharing['desired']), operator['credential'])
        status = request(remote.origin, 'configuration/reload', token=operator['credential'])
        revoke = dict(version=1, request_id=identity(), expected_revision=status['saved_revision'], domains=['remote_agents'])
        pid = remote.process.pid
        request(remote.origin, 'configuration/reload', revoke, operator['credential'])
        observed = eventually(lambda: request(remote.origin, 'configuration/reload/outcome', revoke, operator['credential']), lambda v: v['state'] != 'running')
        assert observed['state'] in ['applied', 'deferred'], observed
        assert remote.process.pid == pid and remote.process.poll() is None
        gateway('agents/chat', command, expected=403)
        gateway('agents/events?session=' + session, expected=403)
        print('PASS loaded allowlist revocation fences retained session/tool/publication authority without server restart', flush=True)
        request(remote.origin, 'directory/identity-revoke', dict(request_id=identity(), token=user['token_id']), identities['credential'])
        gateway('agents/chat', command, expected=403)
        for path in [local.state / 'connections/connections.json', remote.config]:
            assert user['credential'] not in path.read_text()
        print('PASS saved sharing activated without server restart; installed Codex via authenticated gateway: isolated edit, publication, restart without replay, revocation', flush=True)
    finally:
        local.stop()
        remote.stop()
        local.errors.close()
        remote.errors.close()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['server', 'jj', 'rustfs', 'web', 'acp']:
        parser.add_argument('--' + name, type=Path, required=True)
    parser.add_argument('--privacy-cache', type=Path)
    parser.add_argument('--tooling-bin', type=Path, help='Existing installed Codex/ACP adapter bin directory')
    args = parser.parse_args()
    for name in ['server', 'jj', 'rustfs', 'web', 'acp']:
        setattr(args, name, getattr(args, name).resolve(strict=True))
    if args.tooling_bin:
        args.tooling_bin = args.tooling_bin.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix='jraa-', dir='/tmp') as temporary:
        qualify(args, Path(temporary))
    print('PASS isolated roots and processes cleaned up', flush=True)
