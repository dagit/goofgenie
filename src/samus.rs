pub(super) static SAMUS_ADDR_MAP: SamusRamMap = SamusRamMap([
    0x09C2, 0x09C4, 0x09C6, 0x09C8, 0x09CA, 0x09CC, 0x09CE, 0x09D0, 0x09A2, 0x09A4, 0x09A6, 0x09A8,
    0x09D6, 0x09D4,
]);

#[derive(Debug, Clone)]
pub struct Samus {
    pub(super) hp: u16,
    pub(super) max_hp: u16,
    pub(super) missiles: u16,
    pub(super) max_missiles: u16,
    pub(super) supers: u16,
    pub(super) max_supers: u16,
    pub(super) pbs: u16,
    pub(super) max_pbs: u16,
    pub(super) equipped_items: Items,
    pub(super) collected_items: Items,
    pub(super) equipped_beams: Beams,
    pub(super) collected_beams: Beams,
    pub(super) reserve_hp: u16,
    pub(super) max_reserve_hp: u16,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum SamusField {
    HP,
    MaxHP,
    Missiles,
    MaxMissiles,
    Supers,
    MaxSupers,
    PBs,
    MaxPBs,
    EquippedItems,
    CollectedItems,
    EquippedBeams,
    CollectedBeams,
    ReserveHP,
    MaxReserveHP,
}

pub(super) struct SamusRamMap([u16; 14]);

impl std::ops::Index<SamusField> for SamusRamMap {
    type Output = u16;

    fn index(&self, field: SamusField) -> &Self::Output {
        &self.0[field as usize]
    }
}

#[allow(dead_code)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Item {
    Varia = 0x0001,
    SpringBall = 0x0002,
    MorphBall = 0x0004,
    ScrewAttack = 0x0008,
    Gravity = 0x0020,
    HiJumpBoots = 0x0100,
    SpaceJump = 0x0200,
    Bombs = 0x1000,
    SpeedBooster = 0x2000,
    Grapple = 0x4000,
    XRay = 0x8000,
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub(super) struct Items(u16);

#[allow(dead_code)]
impl Items {
    pub fn set(&mut self, item: Item) {
        self.0 |= item as u16;
    }
}

impl From<u16> for Items {
    fn from(bits: u16) -> Self {
        Items(bits)
    }
}

impl From<Items> for u16 {
    fn from(items: Items) -> Self {
        items.0
    }
}

#[allow(dead_code)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Beam {
    Wave = 0x0001,
    Ice = 0x0002,
    Spazer = 0x0004,
    Plasma = 0x0008,
    Charge = 0x1000,
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub(super) struct Beams(u16);

#[allow(dead_code)]
impl Beams {
    pub fn set(&mut self, beam: Beam) {
        self.0 |= beam as u16;
    }
}

impl From<u16> for Beams {
    fn from(bits: u16) -> Self {
        Beams(bits)
    }
}

impl From<Beams> for u16 {
    fn from(beams: Beams) -> Self {
        beams.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_address_map() {
        assert_eq!(SAMUS_ADDR_MAP[SamusField::HP], 0x09C2);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::MaxHP], 0x09C4);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::Missiles], 0x09C6);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::MaxMissiles], 0x09C8);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::Supers], 0x09CA);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::MaxSupers], 0x09CC);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::PBs], 0x09CE);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::MaxPBs], 0x09D0);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::EquippedItems], 0x09A2);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::CollectedItems], 0x09A4);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::EquippedBeams], 0x09A6);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::CollectedBeams], 0x09A8);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::ReserveHP], 0x09D6);
        assert_eq!(SAMUS_ADDR_MAP[SamusField::MaxReserveHP], 0x09D4);
    }
}
