#[doc = "Register `FDCAN_RXF0S` reader"]
pub struct R(crate::R<FDCAN_RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F0FL` reader - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
pub type F0FL_R = crate::FieldReader;
#[doc = "Field `F0GI` reader - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
pub type F0GI_R = crate::FieldReader;
#[doc = "Field `F0PI` reader - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
pub type F0PI_R = crate::FieldReader;
#[doc = "Field `F0F` reader - Rx FIFO 0 full"]
pub type F0F_R = crate::BitReader<F0F_A>;
#[doc = "Rx FIFO 0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F0F_A {
    #[doc = "0: Rx FIFO 0 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B_0X1 = 1,
}
impl From<F0F_A> for bool {
    #[inline(always)]
    fn from(variant: F0F_A) -> Self {
        variant as u8 != 0
    }
}
impl F0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F0F_A {
        match self.bits {
            false => F0F_A::B_0X0,
            true => F0F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F0F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F0F_A::B_0X1
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\]
is reset, this bit is also reset."]
pub type RF0L_R = crate::BitReader<RF0L_A>;
#[doc = "Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\]
is reset, this bit is also reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0L_A {
    #[doc = "0: No Rx FIFO 0 message lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0"]
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
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\]
is reset, this bit is also reset."]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Rx FIFO 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0s](index.html) module"]
pub struct FDCAN_RXF0S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf0s::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF0S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_RXF0S to value 0"]
impl crate::Resettable for FDCAN_RXF0S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
