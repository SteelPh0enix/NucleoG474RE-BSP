#[doc = "Register `RCC_APB1ENR2` reader"]
pub struct R(crate::R<RCC_APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1ENR2` writer"]
pub struct W(crate::W<RCC_APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1ENR2_SPEC>;
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
impl From<crate::W<RCC_APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable Set and cleared by software."]
pub type LPUART1EN_R = crate::BitReader<LPUART1EN_A>;
#[doc = "Low power UART 1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN_A {
    #[doc = "0: LPUART1 clock disable"]
    B_0X0 = 0,
    #[doc = "1: LPUART1 clock enable"]
    B_0X1 = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::B_0X0,
            true => LPUART1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPUART1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPUART1EN_A::B_0X1
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable Set and cleared by software."]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR2_SPEC, O, LPUART1EN_A>;
impl<'a, const O: u8> LPUART1EN_W<'a, O> {
    #[doc = "LPUART1 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPUART1EN_A::B_0X0)
    }
    #[doc = "LPUART1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPUART1EN_A::B_0X1)
    }
}
#[doc = "Field `I2C4EN` reader - I2C4 clock enable Set and cleared by software"]
pub type I2C4EN_R = crate::BitReader<I2C4EN_A>;
#[doc = "I2C4 clock enable Set and cleared by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4EN_A {
    #[doc = "0: I2C4 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: I2C4 clock enabled"]
    B_0X1 = 1,
}
impl From<I2C4EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C4EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4EN_A {
        match self.bits {
            false => I2C4EN_A::B_0X0,
            true => I2C4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C4EN_A::B_0X1
    }
}
#[doc = "Field `I2C4EN` writer - I2C4 clock enable Set and cleared by software"]
pub type I2C4EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR2_SPEC, O, I2C4EN_A>;
impl<'a, const O: u8> I2C4EN_W<'a, O> {
    #[doc = "I2C4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C4EN_A::B_0X0)
    }
    #[doc = "I2C4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C4EN_A::B_0X1)
    }
}
#[doc = "Field `UCPD1EN` reader - UCPD1 clock enable Set and cleared by software."]
pub type UCPD1EN_R = crate::BitReader<UCPD1EN_A>;
#[doc = "UCPD1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD1EN_A {
    #[doc = "0: UCPD1 clock disable"]
    B_0X0 = 0,
    #[doc = "1: UCPD1 clock enable"]
    B_0X1 = 1,
}
impl From<UCPD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: UCPD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPD1EN_A {
        match self.bits {
            false => UCPD1EN_A::B_0X0,
            true => UCPD1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UCPD1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UCPD1EN_A::B_0X1
    }
}
#[doc = "Field `UCPD1EN` writer - UCPD1 clock enable Set and cleared by software."]
pub type UCPD1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR2_SPEC, O, UCPD1EN_A>;
impl<'a, const O: u8> UCPD1EN_W<'a, O> {
    #[doc = "UCPD1 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UCPD1EN_A::B_0X0)
    }
    #[doc = "UCPD1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UCPD1EN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Low power UART 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<0> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 1 - I2C4 clock enable Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<1> {
        I2C4EN_W::new(self)
    }
    #[doc = "Bit 8 - UCPD1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<8> {
        UCPD1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1enr2](index.html) module"]
pub struct RCC_APB1ENR2_SPEC;
impl crate::RegisterSpec for RCC_APB1ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1enr2::R](R) reader structure"]
impl crate::Readable for RCC_APB1ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1enr2::W](W) writer structure"]
impl crate::Writable for RCC_APB1ENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB1ENR2 to value 0"]
impl crate::Resettable for RCC_APB1ENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
