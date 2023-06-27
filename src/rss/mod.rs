mod serde;

#[derive(Debug)]
pub enum Error {
    WrongDataLength,
    InvalidHandle,
    InvalidType,
    InvalidVectors,
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

fn do_measured_boot(req: serde::Request) -> Result<serde::Response, Error>
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

fn do_delegated_attestation(req: serde::Request, key: &[u8], token: &[u8]) -> Result<serde::Response, Error>
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
        if v.len() == 0 {
            break;
        }
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

pub fn do_request(input: &[u8], key: &[u8], token: &[u8]) -> Result<Vec<u8>, Error>
{
    let req = serde::Request::de(&input)?;

    //println!("{:?}", req);

    let res = match req.handle {
        serde::RSS_PLATFORM_SERVICE_HANDLE => {
            println!("   got PLATFORM_SERVICE_HANDLE, unsupported");
            return Err(Error::InvalidHandle);
        },
        serde::RSS_MEASURED_BOOT_HANDLE => {
            println!("   got MEASURED_BOOT_HANDLE");
            do_measured_boot(req)?
        },
        serde::RSS_DELEGATED_SERVICE_HANDLE => {
            println!("   got DELEGATED_SERVICE_HANDLE");
            do_delegated_attestation(req, key, token)?
        },
        handle => {
            println!("   got {:X?}, invalid", handle);
            return Err(Error::InvalidHandle);
        },
    };

    //println!("{:?}", res);

    res.ser()
}
