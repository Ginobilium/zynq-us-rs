//! SPD data decoding
//!
//! JEDEC Standard No. 21-C Release 23A Annex L: Serial Presence Detect (SPD) for DDR4 SDRAM Modules
use core::convert::TryInto;
use libm::ceilf;
use log::debug;

use crate::i2c::{I2C, MUX_ADDR};

// select bits for SODIMM
#[cfg(feature = "target_zcu111")]
const I2C_MUX_SEL: u8 = 0x08;
// SODIMM control addresses
#[cfg(feature = "target_zcu111")]
const I2C_CTRL_ADDR_LO: u16 = 0x36;
#[cfg(feature = "target_zcu111")]
const I2C_CTRL_ADDR_HI: u16 = 0x37;
// SODIMM address
#[cfg(feature = "target_zcu111")]
const I2C_ADDR: u16 = 0x51;

pub fn read_spd_eeprom() -> GeneralConfig {
    let mut spd_data = [0u8; 512];
    let mut i2c = I2C::i2c1();
    // set clock
    i2c.set_sclk(100_000);
    // set mux to DDR
    i2c.master_write_polled(MUX_ADDR, 1, &[I2C_MUX_SEL])
        .unwrap();
    while i2c.busy() {}
    // read back selection to confirm
    i2c.master_read_polled(MUX_ADDR, 1, &mut spd_data).unwrap();
    assert!(spd_data[0] == I2C_MUX_SEL, "Error in I2C mux selection");

    // enable access to lower page
    i2c.master_write_polled(I2C_CTRL_ADDR_LO, 1, &[0x00])
        .unwrap();
    while i2c.busy() {}

    // set start addr
    i2c.master_write_polled(I2C_ADDR, 1, &[0x00]).unwrap();
    while i2c.busy() {}

    // read lower page
    i2c.master_read_polled(I2C_ADDR, 256, &mut spd_data[..256])
        .unwrap();
    while i2c.busy() {}

    // enable access to upper page
    i2c.master_write_polled(I2C_CTRL_ADDR_HI, 1, &[0x01])
        .unwrap();
    while i2c.busy() {}

    // set start addr
    i2c.master_write_polled(I2C_ADDR, 1, &[0x00]).unwrap();
    while i2c.busy() {}

    // read upper page
    i2c.master_read_polled(I2C_ADDR, 256, &mut spd_data[256..])
        .unwrap();
    while i2c.busy() {}

    GeneralConfig::from_spd_data(&spd_data)
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Ddr3,
    Ddr4,
    LpDdr3,
    LpDdr4,
}
#[derive(Debug, Clone, Copy)]
pub enum PackageType {
    Monolithic,
    NonMonolithic,
}

#[derive(Debug, Clone, Copy)]
pub enum SignalLoading {
    Unspecified,
    MultiLoadStack,
    SingleLoadStack,
}

#[derive(Debug, Clone, Copy)]
pub enum FineGranularityRefMode {
    X1,
    X2,
    X4,
}

