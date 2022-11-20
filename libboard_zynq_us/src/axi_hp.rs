///! S_AXI_HP Interface (AFI)
/// UG1085 Chapter 35: PS-PL AXI Interfaces:
/// "The S_AXI_HP1_FPD and S_AXI_HP2_FPD interfaces share exclusive access to an AXI Port
/// Interface (XPI 4). This facilitates high throughput and relatively low-latency access from the
/// PL directly to the DDR memory. S_AXI_HP0_FPD shares an XPI port on the memory
/// controller with the DisplayPort master in the PL and S_AXI_HP3_FPD with the FPD DMA
/// controller."

use volatile_register::{RW, RO, WO};

use libregister::{register, register_at, register_bit, register_bits, register_bits_typed};


#[repr(u8)]
pub enum FabricWidth {
    Width128 = 0b00,
    Width64 = 0b01,
    Width32 = 0b10,
}

#[repr(C)]
pub struct RegisterBlock {
    /// Read Channel Control Register
    pub rdchan_ctrl: RdchanCtrl,
    /// Read Issuing Capability Register
    pub rdchan_issuingcap: RW<u32>,
    /// QOS Read Channel Register
    pub rdqos: RW<u32>,
    unused1: [u32; 1],
    /// Read Channel Debug Register
    pub rddebug: RW<u32>,
    /// Write Channel Control Register
    pub wrchan_ctrl: WrchanCtrl,
    /// Write Issuing Capability Register
    pub wrchan_issuingcap: RW<u32>,
    /// QOS Write Channel Register
    pub wrqos: RW<u32>,
    unused2: [u32; 888],
    pub i_sts: RW<u32>, // WTC LSB
    pub i_en: WO<u32>,
    pub i_dis: WO<u32>,
    pub i_mask: RO<u32>,
    unused3: [u32; 61],
    pub apb_err_resp: RW<u32>,
    unused4: [u32; 1],
    pub safety_chk: RW<u32>,
}
register_at!(RegisterBlock, 0xFD36_0000, s_axi_hpc0_fpd);
register_at!(RegisterBlock, 0xFD37_0000, s_axi_hpc1_fpd);
register_at!(RegisterBlock, 0xFD38_0000, s_axi_hp0_fpd);
register_at!(RegisterBlock, 0xFD39_0000, s_axi_hp1_fpd);
register_at!(RegisterBlock, 0xFD3A_0000, s_axi_hp2_fpd);
register_at!(RegisterBlock, 0xFD3B_0000, s_axi_hp3_fpd);
register_at!(RegisterBlock, 0xFF9B_0000, s_axi_lpd);

register!(rdchan_ctrl, RdchanCtrl, RW, u32);
/// Pause the issuing of new read commands to the PS-side. 
/// Existing outstanding commands will continue to be processed.
register_bit!(rdchan_ctrl, pause, 3);
/// Enable control of QoS from the fabric
register_bit!(rdchan_ctrl, fabric_qos_en, 2);
/// Configures the Read Channel Fabric interface width
register_bits_typed!(rdchan_ctrl, fabric_width, u8, FabricWidth, 0, 1);


register!(wrchan_ctrl, WrchanCtrl, RW, u32);
/// Mode of Write Command Release
register_bit!(wrchan_ctrl, wr_release_mode, 12);
/// Pause the issuing of new write commands to the PS-side.
/// Existing write commands will continue to be processed.
register_bit!(wrchan_ctrl, pause, 3);
/// Enable control of QoS from the fabric
register_bit!(wrchan_ctrl, fabric_qos_en, 2);
/// Configures the Write Channel Fabric interface width
register_bits_typed!(wrchan_ctrl, fabric_width, u8, FabricWidth, 0, 1);
