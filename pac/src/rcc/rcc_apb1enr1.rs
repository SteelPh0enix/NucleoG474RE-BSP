#[doc = "Register `RCC_APB1ENR1` reader"]
pub struct R(crate::R<RCC_APB1ENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1ENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1ENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1ENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1ENR1` writer"]
pub struct W(crate::W<RCC_APB1ENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1ENR1_SPEC>;
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
impl From<crate::W<RCC_APB1ENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1ENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable Set and cleared by software."]
pub type TIM2EN_R = crate::BitReader<TIM2EN_A>;
#[doc = "TIM2 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN_A {
    #[doc = "0: TIM2 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM2 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2EN_A {
        match self.bits {
            false => TIM2EN_A::B_0X0,
            true => TIM2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM2EN_A::B_0X1
    }
}
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable Set and cleared by software."]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, TIM2EN_A>;
impl<'a, const O: u8> TIM2EN_W<'a, O> {
    #[doc = "TIM2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM2EN_A::B_0X0)
    }
    #[doc = "TIM2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM2EN_A::B_0X1)
    }
}
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software."]
pub type TIM3EN_R = crate::BitReader<TIM3EN_A>;
#[doc = "TIM3 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3EN_A {
    #[doc = "0: TIM3 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM3 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM3EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM3EN_A {
        match self.bits {
            false => TIM3EN_A::B_0X0,
            true => TIM3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM3EN_A::B_0X1
    }
}
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software."]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, TIM3EN_A>;
impl<'a, const O: u8> TIM3EN_W<'a, O> {
    #[doc = "TIM3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM3EN_A::B_0X0)
    }
    #[doc = "TIM3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM3EN_A::B_0X1)
    }
}
#[doc = "Field `TIM4EN` reader - TIM4 timer clock enable Set and cleared by software."]
pub type TIM4EN_R = crate::BitReader<TIM4EN_A>;
#[doc = "TIM4 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM4EN_A {
    #[doc = "0: TIM4 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM4 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM4EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM4EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM4EN_A {
        match self.bits {
            false => TIM4EN_A::B_0X0,
            true => TIM4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM4EN_A::B_0X1
    }
}
#[doc = "Field `TIM4EN` writer - TIM4 timer clock enable Set and cleared by software."]
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, TIM4EN_A>;
impl<'a, const O: u8> TIM4EN_W<'a, O> {
    #[doc = "TIM4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM4EN_A::B_0X0)
    }
    #[doc = "TIM4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM4EN_A::B_0X1)
    }
}
#[doc = "Field `TIM5EN` reader - TIM5 timer clock enable Set and cleared by software."]
pub type TIM5EN_R = crate::BitReader<TIM5EN_A>;
#[doc = "TIM5 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5EN_A {
    #[doc = "0: TIM5 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM5 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM5EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM5EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM5EN_A {
        match self.bits {
            false => TIM5EN_A::B_0X0,
            true => TIM5EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM5EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM5EN_A::B_0X1
    }
}
#[doc = "Field `TIM5EN` writer - TIM5 timer clock enable Set and cleared by software."]
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, TIM5EN_A>;
impl<'a, const O: u8> TIM5EN_W<'a, O> {
    #[doc = "TIM5 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM5EN_A::B_0X0)
    }
    #[doc = "TIM5 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM5EN_A::B_0X1)
    }
}
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable Set and cleared by software."]
pub type TIM6EN_R = crate::BitReader<TIM6EN_A>;
#[doc = "TIM6 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6EN_A {
    #[doc = "0: TIM6 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM6 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM6EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM6EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM6EN_A {
        match self.bits {
            false => TIM6EN_A::B_0X0,
            true => TIM6EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM6EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM6EN_A::B_0X1
    }
}
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable Set and cleared by software."]
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, TIM6EN_A>;
impl<'a, const O: u8> TIM6EN_W<'a, O> {
    #[doc = "TIM6 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM6EN_A::B_0X0)
    }
    #[doc = "TIM6 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM6EN_A::B_0X1)
    }
}
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable Set and cleared by software."]
pub type TIM7EN_R = crate::BitReader<TIM7EN_A>;
#[doc = "TIM7 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7EN_A {
    #[doc = "0: TIM7 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: TIM7 clock enabled"]
    B_0X1 = 1,
}
impl From<TIM7EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM7EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM7EN_A {
        match self.bits {
            false => TIM7EN_A::B_0X0,
            true => TIM7EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM7EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM7EN_A::B_0X1
    }
}
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable Set and cleared by software."]
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, TIM7EN_A>;
impl<'a, const O: u8> TIM7EN_W<'a, O> {
    #[doc = "TIM7 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM7EN_A::B_0X0)
    }
    #[doc = "TIM7 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM7EN_A::B_0X1)
    }
}
#[doc = "Field `CRSEN` reader - CRS Recovery System clock enable Set and cleared by software."]
pub type CRSEN_R = crate::BitReader<CRSEN_A>;
#[doc = "CRS Recovery System clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSEN_A {
    #[doc = "0: CRS clock disabled"]
    B_0X0 = 0,
    #[doc = "1: CRS clock enabled"]
    B_0X1 = 1,
}
impl From<CRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSEN_A {
        match self.bits {
            false => CRSEN_A::B_0X0,
            true => CRSEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRSEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRSEN_A::B_0X1
    }
}
#[doc = "Field `CRSEN` writer - CRS Recovery System clock enable Set and cleared by software."]
pub type CRSEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, CRSEN_A>;
impl<'a, const O: u8> CRSEN_W<'a, O> {
    #[doc = "CRS clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRSEN_A::B_0X0)
    }
    #[doc = "CRS clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRSEN_A::B_0X1)
    }
}
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software"]
pub type RTCAPBEN_R = crate::BitReader<RTCAPBEN_A>;
#[doc = "RTC APB clock enable Set and cleared by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBEN_A {
    #[doc = "0: RTC APB clock disabled"]
    B_0X0 = 0,
    #[doc = "1: RTC APB clock enabled"]
    B_0X1 = 1,
}
impl From<RTCAPBEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCAPBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAPBEN_A {
        match self.bits {
            false => RTCAPBEN_A::B_0X0,
            true => RTCAPBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCAPBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCAPBEN_A::B_0X1
    }
}
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software"]
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, RTCAPBEN_A>;
impl<'a, const O: u8> RTCAPBEN_W<'a, O> {
    #[doc = "RTC APB clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCAPBEN_A::B_0X0)
    }
    #[doc = "RTC APB clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCAPBEN_A::B_0X1)
    }
}
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
pub type WWDGEN_R = crate::BitReader<WWDGEN_A>;
#[doc = "Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGEN_A {
    #[doc = "0: Window watchdog clock disabled"]
    B_0X0 = 0,
    #[doc = "1: Window watchdog clock enabled"]
    B_0X1 = 1,
}
impl From<WWDGEN_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDGEN_A {
        match self.bits {
            false => WWDGEN_A::B_0X0,
            true => WWDGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDGEN_A::B_0X1
    }
}
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, WWDGEN_A>;
impl<'a, const O: u8> WWDGEN_W<'a, O> {
    #[doc = "Window watchdog clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WWDGEN_A::B_0X0)
    }
    #[doc = "Window watchdog clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WWDGEN_A::B_0X1)
    }
}
#[doc = "Field `SPI2EN` reader - SPI2 clock enable Set and cleared by software."]
pub type SPI2EN_R = crate::BitReader<SPI2EN_A>;
#[doc = "SPI2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2EN_A {
    #[doc = "0: SPI2 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: SPI2 clock enabled"]
    B_0X1 = 1,
}
impl From<SPI2EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2EN_A {
        match self.bits {
            false => SPI2EN_A::B_0X0,
            true => SPI2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI2EN_A::B_0X1
    }
}
#[doc = "Field `SPI2EN` writer - SPI2 clock enable Set and cleared by software."]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, SPI2EN_A>;
impl<'a, const O: u8> SPI2EN_W<'a, O> {
    #[doc = "SPI2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI2EN_A::B_0X0)
    }
    #[doc = "SPI2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI2EN_A::B_0X1)
    }
}
#[doc = "Field `SPI3EN` reader - SPI3 clock enable Set and cleared by software."]
pub type SPI3EN_R = crate::BitReader<SPI3EN_A>;
#[doc = "SPI3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3EN_A {
    #[doc = "0: SPI3 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: SPI3 clock enabled"]
    B_0X1 = 1,
}
impl From<SPI3EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI3EN_A {
        match self.bits {
            false => SPI3EN_A::B_0X0,
            true => SPI3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI3EN_A::B_0X1
    }
}
#[doc = "Field `SPI3EN` writer - SPI3 clock enable Set and cleared by software."]
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, SPI3EN_A>;
impl<'a, const O: u8> SPI3EN_W<'a, O> {
    #[doc = "SPI3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI3EN_A::B_0X0)
    }
    #[doc = "SPI3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI3EN_A::B_0X1)
    }
}
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and cleared by software."]
pub type USART2EN_R = crate::BitReader<USART2EN_A>;
#[doc = "USART2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2EN_A {
    #[doc = "0: USART2 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: USART2 clock enabled"]
    B_0X1 = 1,
}
impl From<USART2EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2EN_A {
        match self.bits {
            false => USART2EN_A::B_0X0,
            true => USART2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2EN_A::B_0X1
    }
}
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and cleared by software."]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, USART2EN_A>;
impl<'a, const O: u8> USART2EN_W<'a, O> {
    #[doc = "USART2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART2EN_A::B_0X0)
    }
    #[doc = "USART2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART2EN_A::B_0X1)
    }
}
#[doc = "Field `USART3EN` reader - USART3 clock enable Set and cleared by software."]
pub type USART3EN_R = crate::BitReader<USART3EN_A>;
#[doc = "USART3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3EN_A {
    #[doc = "0: USART3 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: USART3 clock enabled"]
    B_0X1 = 1,
}
impl From<USART3EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART3EN_A {
        match self.bits {
            false => USART3EN_A::B_0X0,
            true => USART3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART3EN_A::B_0X1
    }
}
#[doc = "Field `USART3EN` writer - USART3 clock enable Set and cleared by software."]
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, USART3EN_A>;
impl<'a, const O: u8> USART3EN_W<'a, O> {
    #[doc = "USART3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART3EN_A::B_0X0)
    }
    #[doc = "USART3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART3EN_A::B_0X1)
    }
}
#[doc = "Field `UART4EN` reader - UART4 clock enable Set and cleared by software."]
pub type UART4EN_R = crate::BitReader<UART4EN_A>;
#[doc = "UART4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4EN_A {
    #[doc = "0: UART4 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: UART4 clock enabled"]
    B_0X1 = 1,
}
impl From<UART4EN_A> for bool {
    #[inline(always)]
    fn from(variant: UART4EN_A) -> Self {
        variant as u8 != 0
    }
}
impl UART4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4EN_A {
        match self.bits {
            false => UART4EN_A::B_0X0,
            true => UART4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART4EN_A::B_0X1
    }
}
#[doc = "Field `UART4EN` writer - UART4 clock enable Set and cleared by software."]
pub type UART4EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, UART4EN_A>;
impl<'a, const O: u8> UART4EN_W<'a, O> {
    #[doc = "UART4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART4EN_A::B_0X0)
    }
    #[doc = "UART4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART4EN_A::B_0X1)
    }
}
#[doc = "Field `UART5EN` reader - UART5 clock enable Set and cleared by software."]
pub type UART5EN_R = crate::BitReader<UART5EN_A>;
#[doc = "UART5 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART5EN_A {
    #[doc = "0: UART5 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: UART5 clock enabled"]
    B_0X1 = 1,
}
impl From<UART5EN_A> for bool {
    #[inline(always)]
    fn from(variant: UART5EN_A) -> Self {
        variant as u8 != 0
    }
}
impl UART5EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5EN_A {
        match self.bits {
            false => UART5EN_A::B_0X0,
            true => UART5EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART5EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART5EN_A::B_0X1
    }
}
#[doc = "Field `UART5EN` writer - UART5 clock enable Set and cleared by software."]
pub type UART5EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, UART5EN_A>;
impl<'a, const O: u8> UART5EN_W<'a, O> {
    #[doc = "UART5 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART5EN_A::B_0X0)
    }
    #[doc = "UART5 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART5EN_A::B_0X1)
    }
}
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software."]
pub type I2C1EN_R = crate::BitReader<I2C1EN_A>;
#[doc = "I2C1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1EN_A {
    #[doc = "0: I2C1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: I2C1 clock enabled"]
    B_0X1 = 1,
}
impl From<I2C1EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1EN_A {
        match self.bits {
            false => I2C1EN_A::B_0X0,
            true => I2C1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1EN_A::B_0X1
    }
}
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software."]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, I2C1EN_A>;
impl<'a, const O: u8> I2C1EN_W<'a, O> {
    #[doc = "I2C1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C1EN_A::B_0X0)
    }
    #[doc = "I2C1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C1EN_A::B_0X1)
    }
}
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and cleared by software."]
pub type I2C2EN_R = crate::BitReader<I2C2EN_A>;
#[doc = "I2C2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2EN_A {
    #[doc = "0: I2C2 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: I2C2 clock enabled"]
    B_0X1 = 1,
}
impl From<I2C2EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2EN_A {
        match self.bits {
            false => I2C2EN_A::B_0X0,
            true => I2C2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2EN_A::B_0X1
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and cleared by software."]
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, I2C2EN_A>;
impl<'a, const O: u8> I2C2EN_W<'a, O> {
    #[doc = "I2C2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C2EN_A::B_0X0)
    }
    #[doc = "I2C2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C2EN_A::B_0X1)
    }
}
#[doc = "Field `USBEN` reader - USB device clock enable Set and cleared by software."]
pub type USBEN_R = crate::BitReader<USBEN_A>;
#[doc = "USB device clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBEN_A {
    #[doc = "0: USB device clock disabled"]
    B_0X0 = 0,
    #[doc = "1: USB device clock enabled"]
    B_0X1 = 1,
}
impl From<USBEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEN_A {
        match self.bits {
            false => USBEN_A::B_0X0,
            true => USBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBEN_A::B_0X1
    }
}
#[doc = "Field `USBEN` writer - USB device clock enable Set and cleared by software."]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, USBEN_A>;
impl<'a, const O: u8> USBEN_W<'a, O> {
    #[doc = "USB device clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBEN_A::B_0X0)
    }
    #[doc = "USB device clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBEN_A::B_0X1)
    }
}
#[doc = "Field `FDCANEN` reader - FDCAN clock enable Set and cleared by software."]
pub type FDCANEN_R = crate::BitReader<FDCANEN_A>;
#[doc = "FDCAN clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCANEN_A {
    #[doc = "0: FDCAN clock disabled"]
    B_0X0 = 0,
    #[doc = "1: FDCAN clock enabled"]
    B_0X1 = 1,
}
impl From<FDCANEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCANEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FDCANEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCANEN_A {
        match self.bits {
            false => FDCANEN_A::B_0X0,
            true => FDCANEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANEN_A::B_0X1
    }
}
#[doc = "Field `FDCANEN` writer - FDCAN clock enable Set and cleared by software."]
pub type FDCANEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, FDCANEN_A>;
impl<'a, const O: u8> FDCANEN_W<'a, O> {
    #[doc = "FDCAN clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANEN_A::B_0X0)
    }
    #[doc = "FDCAN clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANEN_A::B_0X1)
    }
}
#[doc = "Field `PWREN` reader - Power interface clock enable Set and cleared by software."]
pub type PWREN_R = crate::BitReader<PWREN_A>;
#[doc = "Power interface clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWREN_A {
    #[doc = "0: Power interface clock disabled"]
    B_0X0 = 0,
    #[doc = "1: Power interface clock enabled"]
    B_0X1 = 1,
}
impl From<PWREN_A> for bool {
    #[inline(always)]
    fn from(variant: PWREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWREN_A {
        match self.bits {
            false => PWREN_A::B_0X0,
            true => PWREN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWREN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWREN_A::B_0X1
    }
}
#[doc = "Field `PWREN` writer - Power interface clock enable Set and cleared by software."]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, PWREN_A>;
impl<'a, const O: u8> PWREN_W<'a, O> {
    #[doc = "Power interface clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PWREN_A::B_0X0)
    }
    #[doc = "Power interface clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PWREN_A::B_0X1)
    }
}
#[doc = "Field `I2C3EN` reader - I2C3 clock enable Set and cleared by software."]
pub type I2C3EN_R = crate::BitReader<I2C3EN_A>;
#[doc = "I2C3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3EN_A {
    #[doc = "0: I2C3 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: I2C3 clock enabled"]
    B_0X1 = 1,
}
impl From<I2C3EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3EN_A {
        match self.bits {
            false => I2C3EN_A::B_0X0,
            true => I2C3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C3EN_A::B_0X1
    }
}
#[doc = "Field `I2C3EN` writer - I2C3 clock enable Set and cleared by software."]
pub type I2C3EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, I2C3EN_A>;
impl<'a, const O: u8> I2C3EN_W<'a, O> {
    #[doc = "I2C3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C3EN_A::B_0X0)
    }
    #[doc = "I2C3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C3EN_A::B_0X1)
    }
}
#[doc = "Field `LPTIM1EN` reader - Low power timer 1 clock enable Set and cleared by software."]
pub type LPTIM1EN_R = crate::BitReader<LPTIM1EN_A>;
#[doc = "Low power timer 1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1EN_A {
    #[doc = "0: LPTIM1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: LPTIM1 clock enabled"]
    B_0X1 = 1,
}
impl From<LPTIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1EN_A {
        match self.bits {
            false => LPTIM1EN_A::B_0X0,
            true => LPTIM1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM1EN_A::B_0X1
    }
}
#[doc = "Field `LPTIM1EN` writer - Low power timer 1 clock enable Set and cleared by software."]
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1ENR1_SPEC, O, LPTIM1EN_A>;
impl<'a, const O: u8> LPTIM1EN_W<'a, O> {
    #[doc = "LPTIM1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM1EN_A::B_0X0)
    }
    #[doc = "LPTIM1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM1EN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRS Recovery System clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB device clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 8 - CRS Recovery System clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<8> {
        CRSEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<10> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<19> {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<20> {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - USB device clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<23> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<25> {
        FDCANEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<30> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<31> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1enr1](index.html) module"]
pub struct RCC_APB1ENR1_SPEC;
impl crate::RegisterSpec for RCC_APB1ENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1enr1::R](R) reader structure"]
impl crate::Readable for RCC_APB1ENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1enr1::W](W) writer structure"]
impl crate::Writable for RCC_APB1ENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB1ENR1 to value 0x0400"]
impl crate::Resettable for RCC_APB1ENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
