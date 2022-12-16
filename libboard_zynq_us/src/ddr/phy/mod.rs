use libregister::{RegisterR, RegisterW};

mod regs;

pub struct DdrPhy {
    pub regs: &'static mut regs::RegisterBlock,
}

const DEFAULT_TIMEOUT: u32 = 1_000_000; // arbitrary

impl DdrPhy {
    pub fn ddr_phy() -> Self {
        let self_ = DdrPhy {
            regs: regs::RegisterBlock::ddr_phy(),
        };
        self_
    }
    /// PHY data training configuration
    pub fn configure(&mut self) {
        unsafe {
            self.regs.pgcr0.write(0x07001E00);
            self.regs.pgcr2.write(0x00F10010);
            self.regs.pgcr3.write(0x55AA5480);
            self.regs.pgcr5.write(0x010100F4);

            self.regs.ptr0.write(0x42C21590);
            self.regs.ptr1.write(0xD05512C0);

            self.regs.pll_ctrl0.write(0x01100000);
            self.regs.ddr_sys_cfg.write(0x02A04161);
            self.regs.gpr0.write(0x00000000);
            self.regs.gpr1.write(0x000000E5);
            self.regs.dram_cfg.write(0x0800040C);

            self.regs.dtpr0.write(0x07240F08);
            self.regs.dtpr1.write(0x28200008);
            self.regs.dtpr2.write(0x000F0300);
            self.regs.dtpr3.write(0x83000800);
            self.regs.dtpr4.write(0x01762B07);
            self.regs.dtpr5.write(0x00330F08);
            self.regs.dtpr6.write(0x00000E0F);

            self.regs.rdimm_cfg0.write(0x08400020);
            self.regs.rdimm_cfg1.write(0x00000C80);
            self.regs.rdimm_ctrl0.write(0x00000000);
            self.regs.rdimm_ctrl1.write(0x00000300);

            self.regs.mode0.write(0x00000630);
            self.regs.mode1.write(0x00000301);
            self.regs.mode2.write(0x00000020);
            self.regs.mode3.write(0x00000200);
            self.regs.mode4.write(0x00000000);
            self.regs.mode5.write(0x000006C0);
            self.regs.mode6.write(0x00000819);
            self.regs.mode11.write(0x00000000);
            self.regs.mode12.write(0x0000004D);
            self.regs.mode13.write(0x00000008);
            self.regs.mode14.write(0x0000004D);
            self.regs.mode22.write(0x00000000);

            self.regs.dt_cfg0.write(0x800091C7);
            self.regs.dt_cfg1.write(0x00010236);
            self.regs.catr0.write(0x00141054);
            self.regs.dqs_drift0.write(0x00088000);
            // BISTLSR: self-test, undocumented
            self.regs.rio_cr5.write(0x00000005);
            self.regs.aio_cr0.write(0x30000028);
            self.regs.aio_cr2.write(0x0A000000);
            self.regs.aio_cr3.write(0x00000009);
            self.regs.aio_cr4.write(0x0A000000);

            self.regs.io_vcr0.write(0x0300B0CE);

            self.regs.vtcr0.write(0xF9032019);
            self.regs.vtcr1.write(0x07F001E3);

            self.regs.ac_bdlr1.write(0x00000000);
            self.regs.ac_bdlr2.write(0x00000000);
            self.regs.ac_bdlr6.write(0x00000000);
            self.regs.ac_bdlr7.write(0x00000000);
            self.regs.ac_bdlr8.write(0x00000000);
            self.regs.ac_bdlr9.write(0x00000000);

            self.regs.zqcr.write(0x008AAA58);
            self.regs.zq0_pr0.write(0x000079DD);
            self.regs.zq0_or0.write(0x01E10210);
            self.regs.zq0_or1.write(0x01E10000);
            self.regs.zq1_pr0.write(0x00087BDB);

            self.regs.dx0_gcr0.write(0x40800604);
            self.regs.dx0_gcr1.write(0x00007FFF);
            self.regs.dx0_gcr3.write(0x3F000008);
            self.regs.dx0_gcr4.write(0x0E00B03C);
            self.regs.dx0_gcr5.write(0x09095555);
            self.regs.dx0_gcr6.write(0x09092B2B);

            self.regs.dx1_gcr0.write(0x40800604);
            self.regs.dx1_gcr1.write(0x00007FFF);
            self.regs.dx1_gcr3.write(0x3F000008);
            self.regs.dx1_gcr4.write(0x0E00B03C);
            self.regs.dx1_gcr5.write(0x09095555);
            self.regs.dx1_gcr6.write(0x09092B2B);

            self.regs.dx2_gcr0.write(0x40800604);
            self.regs.dx2_gcr1.write(0x00007FFF);
            self.regs.dx2_gcr3.write(0x3F000008);
            self.regs.dx2_gcr4.write(0x0E00B004);
            self.regs.dx2_gcr5.write(0x09095555);
            self.regs.dx2_gcr6.write(0x09092B2B);

            self.regs.dx3_gcr0.write(0x40800604);
            self.regs.dx3_gcr1.write(0x00007FFF);
            self.regs.dx3_gcr3.write(0x3F000008);
            self.regs.dx3_gcr4.write(0x0E00B004);
            self.regs.dx3_gcr5.write(0x09095555);
            self.regs.dx3_gcr6.write(0x09092B2B);

            self.regs.dx4_gcr0.write(0x40800604);
            self.regs.dx4_gcr1.write(0x00007FFF);
            self.regs.dx4_gcr2.write(0x00000000);
            self.regs.dx4_gcr3.write(0x3F000008);
            self.regs.dx4_gcr4.write(0x0E00B004);
            self.regs.dx4_gcr5.write(0x09095555);
            self.regs.dx4_gcr6.write(0x09092B2B);

            self.regs.dx5_gcr0.write(0x40800604);
            self.regs.dx5_gcr1.write(0x00007FFF);
            self.regs.dx5_gcr2.write(0x00000000);
            self.regs.dx5_gcr3.write(0x3F000008);
            self.regs.dx5_gcr4.write(0x0E00B03C);
            self.regs.dx5_gcr5.write(0x09095555);
            self.regs.dx5_gcr6.write(0x09092B2B);

            self.regs.dx6_gcr0.write(0x40800604);
            self.regs.dx6_gcr1.write(0x00007FFF);
            self.regs.dx6_gcr2.write(0x00000000);
            self.regs.dx6_gcr3.write(0x3F000008);
            self.regs.dx6_gcr4.write(0x0E00B004);
            self.regs.dx6_gcr5.write(0x09095555);
            self.regs.dx6_gcr6.write(0x09092B2B);

            self.regs.dx7_gcr0.write(0x40800604);
            self.regs.dx7_gcr1.write(0x00007FFF);
            self.regs.dx7_gcr2.write(0x00000000);
            self.regs.dx7_gcr3.write(0x3F000008);
            self.regs.dx7_gcr4.write(0x0E00B03C);
            self.regs.dx7_gcr5.write(0x09095555);
            self.regs.dx7_gcr6.write(0x09092B2B);

            self.regs.dx8_gcr0.write(0x80803660);
            self.regs.dx8_gcr1.write(0x55556000);
            self.regs.dx8_gcr2.write(0xAAAAAAAA);
            self.regs.dx8_gcr3.write(0x0029A4A4);
            self.regs.dx8_gcr4.write(0x0C00B000);
            self.regs.dx8_gcr5.write(0x09095555);
            self.regs.dx8_gcr6.write(0x09092B2B);

            self.regs.dx8_sl0_osc.write(0x2A019FFE);
            self.regs.dx8_sl0_pllcr0.write(0x01100000);
            self.regs.dx8_sl0_dqsctl.write(0x01264300);
            self.regs.dx8_sl0_dxctl2.write(0x00041800);
            self.regs.dx8_sl0_iocr.write(0x70800000);

            self.regs.dx8_sl1_osc.write(0x2A019FFE);
            self.regs.dx8_sl1_pllcr0.write(0x01100000);
            self.regs.dx8_sl1_dqsctl.write(0x01264300);
            self.regs.dx8_sl1_dxctl2.write(0x00041800);
            self.regs.dx8_sl1_iocr.write(0x70800000);

            self.regs.dx8_sl2_osc.write(0x2A019FFE);
            self.regs.dx8_sl2_pllcr0.write(0x01100000);
            self.regs.dx8_sl2_dqsctl.write(0x01264300);
            self.regs.dx8_sl2_dxctl2.write(0x00041800);
            self.regs.dx8_sl2_iocr.write(0x70800000);

            self.regs.dx8_sl3_osc.write(0x2A019FFE);
            self.regs.dx8_sl3_pllcr0.write(0x01100000);
            self.regs.dx8_sl3_dqsctl.write(0x01264300);
            self.regs.dx8_sl3_dxctl2.write(0x00041800);
            self.regs.dx8_sl3_iocr.write(0x70800000);

            self.regs.dx8_sl4_osc.write(0x15019FFE);
            self.regs.dx8_sl4_pllcr0.write(0x21100000);
            self.regs.dx8_sl4_dqsctl.write(0x01266300);
            self.regs.dx8_sl4_dxctl2.write(0x00041800);
            self.regs.dx8_sl4_iocr.write(0x70400000);

            self.regs.dx8_slb_dqsctl.write(0x012643C4);
        }
    }

