/// [kube::core::Version]
pub enum Version {
    /// [kube::core::Version::Alpha]
    Alpha(u32, Option<u32>),
    /// [kube::core::Version::Beta]
    Beta(u32, Option<u32>),
    /// [kube::core::Version::Stable]
    Stable(u32),
    /// [kube::core::Version::Nonconformant]
    Nonconformant(String),
}
impl Version {
    /// [kube::core::Version::parse]
    pub fn parse(v: &str) -> Version {
        todo!()
    }
}
