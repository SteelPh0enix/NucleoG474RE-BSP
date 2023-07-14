#[doc = "Register `FDCAN_CCCR` reader"]
pub struct R(crate::R<FDCAN_CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_CCCR` writer"]
pub struct W(crate::W<FDCAN_CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FDCAN_CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialization"]
pub type INIT_R = crate::BitReader<INIT_A>;
#[doc = "Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Normal operation"]
    B_0X0 = 0,
    #[doc = "1: Initialization started"]
    B_0X1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::B_0X0,
            true => INIT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == INIT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == INIT_A::B_0X1
    }
}
#[doc = "Field `INIT` writer - Initialization"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, INIT_A>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(INIT_A::B_0X0)
    }
    #[doc = "Initialization started"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(INIT_A::B_0X1)
    }
}
#[doc = "Field `CCE` reader - Configuration change enable"]
pub type CCE_R = crate::BitReader<CCE_A>;
#[doc = "Configuration change enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: The CPU has no write access to the protected configuration registers."]
    B_0X0 = 0,
    #[doc = "1: The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    B_0X1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::B_0X0,
            true => CCE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCE_A::B_0X1
    }
}
#[doc = "Field `CCE` writer - Configuration change enable"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, CCE_A>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "The CPU has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCE_A::B_0X0)
    }
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCE_A::B_0X1)
    }
}
#[doc = "Field `ASM` reader - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type ASM_R = crate::BitReader<ASM_A>;
#[doc = "ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASM_A {
    #[doc = "0: Normal CAN operation"]
    B_0X0 = 0,
    #[doc = "1: Restricted operation Mode active"]
    B_0X1 = 1,
}
impl From<ASM_A> for bool {
    #[inline(always)]
    fn from(variant: ASM_A) -> Self {
        variant as u8 != 0
    }
}
impl ASM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASM_A {
        match self.bits {
            false => ASM_A::B_0X0,
            true => ASM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ASM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ASM_A::B_0X1
    }
}
#[doc = "Field `ASM` writer - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, ASM_A>;
impl<'a, const O: u8> ASM_W<'a, O> {
    #[doc = "Normal CAN operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ASM_A::B_0X0)
    }
    #[doc = "Restricted operation Mode active"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ASM_A::B_0X1)
    }
}
#[doc = "Field `CSA` reader - Clock stop acknowledge"]
pub type CSA_R = crate::BitReader<CSA_A>;
#[doc = "Clock stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSA_A {
    #[doc = "0: No clock stop acknowledged"]
    B_0X0 = 0,
    #[doc = "1: FDCAN may be set in power down by stopping APB clock and kernel clock."]
    B_0X1 = 1,
}
impl From<CSA_A> for bool {
    #[inline(always)]
    fn from(variant: CSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSA_A {
        match self.bits {
            false => CSA_A::B_0X0,
            true => CSA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSA_A::B_0X1
    }
}
#[doc = "Field `CSR` reader - Clock stop request"]
pub type CSR_R = crate::BitReader<CSR_A>;
#[doc = "Clock stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR_A {
    #[doc = "0: No clock stop requested"]
    B_0X0 = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    B_0X1 = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::B_0X0,
            true => CSR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSR_A::B_0X1
    }
}
#[doc = "Field `CSR` writer - Clock stop request"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, CSR_A>;
impl<'a, const O: u8> CSR_W<'a, O> {
    #[doc = "No clock stop requested"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSR_A::B_0X0)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSR_A::B_0X1)
    }
}
#[doc = "Field `MON` reader - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MON_R = crate::BitReader<MON_A>;
#[doc = "Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MON_A {
    #[doc = "0: Bus monitoring mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Bus monitoring mode enabled"]
    B_0X1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
impl MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::B_0X0,
            true => MON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MON_A::B_0X1
    }
}
#[doc = "Field `MON` writer - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, MON_A>;
impl<'a, const O: u8> MON_W<'a, O> {
    #[doc = "Bus monitoring mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MON_A::B_0X0)
    }
    #[doc = "Bus monitoring mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MON_A::B_0X1)
    }
}
#[doc = "Field `DAR` reader - Disable automatic retransmission"]
pub type DAR_R = crate::BitReader<DAR_A>;
#[doc = "Disable automatic retransmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAR_A {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled"]
    B_0X0 = 0,
    #[doc = "1: Automatic retransmission disabled"]
    B_0X1 = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
impl DAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::B_0X0,
            true => DAR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAR_A::B_0X1
    }
}
#[doc = "Field `DAR` writer - Disable automatic retransmission"]
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, DAR_A>;
impl<'a, const O: u8> DAR_W<'a, O> {
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAR_A::B_0X0)
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAR_A::B_0X1)
    }
}
#[doc = "Field `TEST` reader - Test mode enable"]
pub type TEST_R = crate::BitReader<TEST_A>;
#[doc = "Test mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEST_A {
    #[doc = "0: Normal operation, register TEST holds reset values"]
    B_0X0 = 0,
    #[doc = "1: Test Mode, write access to register TEST enabled"]
    B_0X1 = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
impl TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::B_0X0,
            true => TEST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEST_A::B_0X1
    }
}
#[doc = "Field `TEST` writer - Test mode enable"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, TEST_A>;
impl<'a, const O: u8> TEST_W<'a, O> {
    #[doc = "Normal operation, register TEST holds reset values"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEST_A::B_0X0)
    }
    #[doc = "Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEST_A::B_0X1)
    }
}
#[doc = "Field `FDOE` reader - FD operation enable"]
pub type FDOE_R = crate::BitReader<FDOE_A>;
#[doc = "FD operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDOE_A {
    #[doc = "0: FD operation disabled"]
    B_0X0 = 0,
    #[doc = "1: FD operation enabled"]
    B_0X1 = 1,
}
impl From<FDOE_A> for bool {
    #[inline(always)]
    fn from(variant: FDOE_A) -> Self {
        variant as u8 != 0
    }
}
impl FDOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDOE_A {
        match self.bits {
            false => FDOE_A::B_0X0,
            true => FDOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDOE_A::B_0X1
    }
}
#[doc = "Field `FDOE` writer - FD operation enable"]
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, FDOE_A>;
impl<'a, const O: u8> FDOE_W<'a, O> {
    #[doc = "FD operation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDOE_A::B_0X0)
    }
    #[doc = "FD operation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDOE_A::B_0X1)
    }
}
#[doc = "Field `BRSE` reader - FDCAN bit rate switching"]
pub type BRSE_R = crate::BitReader<BRSE_A>;
#[doc = "FDCAN bit rate switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSE_A {
    #[doc = "0: Bit rate switching for transmissions disabled"]
    B_0X0 = 0,
    #[doc = "1: Bit rate switching for transmissions enabled"]
    B_0X1 = 1,
}
impl From<BRSE_A> for bool {
    #[inline(always)]
    fn from(variant: BRSE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSE_A {
        match self.bits {
            false => BRSE_A::B_0X0,
            true => BRSE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRSE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRSE_A::B_0X1
    }
}
#[doc = "Field `BRSE` writer - FDCAN bit rate switching"]
pub type BRSE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, BRSE_A>;
impl<'a, const O: u8> BRSE_W<'a, O> {
    #[doc = "Bit rate switching for transmissions disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRSE_A::B_0X0)
    }
    #[doc = "Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRSE_A::B_0X1)
    }
}
#[doc = "Field `PXHD` reader - Protocol exception handling disable"]
pub type PXHD_R = crate::BitReader<PXHD_A>;
#[doc = "Protocol exception handling disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PXHD_A {
    #[doc = "0: Protocol exception handling enabled"]
    B_0X0 = 0,
    #[doc = "1: Protocol exception handling disabled"]
    B_0X1 = 1,
}
impl From<PXHD_A> for bool {
    #[inline(always)]
    fn from(variant: PXHD_A) -> Self {
        variant as u8 != 0
    }
}
impl PXHD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PXHD_A {
        match self.bits {
            false => PXHD_A::B_0X0,
            true => PXHD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PXHD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PXHD_A::B_0X1
    }
}
#[doc = "Field `PXHD` writer - Protocol exception handling disable"]
pub type PXHD_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, PXHD_A>;
impl<'a, const O: u8> PXHD_W<'a, O> {
    #[doc = "Protocol exception handling enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PXHD_A::B_0X0)
    }
    #[doc = "Protocol exception handling disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PXHD_A::B_0X1)
    }
}
#[doc = "Field `EFBI` reader - Edge filtering during bus integration"]
pub type EFBI_R = crate::BitReader<EFBI_A>;
#[doc = "Edge filtering during bus integration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFBI_A {
    #[doc = "0: Edge filtering disabled"]
    B_0X0 = 0,
    #[doc = "1: Two consecutive dominant tq required to detect an edge for hard synchronization"]
    B_0X1 = 1,
}
impl From<EFBI_A> for bool {
    #[inline(always)]
    fn from(variant: EFBI_A) -> Self {
        variant as u8 != 0
    }
}
impl EFBI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFBI_A {
        match self.bits {
            false => EFBI_A::B_0X0,
            true => EFBI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EFBI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EFBI_A::B_0X1
    }
}
#[doc = "Field `EFBI` writer - Edge filtering during bus integration"]
pub type EFBI_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, EFBI_A>;
impl<'a, const O: u8> EFBI_W<'a, O> {
    #[doc = "Edge filtering disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EFBI_A::B_0X0)
    }
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EFBI_A::B_0X1)
    }
}
#[doc = "Field `TXP` reader - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TXP_R = crate::BitReader<TXP_A>;
#[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
impl TXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::B_0X0,
            true => TXP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXP_A::B_0X1
    }
}
#[doc = "Field `TXP` writer - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, TXP_A>;
impl<'a, const O: u8> TXP_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXP_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXP_A::B_0X1)
    }
}
#[doc = "Field `NISO` reader - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NISO_R = crate::BitReader<NISO_A>;
#[doc = "Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISO_A {
    #[doc = "0: CAN FD frame format according to ISO11898-1"]
    B_0X0 = 0,
    #[doc = "1: CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    B_0X1 = 1,
}
impl From<NISO_A> for bool {
    #[inline(always)]
    fn from(variant: NISO_A) -> Self {
        variant as u8 != 0
    }
}
impl NISO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NISO_A {
        match self.bits {
            false => NISO_A::B_0X0,
            true => NISO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NISO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NISO_A::B_0X1
    }
}
#[doc = "Field `NISO` writer - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NISO_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_CCCR_SPEC, O, NISO_A>;
impl<'a, const O: u8> NISO_W<'a, O> {
    #[doc = "CAN FD frame format according to ISO11898-1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NISO_A::B_0X0)
    }
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NISO_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock stop acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<5> {
        MON_W::new(self)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<8> {
        FDOE_W::new(self)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<9> {
        BRSE_W::new(self)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<12> {
        PXHD_W::new(self)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<13> {
        EFBI_W::new(self)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<15> {
        NISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN CC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_cccr](index.html) module"]
pub struct FDCAN_CCCR_SPEC;
impl crate::RegisterSpec for FDCAN_CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_cccr::R](R) reader structure"]
impl crate::Readable for FDCAN_CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_cccr::W](W) writer structure"]
impl crate::Writable for FDCAN_CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_CCCR to value 0x01"]
impl crate::Resettable for FDCAN_CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
