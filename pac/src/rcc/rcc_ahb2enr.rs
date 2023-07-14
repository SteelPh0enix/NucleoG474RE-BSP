#[doc = "Register `RCC_AHB2ENR` reader"]
pub struct R(crate::R<RCC_AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB2ENR` writer"]
pub struct W(crate::W<RCC_AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2ENR_SPEC>;
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
impl From<crate::W<RCC_AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable Set and cleared by software."]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
#[doc = "IO port A clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    #[doc = "0: IO port A clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port A clock enabled"]
    B_0X1 = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::B_0X0,
            true => GPIOAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOAEN_A::B_0X1
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable Set and cleared by software."]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIOAEN_A>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    #[doc = "IO port A clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOAEN_A::B_0X0)
    }
    #[doc = "IO port A clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOAEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable Set and cleared by software."]
pub type GPIOBEN_R = crate::BitReader<GPIOBEN_A>;
#[doc = "IO port B clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBEN_A {
    #[doc = "0: IO port B clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port B clock enabled"]
    B_0X1 = 1,
}
impl From<GPIOBEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOBEN_A {
        match self.bits {
            false => GPIOBEN_A::B_0X0,
            true => GPIOBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBEN_A::B_0X1
    }
}
#[doc = "Field `GPIOBEN` writer - IO port B clock enable Set and cleared by software."]
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIOBEN_A>;
impl<'a, const O: u8> GPIOBEN_W<'a, O> {
    #[doc = "IO port B clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOBEN_A::B_0X0)
    }
    #[doc = "IO port B clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOBEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOCEN` reader - IO port C clock enable Set and cleared by software."]
pub type GPIOCEN_R = crate::BitReader<GPIOCEN_A>;
#[doc = "IO port C clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCEN_A {
    #[doc = "0: IO port C clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port C clock enabled"]
    B_0X1 = 1,
}
impl From<GPIOCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOCEN_A {
        match self.bits {
            false => GPIOCEN_A::B_0X0,
            true => GPIOCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCEN_A::B_0X1
    }
}
#[doc = "Field `GPIOCEN` writer - IO port C clock enable Set and cleared by software."]
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIOCEN_A>;
impl<'a, const O: u8> GPIOCEN_W<'a, O> {
    #[doc = "IO port C clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOCEN_A::B_0X0)
    }
    #[doc = "IO port C clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOCEN_A::B_0X1)
    }
}
#[doc = "Field `GPIODEN` reader - IO port D clock enable Set and cleared by software."]
pub type GPIODEN_R = crate::BitReader<GPIODEN_A>;
#[doc = "IO port D clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODEN_A {
    #[doc = "0: IO port D clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port D clock enabled"]
    B_0X1 = 1,
}
impl From<GPIODEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIODEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIODEN_A {
        match self.bits {
            false => GPIODEN_A::B_0X0,
            true => GPIODEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODEN_A::B_0X1
    }
}
#[doc = "Field `GPIODEN` writer - IO port D clock enable Set and cleared by software."]
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIODEN_A>;
impl<'a, const O: u8> GPIODEN_W<'a, O> {
    #[doc = "IO port D clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIODEN_A::B_0X0)
    }
    #[doc = "IO port D clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIODEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOEEN` reader - IO port E clock enable Set and cleared by software."]
pub type GPIOEEN_R = crate::BitReader<GPIOEEN_A>;
#[doc = "IO port E clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOEEN_A {
    #[doc = "0: IO port E clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port E clock enabled"]
    B_0X1 = 1,
}
impl From<GPIOEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOEEN_A {
        match self.bits {
            false => GPIOEEN_A::B_0X0,
            true => GPIOEEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOEEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOEEN_A::B_0X1
    }
}
#[doc = "Field `GPIOEEN` writer - IO port E clock enable Set and cleared by software."]
pub type GPIOEEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIOEEN_A>;
impl<'a, const O: u8> GPIOEEN_W<'a, O> {
    #[doc = "IO port E clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOEEN_A::B_0X0)
    }
    #[doc = "IO port E clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOEEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOFEN` reader - IO port F clock enable Set and cleared by software."]
