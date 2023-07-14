#[doc = "Register `CCMR1_Output` reader"]
pub struct R(crate::R<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR1_Output` writer"]
pub struct W(crate::W<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_OUTPUT_SPEC, 2, O>;
#[doc = "Field `OC1FE` reader - Output Compare 1 fast enable"]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output Compare 1 fast enable"]
pub type OC1FE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
#[doc = "Field `OC1PE` reader - Output Compare 1 preload enable"]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output Compare 1 preload enable"]
pub type OC1PE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
#[doc = "Field `OC1M` reader - Output Compare 1 mode"]
pub type OC1M_R = crate::FieldReader;
#[doc = "Field `OC1M` writer - Output Compare 1 mode"]
pub type OC1M_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_OUTPUT_SPEC, 3, O>;
#[doc = "Field `OC1CE` reader - OC1CE"]
pub type OC1CE_R = crate::BitReader;
#[doc = "Field `OC1CE` writer - OC1CE"]
pub type OC1CE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
#[doc = "Field `CC2S` reader - CC2S"]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - CC2S"]
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_OUTPUT_SPEC, 2, O>;
#[doc = "Field `OC2FE` reader - OC2FE"]
pub type OC2FE_R = crate::BitReader;
#[doc = "Field `OC2FE` writer - OC2FE"]
pub type OC2FE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
#[doc = "Field `OC2PE` reader - OC2PE"]
pub type OC2PE_R = crate::BitReader;
#[doc = "Field `OC2PE` writer - OC2PE"]
pub type OC2PE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
#[doc = "Field `OC2M` reader - OC2M"]
pub type OC2M_R = crate::FieldReader;
#[doc = "Field `OC2M` writer - OC2M"]
pub type OC2M_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_OUTPUT_SPEC, 3, O>;
#[doc = "Field `OC1M_3` reader - Output Compare 1 mode"]
pub type OC1M_3_R = crate::BitReader;
#[doc = "Field `OC1M_3` writer - Output Compare 1 mode"]
pub type OC1M_3_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
#[doc = "Field `OC2M_3` reader - Output Compare 2 mode - bit 3"]
pub type OC2M_3_R = crate::BitReader;
#[doc = "Field `OC2M_3` writer - Output Compare 2 mode - bit 3"]
pub type OC2M_3_W<'a, const O: u8> = crate::BitWriter<'a, CCMR1_OUTPUT_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC1CE"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OC2FE"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC2PE"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC2M"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub fn oc2m_3(&self) -> OC2M_3_R {
        OC2M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<2> {
        OC1FE_W::new(self)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<3> {
        OC1PE_W::new(self)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<4> {
        OC1M_W::new(self)
    }
    #[doc = "Bit 7 - OC1CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<7> {
        OC1CE_W::new(self)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    #[doc = "Bit 10 - OC2FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<10> {
        OC2FE_W::new(self)
    }
    #[doc = "Bit 11 - OC2PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<11> {
        OC2PE_W::new(self)
    }
    #[doc = "Bits 12:14 - OC2M"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<12> {
        OC2M_W::new(self)
    }
    #[doc = "Bit 16 - Output Compare 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<16> {
        OC1M_3_W::new(self)
    }
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m_3(&mut self) -> OC2M_3_W<24> {
        OC2M_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_output](index.html) module"]
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr1_output::R](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr1_output::W](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
