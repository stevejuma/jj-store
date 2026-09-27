use jj_store_model::{
    envelope::{self, EnvelopeError, HEADER_LEN},
    *,
};

#[test]
fn typed_ids_reject_width_and_noncanonical_hex() {
    let id = CommitId::new([42; 64]);
    assert_eq!(id.to_string().parse::<CommitId>().unwrap(), id);
    assert!(id.to_string().to_uppercase().parse::<CommitId>().is_err());
    assert!("aa".parse::<CommitId>().is_err());
    assert!(CommitId::from_slice(&[0; 63]).is_err());
    assert!(RepositoryId::from_slice(&[0; 64]).is_err());
    let upload = ConflictUploadId::new([171; 32]);
    assert_eq!(
        upload.to_string().parse::<ConflictUploadId>().unwrap(),
        upload
    );
    assert!(
        upload
            .to_string()
            .to_uppercase()
            .parse::<ConflictUploadId>()
            .is_err()
    );
    assert!(ConflictUploadId::from_slice(&[0; 64]).is_err());
    let repo = RepositoryId::new([9; 16]);
    let file = ObjectKey::File(FileId::new([42; 64]));
    let commit = ObjectKey::Commit(id);
    assert_ne!(file.storage_path(repo), commit.storage_path(repo));
    assert_ne!(
        file.storage_path(repo),
        file.storage_path(RepositoryId::new([8; 16]))
    );
}

#[test]
fn envelope_rejects_corruption_truncation_wrong_key_and_limits() {
    let key = ObjectKey::File(FileId::new([4; 64]));
    let bytes = envelope::encode(key, b"native\0data", 100).unwrap();
    assert_eq!(envelope::decode(&bytes, key, 100).unwrap(), b"native\0data");
    for i in 0..bytes.len() {
        let mut corrupt = bytes.clone();
        corrupt[i] ^= 1;
        assert!(envelope::decode(&corrupt, key, 100).is_err(), "byte {i}");
        assert!(
            envelope::decode(&bytes[..i], key, 100).is_err(),
            "length {i}"
        );
    }
    let mut extra = bytes.clone();
    extra.push(0);
    assert_eq!(
        envelope::decode(&extra, key, 100),
        Err(EnvelopeError::Length)
    );
    assert_eq!(envelope::decode(&bytes, key, 1), Err(EnvelopeError::Limit));
    assert_eq!(
        envelope::decode(&bytes, ObjectKey::Tree(TreeId::new([4; 64])), 100),
        Err(EnvelopeError::Key)
    );
    assert_eq!(
        envelope::encode(key, b"too much", 1),
        Err(EnvelopeError::Limit)
    );
    assert_eq!(bytes.len(), HEADER_LEN + 11);
}