/// General Configuration Section: Bytes 0-127
///
/// The struct data is intended to be a minimal representation of the data encoded in the SPD
/// EEPROM. Any derived values are exposed as methods.
#[derive(Debug)]
pub struct GeneralConfig {
    // note: "reserved" is also used in comments if all fields in byte are reserved or should be zero.
    // presumably this struct will need to be extended for non-DDR4 SDRAM
    // byte 0
    pub spd_bytes_total: u16, // bits 6-4
    pub spd_bytes_used: u16,  // 3-0
    // 1
    pub spd_encoding: u8,  // 7-4
    pub spd_additions: u8, // 3-0
    // 2
    pub device_type: DeviceType,
    // 3: module type (3-0)
    pub module_config: ModuleConfig,
    // 4
    pub bg_addr_bits: u8,       // 7-6
    pub bank_addr_bits: u8,     // 5-4
    pub capacity_megabits: u16, // 3-0, in megaBITs
    // 5
    pub row_addr_bits: u8, // 5-3
    pub col_addr_bits: u8, // 2-0
    // 6
    pub package_type: PackageType,     // 7
    pub die_count: u8,                 // 6-4
    pub signal_loading: SignalLoading, // 1-0
    // 7
    pub t_maw: u16, // 5-4, units of t_refi
    pub mac: u8,    // 3-0 TODO: decode if needed
    // 8: thermal and refresh options (reserved for DDR4)
    // 9: other optional features (only DDR4 field is post package repair)
    // 10 reserved
    // 11 (other voltages listed as "TBD")
    pub vdd_12_endurant: bool, // 1
    pub vdd_12_operable: bool, // 0
    // 12
    pub package_ranks: u8, // 5-3
    pub device_width: u8,  // 2-0
    // 13
    pub bus_width_extension: u8, // 4-3
    pub bus_width: u8,           // 2-0
    // 14
    pub has_thermal_sensor: bool, // 7
    // 15: extended module type (reserved for DDR4)
    // 16 reserved
    // 17
    pub mtb_ps: u32,
    pub ftb_ps: u32,
    // 18 (MTB = 125 ps) & 125 (offset, FTB = 1 ps)
    pub t_ckavg_min_ps: u32,
    // 19 & 124
    pub t_ckavg_max_ps: u32, // same ^
    // 20 & 21: all bits, 22: 1-0, 23: all reserved
    // bit values from LSB to MSB correspond to CL = 7-24
    pub supported_cas_latencies: u32,
    // 24 & 123
    pub t_aa_min_ps: u32,
    // 25 & 122
    pub t_rcd_min_ps: u32,
    // 26 & 121
    pub t_rp_min_ps: u32,
    // 27: upper nibbles, 28: t_ras_min LSB, 29: t_rc_min LSB
    // 120: t_rc offset
    pub t_ras_min_ps: u32,
    pub t_rc_min_ps: u32,
    // 30, 31: t_rfc1_min LSB, MSB
    pub t_rfc1_min_ps: u32,
    // 32, 33: t_rfc2_min LSB, MSB
    pub t_rfc2_min_ps: u32,
    // 34, 35: t_rfc4_min LSB, MSB
    pub t_rfc4_min_ps: u32,
    // 36 (3-0): upper nibble, 37: LSB
    pub t_faw_min_ps: u32,
    // 38 & 119
    pub t_rrd_s_min_ps: u32,
    // 39 & 118
    pub t_rrd_l_min_ps: u32,
    // 40 & 117
    pub t_ccd_l_min_ps: u32,
    // 41-59 reserved
    // 60-77
    pub dq_map: [u8; 18],
    // 78-116 reserved
    // 117-125 contain offsets for above timing parameters
    // 126-127: CRC for base section

    // hardcoded parameters
    pub dm_en: bool,
    pub rd_dbi_en: bool,
    pub wr_dbi_en: bool,
    pub ecc_en: bool,
    pub en_2nd_clk: bool,
    pub parity_en: bool,
    pub crc_en: bool,
    pub power_down_en: bool,
    pub clock_stop_en: bool,
    pub self_ref_en: bool,
    pub lp_auto_self_ref: bool,
    pub temp_ref_mode: bool,
    pub temp_ref_range: bool,
    pub fine_granularity_ref_mode: FineGranularityRefMode,
    pub self_ref_abort: bool,
    pub v_ref: bool,
    pub geardown: bool,
}

/// Bytes 128-255
/// Annex L.1: Module Specific Bytes for Unbuffered Memory Module Types
/// UDIMM and SO-DIMM
#[derive(Debug, Clone, Copy)]
pub struct UnbufferedConfig {
    // 128
    pub raw_card_extension: u8,    // 7-5
    pub module_nominal_height: u8, // 4-0
    // 129
    pub module_max_thickness_back: u8,  // 7-4
    pub module_max_thickness_front: u8, // 3-0
    // 130
    pub ref_raw_card_rev: u8,
    pub ref_raw_card: u8,
    // 131
    pub rank_1_mirrored: bool,
    // 132-253 reserved
    // 254-255: CRC
}

// TODO
#[derive(Debug)]
pub struct RegisteredConfig;
#[derive(Debug)]
pub struct LoadReducedConfig;

#[derive(Debug)]
pub enum ModuleConfig {
    Unbuffered(UnbufferedConfig),
    #[allow(unused)]
    Registered(RegisteredConfig),
    #[allow(unused)]
    LoadReduced(LoadReducedConfig),
}

