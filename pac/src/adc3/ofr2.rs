#[doc = "Register `OFR2` reader"]
pub struct R(crate::R<OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR2` writer"]
pub struct W(crate::W<OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR2_SPEC>;
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
impl From<crate::W<OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET1` reader - OFFSET1"]
pub type OFFSET1_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET1` writer - OFFSET1"]
pub type OFFSET1_W<'a, const O: u8> = crate::FieldWriter<'a, OFR2_SPEC, 12, O, u16>;
#[doc = "Field `OFFSETPOS` reader - OFFSETPOS"]
pub type OFFSETPOS_R = crate::BitReader;
#[doc = "Field `OFFSETPOS` writer - OFFSETPOS"]
pub type OFFSETPOS_W<'a, const O: u8> = crate::BitWriter<'a, OFR2_SPEC, O>;
#[doc = "Field `SATEN` reader - SATEN"]
pub type SATEN_R = crate::BitReader;
#[doc = "Field `SATEN` writer - SATEN"]
pub type SATEN_W<'a, const O: u8> = crate::BitWriter<'a, OFR2_SPEC, O>;
#[doc = "Field `OFFSET1_CH` reader - OFFSET1_CH"]
pub type OFFSET1_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET1_CH` writer - OFFSET1_CH"]
pub type OFFSET1_CH_W<'a, const O: u8> = crate::FieldWriter<'a, OFR2_SPEC, 5, O>;
#[doc = "Field `OFFSET1_EN` reader - OFFSET1_EN"]
pub type OFFSET1_EN_R = crate::BitReader;
#[doc = "Field `OFFSET1_EN` writer - OFFSET1_EN"]
pub type OFFSET1_EN_W<'a, const O: u8> = crate::BitWriter<'a, OFR2_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - OFFSETPOS"]
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SATEN"]
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<0> {
        OFFSET1_W::new(self)
    }
    #[doc = "Bit 24 - OFFSETPOS"]
    #[inline(always)]
    #[must_use]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<24> {
        OFFSETPOS_W::new(self)
    }
    #[doc = "Bit 25 - SATEN"]
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SATEN_W<25> {
        SATEN_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<26> {
        OFFSET1_CH_W::new(self)
    }
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    #[must_use]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<31> {
        OFFSET1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr2](index.html) module"]
pub struct OFR2_SPEC;
impl crate::RegisterSpec for OFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr2::R](R) reader structure"]
impl crate::Readable for OFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr2::W](W) writer structure"]
impl crate::Writable for OFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for OFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
