#[doc = "Register `RCC_APB2ENR` reader"]
pub struct R(crate::R<RCC_APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2ENR` writer"]
pub struct W(crate::W<RCC_APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2ENR_SPEC>;
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
impl From<crate::W<RCC_APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN_A>;
#[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN_A {
    #[doc = "0: SYSCFG + COMP + VREFBUF + OPAMP clock disabled"]
    B_0X0 = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF + OPAMP clock enabled"]
    B_0X1 = 1,
}
impl From<SYSCFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGEN_A {
        match self.bits {
            false => SYSCFGEN_A::B_0X0,
            true => SYSCFGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGEN_A::B_0X1
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, SYSCFGEN_A>;
impl<'a, const O: u8> SYSCFGEN_W<'a, O> {
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::B_0X0)
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::B_0X1)
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable Set and cleared by software."]
pub type TIM1EN_R = crate::BitReader<TIM1EN_A>;
#[doc = "TIM1 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN_A {
    #[doc = "0: TIM1 timer clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM1P timer clock enabled"]
    B_0X1 = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::B_0X0,
            true => TIM1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1EN_A::B_0X1
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable Set and cleared by software."]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, TIM1EN_A>;
impl<'a, const O: u8> TIM1EN_W<'a, O> {
    #[doc = "TIM1 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM1EN_A::B_0X0)
    }
    #[doc = "TIM1P timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM1EN_A::B_0X1)
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and cleared by software."]
pub type SPI1EN_R = crate::BitReader<SPI1EN_A>;
#[doc = "SPI1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1EN_A {
    #[doc = "0: SPI1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: SPI1 clock enabled"]
    B_0X1 = 1,
}
impl From<SPI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1EN_A {
        match self.bits {
            false => SPI1EN_A::B_0X0,
            true => SPI1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1EN_A::B_0X1
    }
}
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and cleared by software."]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, SPI1EN_A>;
impl<'a, const O: u8> SPI1EN_W<'a, O> {
    #[doc = "SPI1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI1EN_A::B_0X0)
    }
    #[doc = "SPI1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI1EN_A::B_0X1)
    }
}
#[doc = "Field `TIM8EN` reader - TIM8 timer clock enable Set and cleared by software."]
pub type TIM8EN_R = crate::BitReader<TIM8EN_A>;
#[doc = "TIM8 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM8EN_A {
    #[doc = "0: TIM8 timer clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM8 timer clock enabled"]
    B_0X1 = 1,
}
impl From<TIM8EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM8EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM8EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM8EN_A {
        match self.bits {
            false => TIM8EN_A::B_0X0,
            true => TIM8EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM8EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM8EN_A::B_0X1
    }
}
#[doc = "Field `TIM8EN` writer - TIM8 timer clock enable Set and cleared by software."]
pub type TIM8EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, TIM8EN_A>;
impl<'a, const O: u8> TIM8EN_W<'a, O> {
    #[doc = "TIM8 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM8EN_A::B_0X0)
    }
    #[doc = "TIM8 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM8EN_A::B_0X1)
    }
}
#[doc = "Field `USART1EN` reader - USART1clock enable Set and cleared by software."]
pub type USART1EN_R = crate::BitReader<USART1EN_A>;
#[doc = "USART1clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN_A {
    #[doc = "0: USART1clock disabled"]
    B_0X0 = 0,
    #[doc = "1: USART1clock enabled"]
    B_0X1 = 1,
}
impl From<USART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1EN_A {
        match self.bits {
            false => USART1EN_A::B_0X0,
            true => USART1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1EN_A::B_0X1
    }
}
#[doc = "Field `USART1EN` writer - USART1clock enable Set and cleared by software."]
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, USART1EN_A>;
impl<'a, const O: u8> USART1EN_W<'a, O> {
    #[doc = "USART1clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1EN_A::B_0X0)
    }
    #[doc = "USART1clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1EN_A::B_0X1)
    }
}
#[doc = "Field `SPI4EN` reader - SPI4 clock enable Set and cleared by software."]
pub type SPI4EN_R = crate::BitReader<SPI4EN_A>;
#[doc = "SPI4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI4EN_A {
    #[doc = "0: SPI4 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: SPI4 clock enabled"]
    B_0X1 = 1,
}
impl From<SPI4EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI4EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI4EN_A {
        match self.bits {
            false => SPI4EN_A::B_0X0,
            true => SPI4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI4EN_A::B_0X1
    }
}
#[doc = "Field `SPI4EN` writer - SPI4 clock enable Set and cleared by software."]
pub type SPI4EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, SPI4EN_A>;
impl<'a, const O: u8> SPI4EN_W<'a, O> {
    #[doc = "SPI4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI4EN_A::B_0X0)
    }
    #[doc = "SPI4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI4EN_A::B_0X1)
    }
}
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable Set and cleared by software."]
pub type TIM15EN_R = crate::BitReader<TIM15EN_A>;
#[doc = "TIM15 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM15EN_A {
    #[doc = "0: TIM15 timer clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM15 timer clock enabled"]
    B_0X1 = 1,
}
impl From<TIM15EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM15EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM15EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM15EN_A {
        match self.bits {
            false => TIM15EN_A::B_0X0,
            true => TIM15EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM15EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM15EN_A::B_0X1
    }
}
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable Set and cleared by software."]
pub type TIM15EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, TIM15EN_A>;
impl<'a, const O: u8> TIM15EN_W<'a, O> {
    #[doc = "TIM15 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM15EN_A::B_0X0)
    }
    #[doc = "TIM15 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM15EN_A::B_0X1)
    }
}
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software."]
pub type TIM16EN_R = crate::BitReader<TIM16EN_A>;
#[doc = "TIM16 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16EN_A {
    #[doc = "0: TIM16 timer clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM16 timer clock enabled"]
    B_0X1 = 1,
}
impl From<TIM16EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM16EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16EN_A {
        match self.bits {
            false => TIM16EN_A::B_0X0,
            true => TIM16EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16EN_A::B_0X1
    }
}
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software."]
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, TIM16EN_A>;
impl<'a, const O: u8> TIM16EN_W<'a, O> {
    #[doc = "TIM16 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM16EN_A::B_0X0)
    }
    #[doc = "TIM16 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM16EN_A::B_0X1)
    }
}
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable Set and cleared by software."]
pub type TIM17EN_R = crate::BitReader<TIM17EN_A>;
#[doc = "TIM17 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17EN_A {
    #[doc = "0: TIM17 timer clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM17 timer clock enabled"]
    B_0X1 = 1,
}
impl From<TIM17EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM17EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17EN_A {
        match self.bits {
            false => TIM17EN_A::B_0X0,
            true => TIM17EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17EN_A::B_0X1
    }
}
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable Set and cleared by software."]
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, TIM17EN_A>;
impl<'a, const O: u8> TIM17EN_W<'a, O> {
    #[doc = "TIM17 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM17EN_A::B_0X0)
    }
    #[doc = "TIM17 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM17EN_A::B_0X1)
    }
}
#[doc = "Field `TIM20EN` reader - TIM20 timer clock enable Set and cleared by software."]
pub type TIM20EN_R = crate::BitReader<TIM20EN_A>;
#[doc = "TIM20 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM20EN_A {
    #[doc = "0: TIM20 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM20 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM20EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM20EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM20EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM20EN_A {
        match self.bits {
            false => TIM20EN_A::B_0X0,
            true => TIM20EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM20EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM20EN_A::B_0X1
    }
}
#[doc = "Field `TIM20EN` writer - TIM20 timer clock enable Set and cleared by software."]
pub type TIM20EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, TIM20EN_A>;
impl<'a, const O: u8> TIM20EN_W<'a, O> {
    #[doc = "TIM20 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM20EN_A::B_0X0)
    }
    #[doc = "TIM20 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM20EN_A::B_0X1)
    }
}
#[doc = "Field `SAI1EN` reader - SAI1 clock enable Set and cleared by software."]
pub type SAI1EN_R = crate::BitReader<SAI1EN_A>;
#[doc = "SAI1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1EN_A {
    #[doc = "0: SAI1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: SAI1 clock enabled"]
    B_0X1 = 1,
}
impl From<SAI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1EN_A {
        match self.bits {
            false => SAI1EN_A::B_0X0,
            true => SAI1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1EN_A::B_0X1
    }
}
#[doc = "Field `SAI1EN` writer - SAI1 clock enable Set and cleared by software."]
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, SAI1EN_A>;
impl<'a, const O: u8> SAI1EN_W<'a, O> {
    #[doc = "SAI1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1EN_A::B_0X0)
    }
    #[doc = "SAI1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1EN_A::B_0X1)
    }
}
#[doc = "Field `HRTIM1EN` reader - HRTIM1 clock enable Set and cleared by software."]
pub type HRTIM1EN_R = crate::BitReader<HRTIM1EN_A>;
#[doc = "HRTIM1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRTIM1EN_A {
    #[doc = "0: HRTIM1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: HRTIM1 clock enable"]
    B_0X1 = 1,
}
impl From<HRTIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: HRTIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HRTIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRTIM1EN_A {
        match self.bits {
            false => HRTIM1EN_A::B_0X0,
            true => HRTIM1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HRTIM1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HRTIM1EN_A::B_0X1
    }
}
#[doc = "Field `HRTIM1EN` writer - HRTIM1 clock enable Set and cleared by software."]
pub type HRTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2ENR_SPEC, O, HRTIM1EN_A>;
impl<'a, const O: u8> HRTIM1EN_W<'a, O> {
    #[doc = "HRTIM1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HRTIM1EN_A::B_0X0)
    }
    #[doc = "HRTIM1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HRTIM1EN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim20en(&self) -> TIM20EN_R {
        TIM20EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1en(&self) -> HRTIM1EN_R {
        HRTIM1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI4 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<15> {
        SPI4EN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<16> {
        TIM15EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 20 - TIM20 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim20en(&mut self) -> TIM20EN_W<20> {
        TIM20EN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<21> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 26 - HRTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn hrtim1en(&mut self) -> HRTIM1EN_W<26> {
        HRTIM1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2enr](index.html) module"]
pub struct RCC_APB2ENR_SPEC;
impl crate::RegisterSpec for RCC_APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2enr::R](R) reader structure"]
impl crate::Readable for RCC_APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2enr::W](W) writer structure"]
impl crate::Writable for RCC_APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB2ENR to value 0"]
impl crate::Resettable for RCC_APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