// TODO: supplier data (bytes 320-383) if anyone ever cares about it
//  (would require decoding from JEDEC JEP-106)

/// Calculate CRC code for `spd_data` according to the algorithm in the JEDEC spec (Page 4.1.2.12 – 37).
fn crc(spd_data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &b in spd_data.iter() {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Calculate timing from `n_mtb * mtb_ps + offset * ftb_ps`.
fn get_time_ps(mtb_ps: u32, ftb_ps: u32, n_mtb: u16, offset_ftb: i8) -> u32 {
    // For mtb = 125 ps (~2^7) and n_mtb up to 16 bits (unsigned), the product can't be safely
    // cast to i32. Luckily we have 64-bit registers so not a big deal
    ((mtb_ps * (n_mtb as u32)) as i64 + (ftb_ps as i64) * (offset_ftb as i64)) as u32
}

fn bad_data(byte_num: usize, value: u8) -> ! {
    panic!("Invalid SPD data at byte {}. Data: {:#X}", byte_num, value);
}

// hardcoded burst length for DDR4(?)
const DDR4_BURST_LEN: u8 = 8;
// max t_refi for DDR4 (normal temperature range)
const DDR4_TREFI_MAX_PS: u32 = 7_800_000;
// for converting between some size in MiB and number of address bits
const LOG2_1_MIB: u32 = 23;
// addressing maxima
const MAX_ROW_ADDR_BITS: u8 = 18;
const MAX_COL_ADDR_BITS: u8 = 12;
const MAX_BANK_ADDR_BITS: u8 = 3;
const MAX_BG_ADDR_BITS: u8 = 2;
// HIF offsets
const HIF_COL_OFFSET: u32 = 100;
const HIF_ROW_OFFSET: u32 = 200;
const HIF_BANK_OFFSET: u32 = 300;
const HIF_BG_OFFSET: u32 = 400;
const HIF_RANK_OFFSET: u32 = 500;

impl GeneralConfig {
    pub fn from_spd_data(spd_data: &[u8; 512]) -> Self {
        // verify CRC
        let expected_crc = ((spd_data[127] as u16) << 8) | (spd_data[126] as u16);
        let actual_crc = crc(&spd_data[..126]);
        assert_eq!(
            expected_crc, actual_crc,
            "CRC mismatch. Expected: {:#X}, actual: {:#X}",
            expected_crc, actual_crc
        );
        debug!("CRC check passed");

        let spd_bytes_total: u16 = match (spd_data[0] >> 4) & 0x7 {
            0b001 => 256,
            0b010 => 512,
            _ => bad_data(0, spd_data[0]),
        };

        let spd_bytes_used: u16 = match spd_data[0] & 0xf {
            0b001 => 128,
            0b010 => 256,
            0b011 => 384,
            0b100 => 512,
            _ => bad_data(0, spd_data[0]),
        };

        // 1
        let spd_encoding = spd_data[1] >> 4;
        let spd_additions = spd_data[1] & 0xf;

        // 2
        let device_type = match spd_data[2] {
            0x0b => DeviceType::Ddr3,
            0x0c => DeviceType::Ddr4,
            0x0f => DeviceType::LpDdr3,
            0x10 => DeviceType::LpDdr4,
            _ => bad_data(2, spd_data[2]),
        };

        // same as ^, but for SO-DIMM
        let module_config = match spd_data[3] & 0xf {
            // 0b0000 => extended module type (byte 15),
            // 0b0001 | 0b0101 | 0b1000 => rdimms,
            // 0b0100 => lrdimm,
            0b0010 | 0b0011 | 0b0110 | 0b1001 | 0b1100 | 0b1101 => ModuleConfig::Unbuffered(
                UnbufferedConfig::from_spd_data(&spd_data[128..256].try_into().unwrap()),
            ),
            _ => bad_data(3, spd_data[3]),
        };

        // 4
        // check for reserved values
        if spd_data[4] >> 6 == 0x3 || spd_data[4] >> 4 & 0x2 == 0x2 || spd_data[4] & 0x8 == 0x8 {
            bad_data(4, spd_data[4]);
        }
        let bg_addr_bits = spd_data[4] >> 6;
        let bank_addr_bits = (spd_data[4] >> 4 & 0x3) + 2;
        let capacity_megabits: u16 = 256 << (spd_data[4] & 0xf); // 256 * 2^(bits)

        // 5
        // check for reserved values
        if spd_data[5] >> 3 == 0x7 || spd_data[5] & 0x4 == 0x4 {
            bad_data(5, spd_data[5]);
        }
        let row_addr_bits = (spd_data[5] >> 3 & 0x7) + 12;
        let col_addr_bits = (spd_data[5] & 0x7) + 9;

        // 6
        if spd_data[6] & 0x3 == 0x3 {
            bad_data(6, spd_data[6]);
        }
        let package_type = match spd_data[6] >> 7 {
            0b0 => PackageType::Monolithic,
            0b1 => PackageType::NonMonolithic,
            // impossible, but must make rustc happy
            _ => bad_data(6, spd_data[6]),
        };
        let die_count = (spd_data[6] >> 4 & 0x7) + 1;
        let signal_loading = match spd_data[6] & 0x3 {
            0b00 => SignalLoading::Unspecified,
            0b01 => SignalLoading::MultiLoadStack,
            0b10 => SignalLoading::SingleLoadStack,
            // 0b11 reserved
            _ => bad_data(6, spd_data[6]),
        };

        // 7
        if spd_data[7] >> 4 == 0x3 || spd_data[7] & 0x7 == 0x7 || (spd_data[7] & 0xf) > 8 {
            bad_data(7, spd_data[7]);
        }
        let t_maw: u16 = 8192 >> (spd_data[7] >> 4 & 0x3);
        let mac = spd_data[7] & 0xf;

        // 11
        let vdd_12_endurant = spd_data[11] & 0x2 == 0x2;
        let vdd_12_operable = spd_data[11] & 0x1 == 0x1;

        // 12
        if spd_data[12] >> 3 & 0x4 == 0x4 || spd_data[12] & 0x4 == 0x4 {
            bad_data(12, spd_data[12]);
        }
        let package_ranks = (spd_data[12] >> 3 & 0x7) + 1;
        let device_width: u8 = 4 << (spd_data[12] & 0x7); // 4 * 2^(bits)

        // 13
        if spd_data[13] >> 3 & 0x2 == 0x2 || spd_data[13] & 0x4 == 0x4 {
            bad_data(13, spd_data[13]);
        }
        let bus_width_extension = (spd_data[13] >> 3 & 0x3) << 3; // 0 -> 0, 1 -> 8
        let bus_width: u8 = 8 << (spd_data[13] & 0x7); // 8 * 2^(bits)

        // 14
        let has_thermal_sensor = spd_data[14] & 0x80 == 0x80;

        // 17
        let mtb_ps: u32 = match spd_data[17] >> 2 & 0x3 {
            0b00 => 125,
            _ => bad_data(17, spd_data[17]),
        };
        let ftb_ps: u32 = match spd_data[17] & 0x3 {
            0b00 => 1,
            _ => bad_data(17, spd_data[17]),
        };

        // 18 & 125
        let t_ckavg_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[18] as u16, spd_data[125] as i8);

        // 19 & 124
        let t_ckavg_max_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[19] as u16, spd_data[124] as i8);

        // 20-23 (but the last 14 bits are reserved in the spec)
        let supported_cas_latencies = spd_data[20] as u32
            | ((spd_data[21] as u32) << 8)
            | ((spd_data[22] as u32) << 16)
            | ((spd_data[23] as u32) << 24);

        // 24 & 123
        let t_aa_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[24] as u16, spd_data[123] as i8);

        // 25 & 122
        let t_rcd_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[25] as u16, spd_data[122] as i8);

        // 26 & 121
        let t_rp_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[26] as u16, spd_data[121] as i8);

        // 27-29 and 120
        let n_mtb_t_ras = ((spd_data[27] as u16 & 0x0f) << 8) | spd_data[28] as u16;
        let n_mtb_t_rc = ((spd_data[27] as u16 & 0xf0) << 4) | spd_data[29] as u16;
        let t_ras_min_ps = get_time_ps(mtb_ps, ftb_ps, n_mtb_t_ras, 0);
        let t_rc_min_ps = get_time_ps(mtb_ps, ftb_ps, n_mtb_t_rc, spd_data[120] as i8);

        // 30-31
        let n_mtb_t_rfc1 = (spd_data[31] as u16) << 8 | spd_data[30] as u16;
        let t_rfc1_min_ps = get_time_ps(mtb_ps, ftb_ps, n_mtb_t_rfc1, 0);

        // 32-33
        let n_mtb_t_rfc2 = (spd_data[33] as u16) << 8 | spd_data[32] as u16;
        let t_rfc2_min_ps = get_time_ps(mtb_ps, ftb_ps, n_mtb_t_rfc2, 0);

        // 34-35
        let n_mtb_t_rfc4 = (spd_data[35] as u16) << 8 | spd_data[34] as u16;
        let t_rfc4_min_ps = get_time_ps(mtb_ps, ftb_ps, n_mtb_t_rfc4, 0);

        // 36-37
        let n_mtb_t_faw = (spd_data[36] as u16 & 0xf) << 8 | spd_data[37] as u16;
        let t_faw_min_ps = get_time_ps(mtb_ps, ftb_ps, n_mtb_t_faw, 0);

        // 38 & 119
        let t_rrd_s_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[38] as u16, spd_data[119] as i8);

        // 39 & 118
        let t_rrd_l_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[39] as u16, spd_data[118] as i8);

        // 40 & 117
        let t_ccd_l_min_ps = get_time_ps(mtb_ps, ftb_ps, spd_data[40] as u16, spd_data[117] as i8);

        // 60-77
        let mut dq_map = [0_u8; 18];
        dq_map.copy_from_slice(&spd_data[60..78]);

        GeneralConfig {
            spd_bytes_total,
            spd_bytes_used,
            spd_encoding,
            spd_additions,
            device_type,
            module_config,
            bg_addr_bits,
            bank_addr_bits,
            capacity_megabits,
            row_addr_bits,
            col_addr_bits,
            package_type,
            die_count,
            signal_loading,
            t_maw,
            mac,
            vdd_12_endurant,
            vdd_12_operable,
            package_ranks,
            device_width,
            bus_width_extension,
            bus_width,
            has_thermal_sensor,
            mtb_ps,
            ftb_ps,
            t_ckavg_min_ps,
            t_ckavg_max_ps,
            supported_cas_latencies,
            t_aa_min_ps,
            t_rcd_min_ps,
            t_rp_min_ps,
            t_ras_min_ps,
            t_rc_min_ps,
            t_rfc1_min_ps,
            t_rfc2_min_ps,
            t_rfc4_min_ps,
            t_faw_min_ps,
            t_rrd_s_min_ps,
            t_rrd_l_min_ps,
            t_ccd_l_min_ps,
            dq_map,
            dm_en: true,
            rd_dbi_en: false,
            wr_dbi_en: false,
            ecc_en: false,
            en_2nd_clk: false,
            parity_en: false,
            crc_en: false,
            power_down_en: false,
            clock_stop_en: false,
            self_ref_en: false,
            lp_auto_self_ref: false,
            temp_ref_mode: false,
            temp_ref_range: false,
            fine_granularity_ref_mode: FineGranularityRefMode::X1,
            self_ref_abort: false,
            v_ref: true,
            geardown: false,
        }
    }

    /// Number of bank groups.
    pub fn bank_groups(&self) -> u8 {
        // TODO: not sure if 0 vs. 1 really matters - if not, could replace with 1 << bg_addr_bits
        self.bg_addr_bits * 2 // 0 -> 0, 1 -> 2, 2 -> 4
    }

    /// Number of banks per bank group.
    pub fn banks_per_group(&self) -> u8 {
        1 << self.bank_addr_bits
    }

    pub fn min_clock_period_ns(&self) -> f32 {
        self.t_ckavg_min_ps as f32 / 1000.0
    }

    /// Caluclate maximum clock frequency in MHz from t_ck_min.
    pub fn max_clk_mhz(&self) -> u32 {
        1_000_000 / self.t_ckavg_min_ps
    }

    /// Caluclate minimum clock frequency in MHz from t_ck_max.
    pub fn min_clk_mhz(&self) -> u32 {
        1_000_000 / self.t_ckavg_max_ps
    }

    /// DDR speed bin (2 * `max_clk_mhz`).
    pub fn speed_bin_mhz(&self) -> u32 {
        2_000_000_u32.div_ceil(self.t_ckavg_min_ps)
    }

    /// Calculate number of logical ranks as defined in the spec:
    ///
    /// > “Logical rank” refers the individually addressable die in a 3DS stack and has no meaning for monolithic or multi-load
    /// > stacked SDRAMs; however, for the purposes of calculating the capacity of the module, one should treat monolithic and
    /// > multi-load stack SDRAMs as having one logical rank per package rank.
    pub fn logical_ranks(&self) -> u8 {
        match (self.package_type, self.signal_loading) {
            // SDP, DDP, QDP
            (PackageType::Monolithic, _)
            | (PackageType::NonMonolithic, SignalLoading::MultiLoadStack) => self.package_ranks,
            // 3DS
            _ => self.package_ranks * self.die_count,
        }
    }

    /// Calculate number of (logical) rank address bits.
    pub fn rank_addr_bits(&self) -> u8 {
        self.logical_ranks().next_power_of_two().ilog2() as u8
    }

    /// Calculate rank capacity.
    pub fn rank_capacity_megabytes(&self) -> u32 {
        self.capacity_megabits as u32 / 8 * self.bus_width as u32 / self.device_width as u32
    }

    /// Calculate module capacity from spec (page 4.1.2.12 – 15).
    pub fn module_capacity_megabytes(&self) -> u32 {
        self.rank_capacity_megabytes() * self.logical_ranks() as u32
    }

    /// Convert a parameter given in picoseconds to number of clock cycles according to the spec
    /// (page 4.1.2.12 – 19). Internally converts to and from `f32` to accommodate guardband.
    pub fn ps_to_nck(&self, time_ps: u32) -> u32 {
        // guardband factor of 0.01 clocks per the spec
        ceilf(time_ps as f32 / self.t_ckavg_min_ps as f32 - 0.01) as u32
    }

    pub fn burst_len(&self) -> u8 {
        DDR4_BURST_LEN
    }

    pub fn row_density(&self) -> u32 {
        self.rank_capacity_megabytes().ilog2() + LOG2_1_MIB
    }

    pub fn t_refi(&self) -> u32 {
        DDR4_TREFI_MAX_PS
    }

    pub fn cas_latency_ns(&self) -> u32 {
        let mut extra = 0;
        if self.rd_dbi_en {
            if self.max_clk_mhz() <= 933 {
                extra = 2;
            } else {
                extra = 3;
            }
        }
        self.t_aa_min_ps.div_ceil(1000) + 1 + extra
    }

    pub fn cas_write_latency_ns(&self) -> u32 {
        self.t_aa_min_ps.div_ceil(1000)
    }

    pub fn t_rfc_min_ps(&self) -> u32 {
        match self.fine_granularity_ref_mode {
            FineGranularityRefMode::X1 => self.t_rfc1_min_ps,
            FineGranularityRefMode::X2 => self.t_rfc2_min_ps,
            FineGranularityRefMode::X4 => self.t_rfc4_min_ps,
        }
    }

    pub fn ctl_clock_mhz(&self) -> u32 {
        self.max_clk_mhz() / 2
    }

    pub fn ctl_clock_period_ns(&self) -> f32 {
        self.min_clock_period_ns() * 2.0
    }

    /// "greater of 4CK or 6ns" - some random micron doc I found
    pub fn t_xp_nck(&self) -> u32 {
        self.ps_to_nck(6_000).max(4)
    }

    pub fn parity_latency_nck(&self) -> u32 {
        if self.parity_en {
            if self.speed_bin_mhz() < 2400 {
                4
            } else if self.speed_bin_mhz() < 2933 {
                5
            } else {
                6
            }
        } else {
            0
        }
    }

    pub fn additive_latency_nck(&self) -> u32 {
        // TODO: figure out how to decide which setting to use
        // either 0, CL-1, or CL-2 depending on MR1
        0
    }

    pub fn read_latency_nck(&self) -> u32 {
        // UG1087: "Note that, depending on the PHY, if using RDIMM,
        // it may be necessary to use a value of WL + 1 to compensate
        // for the extra cycle of latency through the RDIMM"
        self.ps_to_nck(self.cas_latency_ns() * 1000)
            + self.additive_latency_nck()
            + self.parity_latency_nck()
            + matches!(self.module_config, ModuleConfig::Registered(_)) as u32
    }

    pub fn write_latency_nck(&self) -> u32 {
        // UG1087: "Note that, depending on the PHY, if using RDIMM,
        // it may be necessary to use a value of WL + 1 to compensate
        // for the extra cycle of latency through the RDIMM"
        self.ps_to_nck(self.cas_write_latency_ns() * 1000)
            + self.additive_latency_nck()
            + self.parity_latency_nck()
            + matches!(self.module_config, ModuleConfig::Registered(_)) as u32
    }

    pub fn t_xs_min_ns(&self) -> u32 {
        self.t_rfc_min_ps() / 1000 + 10
    }

    pub fn t_xs_fast_min_ns(&self) -> u32 {
        self.t_rfc4_min_ps / 1000 + 10
    }

    pub fn t_xs_abort_min_ns(&self) -> u32 {
        self.t_xs_fast_min_ns()
    }

    pub fn t_dllk_min_nck(&self) -> u32 {
        if self.speed_bin_mhz() < 2133 {
            597
        } else if self.speed_bin_mhz() < 2666 {
            768
        } else {
            1024
        }
    }

    pub fn t_xs_dll_min_nck(&self) -> u32 {
        self.t_dllk_min_nck()
    }

    pub fn t_mrd_pda_min_nck(&self) -> u32 {
        self.ps_to_nck(10_000).max(16)
    }

    // these addr map implementations seem dumb but I don't know what I'm doing enough to
    // be comfortable changing them
    pub fn hif_addr_map(&self) -> [u32; 40] {
        let mut map = [0; 40];
        let mut i = 0;
        let start = match self.bus_width {
            16 => 2,
            32 => 1,
            _ => 0,
        };
        for j in start..(self.col_addr_bits as u32) {
            map[i] = HIF_COL_OFFSET + j;
            i += 1;
        }
        for j in 0..(self.bg_addr_bits as u32) {
            map[i] = HIF_BG_OFFSET + j;
            i += 1;
        }
        for j in 0..(self.bank_addr_bits as u32) {
            map[i] = HIF_BANK_OFFSET + j;
            i += 1;
        }
        for j in 0..(self.row_addr_bits as u32) {
            map[i] = HIF_ROW_OFFSET + j;
            i += 1;
        }
        for j in 0..(self.rank_addr_bits() as u32) {
            map[i] = HIF_RANK_OFFSET + j;
            i += 1;
        }
        map
    }

    // these are extra dumb, but again, idk what I'm doing
    pub fn bank_addr_map(&self) -> [u32; MAX_BANK_ADDR_BITS as usize] {
        let hif_addr_map = self.hif_addr_map();
        let mut map = [0x1f; MAX_BANK_ADDR_BITS as usize];
        let mut i = 0;
        for j in 0..(self.bank_addr_bits as u32) {
            i = j + 2;
            while hif_addr_map[i as usize] != HIF_BANK_OFFSET + j {
                i += 1;
            }
            map[j as usize] = i - (j + 2);
        }
        map
    }

    pub fn bg_addr_map(&self) -> [u32; MAX_BG_ADDR_BITS as usize] {
        let hif_addr_map = self.hif_addr_map();
        let mut map = [0x1f; MAX_BG_ADDR_BITS as usize];
        let mut i = 0;
        for j in 0..(self.bg_addr_bits as u32) {
            i = j + 2;
            while hif_addr_map[i as usize] != HIF_BG_OFFSET + j {
                i += 1;
            }
            map[j as usize] = i - (j + 2);
        }
        map
    }

    pub fn col_addr_map(&self) -> [u32; MAX_COL_ADDR_BITS as usize] {
        let hif_addr_map = self.hif_addr_map();
        let mut map = [0xf; MAX_COL_ADDR_BITS as usize];
        let mut i = 0;
        for j in 2..(self.col_addr_bits as u32) {
            i = j;
            while hif_addr_map[i as usize] != HIF_COL_OFFSET + j {
                i += 1;
            }
            map[j as usize] = i - j;
        }
        map
    }

    pub fn row_addr_map(&self) -> [u32; MAX_ROW_ADDR_BITS as usize] {
        let hif_addr_map = self.hif_addr_map();
        let mut map = [0xf; MAX_ROW_ADDR_BITS as usize];
        let mut i = 0;
        for j in 0..(self.row_addr_bits as u32) {
            i = j + 6;
            while hif_addr_map[i as usize] != HIF_ROW_OFFSET + j {
                i += 1;
            }
            map[j as usize] = i - (j + 6);
        }
        map
    }

    pub fn t_wr_min_ns(&self) -> u32 {
        // defined as 15 ns for all speed bins in JESD 79-4
        15
    }

    pub fn mr0(&self) -> u16 {
        // calculate MR0 value for DDR4
        // bits A12, 6-4, and 2 calculated from CL - JESD 79-4 Table 3
        // but 12 will always be zero for the supported speed bins (I think?)
        let cas_latency_ns = self.cas_latency_ns() as u16;
        let mut mr0 = 0;
        if cas_latency_ns <= 16 {
            mr0 |= ((cas_latency_ns - 1) % 2) << 2;
            mr0 |= ((cas_latency_ns - 9) / 2) & 0x7 << 4;
        } else if cas_latency_ns % 2 == 1 {
            mr0 |= (((cas_latency_ns + 1) / 2) % 2) << 2;
            mr0 |= ((cas_latency_ns + 2) / 6) & 0x7 << 4;
        } else {
            mr0 |= ((cas_latency_ns / 2 + 1) % 2) << 2;
            mr0 |= ((cas_latency_ns - 1) / 4) & 0x7 << 4;
        }
        // bits A13-9 calculated from write recovery/read-to-precharge time
        // JESD 79-4 Table 2
        let mut t_wr_min_nck = self.ps_to_nck(self.t_wr_min_ns() * 1000) as u16;
        // for SOME reason, 22 (11) and 24 (12) are swapped in the table
        if t_wr_min_nck == 22 {
            // if yes { no }
            t_wr_min_nck = 24;
        } else if t_wr_min_nck == 24 {
            t_wr_min_nck = 22;
        }
        mr0 |= (t_wr_min_nck / 2 - 5) & 0xf << 9;
        // bits A1-0 calculated from burst length
        // Zynq DDRC doesn't support on-the-fly burst length (0b01)
        if self.burst_len() == 4 {
            mr0 |= 0b10;
        } else {
            mr0 |= 0b00;
        }
        mr0
    }
}

