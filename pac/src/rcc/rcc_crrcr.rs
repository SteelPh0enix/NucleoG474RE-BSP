#[doc = "Register `RCC_CRRCR` reader"]
pub struct R(crate::R<RCC_CRRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CRRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CRRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CRRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CRRCR` writer"]
pub struct W(crate::W<RCC_CRRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CRRCR_SPEC>;
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
impl From<crate::W<RCC_CRRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CRRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI48ON` reader - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
pub type HSI48ON_R = crate::BitReader<HSI48ON_A>;
#[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON_A {
    #[doc = "0: HSI48 oscillator OFF"]
    B_0X0 = 0,
    #[doc = "1: HSI48 oscillator ON"]
    B_0X1 = 1,
}
impl From<HSI48ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48ON_A {
        match self.bits {
            false => HSI48ON_A::B_0X0,
            true => HSI48ON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSI48ON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSI48ON_A::B_0X1
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
pub type HSI48ON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CRRCR_SPEC, O, HSI48ON_A>;
impl<'a, const O: u8> HSI48ON_W<'a, O> {
    #[doc = "HSI48 oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSI48ON_A::B_0X0)
    }
    #[doc = "HSI48 oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSI48ON_A::B_0X1)
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
pub type HSI48RDY_R = crate::BitReader<HSI48RDY_A>;
#[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDY_A {
    #[doc = "0: HSI48 oscillator not ready"]
    B_0X0 = 0,
    #[doc = "1: HSI48 oscillator ready"]
    B_0X1 = 1,
}
impl From<HSI48RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDY_A {
        match self.bits {
            false => HSI48RDY_A::B_0X0,
            true => HSI48RDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSI48RDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSI48RDY_A::B_0X1
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value. They are ready only."]
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:15 - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value. They are ready only."]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<0> {
        HSI48ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock recovery RC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_crrcr](index.html) module"]
pub struct RCC_CRRCR_SPEC;
impl crate::RegisterSpec for RCC_CRRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_crrcr::R](R) reader structure"]
impl crate::Readable for RCC_CRRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_crrcr::W](W) writer structure"]
impl crate::Writable for RCC_CRRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CRRCR to value 0"]
impl crate::Resettable for RCC_CRRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
