use super::{Error, serde};

pub(super) fn perform(req: serde::Request, key: &[u8], token: &[u8]) -> Result<serde::Response, Error>
{
    let content = match req.psa_type {
        serde::RSS_DELEGATED_ATTEST_GET_DELEGATED_KEY => {
            println!("   got DELEGATED_KEY");
            key
        },
        serde::RSS_DELEGATED_ATTEST_GET_PLATFORM_TOKEN => {
            println!("   got PLATFORM_TOKEN");
            token
        },
        _ => return Err(Error::InvalidType),
    };

    /* we should probably be checking whats inside in_vecs */
    let mut count = 0;
    for v in &req.in_vecs {
        println!("   got IN{}: {}", count, hex::encode(v));
        count = count + 1;
    }

    /* assume only one output exists... */
    if req.out_lens[0] == 0 || req.out_lens[1] != 0 {
        return Err(Error::InvalidVectors);
    }

    /* ...and that we can fit the data in the output */
    if content.len() > req.out_lens[0] {
        return Err(Error::InvalidVectors);
    }

    let mut out_vecs: [Vec<u8>; serde::PSA_MAX_IOVEC] = Default::default();
    out_vecs[0] = content.to_vec();

    println!("   preparing response");

    Ok(serde::Response {
        protocol_ver: req.protocol_ver,
        seq_num: req.seq_num,
        client_id: req.client_id,
        return_val: serde::PSA_SUCCESS,
        out_vecs,
    })
}
