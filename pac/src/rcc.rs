#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub rcc_cr: RCC_CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub rcc_icscr: RCC_ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub rcc_cfgr: RCC_CFGR,
    #[doc = "0x0c - PLL configuration register"]
    pub rcc_pllcfgr: RCC_PLLCFGR,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub rcc_cier: RCC_CIER,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub rcc_cifr: RCC_CIFR,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub rcc_cicr: RCC_CICR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - AHB1 peripheral reset register"]
    pub rcc_ahb1rstr: RCC_AHB1RSTR,
    #[doc = "0x2c - AHB2 peripheral reset register"]
    pub rcc_ahb2rstr: RCC_AHB2RSTR,
    #[doc = "0x30 - AHB3 peripheral reset register"]
    pub rcc_ahb3rstr: RCC_AHB3RSTR,
    _reserved10: [u8; 0x04],
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    pub rcc_apb1rstr1: RCC_APB1RSTR1,
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    pub rcc_apb1rstr2: RCC_APB1RSTR2,
    #[doc = "0x40 - APB2 peripheral reset register"]
    pub rcc_apb2rstr: RCC_APB2RSTR,
    _reserved13: [u8; 0x04],
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    pub rcc_ahb1enr: RCC_AHB1ENR,
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    pub rcc_ahb2enr: RCC_AHB2ENR,
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    pub rcc_ahb3enr: RCC_AHB3ENR,
    _reserved16: [u8; 0x04],
    #[doc = "0x58 - APB1 peripheral clock enable register 1"]
    pub rcc_apb1enr1: RCC_APB1ENR1,
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    pub rcc_apb1enr2: RCC_APB1ENR2,
    #[doc = "0x60 - APB2 peripheral clock enable register"]
    pub rcc_apb2enr: RCC_APB2ENR,
    _reserved19: [u8; 0x04],
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    pub rcc_ahb1smenr: RCC_AHB1SMENR,
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    pub rcc_ahb2smenr: RCC_AHB2SMENR,
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub rcc_ahb3smenr: RCC_AHB3SMENR,
    _reserved22: [u8; 0x04],
    #[doc = "0x78 - APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    pub rcc_apb1smenr1: RCC_APB1SMENR1,
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    pub rcc_apb1smenr2: RCC_APB1SMENR2,
    #[doc = "0x80 - APB2 peripheral clocks enable in Sleep and Stop modes register"]
    pub rcc_apb2smenr: RCC_APB2SMENR,
    _reserved25: [u8; 0x04],
    #[doc = "0x88 - Peripherals independent clock configuration register"]
    pub rcc_ccipr: RCC_CCIPR,
    _reserved26: [u8; 0x04],
    #[doc = "0x90 - RTC domain control register"]
    pub rcc_bdcr: RCC_BDCR,
    #[doc = "0x94 - Control/status register"]
    pub rcc_csr: RCC_CSR,
    #[doc = "0x98 - Clock recovery RC register"]
    pub rcc_crrcr: RCC_CRRCR,
    #[doc = "0x9c - Peripherals independent clock configuration register"]
    pub rcc_ccipr2: RCC_CCIPR2,
}
#[doc = "RCC_CR (rw) register accessor: an alias for `Reg<RCC_CR_SPEC>`"]
pub type RCC_CR = crate::Reg<rcc_cr::RCC_CR_SPEC>;
#[doc = "Clock control register"]
pub mod rcc_cr;
#[doc = "RCC_ICSCR (rw) register accessor: an alias for `Reg<RCC_ICSCR_SPEC>`"]
pub type RCC_ICSCR = crate::Reg<rcc_icscr::RCC_ICSCR_SPEC>;
#[doc = "Internal clock sources calibration register"]
pub mod rcc_icscr;
#[doc = "RCC_CFGR (rw) register accessor: an alias for `Reg<RCC_CFGR_SPEC>`"]
pub type RCC_CFGR = crate::Reg<rcc_cfgr::RCC_CFGR_SPEC>;
#[doc = "Clock configuration register"]
pub mod rcc_cfgr;
#[doc = "RCC_PLLCFGR (rw) register accessor: an alias for `Reg<RCC_PLLCFGR_SPEC>`"]
pub type RCC_PLLCFGR = crate::Reg<rcc_pllcfgr::RCC_PLLCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod rcc_pllcfgr;
#[doc = "RCC_CIER (rw) register accessor: an alias for `Reg<RCC_CIER_SPEC>`"]
pub type RCC_CIER = crate::Reg<rcc_cier::RCC_CIER_SPEC>;
#[doc = "Clock interrupt enable register"]
pub mod rcc_cier;
#[doc = "RCC_CIFR (r) register accessor: an alias for `Reg<RCC_CIFR_SPEC>`"]
pub type RCC_CIFR = crate::Reg<rcc_cifr::RCC_CIFR_SPEC>;
#[doc = "Clock interrupt flag register"]
pub mod rcc_cifr;
#[doc = "RCC_CICR (w) register accessor: an alias for `Reg<RCC_CICR_SPEC>`"]
pub type RCC_CICR = crate::Reg<rcc_cicr::RCC_CICR_SPEC>;
#[doc = "Clock interrupt clear register"]
pub mod rcc_cicr;
#[doc = "RCC_AHB1RSTR (rw) register accessor: an alias for `Reg<RCC_AHB1RSTR_SPEC>`"]
pub type RCC_AHB1RSTR = crate::Reg<rcc_ahb1rstr::RCC_AHB1RSTR_SPEC>;
#[doc = "AHB1 peripheral reset register"]
pub mod rcc_ahb1rstr;
#[doc = "RCC_AHB2RSTR (rw) register accessor: an alias for `Reg<RCC_AHB2RSTR_SPEC>`"]
pub type RCC_AHB2RSTR = crate::Reg<rcc_ahb2rstr::RCC_AHB2RSTR_SPEC>;
#[doc = "AHB2 peripheral reset register"]
pub mod rcc_ahb2rstr;
#[doc = "RCC_AHB3RSTR (rw) register accessor: an alias for `Reg<RCC_AHB3RSTR_SPEC>`"]
pub type RCC_AHB3RSTR = crate::Reg<rcc_ahb3rstr::RCC_AHB3RSTR_SPEC>;
#[doc = "AHB3 peripheral reset register"]
pub mod rcc_ahb3rstr;
#[doc = "RCC_APB1RSTR1 (rw) register accessor: an alias for `Reg<RCC_APB1RSTR1_SPEC>`"]
pub type RCC_APB1RSTR1 = crate::Reg<rcc_apb1rstr1::RCC_APB1RSTR1_SPEC>;
#[doc = "APB1 peripheral reset register 1"]
pub mod rcc_apb1rstr1;
#[doc = "RCC_APB1RSTR2 (rw) register accessor: an alias for `Reg<RCC_APB1RSTR2_SPEC>`"]
pub type RCC_APB1RSTR2 = crate::Reg<rcc_apb1rstr2::RCC_APB1RSTR2_SPEC>;
#[doc = "APB1 peripheral reset register 2"]
pub mod rcc_apb1rstr2;
#[doc = "RCC_APB2RSTR (rw) register accessor: an alias for `Reg<RCC_APB2RSTR_SPEC>`"]
pub type RCC_APB2RSTR = crate::Reg<rcc_apb2rstr::RCC_APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod rcc_apb2rstr;
#[doc = "RCC_AHB1ENR (rw) register accessor: an alias for `Reg<RCC_AHB1ENR_SPEC>`"]
pub type RCC_AHB1ENR = crate::Reg<rcc_ahb1enr::RCC_AHB1ENR_SPEC>;
#[doc = "AHB1 peripheral clock enable register"]
pub mod rcc_ahb1enr;
#[doc = "RCC_AHB2ENR (rw) register accessor: an alias for `Reg<RCC_AHB2ENR_SPEC>`"]
pub type RCC_AHB2ENR = crate::Reg<rcc_ahb2enr::RCC_AHB2ENR_SPEC>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod rcc_ahb2enr;
#[doc = "RCC_AHB3ENR (rw) register accessor: an alias for `Reg<RCC_AHB3ENR_SPEC>`"]
pub type RCC_AHB3ENR = crate::Reg<rcc_ahb3enr::RCC_AHB3ENR_SPEC>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod rcc_ahb3enr;
#[doc = "RCC_APB1ENR1 (rw) register accessor: an alias for `Reg<RCC_APB1ENR1_SPEC>`"]
pub type RCC_APB1ENR1 = crate::Reg<rcc_apb1enr1::RCC_APB1ENR1_SPEC>;
#[doc = "APB1 peripheral clock enable register 1"]
pub mod rcc_apb1enr1;
#[doc = "RCC_APB1ENR2 (rw) register accessor: an alias for `Reg<RCC_APB1ENR2_SPEC>`"]
pub type RCC_APB1ENR2 = crate::Reg<rcc_apb1enr2::RCC_APB1ENR2_SPEC>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod rcc_apb1enr2;
#[doc = "RCC_APB2ENR (rw) register accessor: an alias for `Reg<RCC_APB2ENR_SPEC>`"]
pub type RCC_APB2ENR = crate::Reg<rcc_apb2enr::RCC_APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod rcc_apb2enr;
#[doc = "RCC_AHB1SMENR (rw) register accessor: an alias for `Reg<RCC_AHB1SMENR_SPEC>`"]
pub type RCC_AHB1SMENR = crate::Reg<rcc_ahb1smenr::RCC_AHB1SMENR_SPEC>;
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_ahb1smenr;
#[doc = "RCC_AHB2SMENR (rw) register accessor: an alias for `Reg<RCC_AHB2SMENR_SPEC>`"]
pub type RCC_AHB2SMENR = crate::Reg<rcc_ahb2smenr::RCC_AHB2SMENR_SPEC>;
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_ahb2smenr;
#[doc = "RCC_AHB3SMENR (rw) register accessor: an alias for `Reg<RCC_AHB3SMENR_SPEC>`"]
pub type RCC_AHB3SMENR = crate::Reg<rcc_ahb3smenr::RCC_AHB3SMENR_SPEC>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_ahb3smenr;
#[doc = "RCC_APB1SMENR1 (rw) register accessor: an alias for `Reg<RCC_APB1SMENR1_SPEC>`"]
pub type RCC_APB1SMENR1 = crate::Reg<rcc_apb1smenr1::RCC_APB1SMENR1_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
pub mod rcc_apb1smenr1;
#[doc = "RCC_APB1SMENR2 (rw) register accessor: an alias for `Reg<RCC_APB1SMENR2_SPEC>`"]
pub type RCC_APB1SMENR2 = crate::Reg<rcc_apb1smenr2::RCC_APB1SMENR2_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod rcc_apb1smenr2;
#[doc = "RCC_APB2SMENR (rw) register accessor: an alias for `Reg<RCC_APB2SMENR_SPEC>`"]
pub type RCC_APB2SMENR = crate::Reg<rcc_apb2smenr::RCC_APB2SMENR_SPEC>;
#[doc = "APB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_apb2smenr;
#[doc = "RCC_CCIPR (rw) register accessor: an alias for `Reg<RCC_CCIPR_SPEC>`"]
pub type RCC_CCIPR = crate::Reg<rcc_ccipr::RCC_CCIPR_SPEC>;
#[doc = "Peripherals independent clock configuration register"]
pub mod rcc_ccipr;
#[doc = "RCC_BDCR (rw) register accessor: an alias for `Reg<RCC_BDCR_SPEC>`"]
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCR_SPEC>;
#[doc = "RTC domain control register"]
pub mod rcc_bdcr;
#[doc = "RCC_CSR (rw) register accessor: an alias for `Reg<RCC_CSR_SPEC>`"]
pub type RCC_CSR = crate::Reg<rcc_csr::RCC_CSR_SPEC>;
#[doc = "Control/status register"]
pub mod rcc_csr;
#[doc = "RCC_CRRCR (rw) register accessor: an alias for `Reg<RCC_CRRCR_SPEC>`"]
pub type RCC_CRRCR = crate::Reg<rcc_crrcr::RCC_CRRCR_SPEC>;
#[doc = "Clock recovery RC register"]
pub mod rcc_crrcr;
#[doc = "RCC_CCIPR2 (rw) register accessor: an alias for `Reg<RCC_CCIPR2_SPEC>`"]
pub type RCC_CCIPR2 = crate::Reg<rcc_ccipr2::RCC_CCIPR2_SPEC>;
#[doc = "Peripherals independent clock configuration register"]
pub mod rcc_ccipr2;
