#[doc = "Register `OPAMP5_TCMR` reader"]
pub struct R(crate::R<OPAMP5_TCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP5_TCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP5_TCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP5_TCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP5_TCMR` writer"]
pub struct W(crate::W<OPAMP5_TCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP5_TCMR_SPEC>;
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
impl From<crate::W<OPAMP5_TCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP5_TCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMS_SEL` reader - VMS_SEL"]
pub type VMS_SEL_R = crate::BitReader;
#[doc = "Field `VMS_SEL` writer - VMS_SEL"]
pub type VMS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP5_TCMR_SPEC, O>;
#[doc = "Field `VPS_SEL` reader - VPS_SEL"]
pub type VPS_SEL_R = crate::FieldReader;
#[doc = "Field `VPS_SEL` writer - VPS_SEL"]
pub type VPS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP5_TCMR_SPEC, 2, O>;
#[doc = "Field `T1CM_EN` reader - T1CM_EN"]
pub type T1CM_EN_R = crate::BitReader;
#[doc = "Field `T1CM_EN` writer - T1CM_EN"]
pub type T1CM_EN_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP5_TCMR_SPEC, O>;
#[doc = "Field `T8CM_EN` reader - T8CM_EN"]
pub type T8CM_EN_R = crate::BitReader;
#[doc = "Field `T8CM_EN` writer - T8CM_EN"]
pub type T8CM_EN_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP5_TCMR_SPEC, O>;
#[doc = "Field `T20CM_EN` reader - T20CM_EN"]
pub type T20CM_EN_R = crate::BitReader;
#[doc = "Field `T20CM_EN` writer - T20CM_EN"]
pub type T20CM_EN_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP5_TCMR_SPEC, O>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP5_TCMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<0> {
        VMS_SEL_W::new(self)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<1> {
        VPS_SEL_W::new(self)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W<3> {
        T1CM_EN_W::new(self)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W<4> {
        T8CM_EN_W::new(self)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W<5> {
        T20CM_EN_W::new(self)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP5 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp5_tcmr](index.html) module"]
pub struct OPAMP5_TCMR_SPEC;
impl crate::RegisterSpec for OPAMP5_TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp5_tcmr::R](R) reader structure"]
impl crate::Readable for OPAMP5_TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp5_tcmr::W](W) writer structure"]
impl crate::Writable for OPAMP5_TCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMP5_TCMR to value 0"]
impl crate::Resettable for OPAMP5_TCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
