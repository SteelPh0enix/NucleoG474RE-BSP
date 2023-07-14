#[doc = "Register `RCC_AHB3RSTR` reader"]
pub struct R(crate::R<RCC_AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB3RSTR` writer"]
pub struct W(crate::W<RCC_AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3RSTR_SPEC>;
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
impl From<crate::W<RCC_AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCRST` reader - Flexible static memory controller reset Set and cleared by software."]
pub type FMCRST_R = crate::BitReader<FMCRST_A>;
#[doc = "Flexible static memory controller reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset FSMC"]
    B_0X1 = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCRST_A {
        match self.bits {
            false => FMCRST_A::B_0X0,
            true => FMCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCRST_A::B_0X1
    }
}
#[doc = "Field `FMCRST` writer - Flexible static memory controller reset Set and cleared by software."]
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB3RSTR_SPEC, O, FMCRST_A>;
impl<'a, const O: u8> FMCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCRST_A::B_0X0)
    }
    #[doc = "Reset FSMC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCRST_A::B_0X1)
    }
}
#[doc = "Field `QSPIRST` reader - QUADSPI reset Set and cleared by software."]
pub type QSPIRST_R = crate::BitReader<QSPIRST_A>;
#[doc = "QUADSPI reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QSPIRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset QUADSPI"]
    B_0X1 = 1,
}
impl From<QSPIRST_A> for bool {
    #[inline(always)]
    fn from(variant: QSPIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl QSPIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPIRST_A {
        match self.bits {
            false => QSPIRST_A::B_0X0,
            true => QSPIRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPIRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPIRST_A::B_0X1
    }
}
#[doc = "Field `QSPIRST` writer - QUADSPI reset Set and cleared by software."]
pub type QSPIRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB3RSTR_SPEC, O, QSPIRST_A>;
impl<'a, const O: u8> QSPIRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPIRST_A::B_0X0)
    }
    #[doc = "Reset QUADSPI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPIRST_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller reset Set and cleared by software."]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI reset Set and cleared by software."]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<0> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 8 - QUADSPI reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<8> {
        QSPIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3rstr](index.html) module"]
pub struct RCC_AHB3RSTR_SPEC;
impl crate::RegisterSpec for RCC_AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb3rstr::R](R) reader structure"]
impl crate::Readable for RCC_AHB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3rstr::W](W) writer structure"]
impl crate::Writable for RCC_AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB3RSTR to value 0"]
impl crate::Resettable for RCC_AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
