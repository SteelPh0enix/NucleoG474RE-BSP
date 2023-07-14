#[doc = "Register `ASR` reader"]
pub struct R(crate::R<ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASR` writer"]
pub struct W(crate::W<ASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASR_SPEC>;
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
impl From<crate::W<ASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRUDR` reader - Overrun / underrun"]
pub type OVRUDR_R = crate::BitReader;
#[doc = "Field `OVRUDR` writer - Overrun / underrun"]
pub type OVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `MUTEDET` reader - Mute detection"]
pub type MUTEDET_R = crate::BitReader;
#[doc = "Field `MUTEDET` writer - Mute detection"]
pub type MUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only"]
pub type WCKCFG_R = crate::BitReader;
#[doc = "Field `WCKCFG` writer - Wrong clock configuration flag. This bit is read only"]
pub type WCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `FREQ` reader - FIFO request"]
pub type FREQ_R = crate::BitReader;
#[doc = "Field `FREQ` writer - FIFO request"]
pub type FREQ_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `CNRDY` reader - Codec not ready"]
pub type CNRDY_R = crate::BitReader;
#[doc = "Field `CNRDY` writer - Codec not ready"]
pub type CNRDY_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection"]
pub type AFSDET_R = crate::BitReader;
#[doc = "Field `AFSDET` writer - Anticipated frame synchronization detection"]
pub type AFSDET_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `LFSDET` reader - Late frame synchronization detection"]
pub type LFSDET_R = crate::BitReader;
#[doc = "Field `LFSDET` writer - Late frame synchronization detection"]
pub type LFSDET_W<'a, const O: u8> = crate::BitWriter<'a, ASR_SPEC, O>;
#[doc = "Field `FLVL` reader - FIFO level threshold"]
pub type FLVL_R = crate::FieldReader;
#[doc = "Field `FLVL` writer - FIFO level threshold"]
pub type FLVL_W<'a, const O: u8> = crate::FieldWriter<'a, ASR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag. This bit is read only"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovrudr(&mut self) -> OVRUDR_W<0> {
        OVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    #[must_use]
    pub fn mutedet(&mut self) -> MUTEDET_W<1> {
        MUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag. This bit is read only"]
    #[inline(always)]
    #[must_use]
    pub fn wckcfg(&mut self) -> WCKCFG_W<2> {
        WCKCFG_W::new(self)
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<3> {
        FREQ_W::new(self)
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    #[must_use]
    pub fn cnrdy(&mut self) -> CNRDY_W<4> {
        CNRDY_W::new(self)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    #[must_use]
    pub fn afsdet(&mut self) -> AFSDET_W<5> {
        AFSDET_W::new(self)
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    #[must_use]
    pub fn lfsdet(&mut self) -> LFSDET_W<6> {
        LFSDET_W::new(self)
    }
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    #[must_use]
    pub fn flvl(&mut self) -> FLVL_W<16> {
        FLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AStatus register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asr](index.html) module"]
pub struct ASR_SPEC;
impl crate::RegisterSpec for ASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asr::R](R) reader structure"]
impl crate::Readable for ASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asr::W](W) writer structure"]
impl crate::Writable for ASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASR to value 0"]
impl crate::Resettable for ASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
