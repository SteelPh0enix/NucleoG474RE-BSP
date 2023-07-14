#[doc = "Register `RCC_AHB3ENR` reader"]
pub struct R(crate::R<RCC_AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB3ENR` writer"]
pub struct W(crate::W<RCC_AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3ENR_SPEC>;
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
impl From<crate::W<RCC_AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCEN` reader - Flexible static memory controller clock enable Set and cleared by software."]
pub type FMCEN_R = crate::BitReader<FMCEN_A>;
#[doc = "Flexible static memory controller clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCEN_A {
    #[doc = "0: FSMC clock disable"]
    B_0X0 = 0,
    #[doc = "1: FSMC clock enable"]
    B_0X1 = 1,
}
impl From<FMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCEN_A {
        match self.bits {
            false => FMCEN_A::B_0X0,
            true => FMCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCEN_A::B_0X1
    }
}
#[doc = "Field `FMCEN` writer - Flexible static memory controller clock enable Set and cleared by software."]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB3ENR_SPEC, O, FMCEN_A>;
impl<'a, const O: u8> FMCEN_W<'a, O> {
    #[doc = "FSMC clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCEN_A::B_0X0)
    }
    #[doc = "FSMC clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCEN_A::B_0X1)
    }
}
#[doc = "Field `QSPIEN` reader - QUADSPI memory interface clock enable Set and cleared by software."]
pub type QSPIEN_R = crate::BitReader<QSPIEN_A>;
#[doc = "QUADSPI memory interface clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QSPIEN_A {
    #[doc = "0: QUADSPI clock disable"]
    B_0X0 = 0,
    #[doc = "1: QUADSPI clock enable"]
    B_0X1 = 1,
}
impl From<QSPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: QSPIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl QSPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPIEN_A {
        match self.bits {
            false => QSPIEN_A::B_0X0,
            true => QSPIEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPIEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPIEN_A::B_0X1
    }
}
#[doc = "Field `QSPIEN` writer - QUADSPI memory interface clock enable Set and cleared by software."]
pub type QSPIEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB3ENR_SPEC, O, QSPIEN_A>;
impl<'a, const O: u8> QSPIEN_W<'a, O> {
    #[doc = "QUADSPI clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPIEN_A::B_0X0)
    }
    #[doc = "QUADSPI clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPIEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<0> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<8> {
        QSPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3enr](index.html) module"]
pub struct RCC_AHB3ENR_SPEC;
impl crate::RegisterSpec for RCC_AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb3enr::R](R) reader structure"]
impl crate::Readable for RCC_AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3enr::W](W) writer structure"]
impl crate::Writable for RCC_AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB3ENR to value 0"]
impl crate::Resettable for RCC_AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
