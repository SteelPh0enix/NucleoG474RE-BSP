#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROVSE` reader - DMAEN"]
pub type ROVSE_R = crate::BitReader;
#[doc = "Field `ROVSE` writer - DMAEN"]
pub type ROVSE_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `JOVSE` reader - DMACFG"]
pub type JOVSE_R = crate::BitReader;
#[doc = "Field `JOVSE` writer - DMACFG"]
pub type JOVSE_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `OVSR` reader - RES"]
pub type OVSR_R = crate::FieldReader;
#[doc = "Field `OVSR` writer - RES"]
pub type OVSR_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR2_SPEC, 3, O>;
#[doc = "Field `OVSS` reader - ALIGN"]
pub type OVSS_R = crate::FieldReader;
#[doc = "Field `OVSS` writer - ALIGN"]
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR2_SPEC, 4, O>;
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling"]
pub type TROVS_R = crate::BitReader;
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling"]
pub type TROVS_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `ROVSM` reader - EXTEN"]
pub type ROVSM_R = crate::BitReader;
#[doc = "Field `ROVSM` writer - EXTEN"]
pub type ROVSM_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `GCOMP` reader - GCOMP"]
pub type GCOMP_R = crate::BitReader;
#[doc = "Field `GCOMP` writer - GCOMP"]
pub type GCOMP_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `SWTRIG` reader - SWTRIG"]
pub type SWTRIG_R = crate::BitReader;
#[doc = "Field `SWTRIG` writer - SWTRIG"]
pub type SWTRIG_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `BULB` reader - BULB"]
pub type BULB_R = crate::BitReader;
#[doc = "Field `BULB` writer - BULB"]
pub type BULB_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
#[doc = "Field `SMPTRIG` reader - SMPTRIG"]
pub type SMPTRIG_R = crate::BitReader;
#[doc = "Field `SMPTRIG` writer - SMPTRIG"]
pub type SMPTRIG_W<'a, const O: u8> = crate::BitWriter<'a, CFGR2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTEN"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - GCOMP"]
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - SWTRIG"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BULB"]
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SMPTRIG"]
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<0> {
        ROVSE_W::new(self)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<1> {
        JOVSE_W::new(self)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<2> {
        OVSR_W::new(self)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<9> {
        TROVS_W::new(self)
    }
    #[doc = "Bit 10 - EXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<10> {
        ROVSM_W::new(self)
    }
    #[doc = "Bit 16 - GCOMP"]
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<16> {
        GCOMP_W::new(self)
    }
    #[doc = "Bit 25 - SWTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<25> {
        SWTRIG_W::new(self)
    }
    #[doc = "Bit 26 - BULB"]
    #[inline(always)]
    #[must_use]
    pub fn bulb(&mut self) -> BULB_W<26> {
        BULB_W::new(self)
    }
    #[doc = "Bit 27 - SMPTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn smptrig(&mut self) -> SMPTRIG_W<27> {
        SMPTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
