#!/usr/bin/env python3
"""Deterministic V3 process fixture for the device supervisor, not runner qualification."""
import json
import os
import sys
from pathlib import Path

workspace = Path(sys.argv[sys.argv.index('--cwd') + 1])
version = int(sys.argv[sys.argv.index("--control-version")+1])
assert version == 3
sequence = 0
pending = None
scope = None
workspaces = {}

def emit(kind, request_id=None, **fields):
    global sequence
    sequence += 1
    frame = dict(version=version, type=kind, sequence=sequence, **fields)
    if request_id is not None:
        frame['requestId'] = request_id
    if version == 3 and scope is not None and kind not in ('ready', 'warmed', 'closed'):
        frame['sessionId'] = scope
    print(json.dumps(frame), flush=True)

def complete(request, status='completed'):
    emit('record', request, record=dict(version=1, status=status), quiescent=True, sessionUsable=True)

emit('ready', runners=[], queueCapacity=16)
for line in sys.stdin:
    frame = json.loads(line)
    assert frame['version'] == 3
    request = frame['requestId']
    kind = frame['type']
    if workspace.name == 'catalog':
        with workspace.joinpath('frames.jsonl').open('a') as log:
            log.write(json.dumps(frame) + '\n')
    scope = frame.get('sessionId')
    if scope in workspaces:
        workspace = workspaces[scope]
    if kind == 'warm':
        emit('warmed', request, capabilities=dict(loadSession=True))
    elif kind == 'open':
        if version == 3:
            workspace = Path(frame['cwd'])
            workspaces[scope] = workspace
        emit('opened', request, adapterSessionId='fixture-' + (scope or 'session'), sessionId='fixture-session', capabilities=dict(loadSession=True),
             session=dict(modes=dict(availableModes=[dict(id='ask', name='Ask')]), models=dict(availableModels=[dict(modelId='fixture-model', name='Fixture model')])))
    elif kind == 'configure':
        emit('configured', request, modeReady=True, configuration=dict(mode=dict(status='applied', value=frame.get('mode'))))
    elif kind == 'prompt':
        emit('queued', request)
        emit('started', request)
        emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=f'Fixture in {workspace}')))
        if frame['text'].endswith('permission'):
            pending = request
            emit('permission', request, permissionId='permission-one', request=dict(toolCall=dict(title='Edit fixture'), options=[dict(optionId='allow', name='Allow once', kind='allow_once'), dict(optionId='deny', name='Reject', kind='reject_once')]))
        else:
            code = list(workspace.glob('repositories/*')) if version == 3 else []
            for directory in code or [workspace]:
                directory.joinpath('agent-edit.txt').write_text('review this\n')
            complete(request)
    elif kind == 'permission' and pending:
        emit('ack', request)
        emit('permission_resolved', pending, permissionId=frame['permissionId'], optionId=frame['optionId'])
        complete(pending)
        pending = None
    elif kind == 'cancel':
        emit('ack', request)
        if pending:
            complete(pending, 'cancelled')
            pending = None
    elif kind == 'close_session':
        workspaces.pop(scope, None)
        emit('session_closed', request)
    elif kind == 'close':
        emit('closed', 'close', confirmedStopped=True)
        sys.exit(0)
