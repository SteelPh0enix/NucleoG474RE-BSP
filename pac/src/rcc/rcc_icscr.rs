#[doc = "Register `RCC_ICSCR` reader"]
pub struct R(crate::R<RCC_ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ICSCR` writer"]
pub struct W(crate::W<RCC_ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ICSCR_SPEC>;
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
impl From<crate::W<RCC_ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSICAL` reader - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
pub type HSICAL_R = crate::FieldReader;
#[doc = "Field `HSITRIM` reader - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_ICSCR_SPEC, 7, O>;
impl R {
    #[doc = "Bits 16:23 - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<24> {
        HSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_icscr](index.html) module"]
pub struct RCC_ICSCR_SPEC;
impl crate::RegisterSpec for RCC_ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_icscr::R](R) reader structure"]
impl crate::Readable for RCC_ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_icscr::W](W) writer structure"]
impl crate::Writable for RCC_ICSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_ICSCR to value 0x4000_0000"]
impl crate::Resettable for RCC_ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
