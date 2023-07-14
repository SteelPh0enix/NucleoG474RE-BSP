#[doc = "Register `BDMADR` writer"]
pub struct W(crate::W<BDMADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMADR_SPEC>;
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
impl From<crate::W<BDMADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDMADR` writer - Burst DMA Data register"]
pub type BDMADR_W<'a, const O: u8> = crate::FieldWriter<'a, BDMADR_SPEC, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    #[must_use]
    pub fn bdmadr(&mut self) -> BDMADR_W<0> {
        BDMADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst DMA Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdmadr](index.html) module"]
pub struct BDMADR_SPEC;
impl crate::RegisterSpec for BDMADR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bdmadr::W](W) writer structure"]
impl crate::Writable for BDMADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDMADR to value 0"]
impl crate::Resettable for BDMADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
