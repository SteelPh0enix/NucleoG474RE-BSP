#[doc = "Register `RCC_AHB2SMENR` reader"]
pub struct R(crate::R<RCC_AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB2SMENR` writer"]
pub struct W(crate::W<RCC_AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2SMENR_SPEC>;
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
impl From<crate::W<RCC_AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN_A>;
#[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN_A {
    #[doc = "0: IO port A clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port A clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIOASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOASMEN_A {
        match self.bits {
            false => GPIOASMEN_A::B_0X0,
            true => GPIOASMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOASMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOASMEN_A::B_0X1
    }
}
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIOASMEN_A>;
impl<'a, const O: u8> GPIOASMEN_W<'a, O> {
    #[doc = "IO port A clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOASMEN_A::B_0X0)
    }
    #[doc = "IO port A clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOASMEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOBSMEN_R = crate::BitReader<GPIOBSMEN_A>;
#[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBSMEN_A {
    #[doc = "0: IO port B clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port B clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIOBSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOBSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOBSMEN_A {
        match self.bits {
            false => GPIOBSMEN_A::B_0X0,
            true => GPIOBSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBSMEN_A::B_0X1
    }
}
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIOBSMEN_A>;
impl<'a, const O: u8> GPIOBSMEN_W<'a, O> {
    #[doc = "IO port B clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOBSMEN_A::B_0X0)
    }
    #[doc = "IO port B clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOBSMEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOCSMEN_R = crate::BitReader<GPIOCSMEN_A>;
#[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCSMEN_A {
    #[doc = "0: IO port C clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port C clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIOCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOCSMEN_A {
        match self.bits {
            false => GPIOCSMEN_A::B_0X0,
            true => GPIOCSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCSMEN_A::B_0X1
    }
}
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIOCSMEN_A>;
impl<'a, const O: u8> GPIOCSMEN_W<'a, O> {
    #[doc = "IO port C clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOCSMEN_A::B_0X0)
    }
    #[doc = "IO port C clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOCSMEN_A::B_0X1)
    }
}
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIODSMEN_R = crate::BitReader<GPIODSMEN_A>;
#[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODSMEN_A {
    #[doc = "0: IO port D clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port D clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIODSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIODSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIODSMEN_A {
        match self.bits {
            false => GPIODSMEN_A::B_0X0,
            true => GPIODSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODSMEN_A::B_0X1
    }
}
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIODSMEN_A>;
impl<'a, const O: u8> GPIODSMEN_W<'a, O> {
    #[doc = "IO port D clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIODSMEN_A::B_0X0)
    }
    #[doc = "IO port D clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIODSMEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOESMEN_R = crate::BitReader<GPIOESMEN_A>;
#[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOESMEN_A {
    #[doc = "0: IO port E clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port E clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIOESMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOESMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOESMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOESMEN_A {
        match self.bits {
            false => GPIOESMEN_A::B_0X0,
            true => GPIOESMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOESMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOESMEN_A::B_0X1
    }
}
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOESMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIOESMEN_A>;
impl<'a, const O: u8> GPIOESMEN_W<'a, O> {
    #[doc = "IO port E clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOESMEN_A::B_0X0)
    }
    #[doc = "IO port E clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOESMEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOFSMEN_R = crate::BitReader<GPIOFSMEN_A>;
#[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOFSMEN_A {
    #[doc = "0: IO port F clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port F clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIOFSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOFSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOFSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOFSMEN_A {
        match self.bits {
            false => GPIOFSMEN_A::B_0X0,
            true => GPIOFSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFSMEN_A::B_0X1
    }
}
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIOFSMEN_A>;
impl<'a, const O: u8> GPIOFSMEN_W<'a, O> {
    #[doc = "IO port F clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOFSMEN_A::B_0X0)
    }
    #[doc = "IO port F clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOFSMEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOGSMEN_R = crate::BitReader<GPIOGSMEN_A>;
#[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOGSMEN_A {
    #[doc = "0: IO port G clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: IO port G clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<GPIOGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOGSMEN_A {
        match self.bits {
            false => GPIOGSMEN_A::B_0X0,
            true => GPIOGSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOGSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOGSMEN_A::B_0X1
    }
}
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GPIOGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, GPIOGSMEN_A>;
impl<'a, const O: u8> GPIOGSMEN_W<'a, O> {
    #[doc = "IO port G clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOGSMEN_A::B_0X0)
    }
    #[doc = "IO port G clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOGSMEN_A::B_0X1)
    }
}
#[doc = "Field `CCMSRAMSMEN` reader - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CCMSRAMSMEN_R = crate::BitReader<CCMSRAMSMEN_A>;
#[doc = "CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCMSRAMSMEN_A {
    #[doc = "0: CCM SRAM interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: CCM SRAM interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<CCMSRAMSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCMSRAMSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCMSRAMSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCMSRAMSMEN_A {
        match self.bits {
            false => CCMSRAMSMEN_A::B_0X0,
            true => CCMSRAMSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCMSRAMSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCMSRAMSMEN_A::B_0X1
    }
}
#[doc = "Field `CCMSRAMSMEN` writer - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CCMSRAMSMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, CCMSRAMSMEN_A>;
impl<'a, const O: u8> CCMSRAMSMEN_W<'a, O> {
    #[doc = "CCM SRAM interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCMSRAMSMEN_A::B_0X0)
    }
    #[doc = "CCM SRAM interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCMSRAMSMEN_A::B_0X1)
    }
}
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SRAM2SMEN_R = crate::BitReader<SRAM2SMEN_A>;
#[doc = "SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2SMEN_A {
    #[doc = "0: SRAM2 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SRAM2 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SRAM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2SMEN_A {
        match self.bits {
            false => SRAM2SMEN_A::B_0X0,
            true => SRAM2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SRAM2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SRAM2SMEN_A::B_0X1
    }
}
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, SRAM2SMEN_A>;
impl<'a, const O: u8> SRAM2SMEN_W<'a, O> {
    #[doc = "SRAM2 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SRAM2SMEN_A::B_0X0)
    }
    #[doc = "SRAM2 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SRAM2SMEN_A::B_0X1)
    }
}
#[doc = "Field `ADC12SMEN` reader - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type ADC12SMEN_R = crate::BitReader<ADC12SMEN_A>;
#[doc = "ADC12 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12SMEN_A {
    #[doc = "0: ADC12 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: ADC12 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<ADC12SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SMEN_A {
        match self.bits {
            false => ADC12SMEN_A::B_0X0,
            true => ADC12SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC12SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC12SMEN_A::B_0X1
    }
}
#[doc = "Field `ADC12SMEN` writer - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type ADC12SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, ADC12SMEN_A>;
impl<'a, const O: u8> ADC12SMEN_W<'a, O> {
    #[doc = "ADC12 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC12SMEN_A::B_0X0)
    }
    #[doc = "ADC12 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC12SMEN_A::B_0X1)
    }
}
#[doc = "Field `ADC345SMEN` reader - ADC345 clock enable Set and cleared by software."]
pub type ADC345SMEN_R = crate::BitReader<ADC345SMEN_A>;
#[doc = "ADC345 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC345SMEN_A {
    #[doc = "0: ADC345 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: ADC345 clock enabled"]
    B_0X1 = 1,
}
impl From<ADC345SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC345SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC345SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC345SMEN_A {
        match self.bits {
            false => ADC345SMEN_A::B_0X0,
            true => ADC345SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC345SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC345SMEN_A::B_0X1
    }
}
#[doc = "Field `ADC345SMEN` writer - ADC345 clock enable Set and cleared by software."]
pub type ADC345SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, ADC345SMEN_A>;
impl<'a, const O: u8> ADC345SMEN_W<'a, O> {
    #[doc = "ADC345 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC345SMEN_A::B_0X0)
    }
    #[doc = "ADC345 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC345SMEN_A::B_0X1)
    }
}
#[doc = "Field `DAC1SMEN` reader - DAC1 clock enable Set and cleared by software."]
pub type DAC1SMEN_R = crate::BitReader<DAC1SMEN_A>;
#[doc = "DAC1 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SMEN_A {
    #[doc = "0: DAC1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC1 clock enabled during sleep and stop modes"]
    B_0X1 = 1,
}
impl From<DAC1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1SMEN_A {
        match self.bits {
            false => DAC1SMEN_A::B_0X0,
            true => DAC1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC1SMEN_A::B_0X1
    }
}
#[doc = "Field `DAC1SMEN` writer - DAC1 clock enable Set and cleared by software."]
pub type DAC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, DAC1SMEN_A>;
impl<'a, const O: u8> DAC1SMEN_W<'a, O> {
    #[doc = "DAC1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC1SMEN_A::B_0X0)
    }
    #[doc = "DAC1 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC1SMEN_A::B_0X1)
    }
}
#[doc = "Field `DAC2SMEN` reader - DAC2 clock enable Set and cleared by software."]
pub type DAC2SMEN_R = crate::BitReader<DAC2SMEN_A>;
#[doc = "DAC2 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC2SMEN_A {
    #[doc = "0: DAC2 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC2 clock enabled during sleep and stop modes"]
    B_0X1 = 1,
}
impl From<DAC2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC2SMEN_A {
        match self.bits {
            false => DAC2SMEN_A::B_0X0,
            true => DAC2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC2SMEN_A::B_0X1
    }
}
#[doc = "Field `DAC2SMEN` writer - DAC2 clock enable Set and cleared by software."]
pub type DAC2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, DAC2SMEN_A>;
impl<'a, const O: u8> DAC2SMEN_W<'a, O> {
    #[doc = "DAC2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC2SMEN_A::B_0X0)
    }
    #[doc = "DAC2 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC2SMEN_A::B_0X1)
    }
}
#[doc = "Field `DAC3SMEN` reader - DAC3 clock enable Set and cleared by software."]
pub type DAC3SMEN_R = crate::BitReader<DAC3SMEN_A>;
#[doc = "DAC3 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC3SMEN_A {
    #[doc = "0: DAC3 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC3 clock enabled during sleep and stop modes"]
    B_0X1 = 1,
}
impl From<DAC3SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC3SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC3SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC3SMEN_A {
        match self.bits {
            false => DAC3SMEN_A::B_0X0,
            true => DAC3SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC3SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC3SMEN_A::B_0X1
    }
}
#[doc = "Field `DAC3SMEN` writer - DAC3 clock enable Set and cleared by software."]
pub type DAC3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, DAC3SMEN_A>;
impl<'a, const O: u8> DAC3SMEN_W<'a, O> {
    #[doc = "DAC3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC3SMEN_A::B_0X0)
    }
    #[doc = "DAC3 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC3SMEN_A::B_0X1)
    }
}
#[doc = "Field `DAC4SMEN` reader - DAC4 clock enable Set and cleared by software."]
pub type DAC4SMEN_R = crate::BitReader<DAC4SMEN_A>;
#[doc = "DAC4 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC4SMEN_A {
    #[doc = "0: DAC4 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC4 clock enabled during sleep and stop modes"]
    B_0X1 = 1,
}
impl From<DAC4SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC4SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC4SMEN_A {
        match self.bits {
            false => DAC4SMEN_A::B_0X0,
            true => DAC4SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC4SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC4SMEN_A::B_0X1
    }
}
#[doc = "Field `DAC4SMEN` writer - DAC4 clock enable Set and cleared by software."]
pub type DAC4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, DAC4SMEN_A>;
impl<'a, const O: u8> DAC4SMEN_W<'a, O> {
    #[doc = "DAC4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC4SMEN_A::B_0X0)
    }
    #[doc = "DAC4 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC4SMEN_A::B_0X1)
    }
}
#[doc = "Field `AESSMEN` reader - AESM clocks enable Set and cleared by software."]
pub type AESSMEN_R = crate::BitReader<AESSMEN_A>;
#[doc = "AESM clocks enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESSMEN_A {
    #[doc = "0: AESM clocks disabled"]
    B_0X0 = 0,
    #[doc = "1: AESM clocks enabled"]
    B_0X1 = 1,
}
impl From<AESSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESSMEN_A {
        match self.bits {
            false => AESSMEN_A::B_0X0,
            true => AESSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AESSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AESSMEN_A::B_0X1
    }
}
#[doc = "Field `AESSMEN` writer - AESM clocks enable Set and cleared by software."]
pub type AESSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, AESSMEN_A>;
impl<'a, const O: u8> AESSMEN_W<'a, O> {
    #[doc = "AESM clocks disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AESSMEN_A::B_0X0)
    }
    #[doc = "AESM clocks enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AESSMEN_A::B_0X1)
    }
}
#[doc = "Field `RNGEN` reader - RNG enable Set and cleared by software."]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
#[doc = "RNG enable Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    #[doc = "0: RNG disabled"]
    B_0X0 = 0,
    #[doc = "1: RNG enabled"]
    B_0X1 = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::B_0X0,
            true => RNGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNGEN_A::B_0X1
    }
}
#[doc = "Field `RNGEN` writer - RNG enable Set and cleared by software."]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2SMENR_SPEC, O, RNGEN_A>;
impl<'a, const O: u8> RNGEN_W<'a, O> {
    #[doc = "RNG disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNGEN_A::B_0X0)
    }
    #[doc = "RNG enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNGEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn ccmsramsmen(&self) -> CCMSRAMSMEN_R {
        CCMSRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn adc12smen(&self) -> ADC12SMEN_R {
        ADC12SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adc345smen(&self) -> ADC345SMEN_R {
        ADC345SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac2smen(&self) -> DAC2SMEN_R {
        DAC2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac3smen(&self) -> DAC3SMEN_R {
        DAC3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac4smen(&self) -> DAC4SMEN_R {
        DAC4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AESM clocks enable Set and cleared by software."]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<5> {
        GPIOFSMEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<6> {
        GPIOGSMEN_W::new(self)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ccmsramsmen(&mut self) -> CCMSRAMSMEN_W<9> {
        CCMSRAMSMEN_W::new(self)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<10> {
        SRAM2SMEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc12smen(&mut self) -> ADC12SMEN_W<13> {
        ADC12SMEN_W::new(self)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc345smen(&mut self) -> ADC345SMEN_W<14> {
        ADC345SMEN_W::new(self)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<16> {
        DAC1SMEN_W::new(self)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac2smen(&mut self) -> DAC2SMEN_W<17> {
        DAC2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac3smen(&mut self) -> DAC3SMEN_W<18> {
        DAC3SMEN_W::new(self)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac4smen(&mut self) -> DAC4SMEN_W<19> {
        DAC4SMEN_W::new(self)
    }
    #[doc = "Bit 24 - AESM clocks enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<24> {
        AESSMEN_W::new(self)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<26> {
        RNGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2smenr](index.html) module"]
pub struct RCC_AHB2SMENR_SPEC;
impl crate::RegisterSpec for RCC_AHB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb2smenr::R](R) reader structure"]
impl crate::Readable for RCC_AHB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2smenr::W](W) writer structure"]
impl crate::Writable for RCC_AHB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB2SMENR to value 0x050f_667f"]
impl crate::Resettable for RCC_AHB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x050f_667f;
}
