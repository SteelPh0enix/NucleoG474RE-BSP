#[doc = "Register `RCC_APB2SMENR` reader"]
pub struct R(crate::R<RCC_APB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2SMENR` writer"]
pub struct W(crate::W<RCC_APB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2SMENR_SPEC>;
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
impl From<crate::W<RCC_APB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN_A>;
#[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN_A {
    #[doc = "0: SYSCFG + COMP + VREFBUF + OPAMP clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF + OPAMP clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SYSCFGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGSMEN_A {
        match self.bits {
            false => SYSCFGSMEN_A::B_0X0,
            true => SYSCFGSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGSMEN_A::B_0X1
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SYSCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, SYSCFGSMEN_A>;
impl<'a, const O: u8> SYSCFGSMEN_W<'a, O> {
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::B_0X0)
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM1SMEN_R = crate::BitReader<TIM1SMEN_A>;
#[doc = "TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SMEN_A {
    #[doc = "0: TIM1 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM1P timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1SMEN_A {
        match self.bits {
            false => TIM1SMEN_A::B_0X0,
            true => TIM1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, TIM1SMEN_A>;
impl<'a, const O: u8> TIM1SMEN_W<'a, O> {
    #[doc = "TIM1 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM1SMEN_A::B_0X0)
    }
    #[doc = "TIM1P timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM1SMEN_A::B_0X1)
    }
}
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI1SMEN_R = crate::BitReader<SPI1SMEN_A>;
#[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1SMEN_A {
    #[doc = "0: SPI1 clocks disabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SPI1 clocks enabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SPI1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1SMEN_A {
        match self.bits {
            false => SPI1SMEN_A::B_0X0,
            true => SPI1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1SMEN_A::B_0X1
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, SPI1SMEN_A>;
impl<'a, const O: u8> SPI1SMEN_W<'a, O> {
    #[doc = "SPI1 clocks disabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI1SMEN_A::B_0X0)
    }
    #[doc = "SPI1 clocks enabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI1SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM8SMEN_R = crate::BitReader<TIM8SMEN_A>;
#[doc = "TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM8SMEN_A {
    #[doc = "0: TIM8 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM8 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM8SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM8SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM8SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM8SMEN_A {
        match self.bits {
            false => TIM8SMEN_A::B_0X0,
            true => TIM8SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM8SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM8SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM8SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, TIM8SMEN_A>;
impl<'a, const O: u8> TIM8SMEN_W<'a, O> {
    #[doc = "TIM8 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM8SMEN_A::B_0X0)
    }
    #[doc = "TIM8 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM8SMEN_A::B_0X1)
    }
}
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USART1SMEN_R = crate::BitReader<USART1SMEN_A>;
#[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1SMEN_A {
    #[doc = "0: USART1clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: USART1clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<USART1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SMEN_A {
        match self.bits {
            false => USART1SMEN_A::B_0X0,
            true => USART1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1SMEN_A::B_0X1
    }
}
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, USART1SMEN_A>;
impl<'a, const O: u8> USART1SMEN_W<'a, O> {
    #[doc = "USART1clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1SMEN_A::B_0X0)
    }
    #[doc = "USART1clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1SMEN_A::B_0X1)
    }
}
#[doc = "Field `SPI4SMEN` reader - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI4SMEN_R = crate::BitReader<SPI4SMEN_A>;
#[doc = "SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI4SMEN_A {
    #[doc = "0: SPI4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SPI4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    B_0X1 = 1,
}
impl From<SPI4SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI4SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI4SMEN_A {
        match self.bits {
            false => SPI4SMEN_A::B_0X0,
            true => SPI4SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI4SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI4SMEN_A::B_0X1
    }
}
#[doc = "Field `SPI4SMEN` writer - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, SPI4SMEN_A>;
impl<'a, const O: u8> SPI4SMEN_W<'a, O> {
    #[doc = "SPI4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI4SMEN_A::B_0X0)
    }
    #[doc = "SPI4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI4SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM15SMEN_R = crate::BitReader<TIM15SMEN_A>;
#[doc = "TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM15SMEN_A {
    #[doc = "0: TIM15 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM15 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    B_0X1 = 1,
}
impl From<TIM15SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM15SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM15SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM15SMEN_A {
        match self.bits {
            false => TIM15SMEN_A::B_0X0,
            true => TIM15SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM15SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM15SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM15SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, TIM15SMEN_A>;
impl<'a, const O: u8> TIM15SMEN_W<'a, O> {
    #[doc = "TIM15 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM15SMEN_A::B_0X0)
    }
    #[doc = "TIM15 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM15SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM16SMEN_R = crate::BitReader<TIM16SMEN_A>;
#[doc = "TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16SMEN_A {
    #[doc = "0: TIM16 timer clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM16 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM16SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM16SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16SMEN_A {
        match self.bits {
            false => TIM16SMEN_A::B_0X0,
            true => TIM16SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM16SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, TIM16SMEN_A>;
impl<'a, const O: u8> TIM16SMEN_W<'a, O> {
    #[doc = "TIM16 timer clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM16SMEN_A::B_0X0)
    }
    #[doc = "TIM16 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM16SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM17SMEN_R = crate::BitReader<TIM17SMEN_A>;
#[doc = "TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17SMEN_A {
    #[doc = "0: TIM17 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM17 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM17SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM17SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17SMEN_A {
        match self.bits {
            false => TIM17SMEN_A::B_0X0,
            true => TIM17SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM17SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, TIM17SMEN_A>;
impl<'a, const O: u8> TIM17SMEN_W<'a, O> {
    #[doc = "TIM17 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM17SMEN_A::B_0X0)
    }
    #[doc = "TIM17 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM17SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM20SMEN` reader - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM20SMEN_R = crate::BitReader<TIM20SMEN_A>;
#[doc = "TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM20SMEN_A {
    #[doc = "0: TIM20 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM20 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM20SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM20SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM20SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM20SMEN_A {
        match self.bits {
            false => TIM20SMEN_A::B_0X0,
            true => TIM20SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM20SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM20SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM20SMEN` writer - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM20SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, TIM20SMEN_A>;
impl<'a, const O: u8> TIM20SMEN_W<'a, O> {
    #[doc = "TIM20 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM20SMEN_A::B_0X0)
    }
    #[doc = "TIM20 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM20SMEN_A::B_0X1)
    }
}
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SAI1SMEN_R = crate::BitReader<SAI1SMEN_A>;
#[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1SMEN_A {
    #[doc = "0: SAI1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SAI1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SAI1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1SMEN_A {
        match self.bits {
            false => SAI1SMEN_A::B_0X0,
            true => SAI1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1SMEN_A::B_0X1
    }
}
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SAI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, SAI1SMEN_A>;
impl<'a, const O: u8> SAI1SMEN_W<'a, O> {
    #[doc = "SAI1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1SMEN_A::B_0X0)
    }
    #[doc = "SAI1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1SMEN_A::B_0X1)
    }
}
#[doc = "Field `HRTIM1SMEN` reader - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type HRTIM1SMEN_R = crate::BitReader<HRTIM1SMEN_A>;
#[doc = "HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRTIM1SMEN_A {
    #[doc = "0: HRTIM1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: HRTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<HRTIM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HRTIM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HRTIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRTIM1SMEN_A {
        match self.bits {
            false => HRTIM1SMEN_A::B_0X0,
            true => HRTIM1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HRTIM1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HRTIM1SMEN_A::B_0X1
    }
}
#[doc = "Field `HRTIM1SMEN` writer - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type HRTIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2SMENR_SPEC, O, HRTIM1SMEN_A>;
impl<'a, const O: u8> HRTIM1SMEN_W<'a, O> {
    #[doc = "HRTIM1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HRTIM1SMEN_A::B_0X0)
    }
    #[doc = "HRTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HRTIM1SMEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi4smen(&self) -> SPI4SMEN_R {
        SPI4SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim20smen(&self) -> TIM20SMEN_R {
        TIM20SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1smen(&self) -> HRTIM1SMEN_R {
        HRTIM1SMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<0> {
        SYSCFGSMEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<11> {
        TIM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<12> {
        SPI1SMEN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<13> {
        TIM8SMEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<14> {
        USART1SMEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4smen(&mut self) -> SPI4SMEN_W<15> {
        SPI4SMEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<16> {
        TIM15SMEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<17> {
        TIM16SMEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<18> {
        TIM17SMEN_W::new(self)
    }
    #[doc = "Bit 20 - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim20smen(&mut self) -> TIM20SMEN_W<20> {
        TIM20SMEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<21> {
        SAI1SMEN_W::new(self)
    }
    #[doc = "Bit 26 - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn hrtim1smen(&mut self) -> HRTIM1SMEN_W<26> {
        HRTIM1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2smenr](index.html) module"]
pub struct RCC_APB2SMENR_SPEC;
impl crate::RegisterSpec for RCC_APB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2smenr::R](R) reader structure"]
impl crate::Readable for RCC_APB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2smenr::W](W) writer structure"]
impl crate::Writable for RCC_APB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB2SMENR to value 0x0437_f801"]
impl crate::Resettable for RCC_APB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0437_f801;
}
