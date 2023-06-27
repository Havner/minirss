#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

type psa_handle_t = i32;

pub(super) const RSS_PLATFORM_SERVICE_HANDLE: psa_handle_t = 0x40000105;
pub(super) const RSS_MEASURED_BOOT_HANDLE: psa_handle_t = 0x40000110;
pub(super) const RSS_DELEGATED_SERVICE_HANDLE: psa_handle_t = 0x40000111;

pub(super) const RSS_MEASURED_BOOT_READ: i16 = 1001;
pub(super) const RSS_MEASURED_BOOT_EXTEND: i16 = 1002;

pub(super) const RSS_DELEGATED_ATTEST_GET_DELEGATED_KEY: i16 = 1001;
pub(super) const RSS_DELEGATED_ATTEST_GET_PLATFORM_TOKEN: i16 = 1002;

pub(super) const PSA_MAX_IOVEC: usize = 4;
const PLAT_RSS_COMMS_PAYLOAD_MAX_SIZE: usize = 0x1000;

const TYPE_OFFSET: u8 = 16;
const TYPE_MASK: u32 = 0xFFFF << TYPE_OFFSET;
const IN_LEN_OFFSET: u8 = 8;
const IN_LEN_MASK: u32 = 0xFF << IN_LEN_OFFSET;
const OUT_LEN_OFFSET: u8 = 0;
const OUT_LEN_MASK: u32 = 0xFF << OUT_LEN_OFFSET;

pub(super) const PSA_SUCCESS: i32 = 0;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct serialized_rss_comms_header_t {
    protocol_ver: u8,
    seq_num: u8,
    client_id: u16,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct rss_embed_msg_t {
    handle: psa_handle_t,
    ctrl_param: u32, /* type, in_len, out_len */
    io_size: [u16; PSA_MAX_IOVEC],
    trailer: [u8; PLAT_RSS_COMMS_PAYLOAD_MAX_SIZE],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct rss_embed_reply_t {
    return_val: i32,
    out_size: [u16; PSA_MAX_IOVEC],
    trailer: [u8; PLAT_RSS_COMMS_PAYLOAD_MAX_SIZE],
}

#[derive(Debug)]
#[repr(C, packed)]
struct serialized_rss_comms_msg_t {
    header: serialized_rss_comms_header_t,
    msg: rss_embed_msg_t,
}

#[derive(Debug)]
#[repr(C, packed)]
struct serialized_rss_comms_reply_t {
    header: serialized_rss_comms_header_t,
    reply: rss_embed_reply_t,
}

#[derive(Debug)]
pub(super) struct Request {
    pub(super) protocol_ver: u8,
    pub(super) seq_num: u8,
    pub(super) client_id: u16,
    pub(super) handle: psa_handle_t,
    pub(super) psa_type: i16,
    pub(super) in_vecs: [Vec<u8>; PSA_MAX_IOVEC],
    pub(super) out_lens: [usize; PSA_MAX_IOVEC],
}

impl Request
{
    pub(super) fn de(input: &[u8]) -> Result<Self, super::Error>
    {
        println!("   deserializing");

        if input.len() > std::mem::size_of::<rss_embed_msg_t>() {
            return Err(super::Error::WrongDataLength);
        }

        let ptr = input.as_ptr() as *const serialized_rss_comms_msg_t;
        let msg: &serialized_rss_comms_msg_t = unsafe { &*ptr };

        let mut counter;

        let psa_type: i16 = ((msg.msg.ctrl_param & TYPE_MASK) >> TYPE_OFFSET).try_into().unwrap();
        let num_in_vecs: usize = ((msg.msg.ctrl_param & IN_LEN_MASK) >> IN_LEN_OFFSET).try_into().unwrap();
        let num_out_vecs: usize = ((msg.msg.ctrl_param & OUT_LEN_MASK) >> OUT_LEN_OFFSET).try_into().unwrap();

        println!("   in_vecs: {}, out_vecs: {}", num_in_vecs, num_out_vecs);

        /* required to copy on stack, msg is packed */
        let io_sizes = msg.msg.io_size;

        let mut in_vecs: [Vec<u8>; PSA_MAX_IOVEC] = Default::default();
        let mut offset = 0;
        counter = 0;
        for len in &io_sizes[..num_in_vecs]  {
            let len = *len as usize;
            in_vecs[counter] = msg.msg.trailer[offset..offset+len].to_vec();
            counter = counter + 1;
            offset = offset + len;
        }

        let mut out_len = [0usize; PSA_MAX_IOVEC];
        counter = 0;
        for len in &io_sizes[num_in_vecs..num_in_vecs + num_out_vecs] {
            out_len[counter] = *len as usize;
            counter = counter + 1;
        }

        Ok(Request {
            protocol_ver: msg.header.protocol_ver,
            seq_num: msg.header.seq_num,
            client_id: msg.header.client_id,
            handle: msg.msg.handle,
            psa_type,
            in_vecs,
            out_lens: out_len,
        })
    }
}

#[derive(Debug)]
pub(super) struct Response {
    pub(super) protocol_ver: u8,
    pub(super) seq_num: u8,
    pub(super) client_id: u16,
    pub(super) return_val: i32,
    pub(super) out_vecs: [Vec<u8>; PSA_MAX_IOVEC],
}

impl Response
{
    pub(super) fn ser(&self) -> Result<Vec<u8>, super::Error>
    {
        println!("   serializing");

        /* pack data */

        let header = serialized_rss_comms_header_t {
            protocol_ver: self.protocol_ver,
            seq_num: self.seq_num,
            client_id: self.client_id,
        };

        let mut out_size = [0u16; PSA_MAX_IOVEC];
        let mut trailer = [0u8; PLAT_RSS_COMMS_PAYLOAD_MAX_SIZE];
        let mut counter = 0;
        let mut offset = 0;
        for out_vec in &self.out_vecs {
            let len = out_vec.len();
            if len == 0 {
                break;
            }
            out_size[counter] = len.try_into().unwrap();
            trailer[offset..offset+len].clone_from_slice(&out_vec);
            offset = offset + len;
            counter = counter + 1;
        }

        println!("   out_vecs: {}", counter);

        let reply = rss_embed_reply_t {
            return_val: self.return_val,
            out_size,
            trailer,
        };

        let res = serialized_rss_comms_reply_t {
            header,
            reply,
        };

        /* convert to bytes */
        let length =
            std::mem::size_of::<serialized_rss_comms_reply_t>() -
            PLAT_RSS_COMMS_PAYLOAD_MAX_SIZE as usize + offset;

        let ptr = &res as *const serialized_rss_comms_reply_t as *const u8;
        let output = unsafe { std::slice::from_raw_parts(ptr, length) }.to_vec();

        Ok(output)
    }
}
