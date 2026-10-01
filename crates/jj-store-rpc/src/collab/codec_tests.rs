use super::*;
use jj_store_model as m;
fn source() -> Vec<u8> {
    "89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000d49444154789c63f8cfc0f01f00050001ff89993d1d0000000049454e44ae426082".as_bytes().as_chunks::<2>().0.iter().map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect()
}
fn fixture() -> (CollabObjectRef, wire::CollabAssetReply) {
    let (mut asset, _) = ImageAssetV1::ingest(&source()).unwrap();
    let reference = CollabObjectV1::ImageAsset(asset.clone())
        .reference()
        .unwrap();
    let image = std::mem::take(&mut asset.bytes);
    let snapshot = CollabReadSnapshotV1 {
        version: 1,
        repo_id: m::RepositoryId::new([1; 16]),
        epoch: m::EpochId::new([2; 16]),
        manifest_digest: m::RecordDigest::new([3; 32]),
        published_seq: m::WalPosition(0),
        code_state_version: 0,
        operation_heads: Default::default(),
        thread_root: None,
        thread_state_version: 0,
        reaction_root: None,
        reaction_state_version: 0,
        agent_root: None,
        review_root: None,
    };
    (
        reference,
        wire::CollabAssetReply {
            metadata_json: encode(&asset).unwrap(),
            image,
            snapshot_json: encode(&snapshot).unwrap(),
        },
    )
}
#[test]
fn binary_response_checks_exact_identity_metadata_and_single_byte_carrier() {
    let (reference, value) = fixture();
    assert_eq!(
        asset_response(reference, value).unwrap().asset.bytes,
        source()
    );
    for mode in 0..5 {
        let (mut reference, mut value) = fixture();
        match mode {
            0 => value.image[0] ^= 1,
            1 => {
                let mut asset: ImageAssetV1 = decode(&value.metadata_json).unwrap();
                asset.width = 2;
                value.metadata_json = encode(&asset).unwrap();
            }
            2 => reference.id = CollabObjectId::new([9; 32]),
            3 => {
                let mut asset: ImageAssetV1 = decode(&value.metadata_json).unwrap();
                asset.bytes = vec![1];
                value.metadata_json = encode(&asset).unwrap();
            }
            _ => value.snapshot_json = b"{}".to_vec(),
        }
        assert_eq!(
            asset_response(reference, value).unwrap_err().code(),
            tonic::Code::DataLoss
        );
    }
}
