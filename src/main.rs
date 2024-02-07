pub mod usb2snes;

use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashSet};
use usb2snes::*;

#[derive(Debug, Clone)]
pub struct Samus {
    hp: u16,
    max_hp: u16,
    missiles: u16,
    max_missiles: u16,
    supers: u16,
    max_supers: u16,
    pbs: u16,
    max_pbs: u16,
    equipped_items: HashSet<Item>,
    collected_items: HashSet<Item>,
    equipped_beams: HashSet<Beam>,
    collected_beams: HashSet<Beam>,
    reserve_hp: u16,
    max_reserve_hp: u16,
    x_position: u16,
    x_subposition: u16,
    y_position: u16,
    y_subposition: u16,
    bosses: BTreeMap<Area, HashSet<Boss>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SamusField {
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
    XPosition,
    XSubPosition,
    YPosition,
    YSubPosition,
    Bosses,
}

lazy_static! {
    static ref SAMUS_ADDR_MAP: BTreeMap<SamusField, u16> = {
        let mut m = BTreeMap::new();
        m.insert(SamusField::HP, 0x09C2);
        m.insert(SamusField::MaxHP, 0x09C4);
        m.insert(SamusField::Missiles, 0x09C6);
        m.insert(SamusField::MaxMissiles, 0x09C8);
        m.insert(SamusField::Supers, 0x09CA);
        m.insert(SamusField::MaxSupers, 0x09CC);
        m.insert(SamusField::PBs, 0x09CE);
        m.insert(SamusField::MaxPBs, 0x09D0);
        m.insert(SamusField::EquippedItems, 0x09A2);
        m.insert(SamusField::CollectedItems, 0x09A4);
        m.insert(SamusField::EquippedBeams, 0x09A6);
        m.insert(SamusField::CollectedBeams, 0x09A8);
        m.insert(SamusField::ReserveHP, 0x09D6);
        m.insert(SamusField::MaxReserveHP, 0x09D4);
        m.insert(SamusField::XPosition, 0x0AF6);
        m.insert(SamusField::XSubPosition, 0x0AF8);
        m.insert(SamusField::YPosition, 0x0AFA);
        m.insert(SamusField::YSubPosition, 0x0AFC);
        m.insert(SamusField::Bosses, 0xD828);
        m
    };
}

const WRAM: u32 = 0xF5_0000;

const VARIA: u16 = 1;
const SPRINGBALL: u16 = 2;
const MORPHBALL: u16 = 4;
const SCREWATTACK: u16 = 8;
const GRAVITY: u16 = 0x20;
const HIJUMPBOOTS: u16 = 0x100;
const SPACEJUMP: u16 = 0x200;
const BOMBS: u16 = 0x1000;
const SPEEDBOOSTER: u16 = 0x2000;
const GRAPPLE: u16 = 0x4000;
const XRAY: u16 = 0x8000;
const WAVE: u16 = 1;
const ICE: u16 = 2;
const SPAZER: u16 = 4;
const PLASMA: u16 = 8;
const CHARGE: u16 = 0x1000;

const CRATERIA: u8 = 0;
const BRINSTAR: u8 = 1;
const NORFAIR: u8 = 2;
const WRECKEDSHIP: u8 = 3;
const MARIDIA: u8 = 4;
const TOURIAN: u8 = 5;
const CERES: u8 = 6;
const DEBUG: u8 = 7;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Item {
    Varia = VARIA,
    SpringBall = SPRINGBALL,
    MorphBall = MORPHBALL,
    ScrewAttack = SCREWATTACK,
    Gravity = GRAVITY,
    HiJumpBoots = HIJUMPBOOTS,
    SpaceJump = SPACEJUMP,
    Bombs = BOMBS,
    SpeedBooster = SPEEDBOOSTER,
    Grapple = GRAPPLE,
    XRay = XRAY,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Beam {
    Wave = WAVE,
    Ice = ICE,
    Spazer = SPAZER,
    Plasma = PLASMA,
    Charge = CHARGE,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Area {
    Crateria = CRATERIA,
    Brinstar = BRINSTAR,
    Norfair = NORFAIR,
    WreckedShip = WRECKEDSHIP,
    Maridia = MARIDIA,
    Tourian = TOURIAN,
    Ceres = CERES,
    Debug = DEBUG,
}

const MAINBOSS: u8 = 1;
const MINIBOSS: u8 = 2;
const TORIZO: u8 = 4;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Boss {
    MainBoss = MAINBOSS,
    MiniBoss = MINIBOSS,
    Torizo = TORIZO,
}

fn items_to_u16(items: &[&Item]) -> u16 {
    let mut r = 0u16;
    for i in items {
        r |= **i as u16;
    }
    r
}

fn u16_to_items(items: u16) -> HashSet<Item> {
    let mut r = HashSet::new();
    if items & VARIA == VARIA {
        r.insert(Item::Varia);
    }
    if items & SPRINGBALL == SPRINGBALL {
        r.insert(Item::SpringBall);
    }
    if items & MORPHBALL == MORPHBALL {
        r.insert(Item::MorphBall);
    }
    if items & SCREWATTACK == SCREWATTACK {
        r.insert(Item::ScrewAttack);
    }
    if items & GRAVITY == GRAVITY {
        r.insert(Item::Gravity);
    }
    if items & HIJUMPBOOTS == HIJUMPBOOTS {
        r.insert(Item::HiJumpBoots);
    }
    if items & SPACEJUMP == SPACEJUMP {
        r.insert(Item::SpaceJump);
    }
    if items & BOMBS == BOMBS {
        r.insert(Item::Bombs);
    }
    if items & SPEEDBOOSTER == SPEEDBOOSTER {
        r.insert(Item::SpeedBooster);
    }
    if items & GRAPPLE == GRAPPLE {
        r.insert(Item::Grapple);
    }
    if items & XRAY == XRAY {
        r.insert(Item::XRay);
    }

    r
}

fn beams_to_u16(beams: &[&Beam]) -> u16 {
    let mut r = 0u16;
    for b in beams {
        r |= **b as u16;
    }
    r
}

fn u16_to_beams(beams: u16) -> HashSet<Beam> {
    let mut r = HashSet::new();
    if beams & WAVE == WAVE {
        r.insert(Beam::Wave);
    }
    if beams & ICE == ICE {
        r.insert(Beam::Ice);
    }
    if beams & SPAZER == SPAZER {
        r.insert(Beam::Spazer);
    }
    if beams & PLASMA == PLASMA {
        r.insert(Beam::Plasma);
    }
    if beams & CHARGE == CHARGE {
        r.insert(Beam::Charge);
    }

    r
}

fn u8_to_bosses(bosses: u8) -> HashSet<Boss> {
    let mut r = HashSet::new();
    if bosses & MAINBOSS == MAINBOSS {
        r.insert(Boss::MainBoss);
    }
    if bosses & MINIBOSS == MINIBOSS {
        r.insert(Boss::MiniBoss);
    }
    if bosses & TORIZO == TORIZO {
        r.insert(Boss::Torizo);
    }

    r
}

fn bosses_to_u8(bosses: &[&Boss]) -> u8 {
    let mut r = 0u8;
    for b in bosses {
        r |= **b as u8;
    }
    r
}

fn u8_to_area(area: u8) -> Area {
    // TODO: okay this got messy. An array would be so much cleaner
    if area == CRATERIA {
        return Area::Crateria;
    }
    if area == BRINSTAR {
        return Area::Brinstar;
    }
    if area == NORFAIR {
        return Area::Norfair;
    }
    if area == WRECKEDSHIP {
        return Area::WreckedShip;
    }
    if area == MARIDIA {
        return Area::Maridia;
    }
    if area == TOURIAN {
        return Area::Tourian;
    }
    if area == CERES {
        return Area::Ceres;
    }
    if area == DEBUG {
        return Area::Debug;
    }
    panic!("unrecognized area");
}

fn area_to_u8(area: &Area) -> u8 {
    match area {
        Area::Crateria => CRATERIA,
        Area::Brinstar => BRINSTAR,
        Area::Norfair => NORFAIR,
        Area::WreckedShip => WRECKEDSHIP,
        Area::Maridia => MARIDIA,
        Area::Tourian => TOURIAN,
        Area::Ceres => CERES,
        Area::Debug => DEBUG,
    }
}

fn to_area_bosses(areas: &[(u8, u8)]) -> BTreeMap<Area, HashSet<Boss>> {
    let mut map = BTreeMap::new();
    for (area, bosses) in areas {
        map.insert(u8_to_area(*area), u8_to_bosses(*bosses));
    }
    map
}

fn area_bosses_to_bytes(areas: &BTreeMap<Area, HashSet<Boss>>) -> Vec<(u16, u8)> {
    let mut ret = vec![];
    for (area, bosses) in areas {
        ret.push((
            *SAMUS_ADDR_MAP.get(&SamusField::Bosses).unwrap() + area_to_u8(area) as u16,
            bosses_to_u8(&bosses.iter().collect::<Vec<_>>()),
        ));
    }
    ret
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut client = crate::usb2snes::SyncClient::connect()?;
    client.set_name("goofgenie")?;
    let device_list = client.list_device()?.to_vec();
    println!("{:#?}", device_list);
    if device_list.len() == 1 {
        client.attach(&device_list[0])?;
    }

    println!("{:#?}", client.info());

    let cmd = client.get_cmd()?;
    //println!("{:02x?}", cmd);
    // preamble corresponds to:
    // php
    // rep #$30
    // pha
    // phx
    // phy
    // phb
    //
    // And postamble corresponds to:
    // plb
    // stz $2c00 ; disable this command
    // ply
    // plx
    // pla
    // plp
    // jmp ($ffea) ; run the normal nmi code
    //
    let preamble: [u8; 7] = [0x08, 0xc2, 0x30, 0x48, 0xda, 0x5a, 0x8b];
    let postamble: [u8; 11] = [
        0xab, 0x9c, 0x00, 0x2c, 0x7a, 0xfa, 0x68, 0x28, 0x6c, 0xea, 0xff,
    ];
    let mut samus = get_samus(&mut client)?;
    println!("{:#?}", samus);
    // Example samus edit:
    //samus.collected_items.insert(Item::Varia);
    samus.collected_items.insert(Item::MorphBall);
    samus.collected_items.insert(Item::Bombs);
    //samus.collected_items.insert(Item::XRay);
    //samus.equipped_items.insert(Item::XRay);
    //samus.collected_items.insert(Item::SpringBall);
    //samus.collected_items.insert(Item::ScrewAttack);
    //samus.collected_items.insert(Item::Gravity);
    //samus.collected_items.insert(Item::HiJumpBoots);
    //samus.collected_items.insert(Item::SpeedBooster);
    //samus.collected_items.insert(Item::Grapple);
    //samus.equipped_items.insert(Item::Grapple);
    //samus.collected_items.insert(Item::SpaceJump);
    //samus.collected_beams.insert(Beam::Wave);
    //samus.equipped_beams.insert(Beam::Wave);
    //samus.collected_beams.insert(Beam::Ice);
    //samus.equipped_beams.insert(Beam::Ice);
    //samus.collected_beams.insert(Beam::Plasma);
    //samus.equipped_beams.insert(Beam::Plasma);
    //samus.collected_beams.insert(Beam::Charge);
    //samus.equipped_beams.insert(Beam::Charge);
    //samus.hp = 3000;
    //samus.max_hp = 3000;
    //samus.max_reserve_hp = 100;
    //samus.reserve_hp = 10;
    //samus.missiles = 255;
    //samus.max_missiles = 255;
    samus.supers = 255;
    samus.max_supers = 255;
    //samus.pbs = 255;
    //samus.max_pbs = 255;
    //samus.max_missiles = 255 * 13 + 255 * 4;
    //samus.supers = samus.max_supers;
    //samus.max_supers = 255*11 + 255*4;
    //samus.x_position -= 8;
    //samus.y_position -= 8;
    let mut all_bosses = HashSet::new();
    all_bosses.insert(Boss::MainBoss);
    all_bosses.insert(Boss::MiniBoss);
    all_bosses.insert(Boss::Torizo);
    samus.bosses.insert(Area::Norfair, all_bosses.clone());
    samus.bosses.insert(Area::Brinstar, all_bosses.clone());
    samus.bosses.insert(Area::Maridia, all_bosses.clone());
    samus.bosses.insert(Area::WreckedShip, all_bosses.clone());
    let mut data = Vec::new();
    data.extend_from_slice(&preamble);
    //data.extend_from_slice(&move_left_half_tile());
    //data.extend_from_slice(&enable_hyperbeam());
    //data.extend_from_slice(&disable_hyperbeam());
    data.extend_from_slice(&samus_overwrite_asm(&samus));
    //data.extend_from_slice(&add_one_minute_to_timer());
    //data.extend_from_slice(&max_kill_count());
    //data.extend_from_slice(&delete_plms());
    //data.extend_from_slice(&spike_suit_asm());
    //data.extend_from_slice(&blue_suit_asm());
    //data.extend_from_slice(&g_mode_asm());
    data.extend_from_slice(&postamble);
    client.put_cmd(&data)?;
    loop {
        let header = client.get_cmd_header_byte()?;
        if header == 0 {
            break;
        }
    }
    client.put_cmd(&cmd)?;
    let new_cmd = client.get_cmd()?;
    assert_eq!(cmd, new_cmd);

    //let samus = get_samus(&mut client)?;
    //println!("{:#?}", samus);
    //println!("{:02x?}", data);

    Ok(())
}

pub fn lda_immediate_u16(data: u16) -> [u8; 3] {
    let bytes = u16_to_le(data);
    [0xa9, bytes[0], bytes[1]]
}

pub fn lda_immediate_u8(data: u8) -> [u8; 2] {
    [0xa9, data]
}

pub fn lda_addr(address: u16) -> [u8; 3] {
    let bytes = u16_to_le(address);
    [0xad, bytes[0], bytes[1]]
}

pub fn inc() -> [u8; 1] {
    [0x1A]
}

pub fn inc_absolute(address: u16) -> [u8; 3] {
    let bytes = address.to_le_bytes();
    [0xee, bytes[0], bytes[1]]
}

pub fn sed() -> [u8; 1] {
    [0xf8]
}

pub fn cld() -> [u8; 1] {
    [0xd8]
}

pub fn adc_direct(data: u8) -> [u8; 2] {
    [0x65, data]
}

pub fn adc_immediate_u16(data: u16) -> [u8; 3] {
    let bytes = u16_to_le(data);
    [0x69, bytes[0], bytes[1]]
}

pub fn adc_immediate_u8(data: u8) -> [u8; 2] {
    [0x69, data]
}

pub fn adc_absolute(address: u16) -> [u8; 3] {
    let bytes = address.to_le_bytes();
    [0x6d, bytes[0], bytes[1]]
}

pub fn sta_absolute(address: u16) -> [u8; 3] {
    let bytes = u16_to_le(address);
    [0x8d, bytes[0], bytes[1]]
}

pub fn sta_long(address: u32) -> [u8; 4] {
    let bytes = address.to_le_bytes();
    [0x8f, bytes[0], bytes[1], bytes[2]]
}

pub fn stz_absolute(address: u16) -> [u8; 3] {
    let bytes = address.to_le_bytes();
    [0x9c, bytes[0], bytes[1]]
}

fn get_u16(
    client: &mut SyncClient,
    address: u32,
) -> std::result::Result<u16, Box<dyn std::error::Error>> {
    let response = client.get_address(address, 2)?;
    Ok(((response[1] as u16) << 8) + response[0] as u16)
}

fn get_wram_addr(field: SamusField) -> u32 {
    *SAMUS_ADDR_MAP.get(&field).unwrap() as u32 + WRAM
}

fn get_samus(client: &mut SyncClient) -> std::result::Result<Samus, Box<dyn std::error::Error>> {
    let hp = get_u16(client, get_wram_addr(SamusField::HP))?;
    let max_hp = get_u16(client, get_wram_addr(SamusField::MaxHP))?;
    let missiles = get_u16(client, get_wram_addr(SamusField::Missiles))?;
    let max_missiles = get_u16(client, get_wram_addr(SamusField::MaxMissiles))?;
    let supers = get_u16(client, get_wram_addr(SamusField::Supers))?;
    let max_supers = get_u16(client, get_wram_addr(SamusField::MaxSupers))?;
    let pbs = get_u16(client, get_wram_addr(SamusField::PBs))?;
    let max_pbs = get_u16(client, get_wram_addr(SamusField::MaxPBs))?;
    let equipped_items = u16_to_items(get_u16(client, get_wram_addr(SamusField::EquippedItems))?);
    let collected_items = u16_to_items(get_u16(client, get_wram_addr(SamusField::CollectedItems))?);
    let equipped_beams = u16_to_beams(get_u16(client, get_wram_addr(SamusField::EquippedBeams))?);
    let collected_beams = u16_to_beams(get_u16(client, get_wram_addr(SamusField::CollectedBeams))?);
    let reserve_hp = get_u16(client, get_wram_addr(SamusField::ReserveHP))?;
    let max_reserve_hp = get_u16(client, get_wram_addr(SamusField::MaxReserveHP))?;
    let x_position = get_u16(client, get_wram_addr(SamusField::XPosition))?;
    let y_position = get_u16(client, get_wram_addr(SamusField::YPosition))?;
    let x_subposition = get_u16(client, get_wram_addr(SamusField::XSubPosition))?;
    let y_subposition = get_u16(client, get_wram_addr(SamusField::YSubPosition))?;
    let mut bosses = vec![];
    // TODO: rewrite this
    for idx in 0..=DEBUG {
        bosses.push((
            idx,
            client.get_address(get_wram_addr(SamusField::Bosses) + idx as u32, 1)?[0],
        ));
    }
    let bosses = to_area_bosses(&bosses);

    Ok(Samus {
        hp,
        max_hp,
        missiles,
        max_missiles,
        supers,
        max_supers,
        pbs,
        max_pbs,
        equipped_items,
        collected_items,
        reserve_hp,
        max_reserve_hp,
        collected_beams,
        equipped_beams,
        x_position,
        y_position,
        x_subposition,
        y_subposition,
        bosses,
    })
}

pub fn u16_to_le(data: u16) -> [u8; 2] {
    let lb: u8 = (data & 0x00FF) as u8;
    let hb: u8 = (data >> 8) as u8;
    assert_eq!(data, ((hb as u16) << 8) + lb as u16);
    [lb, hb]
}

pub fn samus_overwrite_asm(samus: &Samus) -> Vec<u8> {
    let mut r = Vec::new();
    // First write samus's collected items
    r.extend_from_slice(&lda_immediate_u16(items_to_u16(
        &samus.collected_items.iter().collect::<Vec<_>>(),
    )));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::CollectedItems).unwrap(),
    ));
    // Next we do equipped items
    r.extend_from_slice(&lda_immediate_u16(items_to_u16(
        &samus.equipped_items.iter().collect::<Vec<_>>(),
    )));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::EquippedItems).unwrap(),
    ));
    // Next we do collected beams
    r.extend_from_slice(&lda_immediate_u16(beams_to_u16(
        &samus.collected_beams.iter().collect::<Vec<_>>(),
    )));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::CollectedBeams).unwrap(),
    ));
    // Next we do equipped beams
    r.extend_from_slice(&lda_immediate_u16(beams_to_u16(
        &samus.equipped_beams.iter().collect::<Vec<_>>(),
    )));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::EquippedBeams).unwrap(),
    ));
    // Next we do HP
    r.extend_from_slice(&lda_immediate_u16(samus.hp));
    r.extend_from_slice(&sta_absolute(*SAMUS_ADDR_MAP.get(&SamusField::HP).unwrap()));
    // Next we do MaxHP
    r.extend_from_slice(&lda_immediate_u16(samus.max_hp));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::MaxHP).unwrap(),
    ));
    // Next we do Missiles
    r.extend_from_slice(&lda_immediate_u16(samus.missiles));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::Missiles).unwrap(),
    ));
    // Next we do MaxMissiles
    r.extend_from_slice(&lda_immediate_u16(samus.max_missiles));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::MaxMissiles).unwrap(),
    ));
    // Next we do Supers
    r.extend_from_slice(&lda_immediate_u16(samus.supers));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::Supers).unwrap(),
    ));
    // Next we do MaxSupers
    r.extend_from_slice(&lda_immediate_u16(samus.max_supers));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::MaxSupers).unwrap(),
    ));
    // Next we do Power Bombs
    r.extend_from_slice(&lda_immediate_u16(samus.pbs));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::PBs).unwrap(),
    ));
    // Next we do MaxPBs
    r.extend_from_slice(&lda_immediate_u16(samus.max_pbs));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::MaxPBs).unwrap(),
    ));
    // Next we do Reserves
    r.extend_from_slice(&lda_immediate_u16(samus.reserve_hp));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::ReserveHP).unwrap(),
    ));
    // Next we do MaxReserveHP
    r.extend_from_slice(&lda_immediate_u16(samus.max_reserve_hp));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::MaxReserveHP).unwrap(),
    ));
    r.extend_from_slice(&lda_immediate_u16(samus.x_position));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::XPosition).unwrap(),
    ));
    r.extend_from_slice(&lda_immediate_u16(samus.y_position));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::YPosition).unwrap(),
    ));
    r.extend_from_slice(&lda_immediate_u16(samus.x_subposition));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::XSubPosition).unwrap(),
    ));
    r.extend_from_slice(&lda_immediate_u16(samus.y_subposition));
    r.extend_from_slice(&sta_absolute(
        *SAMUS_ADDR_MAP.get(&SamusField::YSubPosition).unwrap(),
    ));

    // Boss event flags
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    let boss_bytes = area_bosses_to_bytes(&samus.bosses);
    for (addr, bosses) in boss_bytes {
        r.extend_from_slice(&lda_immediate_u8(bosses));
        r.extend_from_slice(&sta_long(0x7E_0000 + addr as u32));
    }
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn blue_suit_asm() -> Vec<u8> {
    let mut r = Vec::new();
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    r.extend_from_slice(&lda_immediate_u8(4));
    r.extend_from_slice(&sta_absolute(0x0b3f));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn spike_suit_asm() -> Vec<u8> {
    let mut r = Vec::new();
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    r.extend_from_slice(&lda_immediate_u8(1));
    r.extend_from_slice(&sta_absolute(0x0a68));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn g_mode_asm() -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(&lda_immediate_u16(0x0000));
    r.extend_from_slice(&sta_absolute(0x1c23));

    r
}

pub fn max_kill_count() -> Vec<u8> {
    let mut r = Vec::new();
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    r.extend_from_slice(&lda_immediate_u8(0xFF));
    r.extend_from_slice(&sta_absolute(0x0e50));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn enable_hyperbeam() -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(&inc_absolute(0x0A76));
    r
}

pub fn disable_hyperbeam() -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(&stz_absolute(0x0A76));
    r
}

pub fn add_one_minute_to_timer() -> Vec<u8> {
    const TIMER_MINUTES: u16 = 0x0947;
    let mut r = Vec::new();
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    r.extend_from_slice(&lda_addr(TIMER_MINUTES));
    r.extend_from_slice(&sed());
    r.extend_from_slice(&adc_immediate_u8(1));
    r.extend_from_slice(&cld());
    r.extend_from_slice(&sta_absolute(TIMER_MINUTES));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn move_left_half_tile() -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(&lda_immediate_u8(1));
    r.extend_from_slice(&adc_absolute(0x0AF6));
    r.extend_from_slice(&sta_absolute(0x0AF6));
    r
}

pub fn savestate2snes() -> Vec<u8> {
    vec![
        0x08, 0xc2, 0x30, 0x48, 0xaf, 0x18, 0x42, 0x00, 0x8f, 0x06, 0x20, 0xfc, 0x5c, 0x00, 0x00,
        0xfc, 0xc2, 0x30, 0x68, 0x28, 0x6c, 0xea, 0xff, 0x6c, 0xea, 0xff,
    ]
}
