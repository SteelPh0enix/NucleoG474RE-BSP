#[doc = "Register `PWR_SCR` writer"]
pub struct W(crate::W<PWR_SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SCR_SPEC>;
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
impl From<crate::W<PWR_SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
pub type CWUF1_W<'a, const O: u8> = crate::BitWriter<'a, PWR_SCR_SPEC, O>;
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
pub type CWUF2_W<'a, const O: u8> = crate::BitWriter<'a, PWR_SCR_SPEC, O>;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
pub type CWUF3_W<'a, const O: u8> = crate::BitWriter<'a, PWR_SCR_SPEC, O>;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
pub type CWUF4_W<'a, const O: u8> = crate::BitWriter<'a, PWR_SCR_SPEC, O>;
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
pub type CWUF5_W<'a, const O: u8> = crate::BitWriter<'a, PWR_SCR_SPEC, O>;
#[doc = "Field `CSBF` writer - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, PWR_SCR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<0> {
        CWUF1_W::new(self)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<1> {
        CWUF2_W::new(self)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<2> {
        CWUF3_W::new(self)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<3> {
        CWUF4_W::new(self)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<4> {
        CWUF5_W::new(self)
    }
    #[doc = "Bit 8 - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<8> {
        CSBF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_scr](index.html) module"]
pub struct PWR_SCR_SPEC;
impl crate::RegisterSpec for PWR_SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwr_scr::W](W) writer structure"]
impl crate::Writable for PWR_SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_SCR to value 0"]
impl crate::Resettable for PWR_SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
