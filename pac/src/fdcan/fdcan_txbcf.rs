#[doc = "Register `FDCAN_TXBCF` reader"]
pub struct R(crate::R<FDCAN_TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CF` reader - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type CF_R = crate::FieldReader<CF_A>;
#[doc = "Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_A {
    #[doc = "0: No transmit buffer cancellation"]
    B_0X0 = 0,
    #[doc = "1: Transmit buffer cancellation finished"]
    B_0X1 = 1,
}
impl From<CF_A> for u8 {
    #[inline(always)]
    fn from(variant: CF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CF_A {
    type Ux = u8;
}
impl CF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CF_A> {
        match self.bits {
            0 => Some(CF_A::B_0X0),
            1 => Some(CF_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CF_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcf](index.html) module"]
pub struct FDCAN_TXBCF_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbcf::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXBCF to value 0"]
impl crate::Resettable for FDCAN_TXBCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
