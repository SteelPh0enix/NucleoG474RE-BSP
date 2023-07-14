#[doc = "Register `FDCAN_IE` reader"]
pub struct R(crate::R<FDCAN_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_IE` writer"]
pub struct W(crate::W<FDCAN_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_IE_SPEC>;
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
impl From<crate::W<FDCAN_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable"]
pub type RF0NE_R = crate::BitReader<RF0NE_A>;
#[doc = "Rx FIFO 0 new message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0NE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<RF0NE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0NE_A) -> Self {
        variant as u8 != 0
    }
}
impl RF0NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0NE_A {
        match self.bits {
            false => RF0NE_A::B_0X0,
            true => RF0NE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0NE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0NE_A::B_0X1
    }
}
#[doc = "Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable"]
pub type RF0NE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, RF0NE_A>;
impl<'a, const O: u8> RF0NE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0NE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0NE_A::B_0X1)
    }
}
#[doc = "Field `RF0FE` reader - Rx FIFO 0 full interrupt enable"]
pub type RF0FE_R = crate::BitReader<RF0FE_A>;
#[doc = "Rx FIFO 0 full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0FE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<RF0FE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0FE_A) -> Self {
        variant as u8 != 0
    }
}
impl RF0FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0FE_A {
        match self.bits {
            false => RF0FE_A::B_0X0,
            true => RF0FE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0FE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0FE_A::B_0X1
    }
}
#[doc = "Field `RF0FE` writer - Rx FIFO 0 full interrupt enable"]
pub type RF0FE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, RF0FE_A>;
impl<'a, const O: u8> RF0FE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0FE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0FE_A::B_0X1)
    }
}
#[doc = "Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable"]
pub type RF0LE_R = crate::BitReader<RF0LE_A>;
#[doc = "Rx FIFO 0 message lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0LE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<RF0LE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0LE_A) -> Self {
        variant as u8 != 0
    }
}
impl RF0LE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0LE_A {
        match self.bits {
            false => RF0LE_A::B_0X0,
            true => RF0LE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0LE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0LE_A::B_0X1
    }
}
#[doc = "Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable"]
pub type RF0LE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, RF0LE_A>;
impl<'a, const O: u8> RF0LE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0LE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0LE_A::B_0X1)
    }
}
#[doc = "Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable"]
pub type RF1NE_R = crate::BitReader<RF1NE_A>;
#[doc = "Rx FIFO 1 new message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1NE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<RF1NE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1NE_A) -> Self {
        variant as u8 != 0
    }
}
impl RF1NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1NE_A {
        match self.bits {
            false => RF1NE_A::B_0X0,
            true => RF1NE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1NE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1NE_A::B_0X1
    }
}
#[doc = "Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable"]
pub type RF1NE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, RF1NE_A>;
impl<'a, const O: u8> RF1NE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1NE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1NE_A::B_0X1)
    }
}
#[doc = "Field `RF1FE` reader - Rx FIFO 1 full interrupt enable"]
pub type RF1FE_R = crate::BitReader<RF1FE_A>;
#[doc = "Rx FIFO 1 full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1FE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<RF1FE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1FE_A) -> Self {
        variant as u8 != 0
    }
}
impl RF1FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1FE_A {
        match self.bits {
            false => RF1FE_A::B_0X0,
            true => RF1FE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1FE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1FE_A::B_0X1
    }
}
#[doc = "Field `RF1FE` writer - Rx FIFO 1 full interrupt enable"]
pub type RF1FE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, RF1FE_A>;
impl<'a, const O: u8> RF1FE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1FE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1FE_A::B_0X1)
    }
}
#[doc = "Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable"]
pub type RF1LE_R = crate::BitReader<RF1LE_A>;
#[doc = "Rx FIFO 1 message lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1LE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<RF1LE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1LE_A) -> Self {
        variant as u8 != 0
    }
}
impl RF1LE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1LE_A {
        match self.bits {
            false => RF1LE_A::B_0X0,
            true => RF1LE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1LE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1LE_A::B_0X1
    }
}
#[doc = "Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable"]
pub type RF1LE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, RF1LE_A>;
impl<'a, const O: u8> RF1LE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1LE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1LE_A::B_0X1)
    }
}
#[doc = "Field `HPME` reader - High-priority message interrupt enable"]
pub type HPME_R = crate::BitReader<HPME_A>;
#[doc = "High-priority message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPME_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<HPME_A> for bool {
    #[inline(always)]
    fn from(variant: HPME_A) -> Self {
        variant as u8 != 0
    }
}
impl HPME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPME_A {
        match self.bits {
            false => HPME_A::B_0X0,
            true => HPME_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HPME_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HPME_A::B_0X1
    }
}
#[doc = "Field `HPME` writer - High-priority message interrupt enable"]
pub type HPME_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, HPME_A>;
impl<'a, const O: u8> HPME_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HPME_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HPME_A::B_0X1)
    }
}
#[doc = "Field `TCE` reader - Transmission completed interrupt enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
#[doc = "Transmission completed interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::B_0X0,
            true => TCE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCE_A::B_0X1
    }
}
#[doc = "Field `TCE` writer - Transmission completed interrupt enable"]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TCE_A>;
impl<'a, const O: u8> TCE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCE_A::B_0X1)
    }
}
#[doc = "Field `TCFE` reader - Transmission cancellation finished interrupt enable"]
pub type TCFE_R = crate::BitReader<TCFE_A>;
#[doc = "Transmission cancellation finished interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TCFE_A> for bool {
    #[inline(always)]
    fn from(variant: TCFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFE_A {
        match self.bits {
            false => TCFE_A::B_0X0,
            true => TCFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCFE_A::B_0X1
    }
}
#[doc = "Field `TCFE` writer - Transmission cancellation finished interrupt enable"]
pub type TCFE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TCFE_A>;
impl<'a, const O: u8> TCFE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCFE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCFE_A::B_0X1)
    }
}
#[doc = "Field `TFEE` reader - Tx FIFO empty interrupt enable"]
pub type TFEE_R = crate::BitReader<TFEE_A>;
#[doc = "Tx FIFO empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFEE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TFEE_A> for bool {
    #[inline(always)]
    fn from(variant: TFEE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFEE_A {
        match self.bits {
            false => TFEE_A::B_0X0,
            true => TFEE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFEE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFEE_A::B_0X1
    }
}
#[doc = "Field `TFEE` writer - Tx FIFO empty interrupt enable"]
pub type TFEE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TFEE_A>;
impl<'a, const O: u8> TFEE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TFEE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TFEE_A::B_0X1)
    }
}
#[doc = "Field `TEFNE` reader - Tx event FIFO new entry interrupt enable"]
pub type TEFNE_R = crate::BitReader<TEFNE_A>;
#[doc = "Tx event FIFO new entry interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFNE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TEFNE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFNE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEFNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFNE_A {
        match self.bits {
            false => TEFNE_A::B_0X0,
            true => TEFNE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFNE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFNE_A::B_0X1
    }
}
#[doc = "Field `TEFNE` writer - Tx event FIFO new entry interrupt enable"]
pub type TEFNE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TEFNE_A>;
impl<'a, const O: u8> TEFNE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFNE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFNE_A::B_0X1)
    }
}
#[doc = "Field `TEFFE` reader - Tx event FIFO full interrupt enable"]
pub type TEFFE_R = crate::BitReader<TEFFE_A>;
#[doc = "Tx event FIFO full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFFE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TEFFE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEFFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFFE_A {
        match self.bits {
            false => TEFFE_A::B_0X0,
            true => TEFFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFFE_A::B_0X1
    }
}
#[doc = "Field `TEFFE` writer - Tx event FIFO full interrupt enable"]
pub type TEFFE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TEFFE_A>;
impl<'a, const O: u8> TEFFE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFFE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFFE_A::B_0X1)
    }
}
#[doc = "Field `TEFLE` reader - Tx event FIFO element lost interrupt enable"]
pub type TEFLE_R = crate::BitReader<TEFLE_A>;
#[doc = "Tx event FIFO element lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFLE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TEFLE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEFLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFLE_A {
        match self.bits {
            false => TEFLE_A::B_0X0,
            true => TEFLE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFLE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFLE_A::B_0X1
    }
}
#[doc = "Field `TEFLE` writer - Tx event FIFO element lost interrupt enable"]
pub type TEFLE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TEFLE_A>;
impl<'a, const O: u8> TEFLE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFLE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFLE_A::B_0X1)
    }
}
#[doc = "Field `TSWE` reader - Timestamp wraparound interrupt enable"]
pub type TSWE_R = crate::BitReader<TSWE_A>;
#[doc = "Timestamp wraparound interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSWE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TSWE_A> for bool {
    #[inline(always)]
    fn from(variant: TSWE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSWE_A {
        match self.bits {
            false => TSWE_A::B_0X0,
            true => TSWE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSWE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSWE_A::B_0X1
    }
}
#[doc = "Field `TSWE` writer - Timestamp wraparound interrupt enable"]
pub type TSWE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TSWE_A>;
impl<'a, const O: u8> TSWE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSWE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSWE_A::B_0X1)
    }
}
#[doc = "Field `MRAFE` reader - Message RAM access failure interrupt enable"]
pub type MRAFE_R = crate::BitReader<MRAFE_A>;
#[doc = "Message RAM access failure interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRAFE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<MRAFE_A> for bool {
    #[inline(always)]
    fn from(variant: MRAFE_A) -> Self {
        variant as u8 != 0
    }
}
impl MRAFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRAFE_A {
        match self.bits {
            false => MRAFE_A::B_0X0,
            true => MRAFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRAFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRAFE_A::B_0X1
    }
}
#[doc = "Field `MRAFE` writer - Message RAM access failure interrupt enable"]
pub type MRAFE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, MRAFE_A>;
impl<'a, const O: u8> MRAFE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MRAFE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MRAFE_A::B_0X1)
    }
}
#[doc = "Field `TOOE` reader - Timeout occurred interrupt enable"]
pub type TOOE_R = crate::BitReader<TOOE_A>;
#[doc = "Timeout occurred interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOOE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<TOOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOOE_A {
        match self.bits {
            false => TOOE_A::B_0X0,
            true => TOOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOOE_A::B_0X1
    }
}
#[doc = "Field `TOOE` writer - Timeout occurred interrupt enable"]
pub type TOOE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, TOOE_A>;
impl<'a, const O: u8> TOOE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOOE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOOE_A::B_0X1)
    }
}
#[doc = "Field `ELOE` reader - Error logging overflow interrupt enable"]
pub type ELOE_R = crate::BitReader<ELOE_A>;
#[doc = "Error logging overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELOE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<ELOE_A> for bool {
    #[inline(always)]
    fn from(variant: ELOE_A) -> Self {
        variant as u8 != 0
    }
}
impl ELOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOE_A {
        match self.bits {
            false => ELOE_A::B_0X0,
            true => ELOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ELOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ELOE_A::B_0X1
    }
}
#[doc = "Field `ELOE` writer - Error logging overflow interrupt enable"]
pub type ELOE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, ELOE_A>;
impl<'a, const O: u8> ELOE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ELOE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ELOE_A::B_0X1)
    }
}
#[doc = "Field `EPE` reader - Error passive interrupt enable"]
pub type EPE_R = crate::BitReader<EPE_A>;
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<EPE_A> for bool {
    #[inline(always)]
    fn from(variant: EPE_A) -> Self {
        variant as u8 != 0
    }
}
impl EPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPE_A {
        match self.bits {
            false => EPE_A::B_0X0,
            true => EPE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EPE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EPE_A::B_0X1
    }
}
#[doc = "Field `EPE` writer - Error passive interrupt enable"]
pub type EPE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, EPE_A>;
impl<'a, const O: u8> EPE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EPE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EPE_A::B_0X1)
    }
}
#[doc = "Field `EWE` reader - Warning status interrupt enable"]
pub type EWE_R = crate::BitReader<EWE_A>;
#[doc = "Warning status interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<EWE_A> for bool {
    #[inline(always)]
    fn from(variant: EWE_A) -> Self {
        variant as u8 != 0
    }
}
impl EWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWE_A {
        match self.bits {
            false => EWE_A::B_0X0,
            true => EWE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EWE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EWE_A::B_0X1
    }
}
#[doc = "Field `EWE` writer - Warning status interrupt enable"]
pub type EWE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, EWE_A>;
impl<'a, const O: u8> EWE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EWE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EWE_A::B_0X1)
    }
}
#[doc = "Field `BOE` reader - Bus_Off status"]
pub type BOE_R = crate::BitReader<BOE_A>;
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<BOE_A> for bool {
    #[inline(always)]
    fn from(variant: BOE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOE_A {
        match self.bits {
            false => BOE_A::B_0X0,
            true => BOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BOE_A::B_0X1
    }
}
#[doc = "Field `BOE` writer - Bus_Off status"]
pub type BOE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, BOE_A>;
impl<'a, const O: u8> BOE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BOE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BOE_A::B_0X1)
    }
}
#[doc = "Field `WDIE` reader - Watchdog interrupt enable"]
pub type WDIE_R = crate::BitReader<WDIE_A>;
#[doc = "Watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDIE_A {
    #[doc = "0: Interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0X1 = 1,
}
impl From<WDIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDIE_A {
        match self.bits {
            false => WDIE_A::B_0X0,
            true => WDIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDIE_A::B_0X1
    }
}
#[doc = "Field `WDIE` writer - Watchdog interrupt enable"]
pub type WDIE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O, WDIE_A>;
impl<'a, const O: u8> WDIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDIE_A::B_0X0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDIE_A::B_0X1)
    }
}
#[doc = "Field `PEAE` reader - Protocol error in arbitration phase enable"]
pub type PEAE_R = crate::BitReader;
#[doc = "Field `PEAE` writer - Protocol error in arbitration phase enable"]
pub type PEAE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O>;
#[doc = "Field `PEDE` reader - Protocol error in data phase enable"]
pub type PEDE_R = crate::BitReader;
#[doc = "Field `PEDE` writer - Protocol error in data phase enable"]
pub type PEDE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O>;
#[doc = "Field `ARAE` reader - Access to reserved address enable"]
pub type ARAE_R = crate::BitReader;
#[doc = "Field `ARAE` writer - Access to reserved address enable"]
pub type ARAE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> RF0NE_W<0> {
        RF0NE_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> RF0FE_W<1> {
        RF0FE_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> RF0LE_W<2> {
        RF0LE_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> RF1NE_W<3> {
        RF1NE_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> RF1FE_W<4> {
        RF1FE_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> RF1LE_W<5> {
        RF1LE_W::new(self)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HPME_W<6> {
        HPME_W::new(self)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<7> {
        TCE_W::new(self)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<8> {
        TCFE_W::new(self)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfee(&mut self) -> TFEE_W<9> {
        TFEE_W::new(self)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TEFNE_W<10> {
        TEFNE_W::new(self)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TEFFE_W<11> {
        TEFFE_W::new(self)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TEFLE_W<12> {
        TEFLE_W::new(self)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TSWE_W<13> {
        TSWE_W::new(self)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MRAFE_W<14> {
        MRAFE_W::new(self)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TOOE_W<15> {
        TOOE_W::new(self)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> ELOE_W<16> {
        ELOE_W::new(self)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<17> {
        EPE_W::new(self)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EWE_W<18> {
        EWE_W::new(self)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BOE_W<19> {
        BOE_W::new(self)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<20> {
        WDIE_W::new(self)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PEAE_W<21> {
        PEAE_W::new(self)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PEDE_W<22> {
        PEDE_W::new(self)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> ARAE_W<23> {
        ARAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ie](index.html) module"]
pub struct FDCAN_IE_SPEC;
impl crate::RegisterSpec for FDCAN_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ie::R](R) reader structure"]
impl crate::Readable for FDCAN_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ie::W](W) writer structure"]
impl crate::Writable for FDCAN_IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_IE to value 0"]
impl crate::Resettable for FDCAN_IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
