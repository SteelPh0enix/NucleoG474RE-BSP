#[doc = "Register `BCLRFR` writer"]
pub struct W(crate::W<BCLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCLRFR_SPEC>;
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
impl From<crate::W<BCLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRUDR` writer - Clear overrun / underrun"]
pub type OVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, BCLRFR_SPEC, O>;
#[doc = "Field `MUTEDET` writer - Mute detection flag"]
pub type MUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, BCLRFR_SPEC, O>;
#[doc = "Field `WCKCFG` writer - Clear wrong clock configuration flag"]
pub type WCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, BCLRFR_SPEC, O>;
#[doc = "Field `CNRDY` writer - Clear codec not ready flag"]
pub type CNRDY_W<'a, const O: u8> = crate::BitWriter<'a, BCLRFR_SPEC, O>;
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, BCLRFR_SPEC, O>;
#[doc = "Field `LFSDET` writer - Clear late frame synchronization detection flag"]
pub type LFSDET_W<'a, const O: u8> = crate::BitWriter<'a, BCLRFR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovrudr(&mut self) -> OVRUDR_W<0> {
        OVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn mutedet(&mut self) -> MUTEDET_W<1> {
        MUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    #[must_use]
    pub fn wckcfg(&mut self) -> WCKCFG_W<2> {
        WCKCFG_W::new(self)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnrdy(&mut self) -> CNRDY_W<4> {
        CNRDY_W::new(self)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfsdet(&mut self) -> LFSDET_W<6> {
        LFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BClear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bclrfr](index.html) module"]
pub struct BCLRFR_SPEC;
impl crate::RegisterSpec for BCLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bclrfr::W](W) writer structure"]
impl crate::Writable for BCLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCLRFR to value 0"]
impl crate::Resettable for BCLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