    /// Initialize DDR PLLs
    pub fn init_plls(&mut self) {
        let mut pll_retry: u8 = 10;
        let mut pll_locked: bool = false;

        while pll_retry > 0 && !pll_locked {
            self.regs.phy_init.write(
                regs::PhyInit::zeroed()
                    .ctrl_dram_init(true)
                    .pll_init(true)
                    .init(),
            );
            // self.regs.phy_init.modify(|_, w| w.init());

            let mut timeout = DEFAULT_TIMEOUT;
            while timeout > 0 && !self.regs.pgsr0.read().pl_done() {
                timeout -= 1;
            }
            if timeout == 0 {
                continue;
            };

            pll_locked = self.regs.pgsr0.read().ap_lock()
                && self.regs.dx0_gsr0.read().dp_lock()
                && self.regs.dx2_gsr0.read().dp_lock()
                && self.regs.dx4_gsr0.read().dp_lock()
                && self.regs.dx6_gsr0.read().dp_lock();
            pll_retry -= 1;
        }
        // random write of pll_retry to GPR0?
        assert!(pll_locked, "DDR PLLs failed to lock");
    }

    /// Execute impedance and digital delay line calibrations in parallel
    pub fn execute_zcal_dcal(&mut self) {
        self.regs.phy_init.write(
            regs::PhyInit::zeroed()
                .ctrl_dram_init(true)
                .phy_rst(true)
                .dcal(true)
                .zcal(true)
                .init(),
        );

        let mut timeout = DEFAULT_TIMEOUT;
        let mut pgsr0_read = self.regs.pgsr0.read();
        while timeout > 0 && !(pgsr0_read.zc_done() && pgsr0_read.dc_done() && pgsr0_read.i_done())
        {
            pgsr0_read = self.regs.pgsr0.read();
            timeout -= 1;
        }
        assert!(
            timeout != 0,
            "Timed out while waiting for calibration to finish"
        );

        // wait for DRAM init done
        self.regs
            .phy_init
            .write(regs::PhyInit::zeroed().ctrl_dram_init(true).init());

        timeout = DEFAULT_TIMEOUT;
        pgsr0_read = self.regs.pgsr0.read();
        while timeout > 0 && !(pgsr0_read.di_done() && pgsr0_read.i_done()) {
            pgsr0_read = self.regs.pgsr0.read();
            timeout -= 1;
        }
        assert!(
            timeout != 0,
            "Timed out while waiting for calibration to finish"
        );
    }

