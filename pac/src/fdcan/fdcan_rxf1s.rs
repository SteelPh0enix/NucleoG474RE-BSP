#[doc = "Register `FDCAN_RXF1S` reader"]
pub struct R(crate::R<FDCAN_RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1FL` reader - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
pub type F1FL_R = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
pub type F1GI_R = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
pub type F1PI_R = crate::FieldReader;
#[doc = "Field `F1F` reader - Rx FIFO 1 full"]
pub type F1F_R = crate::BitReader<F1F_A>;
#[doc = "Rx FIFO 1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F1F_A {
    #[doc = "0: Rx FIFO 1 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B_0X1 = 1,
}
impl From<F1F_A> for bool {
    #[inline(always)]
    fn from(variant: F1F_A) -> Self {
        variant as u8 != 0
    }
}
impl F1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1F_A {
        match self.bits {
            false => F1F_A::B_0X0,
            true => F1F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F1F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F1F_A::B_0X1
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset."]
pub type RF1L_R = crate::BitReader<RF1L_A>;
#[doc = "Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1L_A {
    #[doc = "0: No Rx FIFO 1 message lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0"]
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
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1s](index.html) module"]
pub struct FDCAN_RXF1S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1s::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FDCAN_RXF1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
