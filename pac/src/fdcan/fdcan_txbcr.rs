#[doc = "Register `FDCAN_TXBCR` reader"]
pub struct R(crate::R<FDCAN_TXBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBCR` writer"]
pub struct W(crate::W<FDCAN_TXBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBCR_SPEC>;
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
impl From<crate::W<FDCAN_TXBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CR_R = crate::FieldReader<CR_A>;
#[doc = "Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CR_A {
    #[doc = "0: No cancellation pending"]
    B_0X0 = 0,
    #[doc = "1: Cancellation pending"]
    B_0X1 = 1,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CR_A {
    type Ux = u8;
}
impl CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CR_A> {
        match self.bits {
            0 => Some(CR_A::B_0X0),
            1 => Some(CR_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CR_A::B_0X1
    }
}
#[doc = "Field `CR` writer - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_TXBCR_SPEC, 3, O, CR_A>;
impl<'a, const O: u8> CR_W<'a, O> {
    #[doc = "No cancellation pending"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CR_A::B_0X0)
    }
    #[doc = "Cancellation pending"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CR_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer cancellation request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcr](index.html) module"]
pub struct FDCAN_TXBCR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbcr::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbcr::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCR to value 0"]
impl crate::Resettable for FDCAN_TXBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
