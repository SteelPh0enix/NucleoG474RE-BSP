#[doc = "Register `RCC_APB1RSTR1` reader"]
pub struct R(crate::R<RCC_APB1RSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1RSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1RSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1RSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1RSTR1` writer"]
pub struct W(crate::W<RCC_APB1RSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1RSTR1_SPEC>;
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
impl From<crate::W<RCC_APB1RSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1RSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 timer reset Set and cleared by software."]
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
#[doc = "TIM2 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM2"]
    B_0X1 = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2RST_A {
        match self.bits {
            false => TIM2RST_A::B_0X0,
            true => TIM2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM2RST_A::B_0X1
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 timer reset Set and cleared by software."]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, TIM2RST_A>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM2RST_A::B_0X0)
    }
    #[doc = "Reset TIM2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM2RST_A::B_0X1)
    }
}
#[doc = "Field `TIM3RST` reader - TIM3 timer reset Set and cleared by software."]
pub type TIM3RST_R = crate::BitReader<TIM3RST_A>;
#[doc = "TIM3 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM3"]
    B_0X1 = 1,
}
impl From<TIM3RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM3RST_A {
        match self.bits {
            false => TIM3RST_A::B_0X0,
            true => TIM3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM3RST_A::B_0X1
    }
}
#[doc = "Field `TIM3RST` writer - TIM3 timer reset Set and cleared by software."]
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, TIM3RST_A>;
impl<'a, const O: u8> TIM3RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM3RST_A::B_0X0)
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM3RST_A::B_0X1)
    }
}
#[doc = "Field `TIM4RST` reader - TIM3 timer reset Set and cleared by software."]
pub type TIM4RST_R = crate::BitReader<TIM4RST_A>;
#[doc = "TIM3 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM4RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM3"]
    B_0X1 = 1,
}
impl From<TIM4RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM4RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM4RST_A {
        match self.bits {
            false => TIM4RST_A::B_0X0,
            true => TIM4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM4RST_A::B_0X1
    }
}
#[doc = "Field `TIM4RST` writer - TIM3 timer reset Set and cleared by software."]
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, TIM4RST_A>;
impl<'a, const O: u8> TIM4RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM4RST_A::B_0X0)
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM4RST_A::B_0X1)
    }
}
#[doc = "Field `TIM5RST` reader - TIM5 timer reset Set and cleared by software."]
pub type TIM5RST_R = crate::BitReader<TIM5RST_A>;
#[doc = "TIM5 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM5"]
    B_0X1 = 1,
}
impl From<TIM5RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM5RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM5RST_A {
        match self.bits {
            false => TIM5RST_A::B_0X0,
            true => TIM5RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM5RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM5RST_A::B_0X1
    }
}
#[doc = "Field `TIM5RST` writer - TIM5 timer reset Set and cleared by software."]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, TIM5RST_A>;
impl<'a, const O: u8> TIM5RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM5RST_A::B_0X0)
    }
    #[doc = "Reset TIM5"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM5RST_A::B_0X1)
    }
}
#[doc = "Field `TIM6RST` reader - TIM6 timer reset Set and cleared by software."]
pub type TIM6RST_R = crate::BitReader<TIM6RST_A>;
#[doc = "TIM6 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM7"]
    B_0X1 = 1,
}
impl From<TIM6RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM6RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM6RST_A {
        match self.bits {
            false => TIM6RST_A::B_0X0,
            true => TIM6RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM6RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM6RST_A::B_0X1
    }
}
#[doc = "Field `TIM6RST` writer - TIM6 timer reset Set and cleared by software."]
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, TIM6RST_A>;
impl<'a, const O: u8> TIM6RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM6RST_A::B_0X0)
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM6RST_A::B_0X1)
    }
}
#[doc = "Field `TIM7RST` reader - TIM7 timer reset Set and cleared by software."]
pub type TIM7RST_R = crate::BitReader<TIM7RST_A>;
#[doc = "TIM7 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM7"]
    B_0X1 = 1,
}
impl From<TIM7RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM7RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM7RST_A {
        match self.bits {
            false => TIM7RST_A::B_0X0,
            true => TIM7RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM7RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM7RST_A::B_0X1
    }
}
#[doc = "Field `TIM7RST` writer - TIM7 timer reset Set and cleared by software."]
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, TIM7RST_A>;
impl<'a, const O: u8> TIM7RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM7RST_A::B_0X0)
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM7RST_A::B_0X1)
    }
}
#[doc = "Field `CRSRST` reader - CRS reset Set and cleared by software."]
pub type CRSRST_R = crate::BitReader<CRSRST_A>;
#[doc = "CRS reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset CRS"]
    B_0X1 = 1,
}
impl From<CRSRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSRST_A {
        match self.bits {
            false => CRSRST_A::B_0X0,
            true => CRSRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRSRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRSRST_A::B_0X1
    }
}
#[doc = "Field `CRSRST` writer - CRS reset Set and cleared by software."]
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, CRSRST_A>;
impl<'a, const O: u8> CRSRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRSRST_A::B_0X0)
    }
    #[doc = "Reset CRS"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRSRST_A::B_0X1)
    }
}
#[doc = "Field `SPI2RST` reader - SPI2 reset Set and cleared by software."]
pub type SPI2RST_R = crate::BitReader<SPI2RST_A>;
#[doc = "SPI2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset SPI2"]
    B_0X1 = 1,
}
impl From<SPI2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2RST_A {
        match self.bits {
            false => SPI2RST_A::B_0X0,
            true => SPI2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI2RST_A::B_0X1
    }
}
#[doc = "Field `SPI2RST` writer - SPI2 reset Set and cleared by software."]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, SPI2RST_A>;
impl<'a, const O: u8> SPI2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI2RST_A::B_0X0)
    }
    #[doc = "Reset SPI2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI2RST_A::B_0X1)
    }
}
#[doc = "Field `SPI3RST` reader - SPI3 reset Set and cleared by software."]
pub type SPI3RST_R = crate::BitReader<SPI3RST_A>;
#[doc = "SPI3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset SPI3"]
    B_0X1 = 1,
}
impl From<SPI3RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI3RST_A {
        match self.bits {
            false => SPI3RST_A::B_0X0,
            true => SPI3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI3RST_A::B_0X1
    }
}
#[doc = "Field `SPI3RST` writer - SPI3 reset Set and cleared by software."]
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, SPI3RST_A>;
impl<'a, const O: u8> SPI3RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI3RST_A::B_0X0)
    }
    #[doc = "Reset SPI3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI3RST_A::B_0X1)
    }
}
#[doc = "Field `USART2RST` reader - USART2 reset Set and cleared by software."]
pub type USART2RST_R = crate::BitReader<USART2RST_A>;
#[doc = "USART2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset USART2"]
    B_0X1 = 1,
}
impl From<USART2RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2RST_A {
        match self.bits {
            false => USART2RST_A::B_0X0,
            true => USART2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2RST_A::B_0X1
    }
}
#[doc = "Field `USART2RST` writer - USART2 reset Set and cleared by software."]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, USART2RST_A>;
impl<'a, const O: u8> USART2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART2RST_A::B_0X0)
    }
    #[doc = "Reset USART2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART2RST_A::B_0X1)
    }
}
#[doc = "Field `USART3RST` reader - USART3 reset Set and cleared by software."]
pub type USART3RST_R = crate::BitReader<USART3RST_A>;
#[doc = "USART3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset USART3"]
    B_0X1 = 1,
}
impl From<USART3RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART3RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USART3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART3RST_A {
        match self.bits {
            false => USART3RST_A::B_0X0,
            true => USART3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART3RST_A::B_0X1
    }
}
#[doc = "Field `USART3RST` writer - USART3 reset Set and cleared by software."]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, USART3RST_A>;
impl<'a, const O: u8> USART3RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART3RST_A::B_0X0)
    }
    #[doc = "Reset USART3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART3RST_A::B_0X1)
    }
}
#[doc = "Field `UART4RST` reader - UART4 reset Set and cleared by software."]
pub type UART4RST_R = crate::BitReader<UART4RST_A>;
#[doc = "UART4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset UART4"]
    B_0X1 = 1,
}
impl From<UART4RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART4RST_A) -> Self {
        variant as u8 != 0
    }
}
impl UART4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4RST_A {
        match self.bits {
            false => UART4RST_A::B_0X0,
            true => UART4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART4RST_A::B_0X1
    }
}
#[doc = "Field `UART4RST` writer - UART4 reset Set and cleared by software."]
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, UART4RST_A>;
impl<'a, const O: u8> UART4RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART4RST_A::B_0X0)
    }
    #[doc = "Reset UART4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART4RST_A::B_0X1)
    }
}
#[doc = "Field `UART5RST` reader - UART5 reset Set and cleared by software."]
pub type UART5RST_R = crate::BitReader<UART5RST_A>;
#[doc = "UART5 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART5RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset UART5"]
    B_0X1 = 1,
}
impl From<UART5RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART5RST_A) -> Self {
        variant as u8 != 0
    }
}
impl UART5RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5RST_A {
        match self.bits {
            false => UART5RST_A::B_0X0,
            true => UART5RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART5RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART5RST_A::B_0X1
    }
}
#[doc = "Field `UART5RST` writer - UART5 reset Set and cleared by software."]
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, UART5RST_A>;
impl<'a, const O: u8> UART5RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART5RST_A::B_0X0)
    }
    #[doc = "Reset UART5"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART5RST_A::B_0X1)
    }
}
#[doc = "Field `I2C1RST` reader - I2C1 reset Set and cleared by software."]
pub type I2C1RST_R = crate::BitReader<I2C1RST_A>;
#[doc = "I2C1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset I2C1"]
    B_0X1 = 1,
}
impl From<I2C1RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1RST_A {
        match self.bits {
            false => I2C1RST_A::B_0X0,
            true => I2C1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1RST_A::B_0X1
    }
}
#[doc = "Field `I2C1RST` writer - I2C1 reset Set and cleared by software."]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, I2C1RST_A>;
impl<'a, const O: u8> I2C1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C1RST_A::B_0X0)
    }
    #[doc = "Reset I2C1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C1RST_A::B_0X1)
    }
}
#[doc = "Field `I2C2RST` reader - I2C2 reset Set and cleared by software."]
pub type I2C2RST_R = crate::BitReader<I2C2RST_A>;
#[doc = "I2C2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset I2C2"]
    B_0X1 = 1,
}
impl From<I2C2RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2RST_A {
        match self.bits {
            false => I2C2RST_A::B_0X0,
            true => I2C2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2RST_A::B_0X1
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 reset Set and cleared by software."]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, I2C2RST_A>;
impl<'a, const O: u8> I2C2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C2RST_A::B_0X0)
    }
    #[doc = "Reset I2C2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C2RST_A::B_0X1)
    }
}
#[doc = "Field `USBRST` reader - USB device reset Set and reset by software."]
pub type USBRST_R = crate::BitReader<USBRST_A>;
#[doc = "USB device reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset USB device"]
    B_0X1 = 1,
}
impl From<USBRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBRST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRST_A {
        match self.bits {
            false => USBRST_A::B_0X0,
            true => USBRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBRST_A::B_0X1
    }
}
#[doc = "Field `USBRST` writer - USB device reset Set and reset by software."]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, USBRST_A>;
impl<'a, const O: u8> USBRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBRST_A::B_0X0)
    }
    #[doc = "Reset USB device"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBRST_A::B_0X1)
    }
}
#[doc = "Field `FDCANRST` reader - FDCAN reset Set and reset by software."]
pub type FDCANRST_R = crate::BitReader<FDCANRST_A>;
#[doc = "FDCAN reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCANRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset the FDCAN"]
    B_0X1 = 1,
}
impl From<FDCANRST_A> for bool {
    #[inline(always)]
    fn from(variant: FDCANRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FDCANRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCANRST_A {
        match self.bits {
            false => FDCANRST_A::B_0X0,
            true => FDCANRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANRST_A::B_0X1
    }
}
#[doc = "Field `FDCANRST` writer - FDCAN reset Set and reset by software."]
pub type FDCANRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, FDCANRST_A>;
impl<'a, const O: u8> FDCANRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANRST_A::B_0X0)
    }
    #[doc = "Reset the FDCAN"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANRST_A::B_0X1)
    }
}
#[doc = "Field `PWRRST` reader - Power interface reset Set and cleared by software."]
pub type PWRRST_R = crate::BitReader<PWRRST_A>;
#[doc = "Power interface reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset PWR"]
    B_0X1 = 1,
}
impl From<PWRRST_A> for bool {
    #[inline(always)]
    fn from(variant: PWRRST_A) -> Self {
        variant as u8 != 0
    }
}
impl PWRRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRRST_A {
        match self.bits {
            false => PWRRST_A::B_0X0,
            true => PWRRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRRST_A::B_0X1
    }
}
#[doc = "Field `PWRRST` writer - Power interface reset Set and cleared by software."]
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, PWRRST_A>;
impl<'a, const O: u8> PWRRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PWRRST_A::B_0X0)
    }
    #[doc = "Reset PWR"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PWRRST_A::B_0X1)
    }
}
#[doc = "Field `I2C3RST` reader - I2C3 reset Set and cleared by software."]
pub type I2C3RST_R = crate::BitReader<I2C3RST_A>;
#[doc = "I2C3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset I2C3 interface"]
    B_0X1 = 1,
}
impl From<I2C3RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3RST_A {
        match self.bits {
            false => I2C3RST_A::B_0X0,
            true => I2C3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C3RST_A::B_0X1
    }
}
#[doc = "Field `I2C3RST` writer - I2C3 reset Set and cleared by software."]
pub type I2C3RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, I2C3RST_A>;
impl<'a, const O: u8> I2C3RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C3RST_A::B_0X0)
    }
    #[doc = "Reset I2C3 interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C3RST_A::B_0X1)
    }
}
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset Set and cleared by software."]
pub type LPTIM1RST_R = crate::BitReader<LPTIM1RST_A>;
#[doc = "Low Power Timer 1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset LPTIM1"]
    B_0X1 = 1,
}
impl From<LPTIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1RST_A {
        match self.bits {
            false => LPTIM1RST_A::B_0X0,
            true => LPTIM1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM1RST_A::B_0X1
    }
}
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset Set and cleared by software."]
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1RSTR1_SPEC, O, LPTIM1RST_A>;
impl<'a, const O: u8> LPTIM1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM1RST_A::B_0X0)
    }
    #[doc = "Reset LPTIM1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM1RST_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRS reset Set and cleared by software."]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 reset Set and cleared by software."]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB device reset Set and reset by software."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset Set and cleared by software."]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 8 - CRS reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<8> {
        CRSRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - UART4 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 20 - UART5 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - USB device reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<23> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<25> {
        FDCANRST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<30> {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<31> {
        LPTIM1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1rstr1](index.html) module"]
pub struct RCC_APB1RSTR1_SPEC;
impl crate::RegisterSpec for RCC_APB1RSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1rstr1::R](R) reader structure"]
impl crate::Readable for RCC_APB1RSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1rstr1::W](W) writer structure"]
impl crate::Writable for RCC_APB1RSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB1RSTR1 to value 0"]
impl crate::Resettable for RCC_APB1RSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
