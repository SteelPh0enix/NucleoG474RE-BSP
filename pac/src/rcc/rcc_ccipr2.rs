#[doc = "Register `RCC_CCIPR2` reader"]
pub struct R(crate::R<RCC_CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CCIPR2` writer"]
pub struct W(crate::W<RCC_CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CCIPR2_SPEC>;
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
impl From<crate::W<RCC_CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
pub type I2C4SEL_R = crate::FieldReader<I2C4SEL_A>;
#[doc = "I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    #[doc = "0: PCLK selected as I2C4 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C4 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as I2C4 clock"]
    B_0X2 = 2,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C4SEL_A {
    type Ux = u8;
}
impl I2C4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C4SEL_A> {
        match self.bits {
            0 => Some(I2C4SEL_A::B_0X0),
            1 => Some(I2C4SEL_A::B_0X1),
            2 => Some(I2C4SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C4SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C4SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2C4SEL_A::B_0X2
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
pub type I2C4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR2_SPEC, 2, O, I2C4SEL_A>;
impl<'a, const O: u8> I2C4SEL_W<'a, O> {
    #[doc = "PCLK selected as I2C4 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C4SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C4 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C4SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as I2C4 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2C4SEL_A::B_0X2)
    }
}
#[doc = "Field `QSPISEL` reader - QUADSPI clock source selection Set and reset by software."]
pub type QSPISEL_R = crate::FieldReader<QSPISEL_A>;
#[doc = "QUADSPI clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QSPISEL_A {
    #[doc = "0: system clock selected as QUADSPI kernel clock"]
    B_0X0 = 0,
    #[doc = "1: HSI16 clock selected as QUADSPI kernel clock"]
    B_0X1 = 1,
    #[doc = "2: PLL Q clock selected as QUADSPI kernel clock"]
    B_0X2 = 2,
}
impl From<QSPISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPISEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QSPISEL_A {
    type Ux = u8;
}
impl QSPISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<QSPISEL_A> {
        match self.bits {
            0 => Some(QSPISEL_A::B_0X0),
            1 => Some(QSPISEL_A::B_0X1),
            2 => Some(QSPISEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPISEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPISEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == QSPISEL_A::B_0X2
    }
}
#[doc = "Field `QSPISEL` writer - QUADSPI clock source selection Set and reset by software."]
pub type QSPISEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR2_SPEC, 2, O, QSPISEL_A>;
impl<'a, const O: u8> QSPISEL_W<'a, O> {
    #[doc = "system clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPISEL_A::B_0X0)
    }
    #[doc = "HSI16 clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPISEL_A::B_0X1)
    }
    #[doc = "PLL Q clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(QSPISEL_A::B_0X2)
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 20:21 - QUADSPI clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<0> {
        I2C4SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - QUADSPI clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn qspisel(&mut self) -> QSPISEL_W<20> {
        QSPISEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ccipr2](index.html) module"]
pub struct RCC_CCIPR2_SPEC;
impl crate::RegisterSpec for RCC_CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ccipr2::R](R) reader structure"]
impl crate::Readable for RCC_CCIPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ccipr2::W](W) writer structure"]
impl crate::Writable for RCC_CCIPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CCIPR2 to value 0"]
impl crate::Resettable for RCC_CCIPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
