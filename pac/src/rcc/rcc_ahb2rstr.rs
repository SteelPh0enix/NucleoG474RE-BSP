#[doc = "Register `RCC_AHB2RSTR` reader"]
pub struct R(crate::R<RCC_AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB2RSTR` writer"]
pub struct W(crate::W<RCC_AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2RSTR_SPEC>;
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
impl From<crate::W<RCC_AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOARST` reader - IO port A reset Set and cleared by software."]
pub type GPIOARST_R = crate::BitReader<GPIOARST_A>;
#[doc = "IO port A reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port A"]
    B_0X1 = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOARST_A {
        match self.bits {
            false => GPIOARST_A::B_0X0,
            true => GPIOARST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOARST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOARST_A::B_0X1
    }
}
#[doc = "Field `GPIOARST` writer - IO port A reset Set and cleared by software."]
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIOARST_A>;
impl<'a, const O: u8> GPIOARST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOARST_A::B_0X0)
    }
    #[doc = "Reset IO port A"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOARST_A::B_0X1)
    }
}
#[doc = "Field `GPIOBRST` reader - IO port B reset Set and cleared by software."]
pub type GPIOBRST_R = crate::BitReader<GPIOBRST_A>;
#[doc = "IO port B reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port B"]
    B_0X1 = 1,
}
impl From<GPIOBRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBRST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOBRST_A {
        match self.bits {
            false => GPIOBRST_A::B_0X0,
            true => GPIOBRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBRST_A::B_0X1
    }
}
#[doc = "Field `GPIOBRST` writer - IO port B reset Set and cleared by software."]
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIOBRST_A>;
impl<'a, const O: u8> GPIOBRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOBRST_A::B_0X0)
    }
    #[doc = "Reset IO port B"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOBRST_A::B_0X1)
    }
}
#[doc = "Field `GPIOCRST` reader - IO port C reset Set and cleared by software."]
pub type GPIOCRST_R = crate::BitReader<GPIOCRST_A>;
#[doc = "IO port C reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port C"]
    B_0X1 = 1,
}
impl From<GPIOCRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOCRST_A {
        match self.bits {
            false => GPIOCRST_A::B_0X0,
            true => GPIOCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCRST_A::B_0X1
    }
}
#[doc = "Field `GPIOCRST` writer - IO port C reset Set and cleared by software."]
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIOCRST_A>;
impl<'a, const O: u8> GPIOCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOCRST_A::B_0X0)
    }
    #[doc = "Reset IO port C"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOCRST_A::B_0X1)
    }
}
#[doc = "Field `GPIODRST` reader - IO port D reset Set and cleared by software."]
pub type GPIODRST_R = crate::BitReader<GPIODRST_A>;
#[doc = "IO port D reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port D"]
    B_0X1 = 1,
}
impl From<GPIODRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODRST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIODRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIODRST_A {
        match self.bits {
            false => GPIODRST_A::B_0X0,
            true => GPIODRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODRST_A::B_0X1
    }
}
#[doc = "Field `GPIODRST` writer - IO port D reset Set and cleared by software."]
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIODRST_A>;
impl<'a, const O: u8> GPIODRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIODRST_A::B_0X0)
    }
    #[doc = "Reset IO port D"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIODRST_A::B_0X1)
    }
}
#[doc = "Field `GPIOERST` reader - IO port E reset Set and cleared by software."]
pub type GPIOERST_R = crate::BitReader<GPIOERST_A>;
#[doc = "IO port E reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOERST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port E"]
    B_0X1 = 1,
}
impl From<GPIOERST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOERST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOERST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOERST_A {
        match self.bits {
            false => GPIOERST_A::B_0X0,
            true => GPIOERST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOERST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOERST_A::B_0X1
    }
}
#[doc = "Field `GPIOERST` writer - IO port E reset Set and cleared by software."]
pub type GPIOERST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIOERST_A>;
impl<'a, const O: u8> GPIOERST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOERST_A::B_0X0)
    }
    #[doc = "Reset IO port E"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOERST_A::B_0X1)
    }
}
#[doc = "Field `GPIOFRST` reader - IO port F reset Set and cleared by software."]
pub type GPIOFRST_R = crate::BitReader<GPIOFRST_A>;
#[doc = "IO port F reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOFRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port F"]
    B_0X1 = 1,
}
impl From<GPIOFRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOFRST_A {
        match self.bits {
            false => GPIOFRST_A::B_0X0,
            true => GPIOFRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFRST_A::B_0X1
    }
}
#[doc = "Field `GPIOFRST` writer - IO port F reset Set and cleared by software."]
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIOFRST_A>;
impl<'a, const O: u8> GPIOFRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOFRST_A::B_0X0)
    }
    #[doc = "Reset IO port F"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOFRST_A::B_0X1)
    }
}
#[doc = "Field `GPIOGRST` reader - IO port G reset Set and cleared by software."]
pub type GPIOGRST_R = crate::BitReader<GPIOGRST_A>;
#[doc = "IO port G reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOGRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset IO port G"]
    B_0X1 = 1,
}
impl From<GPIOGRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOGRST_A {
        match self.bits {
            false => GPIOGRST_A::B_0X0,
            true => GPIOGRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOGRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOGRST_A::B_0X1
    }
}
#[doc = "Field `GPIOGRST` writer - IO port G reset Set and cleared by software."]
pub type GPIOGRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, GPIOGRST_A>;
impl<'a, const O: u8> GPIOGRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOGRST_A::B_0X0)
    }
    #[doc = "Reset IO port G"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOGRST_A::B_0X1)
    }
}
#[doc = "Field `ADC12RST` reader - ADC12 reset Set and cleared by software."]
pub type ADC12RST_R = crate::BitReader<ADC12RST_A>;
#[doc = "ADC12 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset ADC12 interface"]
    B_0X1 = 1,
}
impl From<ADC12RST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12RST_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC12RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RST_A {
        match self.bits {
            false => ADC12RST_A::B_0X0,
            true => ADC12RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC12RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC12RST_A::B_0X1
    }
}
#[doc = "Field `ADC12RST` writer - ADC12 reset Set and cleared by software."]
pub type ADC12RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, ADC12RST_A>;
impl<'a, const O: u8> ADC12RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC12RST_A::B_0X0)
    }
    #[doc = "Reset ADC12 interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC12RST_A::B_0X1)
    }
}
#[doc = "Field `ADC345RST` reader - ADC345 reset Set and cleared by software."]
pub type ADC345RST_R = crate::BitReader<ADC345RST_A>;
#[doc = "ADC345 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC345RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset ADC345"]
    B_0X1 = 1,
}
impl From<ADC345RST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC345RST_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC345RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC345RST_A {
        match self.bits {
            false => ADC345RST_A::B_0X0,
            true => ADC345RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC345RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC345RST_A::B_0X1
    }
}
#[doc = "Field `ADC345RST` writer - ADC345 reset Set and cleared by software."]
pub type ADC345RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, ADC345RST_A>;
impl<'a, const O: u8> ADC345RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC345RST_A::B_0X0)
    }
    #[doc = "Reset ADC345"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC345RST_A::B_0X1)
    }
}
#[doc = "Field `DAC1RST` reader - DAC1 reset Set and cleared by software."]
pub type DAC1RST_R = crate::BitReader<DAC1RST_A>;
#[doc = "DAC1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DAC1"]
    B_0X1 = 1,
}
impl From<DAC1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1RST_A {
        match self.bits {
            false => DAC1RST_A::B_0X0,
            true => DAC1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC1RST_A::B_0X1
    }
}
#[doc = "Field `DAC1RST` writer - DAC1 reset Set and cleared by software."]
pub type DAC1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, DAC1RST_A>;
impl<'a, const O: u8> DAC1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC1RST_A::B_0X0)
    }
    #[doc = "Reset DAC1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC1RST_A::B_0X1)
    }
}
#[doc = "Field `DAC2RST` reader - DAC2 reset Set and cleared by software."]
pub type DAC2RST_R = crate::BitReader<DAC2RST_A>;
#[doc = "DAC2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC2RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DAC2"]
    B_0X1 = 1,
}
impl From<DAC2RST_A> for bool {
    #[inline(always)]
    fn from(variant: DAC2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC2RST_A {
        match self.bits {
            false => DAC2RST_A::B_0X0,
            true => DAC2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC2RST_A::B_0X1
    }
}
#[doc = "Field `DAC2RST` writer - DAC2 reset Set and cleared by software."]
pub type DAC2RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, DAC2RST_A>;
impl<'a, const O: u8> DAC2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC2RST_A::B_0X0)
    }
    #[doc = "Reset DAC2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC2RST_A::B_0X1)
    }
}
#[doc = "Field `DAC3RST` reader - DAC3 reset Set and cleared by software."]
pub type DAC3RST_R = crate::BitReader<DAC3RST_A>;
#[doc = "DAC3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC3RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DAC3"]
    B_0X1 = 1,
}
impl From<DAC3RST_A> for bool {
    #[inline(always)]
    fn from(variant: DAC3RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC3RST_A {
        match self.bits {
            false => DAC3RST_A::B_0X0,
            true => DAC3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC3RST_A::B_0X1
    }
}
#[doc = "Field `DAC3RST` writer - DAC3 reset Set and cleared by software."]
pub type DAC3RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, DAC3RST_A>;
impl<'a, const O: u8> DAC3RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC3RST_A::B_0X0)
    }
    #[doc = "Reset DAC3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC3RST_A::B_0X1)
    }
}
#[doc = "Field `DAC4RST` reader - DAC4 reset Set and cleared by software."]
pub type DAC4RST_R = crate::BitReader<DAC4RST_A>;
#[doc = "DAC4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC4RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DAC4"]
    B_0X1 = 1,
}
impl From<DAC4RST_A> for bool {
    #[inline(always)]
    fn from(variant: DAC4RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC4RST_A {
        match self.bits {
            false => DAC4RST_A::B_0X0,
            true => DAC4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAC4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAC4RST_A::B_0X1
    }
}
#[doc = "Field `DAC4RST` writer - DAC4 reset Set and cleared by software."]
pub type DAC4RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, DAC4RST_A>;
impl<'a, const O: u8> DAC4RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAC4RST_A::B_0X0)
    }
    #[doc = "Reset DAC4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAC4RST_A::B_0X1)
    }
}
#[doc = "Field `AESRST` reader - AESRST reset Set and cleared by software."]
pub type AESRST_R = crate::BitReader<AESRST_A>;
#[doc = "AESRST reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset AES"]
    B_0X1 = 1,
}
impl From<AESRST_A> for bool {
    #[inline(always)]
    fn from(variant: AESRST_A) -> Self {
        variant as u8 != 0
    }
}
impl AESRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRST_A {
        match self.bits {
            false => AESRST_A::B_0X0,
            true => AESRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AESRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AESRST_A::B_0X1
    }
}
#[doc = "Field `AESRST` writer - AESRST reset Set and cleared by software."]
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, AESRST_A>;
impl<'a, const O: u8> AESRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AESRST_A::B_0X0)
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AESRST_A::B_0X1)
    }
}
#[doc = "Field `RNGRST` reader - RNG reset Set and cleared by software."]
pub type RNGRST_R = crate::BitReader<RNGRST_A>;
#[doc = "RNG reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset RNG"]
    B_0X1 = 1,
}
impl From<RNGRST_A> for bool {
    #[inline(always)]
    fn from(variant: RNGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGRST_A {
        match self.bits {
            false => RNGRST_A::B_0X0,
            true => RNGRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNGRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNGRST_A::B_0X1
    }
}
#[doc = "Field `RNGRST` writer - RNG reset Set and cleared by software."]
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB2RSTR_SPEC, O, RNGRST_A>;
impl<'a, const O: u8> RNGRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNGRST_A::B_0X0)
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNGRST_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - IO port A reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 reset Set and cleared by software."]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC345 reset Set and cleared by software."]
    #[inline(always)]
    pub fn adc345rst(&self) -> ADC345RST_R {
        ADC345RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac3rst(&self) -> DAC3RST_R {
        DAC3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac4rst(&self) -> DAC4RST_R {
        DAC4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AESRST reset Set and cleared by software."]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RNG reset Set and cleared by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    #[doc = "Bit 1 - IO port B reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    #[doc = "Bit 2 - IO port C reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    #[doc = "Bit 3 - IO port D reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    #[doc = "Bit 4 - IO port E reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    #[doc = "Bit 5 - IO port F reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    #[doc = "Bit 6 - IO port G reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    #[doc = "Bit 13 - ADC12 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<13> {
        ADC12RST_W::new(self)
    }
    #[doc = "Bit 14 - ADC345 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc345rst(&mut self) -> ADC345RST_W<14> {
        ADC345RST_W::new(self)
    }
    #[doc = "Bit 16 - DAC1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<16> {
        DAC1RST_W::new(self)
    }
    #[doc = "Bit 17 - DAC2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac2rst(&mut self) -> DAC2RST_W<17> {
        DAC2RST_W::new(self)
    }
    #[doc = "Bit 18 - DAC3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac3rst(&mut self) -> DAC3RST_W<18> {
        DAC3RST_W::new(self)
    }
    #[doc = "Bit 19 - DAC4 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac4rst(&mut self) -> DAC4RST_W<19> {
        DAC4RST_W::new(self)
    }
    #[doc = "Bit 24 - AESRST reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<24> {
        AESRST_W::new(self)
    }
    #[doc = "Bit 26 - RNG reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<26> {
        RNGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2rstr](index.html) module"]
pub struct RCC_AHB2RSTR_SPEC;
impl crate::RegisterSpec for RCC_AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb2rstr::R](R) reader structure"]
impl crate::Readable for RCC_AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2rstr::W](W) writer structure"]
impl crate::Writable for RCC_AHB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB2RSTR to value 0"]
impl crate::Resettable for RCC_AHB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
