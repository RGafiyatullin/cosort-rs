#[derive(Debug)]
#[cfg_attr(feature = "std-error", derive(thiserror::Error))]
pub enum Error {
    #[cfg_attr(feature = "std-error", error("Elements' count mismatch"))]
    ElementCountMismatch,

    #[cfg_attr(feature = "std-error", error("Index too small"))]
    IndexTooSmal,
}
