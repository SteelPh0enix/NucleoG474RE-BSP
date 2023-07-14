#[doc = "Register `RCC_CR` reader"]
pub struct R(crate::R<RCC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CR` writer"]
pub struct W(crate::W<RCC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CR_SPEC>;
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
impl From<crate::W<RCC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSION` reader - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
pub type HSION_R = crate::BitReader<HSION_A>;
#[doc = "HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    #[doc = "0: HSI16 oscillator OFF"]
    B_0X0 = 0,
    #[doc = "1: HSI16 oscillator ON"]
    B_0X1 = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::B_0X0,
            true => HSION_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSION_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSION_A::B_0X1
    }
}
#[doc = "Field `HSION` writer - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CR_SPEC, O, HSION_A>;
impl<'a, const O: u8> HSION_W<'a, O> {
    #[doc = "HSI16 oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSION_A::B_0X0)
    }
    #[doc = "HSI16 oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSION_A::B_0X1)
    }
}
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
pub type HSIKERON_R = crate::BitReader<HSIKERON_A>;
#[doc = "HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON_A {
    #[doc = "0: No effect on HSI16 oscillator."]
    B_0X0 = 0,
    #[doc = "1: HSI16 oscillator is forced ON even in Stop mode."]
    B_0X1 = 1,
}
impl From<HSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIKERON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIKERON_A {
        match self.bits {
            false => HSIKERON_A::B_0X0,
            true => HSIKERON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIKERON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIKERON_A::B_0X1
    }
}
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CR_SPEC, O, HSIKERON_A>;
impl<'a, const O: u8> HSIKERON_W<'a, O> {
    #[doc = "No effect on HSI16 oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIKERON_A::B_0X0)
    }
    #[doc = "HSI16 oscillator is forced ON even in Stop mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIKERON_A::B_0X1)
    }
}
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
pub type HSIRDY_R = crate::BitReader<HSIRDY_A>;
#[doc = "HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY_A {
    #[doc = "0: HSI16 oscillator not ready"]
    B_0X0 = 0,
    #[doc = "1: HSI16 oscillator ready"]
    B_0X1 = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::B_0X0,
            true => HSIRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDY_A::B_0X1
    }
}
#[doc = "Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
pub type HSEON_R = crate::BitReader<HSEON_A>;
#[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON_A {
    #[doc = "0: HSE oscillator OFF"]
    B_0X0 = 0,
    #[doc = "1: HSE oscillator ON"]
    B_0X1 = 1,
}
impl From<HSEON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEON_A {
        match self.bits {
            false => HSEON_A::B_0X0,
            true => HSEON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEON_A::B_0X1
    }
}
#[doc = "Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CR_SPEC, O, HSEON_A>;
impl<'a, const O: u8> HSEON_W<'a, O> {
    #[doc = "HSE oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEON_A::B_0X0)
    }
    #[doc = "HSE oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEON_A::B_0X1)
    }
}
#[doc = "Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
pub type HSERDY_R = crate::BitReader<HSERDY_A>;
#[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY_A {
    #[doc = "0: HSE oscillator not ready"]
    B_0X0 = 0,
    #[doc = "1: HSE oscillator ready"]
    B_0X1 = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDY_A {
        match self.bits {
            false => HSERDY_A::B_0X0,
            true => HSERDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDY_A::B_0X1
    }
}
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
#[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    B_0X0 = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    B_0X1 = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::B_0X0,
            true => HSEBYP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEBYP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEBYP_A::B_0X1
    }
}
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CR_SPEC, O, HSEBYP_A>;
impl<'a, const O: u8> HSEBYP_W<'a, O> {
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEBYP_A::B_0X0)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEBYP_A::B_0X1)
    }
}
#[doc = "Field `CSSON` reader - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
pub type CSSON_R = crate::BitReader<CSSON_A>;
#[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON_A {
    #[doc = "0: Clock security system OFF (clock detector OFF)"]
    B_0X0 = 0,
    #[doc = "1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    B_0X1 = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::B_0X0,
            true => CSSON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSSON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSSON_A::B_0X1
    }
}
#[doc = "Field `CSSON` writer - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CR_SPEC, O, CSSON_A>;
impl<'a, const O: u8> CSSON_W<'a, O> {
    #[doc = "Clock security system OFF (clock detector OFF)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSSON_A::B_0X0)
    }
    #[doc = "Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSSON_A::B_0X1)
    }
}
#[doc = "Field `PLLON` reader - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
pub type PLLON_R = crate::BitReader<PLLON_A>;
#[doc = "Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON_A {
    #[doc = "0: PLL OFF"]
    B_0X0 = 0,
    #[doc = "1: PLL ON"]
    B_0X1 = 1,
}
impl From<PLLON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLON_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLON_A {
        match self.bits {
            false => PLLON_A::B_0X0,
            true => PLLON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLON_A::B_0X1
    }
}
#[doc = "Field `PLLON` writer - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CR_SPEC, O, PLLON_A>;
impl<'a, const O: u8> PLLON_W<'a, O> {
    #[doc = "PLL OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLON_A::B_0X0)
    }
    #[doc = "PLL ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLON_A::B_0X1)
    }
}
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked."]
pub type PLLRDY_R = crate::BitReader<PLLRDY_A>;
#[doc = "Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY_A {
    #[doc = "0: PLL unlocked"]
    B_0X0 = 0,
    #[doc = "1: PLL locked"]
    B_0X1 = 1,
}
impl From<PLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDY_A {
        match self.bits {
            false => PLLRDY_A::B_0X0,
            true => PLLRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked."]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<8> {
        HSION_W::new(self)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<9> {
        HSIKERON_W::new(self)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    #[doc = "Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    #[doc = "Bit 24 - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cr](index.html) module"]
pub struct RCC_CR_SPEC;
impl crate::RegisterSpec for RCC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cr::R](R) reader structure"]
impl crate::Readable for RCC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cr::W](W) writer structure"]
impl crate::Writable for RCC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CR to value 0x63"]
impl crate::Resettable for RCC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x63;
}
