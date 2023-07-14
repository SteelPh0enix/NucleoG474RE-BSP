#[doc = "Register `FDCAN_TXEFS` reader"]
pub struct R(crate::R<FDCAN_TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EFFL` reader - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
pub type EFFL_R = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
pub type EFGI_R = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
pub type EFPI_R = crate::FieldReader;
#[doc = "Field `EFF` reader - Event FIFO full"]
pub type EFF_R = crate::BitReader<EFF_A>;
#[doc = "Event FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFF_A {
    #[doc = "0: Tx event FIFO not full"]
    B_0X0 = 0,
    #[doc = "1: Tx event FIFO full"]
    B_0X1 = 1,
}
impl From<EFF_A> for bool {
    #[inline(always)]
    fn from(variant: EFF_A) -> Self {
        variant as u8 != 0
    }
}
impl EFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFF_A {
        match self.bits {
            false => EFF_A::B_0X0,
            true => EFF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EFF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EFF_A::B_0X1
    }
}
#[doc = "Field `TEFL` reader - Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\]
is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
pub type TEFL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Event FIFO full"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\]
is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Tx event FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txefs](index.html) module"]
pub struct FDCAN_TXEFS_SPEC;
impl crate::RegisterSpec for FDCAN_TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txefs::R](R) reader structure"]
impl crate::Readable for FDCAN_TXEFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXEFS to value 0"]
impl crate::Resettable for FDCAN_TXEFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
