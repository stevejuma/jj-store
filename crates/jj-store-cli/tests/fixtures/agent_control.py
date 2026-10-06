#!/usr/bin/env python3
"""Deterministic v2 process fixture for the device supervisor, not runner qualification."""
import json
import os
import sys
from pathlib import Path

workspace = Path(sys.argv[sys.argv.index('--cwd') + 1])
sequence = 0
pending = None

def emit(kind, request_id=None, **fields):
    global sequence
    sequence += 1
    frame = dict(version=2, type=kind, sequence=sequence, **fields)
    if request_id is not None:
        frame['requestId'] = request_id
    print(json.dumps(frame), flush=True)

def complete(request, status='completed'):
    emit('record', request, record=dict(version=1, status=status), quiescent=True, sessionUsable=True)

emit('ready', runners=[], queueCapacity=16)
for line in sys.stdin:
    frame = json.loads(line)
    request = frame['requestId']
    kind = frame['type']
    if kind == 'open':
        emit('opened', request, sessionId='fixture-session', capabilities=dict(loadSession=True),
             session=dict(modes=dict(availableModes=[dict(id='ask', name='Ask')]), models=dict(availableModels=[])))
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
            workspace.joinpath('agent-edit.txt').write_text('review this\n')
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
    elif kind == 'close':
        emit('closed', 'close', confirmedStopped=True)
        sys.exit(0)
