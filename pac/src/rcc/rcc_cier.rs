#[doc = "Register `RCC_CIER` reader"]
pub struct R(crate::R<RCC_CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CIER` writer"]
pub struct W(crate::W<RCC_CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CIER_SPEC>;
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
impl From<crate::W<RCC_CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
#[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE_A {
    #[doc = "0: LSI ready interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: LSI ready interrupt enabled"]
    B_0X1 = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::B_0X0,
            true => LSIRDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDYIE_A::B_0X1
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, LSIRDYIE_A>;
impl<'a, const O: u8> LSIRDYIE_W<'a, O> {
    #[doc = "LSI ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::B_0X0)
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::B_0X1)
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LSERDYIE_R = crate::BitReader<LSERDYIE_A>;
#[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE_A {
    #[doc = "0: LSE ready interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: LSE ready interrupt enabled"]
    B_0X1 = 1,
}
impl From<LSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYIE_A {
        match self.bits {
            false => LSERDYIE_A::B_0X0,
            true => LSERDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDYIE_A::B_0X1
    }
}
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, LSERDYIE_A>;
impl<'a, const O: u8> LSERDYIE_W<'a, O> {
    #[doc = "LSE ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSERDYIE_A::B_0X0)
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSERDYIE_A::B_0X1)
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE_A>;
#[doc = "HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE_A {
    #[doc = "0: HSI16 ready interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: HSI16 ready interrupt enabled"]
    B_0X1 = 1,
}
impl From<HSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYIE_A {
        match self.bits {
            false => HSIRDYIE_A::B_0X0,
            true => HSIRDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDYIE_A::B_0X1
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, HSIRDYIE_A>;
impl<'a, const O: u8> HSIRDYIE_W<'a, O> {
    #[doc = "HSI16 ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::B_0X0)
    }
    #[doc = "HSI16 ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::B_0X1)
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HSERDYIE_R = crate::BitReader<HSERDYIE_A>;
#[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE_A {
    #[doc = "0: HSE ready interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: HSE ready interrupt enabled"]
    B_0X1 = 1,
}
impl From<HSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYIE_A {
        match self.bits {
            false => HSERDYIE_A::B_0X0,
            true => HSERDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDYIE_A::B_0X1
    }
}
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, HSERDYIE_A>;
impl<'a, const O: u8> HSERDYIE_W<'a, O> {
    #[doc = "HSE ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSERDYIE_A::B_0X0)
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSERDYIE_A::B_0X1)
    }
}
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
pub type PLLRDYIE_R = crate::BitReader<PLLRDYIE_A>;
#[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYIE_A {
    #[doc = "0: PLL lock interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: PLL lock interrupt enabled"]
    B_0X1 = 1,
}
impl From<PLLRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYIE_A {
        match self.bits {
            false => PLLRDYIE_A::B_0X0,
            true => PLLRDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLRDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLRDYIE_A::B_0X1
    }
}
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, PLLRDYIE_A>;
impl<'a, const O: u8> PLLRDYIE_W<'a, O> {
    #[doc = "PLL lock interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::B_0X0)
    }
    #[doc = "PLL lock interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::B_0X1)
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
pub type LSECSSIE_R = crate::BitReader<LSECSSIE_A>;
#[doc = "LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE_A {
    #[doc = "0: Clock security interrupt caused by LSE clock failure disabled"]
    B_0X0 = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure enabled"]
    B_0X1 = 1,
}
impl From<LSECSSIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSIE_A {
        match self.bits {
            false => LSECSSIE_A::B_0X0,
            true => LSECSSIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSIE_A::B_0X1
    }
}
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
pub type LSECSSIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, LSECSSIE_A>;
impl<'a, const O: u8> LSECSSIE_W<'a, O> {
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSECSSIE_A::B_0X0)
    }
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSECSSIE_A::B_0X1)
    }
}
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
pub type HSI48RDYIE_R = crate::BitReader<HSI48RDYIE_A>;
#[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYIE_A {
    #[doc = "0: HSI48 ready interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: HSI48 ready interrupt enabled"]
    B_0X1 = 1,
}
impl From<HSI48RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYIE_A {
        match self.bits {
            false => HSI48RDYIE_A::B_0X0,
            true => HSI48RDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSI48RDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSI48RDYIE_A::B_0X1
    }
}
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
pub type HSI48RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CIER_SPEC, O, HSI48RDYIE_A>;
impl<'a, const O: u8> HSI48RDYIE_W<'a, O> {
    #[doc = "HSI48 ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSI48RDYIE_A::B_0X0)
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSI48RDYIE_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<4> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<5> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<10> {
        HSI48RDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cier](index.html) module"]
pub struct RCC_CIER_SPEC;
impl crate::RegisterSpec for RCC_CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cier::R](R) reader structure"]
impl crate::Readable for RCC_CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cier::W](W) writer structure"]
impl crate::Writable for RCC_CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CIER to value 0"]
impl crate::Resettable for RCC_CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
