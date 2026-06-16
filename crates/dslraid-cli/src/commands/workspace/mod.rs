mod compat;
mod init;
mod migrate;
mod normalize;

pub(crate) use compat::run as compat_check;
pub(crate) use init::run as init_project;
pub(crate) use migrate::run as migrate;
pub(crate) use normalize::run as normalize;
