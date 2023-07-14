#[doc = "Register `FDCAN_TXBAR` reader"]
pub struct R(crate::R<FDCAN_TXBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBAR` writer"]
pub struct W(crate::W<FDCAN_TXBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBAR_SPEC>;
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
impl From<crate::W<FDCAN_TXBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type AR_R = crate::FieldReader<AR_A>;
#[doc = "Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AR_A {
    #[doc = "0: No transmission request added"]
    B_0X0 = 0,
    #[doc = "1: Transmission requested added."]
    B_0X1 = 1,
}
impl From<AR_A> for u8 {
    #[inline(always)]
    fn from(variant: AR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AR_A {
    type Ux = u8;
}
impl AR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AR_A> {
        match self.bits {
            0 => Some(AR_A::B_0X0),
            1 => Some(AR_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AR_A::B_0X1
    }
}
#[doc = "Field `AR` writer - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type AR_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_TXBAR_SPEC, 3, O, AR_A>;
impl<'a, const O: u8> AR_W<'a, O> {
    #[doc = "No transmission request added"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AR_A::B_0X0)
    }
    #[doc = "Transmission requested added."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AR_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<0> {
        AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer add request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbar](index.html) module"]
pub struct FDCAN_TXBAR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbar::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbar::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FDCAN_TXBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
