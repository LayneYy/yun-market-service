use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(usize)]
pub enum ApiStatusCode {
    Success = 0,
    ServiceException = 1000,
}