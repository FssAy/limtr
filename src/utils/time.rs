pub(crate) fn now() -> u64 {
    chrono::Utc::now().timestamp() as u64
}

#[inline]
pub(crate) fn in_future(seconds: u32) -> u64 {
    now() + seconds as u64
}

#[inline]
pub(crate) fn from_point(start: u64, seconds: u32) -> u64 {
    start + seconds as u64
}
