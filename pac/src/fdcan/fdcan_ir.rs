#[doc = "Register `FDCAN_IR` reader"]
pub struct R(crate::R<FDCAN_IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_IR` writer"]
pub struct W(crate::W<FDCAN_IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_IR_SPEC>;
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
impl From<crate::W<FDCAN_IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 new message"]
pub type RF0N_R = crate::BitReader<RF0N_A>;
#[doc = "Rx FIFO 0 new message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0N_A {
    #[doc = "0: No new message written to Rx FIFO 0"]
    B_0X0 = 0,
    #[doc = "1: New message written to Rx FIFO 0"]
    B_0X1 = 1,
}
impl From<RF0N_A> for bool {
    #[inline(always)]
    fn from(variant: RF0N_A) -> Self {
        variant as u8 != 0
    }
}
impl RF0N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0N_A {
        match self.bits {
            false => RF0N_A::B_0X0,
            true => RF0N_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0N_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0N_A::B_0X1
    }
}
#[doc = "Field `RF0N` writer - Rx FIFO 0 new message"]
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, RF0N_A>;
impl<'a, const O: u8> RF0N_W<'a, O> {
    #[doc = "No new message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0N_A::B_0X0)
    }
    #[doc = "New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0N_A::B_0X1)
    }
}
#[doc = "Field `RF0F` reader - Rx FIFO 0 full"]
pub type RF0F_R = crate::BitReader<RF0F_A>;
#[doc = "Rx FIFO 0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0F_A {
    #[doc = "0: Rx FIFO 0 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B_0X1 = 1,
}
impl From<RF0F_A> for bool {
    #[inline(always)]
    fn from(variant: RF0F_A) -> Self {
        variant as u8 != 0
    }
}
impl RF0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0F_A {
        match self.bits {
            false => RF0F_A::B_0X0,
            true => RF0F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0F_A::B_0X1
    }
}
#[doc = "Field `RF0F` writer - Rx FIFO 0 full"]
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, RF0F_A>;
impl<'a, const O: u8> RF0F_W<'a, O> {
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0F_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0F_A::B_0X1)
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost"]
pub type RF0L_R = crate::BitReader<RF0L_A>;
#[doc = "Rx FIFO 0 message lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0L_A {
    #[doc = "0: No Rx FIFO 0 message lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 message lost"]
    B_0X1 = 1,
}
impl From<RF0L_A> for bool {
    #[inline(always)]
    fn from(variant: RF0L_A) -> Self {
        variant as u8 != 0
    }
}
impl RF0L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0L_A {
        match self.bits {
            false => RF0L_A::B_0X0,
            true => RF0L_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0L_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0L_A::B_0X1
    }
}
#[doc = "Field `RF0L` writer - Rx FIFO 0 message lost"]
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, RF0L_A>;
impl<'a, const O: u8> RF0L_W<'a, O> {
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X1)
    }
}
#[doc = "Field `RF1N` reader - Rx FIFO 1 new message"]
pub type RF1N_R = crate::BitReader<RF1N_A>;
#[doc = "Rx FIFO 1 new message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1N_A {
    #[doc = "0: No new message written to Rx FIFO 1"]
    B_0X0 = 0,
    #[doc = "1: New message written to Rx FIFO 1"]
    B_0X1 = 1,
}
impl From<RF1N_A> for bool {
    #[inline(always)]
    fn from(variant: RF1N_A) -> Self {
        variant as u8 != 0
    }
}
impl RF1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1N_A {
        match self.bits {
            false => RF1N_A::B_0X0,
            true => RF1N_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1N_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1N_A::B_0X1
    }
}
#[doc = "Field `RF1N` writer - Rx FIFO 1 new message"]
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, RF1N_A>;
impl<'a, const O: u8> RF1N_W<'a, O> {
    #[doc = "No new message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1N_A::B_0X0)
    }
    #[doc = "New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1N_A::B_0X1)
    }
}
#[doc = "Field `RF1F` reader - Rx FIFO 1 full"]
pub type RF1F_R = crate::BitReader<RF1F_A>;
#[doc = "Rx FIFO 1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1F_A {
    #[doc = "0: Rx FIFO 1 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B_0X1 = 1,
}
impl From<RF1F_A> for bool {
    #[inline(always)]
    fn from(variant: RF1F_A) -> Self {
        variant as u8 != 0
    }
}
impl RF1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1F_A {
        match self.bits {
            false => RF1F_A::B_0X0,
            true => RF1F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1F_A::B_0X1
    }
}
#[doc = "Field `RF1F` writer - Rx FIFO 1 full"]
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, RF1F_A>;
impl<'a, const O: u8> RF1F_W<'a, O> {
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1F_A::B_0X0)
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1F_A::B_0X1)
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost"]
pub type RF1L_R = crate::BitReader<RF1L_A>;
#[doc = "Rx FIFO 1 message lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1L_A {
    #[doc = "0: No Rx FIFO 1 message lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 message lost"]
    B_0X1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
impl RF1L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0X0,
            true => RF1L_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1L_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1L_A::B_0X1
    }
}
#[doc = "Field `RF1L` writer - Rx FIFO 1 message lost"]
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, RF1L_A>;
impl<'a, const O: u8> RF1L_W<'a, O> {
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1L_A::B_0X0)
    }
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1L_A::B_0X1)
    }
}
#[doc = "Field `HPM` reader - High-priority message"]
pub type HPM_R = crate::BitReader<HPM_A>;
#[doc = "High-priority message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPM_A {
    #[doc = "0: No high-priority message received"]
    B_0X0 = 0,
    #[doc = "1: High-priority message received"]
    B_0X1 = 1,
}
impl From<HPM_A> for bool {
    #[inline(always)]
    fn from(variant: HPM_A) -> Self {
        variant as u8 != 0
    }
}
impl HPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPM_A {
        match self.bits {
            false => HPM_A::B_0X0,
            true => HPM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HPM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HPM_A::B_0X1
    }
}
#[doc = "Field `HPM` writer - High-priority message"]
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, HPM_A>;
impl<'a, const O: u8> HPM_W<'a, O> {
    #[doc = "No high-priority message received"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HPM_A::B_0X0)
    }
    #[doc = "High-priority message received"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HPM_A::B_0X1)
    }
}
#[doc = "Field `TC` reader - Transmission completed"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transmission completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: No transmission completed"]
    B_0X0 = 0,
    #[doc = "1: Transmission completed"]
    B_0X1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::B_0X0,
            true => TC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TC_A::B_0X1
    }
}
#[doc = "Field `TC` writer - Transmission completed"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TC_A>;
impl<'a, const O: u8> TC_W<'a, O> {
    #[doc = "No transmission completed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TC_A::B_0X0)
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TC_A::B_0X1)
    }
}
#[doc = "Field `TCF` reader - Transmission cancellation finished"]
pub type TCF_R = crate::BitReader<TCF_A>;
#[doc = "Transmission cancellation finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: No transmission cancellation finished"]
    B_0X0 = 0,
    #[doc = "1: Transmission cancellation finished"]
    B_0X1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::B_0X0,
            true => TCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCF_A::B_0X1
    }
}
#[doc = "Field `TCF` writer - Transmission cancellation finished"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TCF_A>;
impl<'a, const O: u8> TCF_W<'a, O> {
    #[doc = "No transmission cancellation finished"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCF_A::B_0X0)
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCF_A::B_0X1)
    }
}
#[doc = "Field `TFE` reader - Tx FIFO empty"]
pub type TFE_R = crate::BitReader<TFE_A>;
#[doc = "Tx FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    #[doc = "0: Tx FIFO non-empty"]
    B_0X0 = 0,
    #[doc = "1: Tx FIFO empty"]
    B_0X1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::B_0X0,
            true => TFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFE_A::B_0X1
    }
}
#[doc = "Field `TFE` writer - Tx FIFO empty"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TFE_A>;
impl<'a, const O: u8> TFE_W<'a, O> {
    #[doc = "Tx FIFO non-empty"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TFE_A::B_0X0)
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TFE_A::B_0X1)
    }
}
#[doc = "Field `TEFN` reader - Tx event FIFO New Entry"]
pub type TEFN_R = crate::BitReader<TEFN_A>;
#[doc = "Tx event FIFO New Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFN_A {
    #[doc = "0: Tx event FIFO unchanged"]
    B_0X0 = 0,
    #[doc = "1: Tx handler wrote Tx event FIFO element."]
    B_0X1 = 1,
}
impl From<TEFN_A> for bool {
    #[inline(always)]
    fn from(variant: TEFN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEFN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFN_A {
        match self.bits {
            false => TEFN_A::B_0X0,
            true => TEFN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFN_A::B_0X1
    }
}
#[doc = "Field `TEFN` writer - Tx event FIFO New Entry"]
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TEFN_A>;
impl<'a, const O: u8> TEFN_W<'a, O> {
    #[doc = "Tx event FIFO unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFN_A::B_0X0)
    }
    #[doc = "Tx handler wrote Tx event FIFO element."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFN_A::B_0X1)
    }
}
#[doc = "Field `TEFF` reader - Tx event FIFO full"]
pub type TEFF_R = crate::BitReader<TEFF_A>;
#[doc = "Tx event FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFF_A {
    #[doc = "0: Tx event FIFO Not full"]
    B_0X0 = 0,
    #[doc = "1: Tx event FIFO full"]
    B_0X1 = 1,
}
impl From<TEFF_A> for bool {
    #[inline(always)]
    fn from(variant: TEFF_A) -> Self {
        variant as u8 != 0
    }
}
impl TEFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFF_A {
        match self.bits {
            false => TEFF_A::B_0X0,
            true => TEFF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFF_A::B_0X1
    }
}
#[doc = "Field `TEFF` writer - Tx event FIFO full"]
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TEFF_A>;
impl<'a, const O: u8> TEFF_W<'a, O> {
    #[doc = "Tx event FIFO Not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFF_A::B_0X0)
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFF_A::B_0X1)
    }
}
#[doc = "Field `TEFL` reader - Tx event FIFO element lost"]
pub type TEFL_R = crate::BitReader<TEFL_A>;
#[doc = "Tx event FIFO element lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFL_A {
    #[doc = "0: No Tx event FIFO element lost"]
    B_0X0 = 0,
    #[doc = "1: Tx event FIFO element lost"]
    B_0X1 = 1,
}
impl From<TEFL_A> for bool {
    #[inline(always)]
    fn from(variant: TEFL_A) -> Self {
        variant as u8 != 0
    }
}
impl TEFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFL_A {
        match self.bits {
            false => TEFL_A::B_0X0,
            true => TEFL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFL_A::B_0X1
    }
}
#[doc = "Field `TEFL` writer - Tx event FIFO element lost"]
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TEFL_A>;
impl<'a, const O: u8> TEFL_W<'a, O> {
    #[doc = "No Tx event FIFO element lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFL_A::B_0X0)
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFL_A::B_0X1)
    }
}
#[doc = "Field `TSW` reader - Timestamp wraparound"]
pub type TSW_R = crate::BitReader<TSW_A>;
#[doc = "Timestamp wraparound\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSW_A {
    #[doc = "0: No timestamp counter wrap-around"]
    B_0X0 = 0,
    #[doc = "1: Timestamp counter wrapped around"]
    B_0X1 = 1,
}
impl From<TSW_A> for bool {
    #[inline(always)]
    fn from(variant: TSW_A) -> Self {
        variant as u8 != 0
    }
}
impl TSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSW_A {
        match self.bits {
            false => TSW_A::B_0X0,
            true => TSW_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSW_A::B_0X1
    }
}
#[doc = "Field `TSW` writer - Timestamp wraparound"]
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TSW_A>;
impl<'a, const O: u8> TSW_W<'a, O> {
    #[doc = "No timestamp counter wrap-around"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSW_A::B_0X0)
    }
    #[doc = "Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSW_A::B_0X1)
    }
}
#[doc = "Field `MRAF` reader - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MRAF_R = crate::BitReader<MRAF_A>;
#[doc = "Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRAF_A {
    #[doc = "0: No Message RAM access failure occurred"]
    B_0X0 = 0,
    #[doc = "1: Message RAM access failure occurred"]
    B_0X1 = 1,
}
impl From<MRAF_A> for bool {
    #[inline(always)]
    fn from(variant: MRAF_A) -> Self {
        variant as u8 != 0
    }
}
impl MRAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRAF_A {
        match self.bits {
            false => MRAF_A::B_0X0,
            true => MRAF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRAF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRAF_A::B_0X1
    }
}
#[doc = "Field `MRAF` writer - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, MRAF_A>;
impl<'a, const O: u8> MRAF_W<'a, O> {
    #[doc = "No Message RAM access failure occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MRAF_A::B_0X0)
    }
    #[doc = "Message RAM access failure occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MRAF_A::B_0X1)
    }
}
#[doc = "Field `TOO` reader - Timeout occurred"]
pub type TOO_R = crate::BitReader<TOO_A>;
#[doc = "Timeout occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOO_A {
    #[doc = "0: No timeout"]
    B_0X0 = 0,
    #[doc = "1: Timeout reached"]
    B_0X1 = 1,
}
impl From<TOO_A> for bool {
    #[inline(always)]
    fn from(variant: TOO_A) -> Self {
        variant as u8 != 0
    }
}
impl TOO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOO_A {
        match self.bits {
            false => TOO_A::B_0X0,
            true => TOO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOO_A::B_0X1
    }
}
#[doc = "Field `TOO` writer - Timeout occurred"]
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, TOO_A>;
impl<'a, const O: u8> TOO_W<'a, O> {
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOO_A::B_0X0)
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOO_A::B_0X1)
    }
}
#[doc = "Field `ELO` reader - Error logging overflow"]
pub type ELO_R = crate::BitReader<ELO_A>;
#[doc = "Error logging overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELO_A {
    #[doc = "0: CAN error logging counter did not overflow."]
    B_0X0 = 0,
    #[doc = "1: Overflow of CAN error logging counter occurred."]
    B_0X1 = 1,
}
impl From<ELO_A> for bool {
    #[inline(always)]
    fn from(variant: ELO_A) -> Self {
        variant as u8 != 0
    }
}
impl ELO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELO_A {
        match self.bits {
            false => ELO_A::B_0X0,
            true => ELO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ELO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ELO_A::B_0X1
    }
}
#[doc = "Field `ELO` writer - Error logging overflow"]
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, ELO_A>;
impl<'a, const O: u8> ELO_W<'a, O> {
    #[doc = "CAN error logging counter did not overflow."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ELO_A::B_0X0)
    }
    #[doc = "Overflow of CAN error logging counter occurred."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ELO_A::B_0X1)
    }
}
#[doc = "Field `EP` reader - Error passive"]
pub type EP_R = crate::BitReader<EP_A>;
#[doc = "Error passive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP_A {
    #[doc = "0: Error_Passive status unchanged"]
    B_0X0 = 0,
    #[doc = "1: Error_Passive status changed"]
    B_0X1 = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::B_0X0,
            true => EP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EP_A::B_0X1
    }
}
#[doc = "Field `EP` writer - Error passive"]
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, EP_A>;
impl<'a, const O: u8> EP_W<'a, O> {
    #[doc = "Error_Passive status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EP_A::B_0X0)
    }
    #[doc = "Error_Passive status changed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EP_A::B_0X1)
    }
}
#[doc = "Field `EW` reader - Warning status"]
pub type EW_R = crate::BitReader<EW_A>;
#[doc = "Warning status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW_A {
    #[doc = "0: Error_Warning status unchanged"]
    B_0X0 = 0,
    #[doc = "1: Error_Warning status changed"]
    B_0X1 = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
impl EW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::B_0X0,
            true => EW_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EW_A::B_0X1
    }
}
#[doc = "Field `EW` writer - Warning status"]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, EW_A>;
impl<'a, const O: u8> EW_W<'a, O> {
    #[doc = "Error_Warning status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EW_A::B_0X0)
    }
    #[doc = "Error_Warning status changed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EW_A::B_0X1)
    }
}
#[doc = "Field `BO` reader - Bus_Off status"]
pub type BO_R = crate::BitReader<BO_A>;
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BO_A {
    #[doc = "0: Bus_Off status unchanged"]
    B_0X0 = 0,
    #[doc = "1: Bus_Off status changed"]
    B_0X1 = 1,
}
impl From<BO_A> for bool {
    #[inline(always)]
    fn from(variant: BO_A) -> Self {
        variant as u8 != 0
    }
}
impl BO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BO_A {
        match self.bits {
            false => BO_A::B_0X0,
            true => BO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BO_A::B_0X1
    }
}
#[doc = "Field `BO` writer - Bus_Off status"]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, BO_A>;
impl<'a, const O: u8> BO_W<'a, O> {
    #[doc = "Bus_Off status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BO_A::B_0X0)
    }
    #[doc = "Bus_Off status changed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BO_A::B_0X1)
    }
}
#[doc = "Field `WDI` reader - Watchdog interrupt"]
pub type WDI_R = crate::BitReader<WDI_A>;
#[doc = "Watchdog interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDI_A {
    #[doc = "0: No message RAM watchdog event occurred"]
    B_0X0 = 0,
    #[doc = "1: Message RAM watchdog event due to missing READY"]
    B_0X1 = 1,
}
impl From<WDI_A> for bool {
    #[inline(always)]
    fn from(variant: WDI_A) -> Self {
        variant as u8 != 0
    }
}
impl WDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDI_A {
        match self.bits {
            false => WDI_A::B_0X0,
            true => WDI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDI_A::B_0X1
    }
}
#[doc = "Field `WDI` writer - Watchdog interrupt"]
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, WDI_A>;
impl<'a, const O: u8> WDI_W<'a, O> {
    #[doc = "No message RAM watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDI_A::B_0X0)
    }
    #[doc = "Message RAM watchdog event due to missing READY"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDI_A::B_0X1)
    }
}
#[doc = "Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PEA_R = crate::BitReader<PEA_A>;
#[doc = "Protocol error in arbitration phase (nominal bit time is used)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEA_A {
    #[doc = "0: No protocol error in arbitration phase"]
    B_0X0 = 0,
    #[doc = "1: Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    B_0X1 = 1,
}
impl From<PEA_A> for bool {
    #[inline(always)]
    fn from(variant: PEA_A) -> Self {
        variant as u8 != 0
    }
}
impl PEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEA_A {
        match self.bits {
            false => PEA_A::B_0X0,
            true => PEA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PEA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PEA_A::B_0X1
    }
}
#[doc = "Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PEA_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, PEA_A>;
impl<'a, const O: u8> PEA_W<'a, O> {
    #[doc = "No protocol error in arbitration phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PEA_A::B_0X0)
    }
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PEA_A::B_0X1)
    }
}
#[doc = "Field `PED` reader - Protocol error in data phase (data bit time is used)"]
pub type PED_R = crate::BitReader<PED_A>;
#[doc = "Protocol error in data phase (data bit time is used)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PED_A {
    #[doc = "0: No protocol error in data phase"]
    B_0X0 = 0,
    #[doc = "1: Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    B_0X1 = 1,
}
impl From<PED_A> for bool {
    #[inline(always)]
    fn from(variant: PED_A) -> Self {
        variant as u8 != 0
    }
}
impl PED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PED_A {
        match self.bits {
            false => PED_A::B_0X0,
            true => PED_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PED_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PED_A::B_0X1
    }
}
#[doc = "Field `PED` writer - Protocol error in data phase (data bit time is used)"]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, PED_A>;
impl<'a, const O: u8> PED_W<'a, O> {
    #[doc = "No protocol error in data phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PED_A::B_0X0)
    }
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PED_A::B_0X1)
    }
}
#[doc = "Field `ARA` reader - Access to reserved address"]
pub type ARA_R = crate::BitReader<ARA_A>;
#[doc = "Access to reserved address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARA_A {
    #[doc = "0: No access to reserved address occurred"]
    B_0X0 = 0,
    #[doc = "1: Access to reserved address occurred"]
    B_0X1 = 1,
}
impl From<ARA_A> for bool {
    #[inline(always)]
    fn from(variant: ARA_A) -> Self {
        variant as u8 != 0
    }
}
impl ARA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARA_A {
        match self.bits {
            false => ARA_A::B_0X0,
            true => ARA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ARA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ARA_A::B_0X1
    }
}
#[doc = "Field `ARA` writer - Access to reserved address"]
pub type ARA_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_IR_SPEC, O, ARA_A>;
impl<'a, const O: u8> ARA_W<'a, O> {
    #[doc = "No access to reserved address occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ARA_A::B_0X0)
    }
    #[doc = "Access to reserved address occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ARA_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<1> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<2> {
        RF0L_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<3> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<4> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<5> {
        RF1L_W::new(self)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<6> {
        HPM_W::new(self)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<7> {
        TC_W::new(self)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<8> {
        TCF_W::new(self)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<9> {
        TFE_W::new(self)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<10> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<11> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<12> {
        TEFL_W::new(self)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<13> {
        TSW_W::new(self)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<14> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<15> {
        TOO_W::new(self)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<16> {
        ELO_W::new(self)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<17> {
        EP_W::new(self)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<18> {
        EW_W::new(self)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<19> {
        BO_W::new(self)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<20> {
        WDI_W::new(self)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<21> {
        PEA_W::new(self)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<22> {
        PED_W::new(self)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<23> {
        ARA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ir](index.html) module"]
pub struct FDCAN_IR_SPEC;
impl crate::RegisterSpec for FDCAN_IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ir::R](R) reader structure"]
impl crate::Readable for FDCAN_IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ir::W](W) writer structure"]
impl crate::Writable for FDCAN_IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_IR to value 0"]
impl crate::Resettable for FDCAN_IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