pub type GPIOFEN_R = crate::BitReader<GPIOFEN_A>;
#[doc = "IO port F clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOFEN_A {
    #[doc = "0: IO port F clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port F clock enabled"]
    B_0X1 = 1,
}
impl From<GPIOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOFEN_A {
        match self.bits {
            false => GPIOFEN_A::B_0X0,
            true => GPIOFEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFEN_A::B_0X1
    }
}
#[doc = "Field `GPIOFEN` writer - IO port F clock enable Set and cleared by software."]
pub type GPIOFEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIOFEN_A>;
impl<'a, const O: u8> GPIOFEN_W<'a, O> {
    #[doc = "IO port F clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOFEN_A::B_0X0)
    }
    #[doc = "IO port F clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOFEN_A::B_0X1)
    }
}
#[doc = "Field `GPIOGEN` reader - IO port G clock enable Set and cleared by software."]
pub type GPIOGEN_R = crate::BitReader<GPIOGEN_A>;
#[doc = "IO port G clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOGEN_A {
    #[doc = "0: IO port G clock disabled"]
    B_0X0 = 0,
    #[doc = "1: IO port G clock enabled"]
    B_0X1 = 1,
}
impl From<GPIOGEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOGEN_A {
        match self.bits {
            false => GPIOGEN_A::B_0X0,
            true => GPIOGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOGEN_A::B_0X1
    }
}
#[doc = "Field `GPIOGEN` writer - IO port G clock enable Set and cleared by software."]
pub type GPIOGEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, GPIOGEN_A>;
impl<'a, const O: u8> GPIOGEN_W<'a, O> {
    #[doc = "IO port G clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOGEN_A::B_0X0)
    }
    #[doc = "IO port G clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOGEN_A::B_0X1)
    }
}
#[doc = "Field `ADC12EN` reader - ADC12 clock enable Set and cleared by software."]
pub type ADC12EN_R = crate::BitReader<ADC12EN_A>;
#[doc = "ADC12 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12EN_A {
    #[doc = "0: ADC12 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: ADC12 clock enabled"]
    B_0X1 = 1,
}
impl From<ADC12EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12EN_A {
        match self.bits {
            false => ADC12EN_A::B_0X0,
            true => ADC12EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC12EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC12EN_A::B_0X1
    }
}
#[doc = "Field `ADC12EN` writer - ADC12 clock enable Set and cleared by software."]
pub type ADC12EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, ADC12EN_A>;
impl<'a, const O: u8> ADC12EN_W<'a, O> {
    #[doc = "ADC12 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC12EN_A::B_0X0)
    }
    #[doc = "ADC12 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC12EN_A::B_0X1)
    }
}
#[doc = "Field `ADC345EN` reader - ADC345 clock enable Set and cleared by software"]
pub type ADC345EN_R = crate::BitReader<ADC345EN_A>;
#[doc = "ADC345 clock enable Set and cleared by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC345EN_A {
    #[doc = "0: ADC345 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: ADC345 clock enabled"]
    B_0X1 = 1,
}
impl From<ADC345EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC345EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC345EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC345EN_A {
        match self.bits {
            false => ADC345EN_A::B_0X0,
            true => ADC345EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC345EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC345EN_A::B_0X1
    }
}
#[doc = "Field `ADC345EN` writer - ADC345 clock enable Set and cleared by software"]
pub type ADC345EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, ADC345EN_A>;
impl<'a, const O: u8> ADC345EN_W<'a, O> {
    #[doc = "ADC345 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC345EN_A::B_0X0)
    }
    #[doc = "ADC345 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC345EN_A::B_0X1)
    }
}
#[doc = "Field `DAC1EN` reader - DAC1 clock enable Set and cleared by software."]
pub type DAC1EN_R = crate::BitReader<DAC1EN_A>;
#[doc = "DAC1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1EN_A {
    #[doc = "0: DAC1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC1 clock enabled"]
    B_0X1 = 1,
}
impl From<DAC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1EN_A {
        match self.bits {
            false => DAC1EN_A::B_0X0,
            true => DAC1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC1EN_A::B_0X1
    }
}
#[doc = "Field `DAC1EN` writer - DAC1 clock enable Set and cleared by software."]
pub type DAC1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, DAC1EN_A>;
impl<'a, const O: u8> DAC1EN_W<'a, O> {
    #[doc = "DAC1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC1EN_A::B_0X0)
    }
    #[doc = "DAC1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC1EN_A::B_0X1)
    }
}
#[doc = "Field `DAC2EN` reader - DAC2 clock enable Set and cleared by software."]
pub type DAC2EN_R = crate::BitReader<DAC2EN_A>;
#[doc = "DAC2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC2EN_A {
    #[doc = "0: DAC2 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC2 clock enabled"]
    B_0X1 = 1,
}
impl From<DAC2EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC2EN_A {
        match self.bits {
            false => DAC2EN_A::B_0X0,
            true => DAC2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC2EN_A::B_0X1
    }
}
#[doc = "Field `DAC2EN` writer - DAC2 clock enable Set and cleared by software."]
pub type DAC2EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, DAC2EN_A>;
impl<'a, const O: u8> DAC2EN_W<'a, O> {
    #[doc = "DAC2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC2EN_A::B_0X0)
    }
    #[doc = "DAC2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC2EN_A::B_0X1)
    }
}
#[doc = "Field `DAC3EN` reader - DAC3 clock enable Set and cleared by software."]
pub type DAC3EN_R = crate::BitReader<DAC3EN_A>;
#[doc = "DAC3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC3EN_A {
    #[doc = "0: DAC3 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC3 clock enabled"]
    B_0X1 = 1,
}
impl From<DAC3EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC3EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC3EN_A {
        match self.bits {
            false => DAC3EN_A::B_0X0,
            true => DAC3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC3EN_A::B_0X1
    }
}
#[doc = "Field `DAC3EN` writer - DAC3 clock enable Set and cleared by software."]
pub type DAC3EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, DAC3EN_A>;
impl<'a, const O: u8> DAC3EN_W<'a, O> {
    #[doc = "DAC3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC3EN_A::B_0X0)
    }
    #[doc = "DAC3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC3EN_A::B_0X1)
    }
}
#[doc = "Field `DAC4EN` reader - DAC4 clock enable Set and cleared by software."]
pub type DAC4EN_R = crate::BitReader<DAC4EN_A>;
#[doc = "DAC4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC4EN_A {
    #[doc = "0: DAC4 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DAC4 clock enabled"]
    B_0X1 = 1,
}
impl From<DAC4EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC4EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC4EN_A {
        match self.bits {
            false => DAC4EN_A::B_0X0,
            true => DAC4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC4EN_A::B_0X1
    }
}
#[doc = "Field `DAC4EN` writer - DAC4 clock enable Set and cleared by software."]
pub type DAC4EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, DAC4EN_A>;
impl<'a, const O: u8> DAC4EN_W<'a, O> {
    #[doc = "DAC4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC4EN_A::B_0X0)
    }
    #[doc = "DAC4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC4EN_A::B_0X1)
    }
}
#[doc = "Field `AESEN` reader - AES clock enable Set and cleared by software."]
pub type AESEN_R = crate::BitReader<AESEN_A>;
#[doc = "AES clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESEN_A {
    #[doc = "0: AES clock disabled"]
    B_0X0 = 0,
    #[doc = "1: AES clock enabled"]
    B_0X1 = 1,
}
impl From<AESEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESEN_A {
        match self.bits {
            false => AESEN_A::B_0X0,
            true => AESEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AESEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AESEN_A::B_0X1
    }
}
#[doc = "Field `AESEN` writer - AES clock enable Set and cleared by software."]
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, AESEN_A>;
impl<'a, const O: u8> AESEN_W<'a, O> {
    #[doc = "AES clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AESEN_A::B_0X0)
    }
    #[doc = "AES clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AESEN_A::B_0X1)
    }
}
#[doc = "Field `RNGEN` reader - RNG enable Set and cleared by software."]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
#[doc = "RNG enable Set and cleared by software.\n\nValue on reset: 0"]
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
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2ENR_SPEC, O, RNGEN_A>;
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
    #[doc = "Bit 0 - IO port A clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn adc345en(&self) -> ADC345EN_R {
        ADC345EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac2en(&self) -> DAC2EN_R {
        DAC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac3en(&self) -> DAC3EN_R {
        DAC3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac4en(&self) -> DAC4EN_R {
        DAC4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AES clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC12 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<13> {
        ADC12EN_W::new(self)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn adc345en(&mut self) -> ADC345EN_W<14> {
        ADC345EN_W::new(self)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<16> {
        DAC1EN_W::new(self)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac2en(&mut self) -> DAC2EN_W<17> {
        DAC2EN_W::new(self)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac3en(&mut self) -> DAC3EN_W<18> {
        DAC3EN_W::new(self)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac4en(&mut self) -> DAC4EN_W<19> {
        DAC4EN_W::new(self)
    }
    #[doc = "Bit 24 - AES clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<24> {
        AESEN_W::new(self)
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
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2enr](index.html) module"]
pub struct RCC_AHB2ENR_SPEC;
impl crate::RegisterSpec for RCC_AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb2enr::R](R) reader structure"]
impl crate::Readable for RCC_AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2enr::W](W) writer structure"]
impl crate::Writable for RCC_AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB2ENR to value 0"]
impl crate::Resettable for RCC_AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