// bytes 128-255
// global index of section start (for readability)
const MODULE_CONFIG_START: usize = 128;
impl UnbufferedConfig {
    pub fn from_spd_data(spd_data: &[u8; 128]) -> Self {
        let expected_crc = ((spd_data[127] as u16) << 8) | (spd_data[126] as u16);
        let actual_crc = crc(&spd_data[..126]);
        assert_eq!(
            expected_crc, actual_crc,
            "CRC mismatch. Expected: {:#X}, actual: {:#X}",
            expected_crc, actual_crc
        );
        debug!("CRC check passed");

        // 128
        let raw_card_extension = spd_data[128 - MODULE_CONFIG_START] >> 5;
        let module_nominal_height = spd_data[128 - MODULE_CONFIG_START] & 0x1f;

        // 129
        let module_max_thickness_back = spd_data[129 - MODULE_CONFIG_START] >> 4;
        let module_max_thickness_front = spd_data[129 - MODULE_CONFIG_START] & 0xf;

        // 130
        let ref_raw_card_rev = spd_data[130 - MODULE_CONFIG_START] >> 5 & 0x3;
        let ref_raw_card = spd_data[130 - MODULE_CONFIG_START] & 0x1f;

        // 131
        let rank_1_mirrored = spd_data[131 - MODULE_CONFIG_START] & 0x1 == 0x1;

        UnbufferedConfig {
            raw_card_extension,
            module_nominal_height,
            module_max_thickness_back,
            module_max_thickness_front,
            ref_raw_card_rev,
            ref_raw_card,
            rank_1_mirrored,
        }
    }
}
