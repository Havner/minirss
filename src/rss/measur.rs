use super::{Error, serde};

const SW_TYPE_MAX_SIZE: usize = 20;

#[derive(Debug)]
#[repr(C)]
struct measured_boot_extend_iovec_t {
	index: u8,
	lock_measurement: u8,
	measurement_algo: u32,
	sw_type: [u8; SW_TYPE_MAX_SIZE],
	sw_type_size: u8,
}


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

    let ptr = req.in_vecs[0].as_ptr() as *const measured_boot_extend_iovec_t;
    let extend: &measured_boot_extend_iovec_t = unsafe { &*ptr };

    println!("{:X?}", extend);

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
