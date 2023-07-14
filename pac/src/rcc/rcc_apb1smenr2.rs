#[doc = "Register `RCC_APB1SMENR2` reader"]
pub struct R(crate::R<RCC_APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1SMENR2` writer"]
pub struct W(crate::W<RCC_APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1SMENR2_SPEC>;
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
impl From<crate::W<RCC_APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type LPUART1SMEN_R = crate::BitReader<LPUART1SMEN_A>;
#[doc = "Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1SMEN_A {
    #[doc = "0: LPUART1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: LPUART1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<LPUART1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SMEN_A {
        match self.bits {
            false => LPUART1SMEN_A::B_0X0,
            true => LPUART1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPUART1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPUART1SMEN_A::B_0X1
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type LPUART1SMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, RCC_APB1SMENR2_SPEC, O, LPUART1SMEN_A>;
impl<'a, const O: u8> LPUART1SMEN_W<'a, O> {
    #[doc = "LPUART1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::B_0X0)
    }
    #[doc = "LPUART1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::B_0X1)
    }
}
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
pub type I2C4SMEN_R = crate::BitReader<I2C4SMEN_A>;
#[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4SMEN_A {
    #[doc = "0: I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: I2C4 clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<I2C4SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C4SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4SMEN_A {
        match self.bits {
            false => I2C4SMEN_A::B_0X0,
            true => I2C4SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C4SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C4SMEN_A::B_0X1
    }
}
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
pub type I2C4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR2_SPEC, O, I2C4SMEN_A>;
impl<'a, const O: u8> I2C4SMEN_W<'a, O> {
    #[doc = "I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C4SMEN_A::B_0X0)
    }
    #[doc = "I2C4 clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C4SMEN_A::B_0X1)
    }
}
#[doc = "Field `UCPD1SMEN` reader - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UCPD1SMEN_R = crate::BitReader<UCPD1SMEN_A>;
#[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD1SMEN_A {
    #[doc = "0: UCPD1 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: UCPD1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<UCPD1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCPD1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPD1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPD1SMEN_A {
        match self.bits {
            false => UCPD1SMEN_A::B_0X0,
            true => UCPD1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UCPD1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UCPD1SMEN_A::B_0X1
    }
}
#[doc = "Field `UCPD1SMEN` writer - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UCPD1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR2_SPEC, O, UCPD1SMEN_A>;
impl<'a, const O: u8> UCPD1SMEN_W<'a, O> {
    #[doc = "UCPD1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UCPD1SMEN_A::B_0X0)
    }
    #[doc = "UCPD1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UCPD1SMEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<0> {
        LPUART1SMEN_W::new(self)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<1> {
        I2C4SMEN_W::new(self)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<8> {
        UCPD1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1smenr2](index.html) module"]
pub struct RCC_APB1SMENR2_SPEC;
impl crate::RegisterSpec for RCC_APB1SMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1smenr2::R](R) reader structure"]
impl crate::Readable for RCC_APB1SMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1smenr2::W](W) writer structure"]
impl crate::Writable for RCC_APB1SMENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB1SMENR2 to value 0x0103"]
impl crate::Resettable for RCC_APB1SMENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0103;
}