    /// Execute training sequences
    pub fn execute_training(&mut self) {
        unsafe {
            self.regs.pgcr1.write(0x00000040);
        }
        self.regs.phy_init.write(
            regs::PhyInit::zeroed()
                .ctrl_dram_init(true)
                .wr_eye(true)
                .rd_eye(true)
                .wr_deskew(true)
                .rd_deskew(true)
                .wr_lev_adj(true)
                .qs_gate(true)
                .wr_lev(true)
                .init(),
        );

        let mut timeout = DEFAULT_TIMEOUT;
        let mut pgsr0_read = self.regs.pgsr0.read();
        while timeout > 0
            && !(pgsr0_read.we_done()
                && pgsr0_read.re_done()
                && pgsr0_read.wd_done()
                && pgsr0_read.rd_done()
                && pgsr0_read.wla_done()
                && pgsr0_read.qsg_done()
                && pgsr0_read.wl_done())
        {
            pgsr0_read = self.regs.pgsr0.read();
            timeout -= 1;
        }
        assert!(
            timeout != 0,
            "Timed out while waiting for training to finish"
        );

        assert!(
            !(pgsr0_read.ca_err()
                || pgsr0_read.we_err()
                || pgsr0_read.re_err()
                || pgsr0_read.wd_err()
                || pgsr0_read.rd_err()
                || pgsr0_read.wla_err()
                || pgsr0_read.qsg_err()
                || pgsr0_read.wl_err()
                || pgsr0_read.zc_err()
                || pgsr0_read.v_err()
                || pgsr0_read.dqs2dq_err()),
            "Error flag(s) set in PGSR0"
        );

        // vref training
        unsafe {
            // set static read mode
            self.regs.dt_cfg0.write(0x100091C7);
            self.regs.pgcr3.modify(|cur| cur | (0x3 << 3));
            self.regs.dx8_sl0_dxctl2.modify(|cur| cur | (0x3 << 4));
            self.regs.dx8_sl1_dxctl2.modify(|cur| cur | (0x3 << 4));
            self.regs.dx8_sl2_dxctl2.modify(|cur| cur | (0x3 << 4));
            self.regs.dx8_sl3_dxctl2.modify(|cur| cur | (0x3 << 4));
            self.regs.dx8_sl4_dxctl2.modify(|cur| cur | (0x3 << 4));
        }

        self.regs.phy_init.write(
            regs::PhyInit::zeroed().ctrl_dram_init(true).vref(true).init()
        );

        timeout = DEFAULT_TIMEOUT;
        pgsr0_read = self.regs.pgsr0.read();
        while timeout > 0 && !(pgsr0_read.v_done() && pgsr0_read.i_done()) {
            pgsr0_read = self.regs.pgsr0.read();
            timeout -= 1;
        }
        assert!(
            timeout != 0,
            "Timed out while waiting for training to finish"
        );

        assert!(!pgsr0_read.v_err(), "VREF training error");


        unsafe {
            // disable static read mode
            self.regs.pgcr3.modify(|cur| cur & !(0x3 << 3));
            self.regs.dx8_sl0_dxctl2.modify(|cur| cur & !(0x3 << 4));
            self.regs.dx8_sl1_dxctl2.modify(|cur| cur & !(0x3 << 4));
            self.regs.dx8_sl2_dxctl2.modify(|cur| cur & !(0x3 << 4));
            self.regs.dx8_sl3_dxctl2.modify(|cur| cur & !(0x3 << 4));
            self.regs.dx8_sl4_dxctl2.modify(|cur| cur & !(0x3 << 4));
            self.regs.dt_cfg0.write(0x800091C7);
        }

        // execute wr/rd eye training again...?
        self.regs.phy_init.write(
            regs::PhyInit::zeroed().wr_eye(true).rd_eye(true).init()
        );

        timeout = DEFAULT_TIMEOUT;
        pgsr0_read = self.regs.pgsr0.read();
        while timeout > 0 && !(pgsr0_read.we_done() && pgsr0_read.re_done() && pgsr0_read.i_done()) {
            pgsr0_read = self.regs.pgsr0.read();
            timeout -= 1;
        }
        assert!(
            timeout != 0,
            "Timed out while waiting for training to finish"
        );
    }
}
