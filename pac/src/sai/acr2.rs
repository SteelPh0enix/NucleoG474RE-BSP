#[doc = "Register `ACR2` reader"]
pub struct R(crate::R<ACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR2` writer"]
pub struct W(crate::W<ACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR2_SPEC>;
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
impl From<crate::W<ACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FIFO threshold"]
pub type FTH_R = crate::FieldReader;
#[doc = "Field `FTH` writer - FIFO threshold"]
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, ACR2_SPEC, 3, O>;
#[doc = "Field `FFLUS` reader - FIFO flush"]
pub type FFLUS_R = crate::BitReader;
#[doc = "Field `FFLUS` writer - FIFO flush"]
pub type FFLUS_W<'a, const O: u8> = crate::BitWriter<'a, ACR2_SPEC, O>;
#[doc = "Field `TRIS` reader - Tristate management on data line"]
pub type TRIS_R = crate::BitReader;
#[doc = "Field `TRIS` writer - Tristate management on data line"]
pub type TRIS_W<'a, const O: u8> = crate::BitWriter<'a, ACR2_SPEC, O>;
#[doc = "Field `MUTE` reader - Mute"]
pub type MUTE_R = crate::BitReader;
#[doc = "Field `MUTE` writer - Mute"]
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, ACR2_SPEC, O>;
#[doc = "Field `MUTEVAL` reader - Mute value"]
pub type MUTEVAL_R = crate::BitReader;
#[doc = "Field `MUTEVAL` writer - Mute value"]
pub type MUTEVAL_W<'a, const O: u8> = crate::BitWriter<'a, ACR2_SPEC, O>;
#[doc = "Field `MUTECN` reader - Mute counter"]
pub type MUTECN_R = crate::FieldReader;
#[doc = "Field `MUTECN` writer - Mute counter"]
pub type MUTECN_W<'a, const O: u8> = crate::FieldWriter<'a, ACR2_SPEC, 6, O>;
#[doc = "Field `CPL` reader - Complement bit"]
pub type CPL_R = crate::BitReader;
#[doc = "Field `CPL` writer - Complement bit"]
pub type CPL_W<'a, const O: u8> = crate::BitWriter<'a, ACR2_SPEC, O>;
#[doc = "Field `COMP` reader - Companding mode"]
pub type COMP_R = crate::FieldReader;
#[doc = "Field `COMP` writer - Companding mode"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, ACR2_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:2 - FIFO threshold"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - FIFO flush"]
    #[inline(always)]
    pub fn fflus(&self) -> FFLUS_R {
        FFLUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tristate management on data line"]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mute"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mute value"]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Mute counter"]
    #[inline(always)]
    pub fn mutecn(&self) -> MUTECN_R {
        MUTECN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Complement bit"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Companding mode"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    #[doc = "Bit 3 - FIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn fflus(&mut self) -> FFLUS_W<3> {
        FFLUS_W::new(self)
    }
    #[doc = "Bit 4 - Tristate management on data line"]
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<4> {
        TRIS_W::new(self)
    }
    #[doc = "Bit 5 - Mute"]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<5> {
        MUTE_W::new(self)
    }
    #[doc = "Bit 6 - Mute value"]
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<6> {
        MUTEVAL_W::new(self)
    }
    #[doc = "Bits 7:12 - Mute counter"]
    #[inline(always)]
    #[must_use]
    pub fn mutecn(&mut self) -> MUTECN_W<7> {
        MUTECN_W::new(self)
    }
    #[doc = "Bit 13 - Complement bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<13> {
        CPL_W::new(self)
    }
    #[doc = "Bits 14:15 - Companding mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<14> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AConfiguration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr2](index.html) module"]
pub struct ACR2_SPEC;
impl crate::RegisterSpec for ACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr2::R](R) reader structure"]
impl crate::Readable for ACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr2::W](W) writer structure"]
impl crate::Writable for ACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR2 to value 0"]
impl crate::Resettable for ACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
