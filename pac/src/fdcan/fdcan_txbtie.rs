#[doc = "Register `FDCAN_TXBTIE` reader"]
pub struct R(crate::R<FDCAN_TXBTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBTIE` writer"]
pub struct W(crate::W<FDCAN_TXBTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBTIE_SPEC>;
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
impl From<crate::W<FDCAN_TXBTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
pub type TIE_R = crate::FieldReader<TIE_A>;
#[doc = "Transmission interrupt enable Each Tx buffer has its own TIE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIE_A {
    #[doc = "0: Transmission interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Transmission interrupt enable"]
    B_0X1 = 1,
}
impl From<TIE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIE_A {
    type Ux = u8;
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIE_A> {
        match self.bits {
            0 => Some(TIE_A::B_0X0),
            1 => Some(TIE_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIE_A::B_0X1
    }
}
#[doc = "Field `TIE` writer - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
pub type TIE_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_TXBTIE_SPEC, 3, O, TIE_A>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Transmission interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIE_A::B_0X0)
    }
    #[doc = "Transmission interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIE_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer transmission interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbtie](index.html) module"]
pub struct FDCAN_TXBTIE_SPEC;
impl crate::RegisterSpec for FDCAN_TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbtie::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbtie::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBTIE to value 0"]
impl crate::Resettable for FDCAN_TXBTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
