#!/usr/bin/env python3
"""Isolated real-storage application reload, retained workers and cold built UI."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

spec = importlib.util.spec_from_file_location('remote_acceptance', Path(__file__).with_name('remote-agents-acceptance.py'))
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
request, identity, eventually = base.request, base.identity, base.eventually


def reload_saved(instance, domains):
    status = request(instance.origin, 'configuration/reload')
    command = dict(version=1, request_id=identity(), expected_revision=status['saved_revision'], domains=domains)
    pid = instance.process.pid
    request(instance.origin, 'configuration/reload', command)
    result = eventually(lambda: request(instance.origin, 'configuration/reload/outcome', command), lambda v: v['state'] != 'running')
    assert result['state'] in ['applied', 'current'], result
    assert instance.process.pid == pid and instance.process.poll() is None
    assert request(instance.origin, 'configuration/reload', command) == result
    return result


def save(instance, section, configuration):
    current = request(instance.origin, 'local/settings')
    return request(instance.origin, 'local/settings', dict(version=1, expected_revision=current['revision'], update=dict(section=section, configuration=configuration)))


def qualify(args, root):
    environment = dict(os.environ, HOME=str(root / 'home'), CODEX_HOME=str(root / 'home/codex'))
    (root / 'home/codex').mkdir(parents=True)
    for key in list(environment):
        if key.startswith(('JJ_STORE_TOKEN', 'JJ_STORE_OPERATOR_TOKEN', 'JJ_STORE_S3_', 'JJ_STORE_ACP_', 'JJ_STORE_AGENT_JJ_', 'JJ_STORE_REPLICATION_TOKEN_')):
            environment.pop(key)
    local = base.Installation(args, root, 'local', environment)
    remote = base.Installation(args, root, 'remote', environment)
    try:
        creator, admin = [json.loads(remote.cli(args.server, ['token', 'generate', '--format=json'])) for _ in range(2)]
        remote.cli(args.server, ['operator', 'grant', '--policy', str(remote.state / 'operators.json'), '--endpoint', remote.origin, '--insecure-loopback', '--principal', identity(), '--permission', 'create-repository'], {'JJ_STORE_OPERATOR_TOKEN': creator['credential']})
        text = remote.config.read_text()
        text = re.sub(r'\[access\].*?(?=\n\[)', '[access]\nmode = "authenticated"\ninstallation = ' + json.dumps(str(remote.state / 'installation.json')) + '\ncatalog = ' + json.dumps(str(remote.state / 'repositories.json')) + '\n', text, flags=re.S)
        remote.config.write_text(text)
        local.env = dict(environment, JJ_STORE_REPLICATION_TOKEN_ACCEPTANCE=admin['credential'])
        remote.start()
        local.start()
        source = root / 'source'
        created = json.loads(remote.cli(args.jj, ['store', 'init', str(source), '--name', 'reload/source', '--endpoint', remote.origin, '--insecure-loopback', '--cache', str(root / 'source-cache'), '--state-dir', str(root / 'source-client'), '--format=json'], {'JJ_STORE_OPERATOR_TOKEN': creator['credential'], 'JJ_STORE_TOKEN': admin['credential']}))
        repo = created['repository']
        (source / 'source.txt').write_text('Original source\n')
        remote.cli(args.jj, ['-R', str(source), 'status'], {'JJ_STORE_TOKEN': admin['credential']})
        remote.cli(args.jj, ['-R', str(source), 'store', 'migrate-local', '--unix-socket', str(local.state / 'native.sock'), '--cache', str(root / 'local-cache'), '--state-dir', str(root / 'migration')], {'JJ_STORE_TOKEN': admin['credential']})
        repos_before = request(local.origin, 'repositories')
        settings = request(local.origin, 'local/settings')
        limits = dict(settings['desired']['service'], concurrent_requests=5, request_seconds=45)
        saved = save(local, 'service', limits)
        assert saved['restart_required'] and saved['effective'] == settings['effective']
        pid = local.process.pid
        subprocess.run(['node', str(Path(__file__).with_name('runtime-reload-browser.mjs'))], input=json.dumps(dict(origin=local.origin, configuration=limits)), text=True, check=True, timeout=180)
        assert local.process.pid == pid and local.process.poll() is None
        assert request(local.origin, 'local/settings')['effective']['service'] == limits
        assert not request(local.origin, 'local/settings')['restart_required']
        assert request(local.origin, 'repositories') == repos_before
        print('PASS real storage: UI activation, draft retention, unchanged PID/catalog; cold mobile/dark/Axe', flush=True)
        protected = dict(bookmarks=['main'], commits=[])
        save(local, 'protection', protected)
        reload_saved(local, ['protection'])
        assert request(local.origin, 'local/settings')['effective']['protection'] == protected
        search = request(local.origin, 'repos/' + repo + '/local/search')
        request(local.origin, 'repos/' + repo + '/local/search/save', dict(version=1, expected_revision=search['revision'], configuration=dict(search['desired'], poll_millis=250)))
        reload_saved(local, ['search'])
        assert request(local.origin, 'repos/' + repo + '/local/search')['effective']['poll_millis'] == 250
        capture = request(local.origin, 'local/capture/settings')
        configuration = dict(capture['desired'], enabled=True, harnesses=[dict(profile='codex-cli-jsonl-v1', product_version='0.156.1', roots=[str(root / 'transcripts')])])
        (root / 'transcripts').mkdir()
        request(local.origin, 'local/capture/settings', dict(version=1, expected_revision=capture['revision'], configuration=configuration))
        reload_saved(local, ['capture'])
        active = request(local.origin, 'local/capture')
        policy = active['policy']
        paused = request(local.origin, 'local/capture/control', dict(version=1, request_id=identity(), generation=policy['generation'], expected_revision=policy['revision'], action='pause'))
        capture = request(local.origin, 'local/capture/settings')
        request(local.origin, 'local/capture/settings', dict(version=1, expected_revision=capture['revision'], configuration=dict(configuration, enabled=False)))
        reload_saved(local, ['capture'])
        capture = request(local.origin, 'local/capture/settings')
        request(local.origin, 'local/capture/settings', dict(version=1, expected_revision=capture['revision'], configuration=configuration))
        reload_saved(local, ['capture'])
        after = request(local.origin, 'local/capture')
        assert after['policy']['generation'] == policy['generation'] and after['policy']['paused']
        assert after['policy']['revision'] >= policy['revision']
        print('PASS capture: first enable, drain/disable/re-enable, retained generation and pause policy', flush=True)
        settings = request(local.origin, 'local/settings')
        sync = dict(repository=repo, upstream=remote.origin, insecure_loopback=True, token_environment='JJ_STORE_REPLICATION_TOKEN_ACCEPTANCE', ca_certificate=None, limits=settings['sync_defaults'])
        save(local, 'sync', [sync])
        reload_saved(local, ['sync'])
        status_path = 'repos/' + repo + '/local/sync'
        before = eventually(lambda: request(local.origin, status_path)['worker'], lambda v: v and v['phase'] == 'running')
        request(local.origin, status_path + '/control', dict(version=1, action='pause'))
        paused = eventually(lambda: request(local.origin, status_path)['worker'], lambda v: v and v['phase'] == 'paused')
        sync['limits'] = dict(sync['limits'], poll_millis=300)
        save(local, 'sync', [sync])
        reload_saved(local, ['sync'])
        after = eventually(lambda: request(local.origin, status_path)['worker'], lambda v: v and v['phase'] == 'paused')
        assert after['local_replica'] == paused['local_replica']
        assert after['checkpoint_revision'] >= paused['checkpoint_revision']
        assert after['peer_replica'] in [None, before['peer_replica']]
        assert request(local.origin, 'repositories') == repos_before
        print('PASS two real-storage instances: sync activation/replacement, retained pause/checkpoint and repository identity', flush=True)
    finally:
        local.stop()
        remote.stop()
        local.errors.close()
        remote.errors.close()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['server', 'jj', 'rustfs', 'web']:
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    for name in ['server', 'jj', 'rustfs', 'web']:
        setattr(args, name, getattr(args, name).resolve(strict=True))
    with tempfile.TemporaryDirectory(prefix='jreload-', dir='/tmp') as temporary:
        qualify(args, Path(temporary))
    print('PASS isolated roots/processes cleaned up', flush=True)
