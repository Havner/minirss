use super::{Error, serde};

pub(super) fn perform(req: serde::Request) -> Result<serde::Response, Error>
{
    match req.psa_type {
        serde::RSS_MEASURED_BOOT_READ => {
            println!("   got READ, unsupported");
            return Err(Error::InvalidType);
        },
        serde::RSS_MEASURED_BOOT_EXTEND => {
            println!("   got EXTEND");
        },
        _ => return Err(Error::InvalidType),
    };

    /* we should probably be checking whats inside in_vecs */
    let mut count = 0;
    for v in &req.in_vecs {
        println!("   got IN{}: {}", count, hex::encode(v));
        count = count + 1;
    }

    /* assume no output exists... */
    if req.out_lens[0] != 0 {
        return Err(Error::InvalidVectors);
    }

    let out_vecs: [Vec<u8>; serde::PSA_MAX_IOVEC] = Default::default();

    println!("   preparing and serializing response");

    Ok(serde::Response {
        protocol_ver: req.protocol_ver,
        seq_num: req.seq_num,
        client_id: req.client_id,
        return_val: serde::PSA_SUCCESS,
        out_vecs,
    })
}
