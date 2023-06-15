mod serde;

#[derive(Debug)]
pub enum Error {
    WrongDataLength,
    InvalidHandle,
    InvalidType,
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub fn do_key(input: &[u8], key: &[u8]) -> Result<Vec<u8>, Error>
{
    let req = serde::Request::de(&input)?;

    println!("{:?}", req);

    if req.handle != serde::RSS_DELEGATED_SERVICE_HANDLE {
        return Err(Error::InvalidHandle);
    }

    if req.psa_type != serde::RSS_DELEGATED_ATTEST_GET_DELEGATED_KEY {
        return Err(Error::InvalidType);
    }

    /* we should probably be checking whats inside in_vecs */
    req.in_vecs[0][0];

    /* assume only one output should exist */
    assert!(req.out_lens[0] != 0);
    assert!(req.out_lens[1] == 0);

    let mut out_vecs: [Vec<u8>; serde::PSA_MAX_IOVEC] = Default::default();
    assert!(key.len() < req.out_lens[0]);
    out_vecs[0] = key.to_vec();

    let res = serde::Response {
        protocol_ver: req.protocol_ver,
        seq_num: req.seq_num,
        client_id: req.client_id,
        return_val: serde::PSA_SUCCESS,
        out_vecs,
    };

    println!("{:?}", res);

    res.ser()
}

pub fn do_token(input: &[u8], token: &[u8]) -> Result<Vec<u8>, Error>
{
    let req = serde::Request::de(&input)?;

    println!("{:?}", req);

    if req.handle != serde::RSS_DELEGATED_SERVICE_HANDLE {
        return Err(Error::InvalidHandle);
    }

    if req.psa_type != serde::RSS_DELEGATED_ATTEST_GET_PLATFORM_TOKEN {
        return Err(Error::InvalidType);
    }

    /* we should probably be checking whats inside in_vecs */
    req.in_vecs[0][0];

    /* assume only one output exists */
    assert!(req.out_lens[0] != 0);
    assert!(req.out_lens[1] == 0);

    let mut out_vecs: [Vec<u8>; serde::PSA_MAX_IOVEC] = Default::default();
    assert!(token.len() < req.out_lens[0]);
    out_vecs[0] = token.to_vec();

    let res = serde::Response {
        protocol_ver: req.protocol_ver,
        seq_num: req.seq_num,
        client_id: req.client_id,
        return_val: serde::PSA_SUCCESS,
        out_vecs,
    };

    println!("{:?}", res);

    res.ser()
}

pub fn do_request(input: &[u8], key: &[u8], token: &[u8]) -> Result<Vec<u8>, Error>
{
    println!("   deserializing");

    let req = serde::Request::de(&input)?;

    //println!("{:?}", req);

    if req.handle != serde::RSS_DELEGATED_SERVICE_HANDLE {
        return Err(Error::InvalidHandle);
    }

    /* we should probably be checking whats inside in_vecs */
    req.in_vecs[0][0];

    /* assume only one output should exist */
    assert!(req.out_lens[0] != 0);
    assert!(req.out_lens[1] == 0);

    let mut out_vecs: [Vec<u8>; serde::PSA_MAX_IOVEC] = Default::default();

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

    assert!(content.len() < req.out_lens[0]);
    out_vecs[0] = content.to_vec();

    println!("   preparing and serializing response");

    let res = serde::Response {
        protocol_ver: req.protocol_ver,
        seq_num: req.seq_num,
        client_id: req.client_id,
        return_val: serde::PSA_SUCCESS,
        out_vecs,
    };

    //println!("{:?}", res);

    res.ser()
}
