#[doc = "Register `ACLRFR` reader"]
pub struct R(crate::R<ACLRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACLRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACLRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACLRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACLRFR` writer"]
pub struct W(crate::W<ACLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACLRFR_SPEC>;
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
impl From<crate::W<ACLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRUDR` reader - Clear overrun / underrun"]
pub type OVRUDR_R = crate::BitReader;
#[doc = "Field `OVRUDR` writer - Clear overrun / underrun"]
pub type OVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, ACLRFR_SPEC, O>;
#[doc = "Field `MUTEDET` reader - Mute detection flag"]
pub type MUTEDET_R = crate::BitReader;
#[doc = "Field `MUTEDET` writer - Mute detection flag"]
pub type MUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, ACLRFR_SPEC, O>;
#[doc = "Field `WCKCFG` reader - Clear wrong clock configuration flag"]
pub type WCKCFG_R = crate::BitReader;
#[doc = "Field `WCKCFG` writer - Clear wrong clock configuration flag"]
pub type WCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, ACLRFR_SPEC, O>;
#[doc = "Field `CNRDY` reader - Clear codec not ready flag"]
pub type CNRDY_R = crate::BitReader;
#[doc = "Field `CNRDY` writer - Clear codec not ready flag"]
pub type CNRDY_W<'a, const O: u8> = crate::BitWriter<'a, ACLRFR_SPEC, O>;
#[doc = "Field `CAFSDET` reader - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_R = crate::BitReader;
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, ACLRFR_SPEC, O>;
#[doc = "Field `LFSDET` reader - Clear late frame synchronization detection flag"]
pub type LFSDET_R = crate::BitReader;
#[doc = "Field `LFSDET` writer - Clear late frame synchronization detection flag"]
pub type LFSDET_W<'a, const O: u8> = crate::BitWriter<'a, ACLRFR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub fn cafsdet(&self) -> CAFSDET_R {
        CAFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
}
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
#[doc = "AClear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclrfr](index.html) module"]
pub struct ACLRFR_SPEC;
impl crate::RegisterSpec for ACLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aclrfr::R](R) reader structure"]
impl crate::Readable for ACLRFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aclrfr::W](W) writer structure"]
impl crate::Writable for ACLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACLRFR to value 0"]
impl crate::Resettable for ACLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
