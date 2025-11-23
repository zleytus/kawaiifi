use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum InterfaceType {
    Unspecified = 0,
    Adhoc,
    Station,
    Ap,
    ApVlan,
    Wds,
    Monitor,
    MeshPoint,
    P2pClient,
    P2pGo,
    P2pDevice,
    Ocb,
    Nan,
}
