#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    pub pwr_cr1: PWR_CR1,
    #[doc = "0x04 - Power control register 2"]
    pub pwr_cr2: PWR_CR2,
    #[doc = "0x08 - Power control register 3"]
    pub pwr_cr3: PWR_CR3,
    #[doc = "0x0c - Power control register 4"]
    pub pwr_cr4: PWR_CR4,
    #[doc = "0x10 - Power status register 1"]
    pub pwr_sr1: PWR_SR1,
    #[doc = "0x14 - Power status register 2"]
    pub pwr_sr2: PWR_SR2,
    #[doc = "0x18 - Power status clear register"]
    pub pwr_scr: PWR_SCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Power Port A pull-up control register"]
    pub pwr_pucra: PWR_PUCRA,
    #[doc = "0x24 - Power Port A pull-down control register"]
    pub pwr_pdcra: PWR_PDCRA,
    #[doc = "0x28 - Power Port B pull-up control register"]
    pub pwr_pucrb: PWR_PUCRB,
    #[doc = "0x2c - Power Port B pull-down control register"]
    pub pwr_pdcrb: PWR_PDCRB,
    #[doc = "0x30 - Power Port C pull-up control register"]
    pub pwr_pucrc: PWR_PUCRC,
    #[doc = "0x34 - Power Port C pull-down control register"]
    pub pwr_pdcrc: PWR_PDCRC,
    #[doc = "0x38 - Power Port D pull-up control register"]
    pub pwr_pucrd: PWR_PUCRD,
    #[doc = "0x3c - Power Port D pull-down control register"]
    pub pwr_pdcrd: PWR_PDCRD,
    #[doc = "0x40 - Power Port E pull-up control register"]
    pub pwr_pucre: PWR_PUCRE,
    #[doc = "0x44 - Power Port E pull-down control register"]
    pub pwr_pdcre: PWR_PDCRE,
    #[doc = "0x48 - Power Port F pull-up control register"]
    pub pwr_pucrf: PWR_PUCRF,
    #[doc = "0x4c - Power Port F pull-down control register"]
    pub pwr_pdcrf: PWR_PDCRF,
    #[doc = "0x50 - Power Port G pull-up control register"]
    pub pwr_pucrg: PWR_PUCRG,
    #[doc = "0x54 - Power Port G pull-down control register"]
    pub pwr_pdcrg: PWR_PDCRG,
    _reserved21: [u8; 0x28],
    #[doc = "0x80 - Power control register"]
    pub pwr_cr5: PWR_CR5,
}
#[doc = "PWR_CR1 (rw) register accessor: an alias for `Reg<PWR_CR1_SPEC>`"]
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1_SPEC>;
#[doc = "Power control register 1"]
pub mod pwr_cr1;
#[doc = "PWR_CR2 (rw) register accessor: an alias for `Reg<PWR_CR2_SPEC>`"]
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2_SPEC>;
#[doc = "Power control register 2"]
pub mod pwr_cr2;
#[doc = "PWR_CR3 (rw) register accessor: an alias for `Reg<PWR_CR3_SPEC>`"]
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3_SPEC>;
#[doc = "Power control register 3"]
pub mod pwr_cr3;
#[doc = "PWR_CR4 (rw) register accessor: an alias for `Reg<PWR_CR4_SPEC>`"]
pub type PWR_CR4 = crate::Reg<pwr_cr4::PWR_CR4_SPEC>;
#[doc = "Power control register 4"]
pub mod pwr_cr4;
#[doc = "PWR_SR1 (r) register accessor: an alias for `Reg<PWR_SR1_SPEC>`"]
pub type PWR_SR1 = crate::Reg<pwr_sr1::PWR_SR1_SPEC>;
#[doc = "Power status register 1"]
pub mod pwr_sr1;
#[doc = "PWR_SR2 (r) register accessor: an alias for `Reg<PWR_SR2_SPEC>`"]
pub type PWR_SR2 = crate::Reg<pwr_sr2::PWR_SR2_SPEC>;
#[doc = "Power status register 2"]
pub mod pwr_sr2;
#[doc = "PWR_SCR (w) register accessor: an alias for `Reg<PWR_SCR_SPEC>`"]
pub type PWR_SCR = crate::Reg<pwr_scr::PWR_SCR_SPEC>;
#[doc = "Power status clear register"]
pub mod pwr_scr;
#[doc = "PWR_PUCRA (rw) register accessor: an alias for `Reg<PWR_PUCRA_SPEC>`"]
pub type PWR_PUCRA = crate::Reg<pwr_pucra::PWR_PUCRA_SPEC>;
#[doc = "Power Port A pull-up control register"]
pub mod pwr_pucra;
#[doc = "PWR_PDCRA (rw) register accessor: an alias for `Reg<PWR_PDCRA_SPEC>`"]
pub type PWR_PDCRA = crate::Reg<pwr_pdcra::PWR_PDCRA_SPEC>;
#[doc = "Power Port A pull-down control register"]
pub mod pwr_pdcra;
#[doc = "PWR_PUCRB (rw) register accessor: an alias for `Reg<PWR_PUCRB_SPEC>`"]
pub type PWR_PUCRB = crate::Reg<pwr_pucrb::PWR_PUCRB_SPEC>;
#[doc = "Power Port B pull-up control register"]
pub mod pwr_pucrb;
#[doc = "PWR_PDCRB (rw) register accessor: an alias for `Reg<PWR_PDCRB_SPEC>`"]
pub type PWR_PDCRB = crate::Reg<pwr_pdcrb::PWR_PDCRB_SPEC>;
#[doc = "Power Port B pull-down control register"]
pub mod pwr_pdcrb;
#[doc = "PWR_PUCRC (rw) register accessor: an alias for `Reg<PWR_PUCRC_SPEC>`"]
pub type PWR_PUCRC = crate::Reg<pwr_pucrc::PWR_PUCRC_SPEC>;
#[doc = "Power Port C pull-up control register"]
pub mod pwr_pucrc;
#[doc = "PWR_PDCRC (rw) register accessor: an alias for `Reg<PWR_PDCRC_SPEC>`"]
pub type PWR_PDCRC = crate::Reg<pwr_pdcrc::PWR_PDCRC_SPEC>;
#[doc = "Power Port C pull-down control register"]
pub mod pwr_pdcrc;
#[doc = "PWR_PUCRD (rw) register accessor: an alias for `Reg<PWR_PUCRD_SPEC>`"]
pub type PWR_PUCRD = crate::Reg<pwr_pucrd::PWR_PUCRD_SPEC>;
#[doc = "Power Port D pull-up control register"]
pub mod pwr_pucrd;
#[doc = "PWR_PDCRD (rw) register accessor: an alias for `Reg<PWR_PDCRD_SPEC>`"]
pub type PWR_PDCRD = crate::Reg<pwr_pdcrd::PWR_PDCRD_SPEC>;
#[doc = "Power Port D pull-down control register"]
pub mod pwr_pdcrd;
#[doc = "PWR_PUCRE (rw) register accessor: an alias for `Reg<PWR_PUCRE_SPEC>`"]
pub type PWR_PUCRE = crate::Reg<pwr_pucre::PWR_PUCRE_SPEC>;
#[doc = "Power Port E pull-up control register"]
pub mod pwr_pucre;
#[doc = "PWR_PDCRE (rw) register accessor: an alias for `Reg<PWR_PDCRE_SPEC>`"]
pub type PWR_PDCRE = crate::Reg<pwr_pdcre::PWR_PDCRE_SPEC>;
#[doc = "Power Port E pull-down control register"]
pub mod pwr_pdcre;
#[doc = "PWR_PUCRF (rw) register accessor: an alias for `Reg<PWR_PUCRF_SPEC>`"]
pub type PWR_PUCRF = crate::Reg<pwr_pucrf::PWR_PUCRF_SPEC>;
#[doc = "Power Port F pull-up control register"]
pub mod pwr_pucrf;
#[doc = "PWR_PDCRF (rw) register accessor: an alias for `Reg<PWR_PDCRF_SPEC>`"]
pub type PWR_PDCRF = crate::Reg<pwr_pdcrf::PWR_PDCRF_SPEC>;
#[doc = "Power Port F pull-down control register"]
pub mod pwr_pdcrf;
#[doc = "PWR_PUCRG (rw) register accessor: an alias for `Reg<PWR_PUCRG_SPEC>`"]
pub type PWR_PUCRG = crate::Reg<pwr_pucrg::PWR_PUCRG_SPEC>;
#[doc = "Power Port G pull-up control register"]
pub mod pwr_pucrg;
#[doc = "PWR_PDCRG (rw) register accessor: an alias for `Reg<PWR_PDCRG_SPEC>`"]
pub type PWR_PDCRG = crate::Reg<pwr_pdcrg::PWR_PDCRG_SPEC>;
#[doc = "Power Port G pull-down control register"]
pub mod pwr_pdcrg;
#[doc = "PWR_CR5 (rw) register accessor: an alias for `Reg<PWR_CR5_SPEC>`"]
pub type PWR_CR5 = crate::Reg<pwr_cr5::PWR_CR5_SPEC>;
#[doc = "Power control register"]
pub mod pwr_cr5;
