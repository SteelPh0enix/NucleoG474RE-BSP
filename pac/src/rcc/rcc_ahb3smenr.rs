#[doc = "Register `RCC_AHB3SMENR` reader"]
pub struct R(crate::R<RCC_AHB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB3SMENR` writer"]
pub struct W(crate::W<RCC_AHB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3SMENR_SPEC>;
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
impl From<crate::W<RCC_AHB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCSMEN` reader - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FMCSMEN_R = crate::BitReader<FMCSMEN_A>;
#[doc = "Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCSMEN_A {
    #[doc = "0: FSMC clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: FSMC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<FMCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCSMEN_A {
        match self.bits {
            false => FMCSMEN_A::B_0X0,
            true => FMCSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCSMEN_A::B_0X1
    }
}
#[doc = "Field `FMCSMEN` writer - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FMCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB3SMENR_SPEC, O, FMCSMEN_A>;
impl<'a, const O: u8> FMCSMEN_W<'a, O> {
    #[doc = "FSMC clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCSMEN_A::B_0X0)
    }
    #[doc = "FSMC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCSMEN_A::B_0X1)
    }
}
#[doc = "Field `QSPISMEN` reader - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
pub type QSPISMEN_R = crate::BitReader<QSPISMEN_A>;
#[doc = "QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QSPISMEN_A {
    #[doc = "0: QUADSPI clock disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: QUADSPI clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<QSPISMEN_A> for bool {
    #[inline(always)]
    fn from(variant: QSPISMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl QSPISMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPISMEN_A {
        match self.bits {
            false => QSPISMEN_A::B_0X0,
            true => QSPISMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPISMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPISMEN_A::B_0X1
    }
}
#[doc = "Field `QSPISMEN` writer - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
pub type QSPISMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB3SMENR_SPEC, O, QSPISMEN_A>;
impl<'a, const O: u8> QSPISMEN_W<'a, O> {
    #[doc = "QUADSPI clock disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPISMEN_A::B_0X0)
    }
    #[doc = "QUADSPI clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPISMEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FMCSMEN_R {
        FMCSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn qspismen(&self) -> QSPISMEN_R {
        QSPISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcsmen(&mut self) -> FMCSMEN_W<0> {
        FMCSMEN_W::new(self)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn qspismen(&mut self) -> QSPISMEN_W<8> {
        QSPISMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3smenr](index.html) module"]
pub struct RCC_AHB3SMENR_SPEC;
impl crate::RegisterSpec for RCC_AHB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb3smenr::R](R) reader structure"]
impl crate::Readable for RCC_AHB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3smenr::W](W) writer structure"]
impl crate::Writable for RCC_AHB3SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB3SMENR to value 0x0101"]
impl crate::Resettable for RCC_AHB3SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
