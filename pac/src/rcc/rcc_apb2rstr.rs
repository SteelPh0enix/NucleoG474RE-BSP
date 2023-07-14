#[doc = "Register `RCC_APB2RSTR` reader"]
pub struct R(crate::R<RCC_APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2RSTR` writer"]
pub struct W(crate::W<RCC_APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2RSTR_SPEC>;
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
impl From<crate::W<RCC_APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG + COMP + OPAMP + VREFBUF reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST_A>;
#[doc = "SYSCFG + COMP + OPAMP + VREFBUF reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset SYSCFG + COMP + OPAMP + VREFBUF"]
    B_0X1 = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGRST_A {
        match self.bits {
            false => SYSCFGRST_A::B_0X0,
            true => SYSCFGRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGRST_A::B_0X1
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG + COMP + OPAMP + VREFBUF reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, SYSCFGRST_A>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::B_0X0)
    }
    #[doc = "Reset SYSCFG + COMP + OPAMP + VREFBUF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::B_0X1)
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 timer reset Set and cleared by software."]
pub type TIM1RST_R = crate::BitReader<TIM1RST_A>;
#[doc = "TIM1 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM1 timer"]
    B_0X1 = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1RST_A {
        match self.bits {
            false => TIM1RST_A::B_0X0,
            true => TIM1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1RST_A::B_0X1
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 timer reset Set and cleared by software."]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, TIM1RST_A>;
impl<'a, const O: u8> TIM1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM1RST_A::B_0X0)
    }
    #[doc = "Reset TIM1 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM1RST_A::B_0X1)
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 reset Set and cleared by software."]
pub type SPI1RST_R = crate::BitReader<SPI1RST_A>;
#[doc = "SPI1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset SPI1"]
    B_0X1 = 1,
}
impl From<SPI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1RST_A {
        match self.bits {
            false => SPI1RST_A::B_0X0,
            true => SPI1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1RST_A::B_0X1
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 reset Set and cleared by software."]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, SPI1RST_A>;
impl<'a, const O: u8> SPI1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI1RST_A::B_0X0)
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI1RST_A::B_0X1)
    }
}
#[doc = "Field `TIM8RST` reader - TIM8 timer reset Set and cleared by software."]
pub type TIM8RST_R = crate::BitReader<TIM8RST_A>;
#[doc = "TIM8 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM8RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM8 timer"]
    B_0X1 = 1,
}
impl From<TIM8RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM8RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM8RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM8RST_A {
        match self.bits {
            false => TIM8RST_A::B_0X0,
            true => TIM8RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM8RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM8RST_A::B_0X1
    }
}
#[doc = "Field `TIM8RST` writer - TIM8 timer reset Set and cleared by software."]
pub type TIM8RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, TIM8RST_A>;
impl<'a, const O: u8> TIM8RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM8RST_A::B_0X0)
    }
    #[doc = "Reset TIM8 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM8RST_A::B_0X1)
    }
}
#[doc = "Field `USART1RST` reader - USART1 reset Set and cleared by software."]
pub type USART1RST_R = crate::BitReader<USART1RST_A>;
#[doc = "USART1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset USART1"]
    B_0X1 = 1,
}
impl From<USART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1RST_A {
        match self.bits {
            false => USART1RST_A::B_0X0,
            true => USART1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1RST_A::B_0X1
    }
}
#[doc = "Field `USART1RST` writer - USART1 reset Set and cleared by software."]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, USART1RST_A>;
impl<'a, const O: u8> USART1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1RST_A::B_0X0)
    }
    #[doc = "Reset USART1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1RST_A::B_0X1)
    }
}
#[doc = "Field `SPI4RST` reader - SPI4 reset Set and cleared by software."]
pub type SPI4RST_R = crate::BitReader<SPI4RST_A>;
#[doc = "SPI4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI4RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset SPI4"]
    B_0X1 = 1,
}
impl From<SPI4RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI4RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI4RST_A {
        match self.bits {
            false => SPI4RST_A::B_0X0,
            true => SPI4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI4RST_A::B_0X1
    }
}
#[doc = "Field `SPI4RST` writer - SPI4 reset Set and cleared by software."]
pub type SPI4RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, SPI4RST_A>;
impl<'a, const O: u8> SPI4RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI4RST_A::B_0X0)
    }
    #[doc = "Reset SPI4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI4RST_A::B_0X1)
    }
}
#[doc = "Field `TIM15RST` reader - TIM15 timer reset Set and cleared by software."]
pub type TIM15RST_R = crate::BitReader<TIM15RST_A>;
#[doc = "TIM15 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM15RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM15 timer"]
    B_0X1 = 1,
}
impl From<TIM15RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM15RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM15RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM15RST_A {
        match self.bits {
            false => TIM15RST_A::B_0X0,
            true => TIM15RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM15RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM15RST_A::B_0X1
    }
}
#[doc = "Field `TIM15RST` writer - TIM15 timer reset Set and cleared by software."]
pub type TIM15RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, TIM15RST_A>;
impl<'a, const O: u8> TIM15RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM15RST_A::B_0X0)
    }
    #[doc = "Reset TIM15 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM15RST_A::B_0X1)
    }
}
#[doc = "Field `TIM16RST` reader - TIM16 timer reset Set and cleared by software."]
pub type TIM16RST_R = crate::BitReader<TIM16RST_A>;
#[doc = "TIM16 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM16 timer"]
    B_0X1 = 1,
}
impl From<TIM16RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM16RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16RST_A {
        match self.bits {
            false => TIM16RST_A::B_0X0,
            true => TIM16RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16RST_A::B_0X1
    }
}
#[doc = "Field `TIM16RST` writer - TIM16 timer reset Set and cleared by software."]
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, TIM16RST_A>;
impl<'a, const O: u8> TIM16RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM16RST_A::B_0X0)
    }
    #[doc = "Reset TIM16 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM16RST_A::B_0X1)
    }
}
#[doc = "Field `TIM17RST` reader - TIM17 timer reset Set and cleared by software."]
pub type TIM17RST_R = crate::BitReader<TIM17RST_A>;
#[doc = "TIM17 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM17 timer"]
    B_0X1 = 1,
}
impl From<TIM17RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM17RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17RST_A {
        match self.bits {
            false => TIM17RST_A::B_0X0,
            true => TIM17RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17RST_A::B_0X1
    }
}
#[doc = "Field `TIM17RST` writer - TIM17 timer reset Set and cleared by software."]
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, TIM17RST_A>;
impl<'a, const O: u8> TIM17RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM17RST_A::B_0X0)
    }
    #[doc = "Reset TIM17 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM17RST_A::B_0X1)
    }
}
#[doc = "Field `TIM20RST` reader - TIM20 reset Set and cleared by software."]
pub type TIM20RST_R = crate::BitReader<TIM20RST_A>;
#[doc = "TIM20 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM20RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset TIM20"]
    B_0X1 = 1,
}
impl From<TIM20RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM20RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM20RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM20RST_A {
        match self.bits {
            false => TIM20RST_A::B_0X0,
            true => TIM20RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM20RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM20RST_A::B_0X1
    }
}
#[doc = "Field `TIM20RST` writer - TIM20 reset Set and cleared by software."]
pub type TIM20RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, TIM20RST_A>;
impl<'a, const O: u8> TIM20RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM20RST_A::B_0X0)
    }
    #[doc = "Reset TIM20"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM20RST_A::B_0X1)
    }
}
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
pub type SAI1RST_R = crate::BitReader<SAI1RST_A>;
#[doc = "Serial audio interface 1 (SAI1) reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset SAI1"]
    B_0X1 = 1,
}
impl From<SAI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1RST_A {
        match self.bits {
            false => SAI1RST_A::B_0X0,
            true => SAI1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1RST_A::B_0X1
    }
}
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
pub type SAI1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, SAI1RST_A>;
impl<'a, const O: u8> SAI1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1RST_A::B_0X0)
    }
    #[doc = "Reset SAI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1RST_A::B_0X1)
    }
}
#[doc = "Field `HRTIM1RST` reader - HRTIM1 reset Set and cleared by software."]
pub type HRTIM1RST_R = crate::BitReader<HRTIM1RST_A>;
#[doc = "HRTIM1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRTIM1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset HRTIM1"]
    B_0X1 = 1,
}
impl From<HRTIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: HRTIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl HRTIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRTIM1RST_A {
        match self.bits {
            false => HRTIM1RST_A::B_0X0,
            true => HRTIM1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HRTIM1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HRTIM1RST_A::B_0X1
    }
}
#[doc = "Field `HRTIM1RST` writer - HRTIM1 reset Set and cleared by software."]
pub type HRTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_APB2RSTR_SPEC, O, HRTIM1RST_A>;
impl<'a, const O: u8> HRTIM1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HRTIM1RST_A::B_0X0)
    }
    #[doc = "Reset HRTIM1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HRTIM1RST_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG + COMP + OPAMP + VREFBUF reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim20rst(&self) -> TIM20RST_R {
        TIM20RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIM1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1rst(&self) -> HRTIM1RST_R {
        HRTIM1RST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG + COMP + OPAMP + VREFBUF reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI4 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<15> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 20 - TIM20 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim20rst(&mut self) -> TIM20RST_W<20> {
        TIM20RST_W::new(self)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<21> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 26 - HRTIM1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn hrtim1rst(&mut self) -> HRTIM1RST_W<26> {
        HRTIM1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2rstr](index.html) module"]
pub struct RCC_APB2RSTR_SPEC;
impl crate::RegisterSpec for RCC_APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2rstr::R](R) reader structure"]
impl crate::Readable for RCC_APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2rstr::W](W) writer structure"]
impl crate::Writable for RCC_APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB2RSTR to value 0"]
impl crate::Resettable for RCC_APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
