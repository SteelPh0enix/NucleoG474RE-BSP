#[doc = "Register `FDCAN_TXFQS` reader"]
pub struct R(crate::R<FDCAN_TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)."]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Field `TFQF` reader - Tx FIFO/queue full"]
pub type TFQF_R = crate::BitReader<TFQF_A>;
#[doc = "Tx FIFO/queue full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFQF_A {
    #[doc = "0: Tx FIFO/queue not full"]
    B_0X0 = 0,
    #[doc = "1: Tx FIFO/queue full"]
    B_0X1 = 1,
}
impl From<TFQF_A> for bool {
    #[inline(always)]
    fn from(variant: TFQF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFQF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFQF_A {
        match self.bits {
            false => TFQF_A::B_0X0,
            true => TFQF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFQF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFQF_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)."]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "FDCAN Tx FIFO/queue status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txfqs](index.html) module"]
pub struct FDCAN_TXFQS_SPEC;
impl crate::RegisterSpec for FDCAN_TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txfqs::R](R) reader structure"]
impl crate::Readable for FDCAN_TXFQS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXFQS to value 0x03"]
impl crate::Resettable for FDCAN_TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
