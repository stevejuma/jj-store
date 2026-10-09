#!/usr/bin/env python3
"""Disposable native/local durable-comment and cold built-browser acceptance.
Uses a deterministic ACP and compatible provider; this is controlled inference evidence.
"""
import argparse
import http.server
import threading
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
for name in ['server', 'jj', 'rustfs', 'web', 'helper']:
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
    raise RuntimeError('conversation acceptance deadline; runtime='+str(value.get('runtime')))

with tempfile.TemporaryDirectory(prefix='jj-conversation-', dir='/private/tmp') as temporary:
    root = Path(temporary)
    calls=[]
    class Provider(http.server.BaseHTTPRequestHandler):
        def log_message(self,*args): pass
        def do_POST(self):
            body=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
            assert body['model']=='metadata-fixture' and not body.get('tools')
            calls.append(body)
            result=dict(title='Review conversation metadata',summary='The user requested metadata verification. The fixture assistant responded.',keywords=['metadata-discovery-keyword'],goals=['Verify conversation discovery'])
            payload=json.dumps(dict(choices=[dict(message=dict(role='assistant',content=json.dumps(result)))] )).encode()
            self.send_response(200);self.send_header('Content-Type','application/json');self.send_header('Content-Length',str(len(payload)));self.end_headers();self.wfile.write(payload)
    provider=http.server.ThreadingHTTPServer(('127.0.0.1',0),Provider)
    threading.Thread(target=provider.serve_forever,daemon=True).start()
    provider_origin='http://127.0.0.1:'+str(provider.server_port)
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
    environment = dict(os.environ, JJ_STORE_ACP_EXECUTABLE=str(acp), JJ_STORE_AGENT_JJ_EXECUTABLE=str(args.jj.resolve()), JJ_STORE_LOCAL_AGENT_EXECUTABLE=str(args.helper.resolve()),METADATA_FIXTURE_KEY='fake-metadata-fixture')
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
            wait(origin, lambda v: v['runtime'] == 'ready', timeout=240)
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
            settings = request(origin, '/local/agents/settings')
            settings['metadata'] = dict(model='metadata-fixture', options=dict(inherit_defaults=False, provider='openai-compat', endpoint=provider_origin+'/v1', environment={'OPENAI_COMPAT_API_KEY':dict(source='environment',value='METADATA_FIXTURE_KEY')}))
            request(origin, '/local/agents/settings', dict(version=1,request_id=uuid.uuid4().hex,expected_revision=settings['revision'],settings=settings))
            sessions = []
            def metadata(session):
                return request(origin, '/local/agents/metadata?session='+session)
            def command(session, action):
                value=metadata(session)
                body=dict(version=1,request_id=uuid.uuid4().hex,query=dict(session=session,repository=None),expected=value['revision'],action=action)
                result=request(origin,'/local/agents/metadata',body)
                deadline=time.monotonic()+30
                while result['state'] in ['prepared','generating','publishing']:
                    if time.monotonic()>deadline: raise RuntimeError('metadata job deadline: '+str(result))
                    time.sleep(.1)
                    result=request(origin,'/local/agents/metadata/job?request='+body['request_id'])
                return body,result
            for profile, repository in [('native', created['repository']), ('local_jj_git', registered['binding']['repository'])]:
                if args.profile not in ['both',profile]: continue
                print('Qualifying metadata '+profile,flush=True)
                current=request(origin,'/local/agents')
                setup=dict(id='metadata-'+profile,name='Metadata agent identity',avatar='🤖',description='Controlled metadata acceptance',instructions='Fixture instructions',runner='codex',preferences=dict(model=None,effort=None,mode='ask'))
                request(origin,'/local/agents/setups',dict(version=1,request_id=uuid.uuid4().hex,expected_revision=current['revision'],setup=setup,remove=None))
                target=next(t for t in request(origin,'/local/agents/targets?repository='+repository) if t['profile']==profile)
                launched=request(origin,'/local/agents/launch',dict(version=1,request_id=uuid.uuid4().hex,target_id=target['id'],repository=repository,operation=target['operation'],commit=target['commit'],runner='codex',setup_id=setup['id'],context=None))
                session=launched['session_id']
                wait(origin,lambda v:any(s['id']==session and s['state']=='idle' for s in v['sessions']))
                request(origin,'/local/agents/command',dict(version=1,request_id=uuid.uuid4().hex,session_id=session,action=dict(action='prompt',text='Review discovery metadata 🦀',max_duration_ms=5000)))
                wait(origin,lambda v:any(s['id']==session and s['state']=='idle' and s['published_sequence']==s['last_sequence'] and s['last_sequence']>3 for s in v['sessions']))
                if profile=='native':
                    assert not metadata(session)['writable'], 'native origin capability refusal lost'
                    project=request(origin,'/repos/'+repository+'/work/commands',dict(version=1,request_id=uuid.uuid4().hex,command=dict(command='create_project',id=uuid.uuid4().hex,title='Metadata qualification home',body='Disposable supported native WorkStore fixture')))
                    assert project['status']=='committed'
                count=len(calls)
                assert metadata(session)['value']['generated'] is None
                metadata(session); request(origin,'/local/agents')
                assert len(calls)==count, 'a read initiated inference'
                original,result=command(session,dict(action='refresh'))
                assert result['state']=='completed',result
                value=metadata(session)
                assert value['fresh'] and value['value']['generated']['keywords']==['metadata-discovery-keyword']
                assert len(calls)==count+1
                projected=json.loads(calls[-1]['messages'][1]['content'])
                assert 'Review discovery metadata 🦀' in projected
                assert 'JJ_STORE_AGENT_JJ_EXECUTABLE' not in projected
                request(origin,'/local/agents/metadata',original)
                assert len(calls)==count+1,'duplicate generated another result'
                _,result=command(session,dict(action='rename',title='Custom '+profile+' 🦀'))
                assert result['state']=='completed',result
                _,result=command(session,dict(action='refresh'));assert result['state']=='completed',result
                value=metadata(session);assert value['title']=='Custom '+profile+' 🦀'
                directory=request(origin,'/local/agents');row=next(s for s in directory['sessions'] if s['id']==session)
                assert 'metadata-discovery-keyword' in row['discovery_text'] and row['title']=='Metadata agent identity'
                subprocess.run(['node',str(checkout/'scripts/conversation-metadata-browser.mjs'),origin,session,'Custom '+profile+' 🦀'],check=True,timeout=90)
                sessions.append((session,value['revision'],value['title']))
                request(origin,'/local/agents/command',dict(version=1,request_id=uuid.uuid4().hex,session_id=session,action=dict(action='end')))
                wait(origin,lambda v:any(s['id']==session and not s['connected'] for s in v['sessions']))
            count=len(calls)
            server.send_signal(signal.SIGTERM);server.wait(timeout=40);assert server.returncode==0
            server=start();wait(origin,lambda v:v['runtime']=='ready',timeout=240)
            for session,revision,title in sessions:
                value=metadata(session);assert value['revision']==revision and value['title']==title
                subprocess.run(['node',str(checkout/'scripts/conversation-metadata-browser.mjs'),origin,session,title],check=True,timeout=90)
            assert len(calls)==count,'restart or browser opening initiated inference'
            assert all(not call.get('tools') and not call.get('tool_choice') for call in calls)
            print('PASS: native/local metadata publication, custom titles, receipts, discovery, built UI, restart and tool-free requests. Provider is controlled fixture evidence.',flush=True)
        finally:
            if server.poll() is None:
                server.send_signal(signal.SIGTERM)
                server.wait(timeout=40)
