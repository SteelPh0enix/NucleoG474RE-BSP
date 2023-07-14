#[doc = "Register `FDCAN_TXBTO` reader"]
pub struct R(crate::R<FDCAN_TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TO` reader - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type TO_R = crate::FieldReader<TO_A>;
#[doc = "Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO_A {
    #[doc = "0: No transmission occurred"]
    B_0X0 = 0,
    #[doc = "1: Transmission occurred"]
    B_0X1 = 1,
}
impl From<TO_A> for u8 {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TO_A {
    type Ux = u8;
}
impl TO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TO_A> {
        match self.bits {
            0 => Some(TO_A::B_0X0),
            1 => Some(TO_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TO_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbto](index.html) module"]
pub struct FDCAN_TXBTO_SPEC;
impl crate::RegisterSpec for FDCAN_TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbto::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBTO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXBTO to value 0"]
impl crate::Resettable for FDCAN_TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
