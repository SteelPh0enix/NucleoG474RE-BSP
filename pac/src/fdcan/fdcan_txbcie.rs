#[doc = "Register `FDCAN_TXBCIE` reader"]
pub struct R(crate::R<FDCAN_TXBCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBCIE` writer"]
pub struct W(crate::W<FDCAN_TXBCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBCIE_SPEC>;
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
impl From<crate::W<FDCAN_TXBCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFIE` reader - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
pub type CFIE_R = crate::FieldReader<CFIE_A>;
#[doc = "Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFIE_A {
    #[doc = "0: Cancellation finished interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Cancellation finished interrupt enabled"]
    B_0X1 = 1,
}
impl From<CFIE_A> for u8 {
    #[inline(always)]
    fn from(variant: CFIE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFIE_A {
    type Ux = u8;
}
impl CFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFIE_A> {
        match self.bits {
            0 => Some(CFIE_A::B_0X0),
            1 => Some(CFIE_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CFIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CFIE_A::B_0X1
    }
}
#[doc = "Field `CFIE` writer - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
pub type CFIE_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_TXBCIE_SPEC, 3, O, CFIE_A>;
impl<'a, const O: u8> CFIE_W<'a, O> {
    #[doc = "Cancellation finished interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CFIE_A::B_0X0)
    }
    #[doc = "Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CFIE_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self) -> CFIE_W<0> {
        CFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcie](index.html) module"]
pub struct FDCAN_TXBCIE_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbcie::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbcie::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBCIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCIE to value 0"]
impl crate::Resettable for FDCAN_TXBCIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
