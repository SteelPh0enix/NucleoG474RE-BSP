#[doc = "Register `OPAMP6_CSR` reader"]
pub struct R(crate::R<OPAMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP6_CSR` writer"]
pub struct W(crate::W<OPAMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP6_CSR_SPEC>;
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
impl From<crate::W<OPAMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OPAEN_R = crate::BitReader;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub type FORCE_VP_R = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type FORCE_VP_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `VP_SEL` reader - VP_SEL"]
pub type VP_SEL_R = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - VP_SEL"]
pub type VP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP6_CSR_SPEC, 2, O>;
#[doc = "Field `USERTRIM` reader - USERTRIM"]
pub type USERTRIM_R = crate::BitReader;
#[doc = "Field `USERTRIM` writer - USERTRIM"]
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `VM_SEL` reader - VM_SEL"]
pub type VM_SEL_R = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - VM_SEL"]
pub type VM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP6_CSR_SPEC, 2, O>;
#[doc = "Field `OPAHSM` reader - OPAHSM"]
pub type OPAHSM_R = crate::BitReader;
#[doc = "Field `OPAHSM` writer - OPAHSM"]
pub type OPAHSM_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `OPAINTOEN` reader - OPAINTOEN"]
pub type OPAINTOEN_R = crate::BitReader;
#[doc = "Field `OPAINTOEN` writer - OPAINTOEN"]
pub type OPAINTOEN_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `CALON` reader - CALON"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - CALON"]
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `CALSEL` reader - CALSEL"]
pub type CALSEL_R = crate::FieldReader;
#[doc = "Field `CALSEL` writer - CALSEL"]
pub type CALSEL_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP6_CSR_SPEC, 2, O>;
#[doc = "Field `PGA_GAIN` reader - PGA_GAIN"]
pub type PGA_GAIN_R = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - PGA_GAIN"]
pub type PGA_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP6_CSR_SPEC, 5, O>;
#[doc = "Field `TRIMOFFSETP` reader - TRIMOFFSETP"]
pub type TRIMOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - TRIMOFFSETP"]
pub type TRIMOFFSETP_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP6_CSR_SPEC, 5, O>;
#[doc = "Field `TRIMOFFSETN` reader - TRIMOFFSETN"]
pub type TRIMOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - TRIMOFFSETN"]
pub type TRIMOFFSETN_W<'a, const O: u8> = crate::FieldWriter<'a, OPAMP6_CSR_SPEC, 5, O>;
#[doc = "Field `CALOUT` reader - CALOUT"]
pub type CALOUT_R = crate::BitReader;
#[doc = "Field `CALOUT` writer - CALOUT"]
pub type CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, OPAMP6_CSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&self) -> OPAINTOEN_R {
        OPAINTOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<1> {
        FORCE_VP_W::new(self)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<2> {
        VP_SEL_W::new(self)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<4> {
        USERTRIM_W::new(self)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<5> {
        VM_SEL_W::new(self)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<7> {
        OPAHSM_W::new(self)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn opaintoen(&mut self) -> OPAINTOEN_W<8> {
        OPAINTOEN_W::new(self)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<11> {
        CALON_W::new(self)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<12> {
        CALSEL_W::new(self)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<14> {
        PGA_GAIN_W::new(self)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<19> {
        TRIMOFFSETP_W::new(self)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<24> {
        TRIMOFFSETN_W::new(self)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<30> {
        CALOUT_W::new(self)
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
#[doc = "OPAMP6 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp6_csr](index.html) module"]
pub struct OPAMP6_CSR_SPEC;
impl crate::RegisterSpec for OPAMP6_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp6_csr::R](R) reader structure"]
impl crate::Readable for OPAMP6_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp6_csr::W](W) writer structure"]
impl crate::Writable for OPAMP6_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMP6_CSR to value 0"]
impl crate::Resettable for OPAMP6_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
