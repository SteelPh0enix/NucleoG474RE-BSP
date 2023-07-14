#[doc = "Register `RCC_APB1SMENR1` reader"]
pub struct R(crate::R<RCC_APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1SMENR1` writer"]
pub struct W(crate::W<RCC_APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1SMENR1_SPEC>;
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
impl From<crate::W<RCC_APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN_A>;
#[doc = "TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN_A {
    #[doc = "0: TIM2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2SMEN_A {
        match self.bits {
            false => TIM2SMEN_A::B_0X0,
            true => TIM2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM2SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, TIM2SMEN_A>;
impl<'a, const O: u8> TIM2SMEN_W<'a, O> {
    #[doc = "TIM2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::B_0X0)
    }
    #[doc = "TIM2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM3SMEN_R = crate::BitReader<TIM3SMEN_A>;
#[doc = "TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3SMEN_A {
    #[doc = "0: TIM3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM3SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM3SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM3SMEN_A {
        match self.bits {
            false => TIM3SMEN_A::B_0X0,
            true => TIM3SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM3SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM3SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, TIM3SMEN_A>;
impl<'a, const O: u8> TIM3SMEN_W<'a, O> {
    #[doc = "TIM3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM3SMEN_A::B_0X0)
    }
    #[doc = "TIM3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM3SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM4SMEN_R = crate::BitReader<TIM4SMEN_A>;
#[doc = "TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM4SMEN_A {
    #[doc = "0: TIM4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM4SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM4SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM4SMEN_A {
        match self.bits {
            false => TIM4SMEN_A::B_0X0,
            true => TIM4SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM4SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM4SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, TIM4SMEN_A>;
impl<'a, const O: u8> TIM4SMEN_W<'a, O> {
    #[doc = "TIM4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM4SMEN_A::B_0X0)
    }
    #[doc = "TIM4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM4SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM5SMEN_R = crate::BitReader<TIM5SMEN_A>;
#[doc = "TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5SMEN_A {
    #[doc = "0: TIM5 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM5 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM5SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM5SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM5SMEN_A {
        match self.bits {
            false => TIM5SMEN_A::B_0X0,
            true => TIM5SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM5SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM5SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, TIM5SMEN_A>;
impl<'a, const O: u8> TIM5SMEN_W<'a, O> {
    #[doc = "TIM5 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM5SMEN_A::B_0X0)
    }
    #[doc = "TIM5 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM5SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM6SMEN_R = crate::BitReader<TIM6SMEN_A>;
#[doc = "TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6SMEN_A {
    #[doc = "0: TIM6 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM6 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM6SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM6SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM6SMEN_A {
        match self.bits {
            false => TIM6SMEN_A::B_0X0,
            true => TIM6SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM6SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM6SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM6SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, TIM6SMEN_A>;
impl<'a, const O: u8> TIM6SMEN_W<'a, O> {
    #[doc = "TIM6 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM6SMEN_A::B_0X0)
    }
    #[doc = "TIM6 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM6SMEN_A::B_0X1)
    }
}
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM7SMEN_R = crate::BitReader<TIM7SMEN_A>;
#[doc = "TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7SMEN_A {
    #[doc = "0: TIM7 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: TIM7 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<TIM7SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM7SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM7SMEN_A {
        match self.bits {
            false => TIM7SMEN_A::B_0X0,
            true => TIM7SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM7SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM7SMEN_A::B_0X1
    }
}
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type TIM7SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, TIM7SMEN_A>;
impl<'a, const O: u8> TIM7SMEN_W<'a, O> {
    #[doc = "TIM7 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM7SMEN_A::B_0X0)
    }
    #[doc = "TIM7 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM7SMEN_A::B_0X1)
    }
}
#[doc = "Field `CRSSMEN` reader - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CRSSMEN_R = crate::BitReader<CRSSMEN_A>;
#[doc = "CRS timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSSMEN_A {
    #[doc = "0: CRS clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: CRS clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<CRSSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSSMEN_A {
        match self.bits {
            false => CRSSMEN_A::B_0X0,
            true => CRSSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRSSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRSSMEN_A::B_0X1
    }
}
#[doc = "Field `CRSSMEN` writer - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CRSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, CRSSMEN_A>;
impl<'a, const O: u8> CRSSMEN_W<'a, O> {
    #[doc = "CRS clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRSSMEN_A::B_0X0)
    }
    #[doc = "CRS clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRSSMEN_A::B_0X1)
    }
}
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
pub type RTCAPBSMEN_R = crate::BitReader<RTCAPBSMEN_A>;
#[doc = "RTC APB clock enable during Sleep and Stop modes Set and cleared by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBSMEN_A {
    #[doc = "0: RTC APB clock disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: RTC APB clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<RTCAPBSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCAPBSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAPBSMEN_A {
        match self.bits {
            false => RTCAPBSMEN_A::B_0X0,
            true => RTCAPBSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCAPBSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCAPBSMEN_A::B_0X1
    }
}
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
pub type RTCAPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, RTCAPBSMEN_A>;
impl<'a, const O: u8> RTCAPBSMEN_W<'a, O> {
    #[doc = "RTC APB clock disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCAPBSMEN_A::B_0X0)
    }
    #[doc = "RTC APB clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCAPBSMEN_A::B_0X1)
    }
}
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
pub type WWDGSMEN_R = crate::BitReader<WWDGSMEN_A>;
#[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGSMEN_A {
    #[doc = "0: Window watchdog clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: Window watchdog clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<WWDGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDGSMEN_A {
        match self.bits {
            false => WWDGSMEN_A::B_0X0,
            true => WWDGSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDGSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDGSMEN_A::B_0X1
    }
}
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, WWDGSMEN_A>;
impl<'a, const O: u8> WWDGSMEN_W<'a, O> {
    #[doc = "Window watchdog clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WWDGSMEN_A::B_0X0)
    }
    #[doc = "Window watchdog clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WWDGSMEN_A::B_0X1)
    }
}
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI2SMEN_R = crate::BitReader<SPI2SMEN_A>;
#[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2SMEN_A {
    #[doc = "0: SPI2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SPI2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SPI2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2SMEN_A {
        match self.bits {
            false => SPI2SMEN_A::B_0X0,
            true => SPI2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI2SMEN_A::B_0X1
    }
}
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, SPI2SMEN_A>;
impl<'a, const O: u8> SPI2SMEN_W<'a, O> {
    #[doc = "SPI2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI2SMEN_A::B_0X0)
    }
    #[doc = "SPI2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI2SMEN_A::B_0X1)
    }
}
#[doc = "Field `SPI3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI3SMEN_R = crate::BitReader<SPI3SMEN_A>;
#[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3SMEN_A {
    #[doc = "0: SPI3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SPI3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SPI3SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI3SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI3SMEN_A {
        match self.bits {
            false => SPI3SMEN_A::B_0X0,
            true => SPI3SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI3SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI3SMEN_A::B_0X1
    }
}
#[doc = "Field `SPI3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SPI3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, SPI3SMEN_A>;
impl<'a, const O: u8> SPI3SMEN_W<'a, O> {
    #[doc = "SPI3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI3SMEN_A::B_0X0)
    }
    #[doc = "SPI3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI3SMEN_A::B_0X1)
    }
}
#[doc = "Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USART2SMEN_R = crate::BitReader<USART2SMEN_A>;
#[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2SMEN_A {
    #[doc = "0: USART2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: USART2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<USART2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2SMEN_A {
        match self.bits {
            false => USART2SMEN_A::B_0X0,
            true => USART2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2SMEN_A::B_0X1
    }
}
#[doc = "Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USART2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, USART2SMEN_A>;
impl<'a, const O: u8> USART2SMEN_W<'a, O> {
    #[doc = "USART2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART2SMEN_A::B_0X0)
    }
    #[doc = "USART2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART2SMEN_A::B_0X1)
    }
}
#[doc = "Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USART3SMEN_R = crate::BitReader<USART3SMEN_A>;
#[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3SMEN_A {
    #[doc = "0: USART3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: USART3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<USART3SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART3SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART3SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART3SMEN_A {
        match self.bits {
            false => USART3SMEN_A::B_0X0,
            true => USART3SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART3SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART3SMEN_A::B_0X1
    }
}
#[doc = "Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USART3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, USART3SMEN_A>;
impl<'a, const O: u8> USART3SMEN_W<'a, O> {
    #[doc = "USART3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART3SMEN_A::B_0X0)
    }
    #[doc = "USART3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART3SMEN_A::B_0X1)
    }
}
#[doc = "Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UART4SMEN_R = crate::BitReader<UART4SMEN_A>;
#[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4SMEN_A {
    #[doc = "0: UART4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: UART4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<UART4SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART4SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UART4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4SMEN_A {
        match self.bits {
            false => UART4SMEN_A::B_0X0,
            true => UART4SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART4SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART4SMEN_A::B_0X1
    }
}
#[doc = "Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UART4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, UART4SMEN_A>;
impl<'a, const O: u8> UART4SMEN_W<'a, O> {
    #[doc = "UART4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART4SMEN_A::B_0X0)
    }
    #[doc = "UART4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART4SMEN_A::B_0X1)
    }
}
#[doc = "Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UART5SMEN_R = crate::BitReader<UART5SMEN_A>;
#[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART5SMEN_A {
    #[doc = "0: UART5 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: UART5 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<UART5SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: UART5SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UART5SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5SMEN_A {
        match self.bits {
            false => UART5SMEN_A::B_0X0,
            true => UART5SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART5SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART5SMEN_A::B_0X1
    }
}
#[doc = "Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UART5SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, UART5SMEN_A>;
impl<'a, const O: u8> UART5SMEN_W<'a, O> {
    #[doc = "UART5 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART5SMEN_A::B_0X0)
    }
    #[doc = "UART5 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART5SMEN_A::B_0X1)
    }
}
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2C1SMEN_R = crate::BitReader<I2C1SMEN_A>;
#[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SMEN_A {
    #[doc = "0: I2C1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: I2C1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<I2C1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1SMEN_A {
        match self.bits {
            false => I2C1SMEN_A::B_0X0,
            true => I2C1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1SMEN_A::B_0X1
    }
}
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, I2C1SMEN_A>;
impl<'a, const O: u8> I2C1SMEN_W<'a, O> {
    #[doc = "I2C1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::B_0X0)
    }
    #[doc = "I2C1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::B_0X1)
    }
}
#[doc = "Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2C2SMEN_R = crate::BitReader<I2C2SMEN_A>;
#[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2SMEN_A {
    #[doc = "0: I2C2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: I2C2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<I2C2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2SMEN_A {
        match self.bits {
            false => I2C2SMEN_A::B_0X0,
            true => I2C2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2SMEN_A::B_0X1
    }
}
#[doc = "Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2C2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, I2C2SMEN_A>;
impl<'a, const O: u8> I2C2SMEN_W<'a, O> {
    #[doc = "I2C2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C2SMEN_A::B_0X0)
    }
    #[doc = "I2C2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C2SMEN_A::B_0X1)
    }
}
#[doc = "Field `USBSMEN` reader - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USBSMEN_R = crate::BitReader<USBSMEN_A>;
#[doc = "USB device clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSMEN_A {
    #[doc = "0: USB device clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: USB device clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<USBSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSMEN_A {
        match self.bits {
            false => USBSMEN_A::B_0X0,
            true => USBSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBSMEN_A::B_0X1
    }
}
#[doc = "Field `USBSMEN` writer - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type USBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, USBSMEN_A>;
impl<'a, const O: u8> USBSMEN_W<'a, O> {
    #[doc = "USB device clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBSMEN_A::B_0X0)
    }
    #[doc = "USB device clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBSMEN_A::B_0X1)
    }
}
#[doc = "Field `FDCANSMEN` reader - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FDCANSMEN_R = crate::BitReader<FDCANSMEN_A>;
#[doc = "FDCAN clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCANSMEN_A {
    #[doc = "0: FDCAN clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: FDCAN clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<FDCANSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCANSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FDCANSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCANSMEN_A {
        match self.bits {
            false => FDCANSMEN_A::B_0X0,
            true => FDCANSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANSMEN_A::B_0X1
    }
}
#[doc = "Field `FDCANSMEN` writer - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FDCANSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, FDCANSMEN_A>;
impl<'a, const O: u8> FDCANSMEN_W<'a, O> {
    #[doc = "FDCAN clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANSMEN_A::B_0X0)
    }
    #[doc = "FDCAN clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANSMEN_A::B_0X1)
    }
}
#[doc = "Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type PWRSMEN_R = crate::BitReader<PWRSMEN_A>;
#[doc = "Power interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSMEN_A {
    #[doc = "0: Power interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: Power interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<PWRSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWRSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSMEN_A {
        match self.bits {
            false => PWRSMEN_A::B_0X0,
            true => PWRSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRSMEN_A::B_0X1
    }
}
#[doc = "Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type PWRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, PWRSMEN_A>;
impl<'a, const O: u8> PWRSMEN_W<'a, O> {
    #[doc = "Power interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PWRSMEN_A::B_0X0)
    }
    #[doc = "Power interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PWRSMEN_A::B_0X1)
    }
}
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2C3SMEN_R = crate::BitReader<I2C3SMEN_A>;
#[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3SMEN_A {
    #[doc = "0: I2C3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: I2C3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<I2C3SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C3SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3SMEN_A {
        match self.bits {
            false => I2C3SMEN_A::B_0X0,
            true => I2C3SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C3SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C3SMEN_A::B_0X1
    }
}
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2C3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, I2C3SMEN_A>;
impl<'a, const O: u8> I2C3SMEN_W<'a, O> {
    #[doc = "I2C3 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C3SMEN_A::B_0X0)
    }
    #[doc = "I2C3 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C3SMEN_A::B_0X1)
    }
}
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type LPTIM1SMEN_R = crate::BitReader<LPTIM1SMEN_A>;
#[doc = "Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1SMEN_A {
    #[doc = "0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: LPTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<LPTIM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SMEN_A {
        match self.bits {
            false => LPTIM1SMEN_A::B_0X0,
            true => LPTIM1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM1SMEN_A::B_0X1
    }
}
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type LPTIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB1SMENR1_SPEC, O, LPTIM1SMEN_A>;
impl<'a, const O: u8> LPTIM1SMEN_W<'a, O> {
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::B_0X0)
    }
    #[doc = "LPTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn fdcansmen(&self) -> FDCANSMEN_R {
        FDCANSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<1> {
        TIM3SMEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<2> {
        TIM4SMEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<3> {
        TIM5SMEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<4> {
        TIM6SMEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<5> {
        TIM7SMEN_W::new(self)
    }
    #[doc = "Bit 8 - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CRSSMEN_W<8> {
        CRSSMEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<15> {
        SPI3SMEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<17> {
        USART2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<18> {
        USART3SMEN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<19> {
        UART4SMEN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<20> {
        UART5SMEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<22> {
        I2C2SMEN_W::new(self)
    }
    #[doc = "Bit 23 - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> USBSMEN_W<23> {
        USBSMEN_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcansmen(&mut self) -> FDCANSMEN_W<25> {
        FDCANSMEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<28> {
        PWRSMEN_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<30> {
        I2C3SMEN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<31> {
        LPTIM1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1smenr1](index.html) module"]
pub struct RCC_APB1SMENR1_SPEC;
impl crate::RegisterSpec for RCC_APB1SMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1smenr1::R](R) reader structure"]
impl crate::Readable for RCC_APB1SMENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1smenr1::W](W) writer structure"]
impl crate::Writable for RCC_APB1SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB1SMENR1 to value 0xd2fe_cd3f"]
impl crate::Resettable for RCC_APB1SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xd2fe_cd3f;
}
