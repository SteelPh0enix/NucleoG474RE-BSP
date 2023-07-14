#[doc = "Register `RCR` reader"]
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR` writer"]
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - Repetition counter value"]
pub type REP_R = crate::FieldReader<u16>;
#[doc = "Field `REP` writer - Repetition counter value"]
pub type REP_W<'a, const O: u8> = crate::FieldWriter<'a, RCR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<0> {
        REP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](index.html) module"]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr::R](R) reader structure"]
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr::W](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
