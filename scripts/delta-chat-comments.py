"""Add allowlisted comments to a projection using explicitly reviewed source ranges.

Run after delta-chat-fixture.py: raw.json projected.json reviewed-sources.json.
No CRDT resolution, replay, credentials, actor identities or image URLs are kept.
"""
import json
import pathlib
import re
import sys

raw_path, destination, mapping_path = map(pathlib.Path, sys.argv[1:])
raw = json.loads(raw_path.read_text())
result = json.loads(destination.read_text())
reviewed = json.loads(mapping_path.read_text())
comments = {c['value']['id']: c['value'] for c in raw['comment_set']['comments']}
comment_buffer = raw['comment_set']['buffer'].encode()
main_buffer = raw['buffer'].encode()

def text(c):
    span = c['text']
    return comment_buffer[span['start']['$resolved_offset']:span['end']['$resolved_offset']].decode().strip()

def offset(anchor):
    return len(main_buffer[:anchor['$resolved_offset']].decode().replace('/Users/stevejuma', '/workspace/demo').replace('stevejuma', 'demo-user').encode())

replies = {r['value']: offset(r['key']) for r in raw['agent_comment_replies']}
deliveries = {c['key']: offset(c['value']) for c in raw['comment_deliveries']}
pending = {c['key'] for c in raw['pending_comment_destinations']}
users = {r['start']: r for r in result['records'] if r['kind'] == 'User'}
for segment in raw['$snapshot']['segments']:
    record = users.get(segment['buffer_byte_offset'])
    if record: record['text'] = segment['user_prompt'] or ''
for i, header in enumerate(raw['comment_summary_headers']):
    span = header['value']['range']
    result['records'].append({'id': f'comment-summary-{i}', 'kind': 'CommentSummary', 'start': offset(span['start']), 'end': offset(span['end'])})
result['discussions'] = []
reply_index = 0
for index, selected in enumerate(reviewed['selections']):
    root = comments[selected['commentId']]
    assert text(root) == selected['text'], 'Reviewed comment changed; re-review the source map'
    thread_id = f'discussion-{index}'
    root_id = f'{thread_id}/user'
    batch = users.get(deliveries.get(root['id'], -1))
    entry = {'id': root_id, 'author': 'user', 'text': text(root), 'status': 'sent' if batch else 'draft'}
    if batch:
        entry['batchId'] = batch['id']
        batch['commentBatchId'] = batch['id']
    else:
        assert root['id'] in pending, 'Comment is neither delivered nor pending'
    discussion = {'id': thread_id, 'target': selected['target'], 'comments': [entry]}
    for c in comments.values():
        if c['id'] not in replies or c['legacy_root'] != root['id']: continue
        start = replies[c['id']]
        capture = result['buffer'].encode()[start:]
        match = re.match(rb'<comment-reply>[\s\S]*?</comment-reply>', capture)
        assert match, 'Reply boundary unavailable'
        reply_id = f'reply-{reply_index}'
        reply_index += 1
        entry.update(status='replied', replyItemId=reply_id)
        discussion['comments'].append({'id': f'{thread_id}/agent', 'author': 'agent', 'text': text(c), 'status': 'sent', 'replyTo': root_id})
        result['records'].append({'id': reply_id, 'kind': 'Reply', 'start': start, 'end': start + len(match[0]), 'text': text(c), 'replyContext': {'threadId': thread_id, 'commentIds': [root_id]}})
    result['discussions'].append(discussion)
# Check each explicitly supplied range against the exact, unrendered source.
for discussion in result['discussions']:
    target = discussion['target']
    record = next(r for r in result['records'] if r['id'] == target['recordId'])
    source = (record['output'] if record['kind'] == 'ToolUse' else record['text']).encode()
    assert source[target['from']:target['to']].decode() == target['quote'], 'Reviewed source range changed'
draft = next(m for m in raw['messages'] if m['value']['$variant'] == 'ComposerDraft')
draft_start = offset(draft['key'])
for header in raw['comment_summary_headers']:
    if offset(header['key']) == draft_start:
        draft_start = offset(header['value']['range']['end'])
result['draftText'] = result['buffer'].encode()[draft_start:offset(draft['value']['$value']['enclosure']['content_end'])].decode().strip()
result['version'] = 2
result['source']['commentMapping'] = reviewed['method']
result['records'].sort(key=lambda r: r['start'])
destination.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
print(f"Projected {len(result['discussions'])} discussions and {reply_index} linked replies")
