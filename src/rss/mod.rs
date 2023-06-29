mod serde;
mod measur;
mod attest;

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
            measur::perform(req)?
        },
        serde::RSS_DELEGATED_SERVICE_HANDLE => {
            println!("   got DELEGATED_SERVICE_HANDLE");
            attest::perform(req, key, token)?
        },
        handle => {
            println!("   got {:X?}, invalid", handle);
            return Err(Error::InvalidHandle);
        },
    };

    //println!("{:?}", res);

    res.ser()
}
