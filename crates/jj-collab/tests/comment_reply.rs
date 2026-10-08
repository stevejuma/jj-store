use jj_collab::*;

#[test]
fn answers_keep_exact_unicode_offsets_multiple_discussions_and_shared_comments() {
    let text = "Prose 🦀\n<comment-reply discussion=\"D1\" comments=\"C1 C2\">\nYes é.\n</comment-reply>\nMore prose\n<comment-reply discussion=\"D2\" comments=\"C3\">Second</comment-reply>";
    let replies = parse_comment_replies(text);
    assert_eq!(replies.len(), 2);
    assert_eq!(replies[0].comments, ["C1", "C2"]);
    assert_eq!(&text[replies[0].answer.clone()], "Yes é.");
    assert_eq!(&text[replies[1].answer.clone()], "Second");
    for end in text.char_indices().map(|(i, _)| i) {
        let prefix = parse_comment_replies(&text[..end]);
        assert!(prefix.iter().all(|r| replies.contains(r)));
    }
}
#[test]
fn literal_code_malformed_and_nested_tags_remain_unlinked() {
    for text in [
        "`<comment-reply discussion=\"D\" comments=\"C\">literal</comment-reply>`",
        "```xml\n<comment-reply discussion=\"D\" comments=\"C\">literal</comment-reply>\n```",
        "    <comment-reply discussion=\"D\" comments=\"C\">literal</comment-reply>",
        "<comment-reply discussion=\"D\" comments=\"C C\">duplicate</comment-reply>",
        "<comment-reply discussion=\"D\" comments=\"C\" extra=\"x\">extra</comment-reply>",
        "<comment-reply discussion=\"D\" comments=\"C\">unfinished",
        "<comment-reply discussion=\"D\" comments=\"C\"><comment-reply discussion=\"D\" comments=\"C\">nested</comment-reply></comment-reply>",
    ] {
        assert!(parse_comment_replies(text).is_empty(), "{text}");
    }
}
#[test]
fn protocol_tokens_are_revision_bound_and_cannot_be_supplied_arbitrarily() {
    let reference = |byte| {
        CommentReplyReferenceV1::new(ReactionTargetV1::HumanMessage {
            thread_id: ThreadId::new([1; 16]),
            message_id: HumanMessageId::new([2; 16]),
            revision: CollabObjectRef {
                kind: CollabObjectKind::HumanMessageRevision,
                id: CollabObjectId::new([byte; 32]),
            },
        })
        .unwrap()
    };
    let a = reference(3);
    let b = reference(4);
    assert_eq!(a.discussion, b.discussion);
    assert_ne!(a.comment, b.comment);
    let mut protocol = CommentReplyProtocolV1 {
        version: 1,
        references: vec![a],
    };
    protocol.validate().unwrap();
    protocol.references[0].comment = b.comment;
    assert!(protocol.validate().is_err());
}

#[test]
fn reply_body_fences_do_not_close_or_nest_protocol_blocks() {
    let text = "<comment-reply discussion=\"D\" comments=\"C\">\nAn example:\n\n```xml\n</comment-reply>\n<comment-reply discussion=\"example\" comments=\"example\">\n```\n\nActual answer.\n</comment-reply>";
    let parsed = parse_comment_replies(text);
    assert_eq!(parsed.len(), 1);
    assert!(text[parsed[0].answer.clone()].ends_with("Actual answer."));
}
