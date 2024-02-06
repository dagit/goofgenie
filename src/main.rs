pub mod samus;
pub mod usb2snes;

use samus::{Beam, Item, Samus, SamusField, SAMUS_ADDR_MAP};
use usb2snes::*;

const WRAM: u32 = 0xF5_0000;
const CMD_SPACE: u32 = 0x2c00;

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
    // Example samus edit:
    samus.collected_items.set(Item::Varia);
    samus.collected_items.set(Item::MorphBall);
    samus.collected_items.set(Item::Bombs);
    samus.collected_items.set(Item::XRay);
    samus.equipped_items.set(Item::XRay);
    samus.collected_items.set(Item::SpringBall);
    samus.collected_items.set(Item::ScrewAttack);
    samus.collected_items.set(Item::Gravity);
    samus.collected_items.set(Item::HiJumpBoots);
    samus.collected_items.set(Item::SpeedBooster);
    samus.collected_items.set(Item::Grapple);
    samus.equipped_items.set(Item::Grapple);
    samus.collected_items.set(Item::SpaceJump);
    samus.collected_beams.set(Beam::Wave);
    samus.equipped_beams.set(Beam::Wave);
    samus.collected_beams.set(Beam::Ice);
    samus.equipped_beams.set(Beam::Ice);
    samus.collected_beams.set(Beam::Plasma);
    samus.equipped_beams.set(Beam::Plasma);
    samus.collected_beams.set(Beam::Charge);
    samus.equipped_beams.set(Beam::Charge);
    samus.hp = 3000;
    samus.max_hp = 3000;
    samus.max_reserve_hp = 100;
    samus.reserve_hp = 10;
    samus.missiles = 255;
    samus.max_missiles = 255;
    samus.supers = 255;
    samus.max_supers = 255;
    samus.pbs = 255;
    samus.max_pbs = 255;
    //samus.max_missiles = 255 * 13 + 255 * 4;
    //samus.supers = samus.max_supers;
    //samus.max_supers = 255*11 + 255*4;
    let mut data = Vec::new();
    data.extend_from_slice(&preamble);
    //data.extend_from_slice(&samus_overwrite_asm(&samus));
    data.extend_from_slice(&add_one_minute_to_timer());
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
    let bytes = data.to_le_bytes();
    [0xa9, bytes[0], bytes[1]]
}

pub fn lda_immediate_u8(data: u8) -> [u8; 2] {
    [0xa9, data]
}

pub fn lda_addr(address: u16) -> [u8; 3] {
    let bytes = address.to_le_bytes();
    [0xad, bytes[0], bytes[1]]
}

pub fn inc() -> [u8; 1] {
    [0x1A]
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
    let bytes = data.to_le_bytes();
    [0x69, bytes[0], bytes[1]]
}

pub fn adc_immediate_u8(data: u8) -> [u8; 2] {
    [0x69, data]
}

pub fn sta_u16(data: u16) -> [u8; 3] {
    let bytes = data.to_le_bytes();
    [0x8d, bytes[0], bytes[1]]
}

fn get_u16(
    client: &mut SyncClient,
    address: u32,
) -> std::result::Result<u16, Box<dyn std::error::Error>> {
    let response = client.get_address(address, 2)?;
    Ok(u16::from_le_bytes([response[0], response[1]]))
}

fn get_wram_addr(field: SamusField) -> u32 {
    SAMUS_ADDR_MAP[field] as u32 + WRAM
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
    let equipped_items = get_u16(client, get_wram_addr(SamusField::EquippedItems))?.into();
    let collected_items = get_u16(client, get_wram_addr(SamusField::CollectedItems))?.into();
    let equipped_beams = get_u16(client, get_wram_addr(SamusField::EquippedBeams))?.into();
    let collected_beams = get_u16(client, get_wram_addr(SamusField::CollectedBeams))?.into();
    let reserve_hp = get_u16(client, get_wram_addr(SamusField::ReserveHP))?;
    let max_reserve_hp = get_u16(client, get_wram_addr(SamusField::MaxReserveHP))?;

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
    })
}

pub fn samus_overwrite_asm(samus: &Samus) -> Vec<u8> {
    let mut r = Vec::new();
    // First write samus's collected items
    r.extend_from_slice(&lda_immediate_u16(samus.collected_items.into()));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::CollectedItems]));
    // Next we do equipped items
    r.extend_from_slice(&lda_immediate_u16(samus.equipped_items.into()));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::EquippedItems]));
    // Next we do collected beams
    r.extend_from_slice(&lda_immediate_u16(samus.collected_beams.into()));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::CollectedBeams]));
    // Next we do equipped beams
    r.extend_from_slice(&lda_immediate_u16(samus.equipped_beams.into()));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::EquippedBeams]));
    // Next we do HP
    r.extend_from_slice(&lda_immediate_u16(samus.hp));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::HP]));
    // Next we do MaxHP
    r.extend_from_slice(&lda_immediate_u16(samus.max_hp));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::MaxHP]));
    // Next we do Missiles
    r.extend_from_slice(&lda_immediate_u16(samus.missiles));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::Missiles]));
    // Next we do MaxMissiles
    r.extend_from_slice(&lda_immediate_u16(samus.max_missiles));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::MaxMissiles]));
    // Next we do Supers
    r.extend_from_slice(&lda_immediate_u16(samus.supers));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::Supers]));
    // Next we do MaxSupers
    r.extend_from_slice(&lda_immediate_u16(samus.max_supers));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::MaxSupers]));
    // Next we do Power Bombs
    r.extend_from_slice(&lda_immediate_u16(samus.pbs));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::PBs]));
    // Next we do MaxPBs
    r.extend_from_slice(&lda_immediate_u16(samus.max_pbs));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::MaxPBs]));
    // Next we do Reserves
    r.extend_from_slice(&lda_immediate_u16(samus.reserve_hp));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::ReserveHP]));
    // Next we do MaxPBs
    r.extend_from_slice(&lda_immediate_u16(samus.max_reserve_hp));
    r.extend_from_slice(&sta_u16(SAMUS_ADDR_MAP[SamusField::MaxReserveHP]));

    r
}

pub fn blue_suit_asm() -> Vec<u8> {
    let mut r = Vec::new();
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    r.extend_from_slice(&lda_immediate_u8(4));
    r.extend_from_slice(&sta_u16(0x0b3f));
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
    r.extend_from_slice(&sta_u16(0x0a68));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn g_mode_asm() -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(&lda_immediate_u16(0x0000));
    r.extend_from_slice(&sta_u16(0x1c23));

    r
}

pub fn max_kill_count() -> Vec<u8> {
    let mut r = Vec::new();
    // sep #$20
    r.push(0xe2);
    r.push(0x20);
    r.extend_from_slice(&lda_immediate_u8(0xFF));
    r.extend_from_slice(&sta_u16(0x0e50));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

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
    r.extend_from_slice(&sta_u16(TIMER_MINUTES));
    // rep #$20
    r.push(0xc2);
    r.push(0x20);

    r
}

pub fn savestate2snes() -> Vec<u8> {
    vec![
        0x08, 0xc2, 0x30, 0x48, 0xaf, 0x18, 0x42, 0x00, 0x8f, 0x06, 0x20, 0xfc, 0x5c, 0x00, 0x00,
        0xfc, 0xc2, 0x30, 0x68, 0x28, 0x6c, 0xea, 0xff, 0x6c, 0xea, 0xff,
    ]
}
