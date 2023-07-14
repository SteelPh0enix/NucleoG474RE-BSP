#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - AConfiguration register 1"]
    pub acr1: ACR1,
    #[doc = "0x08 - AConfiguration register 2"]
    pub acr2: ACR2,
    #[doc = "0x0c - AFRCR"]
    pub afrcr: AFRCR,
    #[doc = "0x10 - ASlot register"]
    pub aslotr: ASLOTR,
    #[doc = "0x14 - AInterrupt mask register2"]
    pub aim: AIM,
    #[doc = "0x18 - AStatus register"]
    pub asr: ASR,
    #[doc = "0x1c - AClear flag register"]
    pub aclrfr: ACLRFR,
    #[doc = "0x20 - AData register"]
    pub adr: ADR,
    #[doc = "0x24 - BConfiguration register 1"]
    pub bcr1: BCR1,
    #[doc = "0x28 - BConfiguration register 2"]
    pub bcr2: BCR2,
    #[doc = "0x2c - BFRCR"]
    pub bfrcr: BFRCR,
    #[doc = "0x30 - BSlot register"]
    pub bslotr: BSLOTR,
    #[doc = "0x34 - BInterrupt mask register2"]
    pub bim: BIM,
    #[doc = "0x38 - BStatus register"]
    pub bsr: BSR,
    #[doc = "0x3c - BClear flag register"]
    pub bclrfr: BCLRFR,
    #[doc = "0x40 - BData register"]
    pub bdr: BDR,
    #[doc = "0x44 - PDM control register"]
    pub pdmcr: PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub pdmdly: PDMDLY,
}
#[doc = "BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "BConfiguration register 1"]
pub mod bcr1;
#[doc = "BCR2 (rw) register accessor: an alias for `Reg<BCR2_SPEC>`"]
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
#[doc = "BConfiguration register 2"]
pub mod bcr2;
#[doc = "BFRCR (rw) register accessor: an alias for `Reg<BFRCR_SPEC>`"]
pub type BFRCR = crate::Reg<bfrcr::BFRCR_SPEC>;
#[doc = "BFRCR"]
pub mod bfrcr;
#[doc = "BSLOTR (rw) register accessor: an alias for `Reg<BSLOTR_SPEC>`"]
pub type BSLOTR = crate::Reg<bslotr::BSLOTR_SPEC>;
#[doc = "BSlot register"]
pub mod bslotr;
#[doc = "BIM (rw) register accessor: an alias for `Reg<BIM_SPEC>`"]
pub type BIM = crate::Reg<bim::BIM_SPEC>;
#[doc = "BInterrupt mask register2"]
pub mod bim;
#[doc = "BSR (r) register accessor: an alias for `Reg<BSR_SPEC>`"]
pub type BSR = crate::Reg<bsr::BSR_SPEC>;
#[doc = "BStatus register"]
pub mod bsr;
#[doc = "BCLRFR (w) register accessor: an alias for `Reg<BCLRFR_SPEC>`"]
pub type BCLRFR = crate::Reg<bclrfr::BCLRFR_SPEC>;
#[doc = "BClear flag register"]
pub mod bclrfr;
#[doc = "BDR (rw) register accessor: an alias for `Reg<BDR_SPEC>`"]
pub type BDR = crate::Reg<bdr::BDR_SPEC>;
#[doc = "BData register"]
pub mod bdr;
#[doc = "ACR1 (rw) register accessor: an alias for `Reg<ACR1_SPEC>`"]
pub type ACR1 = crate::Reg<acr1::ACR1_SPEC>;
#[doc = "AConfiguration register 1"]
pub mod acr1;
#[doc = "ACR2 (rw) register accessor: an alias for `Reg<ACR2_SPEC>`"]
pub type ACR2 = crate::Reg<acr2::ACR2_SPEC>;
#[doc = "AConfiguration register 2"]
pub mod acr2;
#[doc = "AFRCR (rw) register accessor: an alias for `Reg<AFRCR_SPEC>`"]
pub type AFRCR = crate::Reg<afrcr::AFRCR_SPEC>;
#[doc = "AFRCR"]
pub mod afrcr;
#[doc = "ASLOTR (rw) register accessor: an alias for `Reg<ASLOTR_SPEC>`"]
pub type ASLOTR = crate::Reg<aslotr::ASLOTR_SPEC>;
#[doc = "ASlot register"]
pub mod aslotr;
#[doc = "AIM (rw) register accessor: an alias for `Reg<AIM_SPEC>`"]
pub type AIM = crate::Reg<aim::AIM_SPEC>;
#[doc = "AInterrupt mask register2"]
pub mod aim;
#[doc = "ASR (rw) register accessor: an alias for `Reg<ASR_SPEC>`"]
pub type ASR = crate::Reg<asr::ASR_SPEC>;
#[doc = "AStatus register"]
pub mod asr;
#[doc = "ACLRFR (rw) register accessor: an alias for `Reg<ACLRFR_SPEC>`"]
pub type ACLRFR = crate::Reg<aclrfr::ACLRFR_SPEC>;
#[doc = "AClear flag register"]
pub mod aclrfr;
#[doc = "ADR (rw) register accessor: an alias for `Reg<ADR_SPEC>`"]
pub type ADR = crate::Reg<adr::ADR_SPEC>;
#[doc = "AData register"]
pub mod adr;
#[doc = "PDMCR (rw) register accessor: an alias for `Reg<PDMCR_SPEC>`"]
pub type PDMCR = crate::Reg<pdmcr::PDMCR_SPEC>;
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDMDLY (rw) register accessor: an alias for `Reg<PDMDLY_SPEC>`"]
pub type PDMDLY = crate::Reg<pdmdly::PDMDLY_SPEC>;
#[doc = "PDM delay register"]
pub mod pdmdly;
