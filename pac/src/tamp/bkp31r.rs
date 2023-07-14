#[doc = "Register `BKP31R` reader"]
pub struct R(crate::R<BKP31R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP31R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP31R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP31R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP31R` writer"]
pub struct W(crate::W<BKP31R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP31R_SPEC>;
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
impl From<crate::W<BKP31R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP31R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, const O: u8> = crate::FieldWriter<'a, BKP31R_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<0> {
        BKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp31r](index.html) module"]
pub struct BKP31R_SPEC;
impl crate::RegisterSpec for BKP31R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp31r::R](R) reader structure"]
impl crate::Readable for BKP31R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp31r::W](W) writer structure"]
impl crate::Writable for BKP31R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKP31R to value 0"]
impl crate::Resettable for BKP31R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}