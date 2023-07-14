#[doc = "Register `RCC_BDCR` reader"]
pub struct R(crate::R<RCC_BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_BDCR` writer"]
pub struct W(crate::W<RCC_BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_BDCR_SPEC>;
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
impl From<crate::W<RCC_BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable Set and cleared by software."]
pub type LSEON_R = crate::BitReader<LSEON_A>;
#[doc = "LSE oscillator enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator OFF"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator ON"]
    B_0X1 = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::B_0X0,
            true => LSEON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEON_A::B_0X1
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable Set and cleared by software."]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, LSEON_A>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    #[doc = "LSE oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSEON_A::B_0X0)
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSEON_A::B_0X1)
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
pub type LSERDY_R = crate::BitReader<LSERDY_A>;
#[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY_A {
    #[doc = "0: LSE oscillator not ready"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator ready"]
    B_0X1 = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::B_0X0,
            true => LSERDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDY_A::B_0X1
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
#[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE oscillator not bypassed"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator bypassed"]
    B_0X1 = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::B_0X0,
            true => LSEBYP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEBYP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEBYP_A::B_0X1
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, LSEBYP_A>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSEBYP_A::B_0X0)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSEBYP_A::B_0X1)
    }
}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
pub type LSEDRV_R = crate::FieldReader<LSEDRV_A>;
#[doc = "LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: Xtal mode lower driving capability"]
    B_0X0 = 0,
    #[doc = "1: Xtal mode medium low driving capability"]
    B_0X1 = 1,
    #[doc = "2: Xtal mode medium high driving capability"]
    B_0X2 = 2,
    #[doc = "3: Xtal mode higher driving capability"]
    B_0X3 = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSEDRV_A {
    type Ux = u8;
}
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::B_0X0,
            1 => LSEDRV_A::B_0X1,
            2 => LSEDRV_A::B_0X2,
            3 => LSEDRV_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEDRV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEDRV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LSEDRV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LSEDRV_A::B_0X3
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_BDCR_SPEC, 2, O, LSEDRV_A>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X0)
    }
    #[doc = "Xtal mode medium low driving capability"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X1)
    }
    #[doc = "Xtal mode medium high driving capability"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X2)
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X3)
    }
}
#[doc = "Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
pub type LSECSSON_R = crate::BitReader<LSECSSON_A>;
#[doc = "CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON_A {
    #[doc = "0: CSS on LSE (32 kHz external oscillator) OFF"]
    B_0X0 = 0,
    #[doc = "1: CSS on LSE (32 kHz external oscillator) ON"]
    B_0X1 = 1,
}
impl From<LSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSON_A {
        match self.bits {
            false => LSECSSON_A::B_0X0,
            true => LSECSSON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSON_A::B_0X1
    }
}
#[doc = "Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, LSECSSON_A>;
impl<'a, const O: u8> LSECSSON_W<'a, O> {
    #[doc = "CSS on LSE (32 kHz external oscillator) OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSECSSON_A::B_0X0)
    }
    #[doc = "CSS on LSE (32 kHz external oscillator) ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSECSSON_A::B_0X1)
    }
}
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE)."]
pub type LSECSSD_R = crate::BitReader<LSECSSD_A>;
#[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD_A {
    #[doc = "0: No failure detected on LSE (32 kHz oscillator)"]
    B_0X0 = 0,
    #[doc = "1: Failure detected on LSE (32 kHz oscillator)"]
    B_0X1 = 1,
}
impl From<LSECSSD_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSD_A {
        match self.bits {
            false => LSECSSD_A::B_0X0,
            true => LSECSSD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSD_A::B_0X1
    }
}
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
pub type RTCSEL_R = crate::FieldReader<RTCSEL_A>;
#[doc = "RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: No clock"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    B_0X1 = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    B_0X2 = 2,
    #[doc = "3: HSE oscillator clock divided by 32 used as RTC clock"]
    B_0X3 = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL_A {
    type Ux = u8;
}
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::B_0X0,
            1 => RTCSEL_A::B_0X1,
            2 => RTCSEL_A::B_0X2,
            3 => RTCSEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCSEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RTCSEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RTCSEL_A::B_0X3
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_BDCR_SPEC, 2, O, RTCSEL_A>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCSEL_A::B_0X0)
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCSEL_A::B_0X1)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(RTCSEL_A::B_0X2)
    }
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(RTCSEL_A::B_0X3)
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable Set and cleared by software."]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "RTC clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: RTC clock disabled"]
    B_0X0 = 0,
    #[doc = "1: RTC clock enabled"]
    B_0X1 = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::B_0X0,
            true => RTCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCEN_A::B_0X1
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable Set and cleared by software."]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, RTCEN_A>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCEN_A::B_0X0)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCEN_A::B_0X1)
    }
}
#[doc = "Field `BDRST` reader - RTC domain software reset Set and cleared by software."]
pub type BDRST_R = crate::BitReader<BDRST_A>;
#[doc = "RTC domain software reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST_A {
    #[doc = "0: Reset not activated"]
    B_0X0 = 0,
    #[doc = "1: Reset the entire RTC domain"]
    B_0X1 = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
impl BDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::B_0X0,
            true => BDRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BDRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BDRST_A::B_0X1
    }
}
#[doc = "Field `BDRST` writer - RTC domain software reset Set and cleared by software."]
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, BDRST_A>;
impl<'a, const O: u8> BDRST_W<'a, O> {
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BDRST_A::B_0X0)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BDRST_A::B_0X1)
    }
}
#[doc = "Field `LSCOEN` reader - Low speed clock output enable Set and cleared by software."]
pub type LSCOEN_R = crate::BitReader<LSCOEN_A>;
#[doc = "Low speed clock output enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN_A {
    #[doc = "0: Low speed clock output (LSCO) disable"]
    B_0X0 = 0,
    #[doc = "1: Low speed clock output (LSCO) enable"]
    B_0X1 = 1,
}
impl From<LSCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LSCOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSCOEN_A {
        match self.bits {
            false => LSCOEN_A::B_0X0,
            true => LSCOEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSCOEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSCOEN_A::B_0X1
    }
}
#[doc = "Field `LSCOEN` writer - Low speed clock output enable Set and cleared by software."]
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, LSCOEN_A>;
impl<'a, const O: u8> LSCOEN_W<'a, O> {
    #[doc = "Low speed clock output (LSCO) disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSCOEN_A::B_0X0)
    }
    #[doc = "Low speed clock output (LSCO) enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSCOEN_A::B_0X1)
    }
}
#[doc = "Field `LSCOSEL` reader - Low speed clock output selection Set and cleared by software."]
pub type LSCOSEL_R = crate::BitReader<LSCOSEL_A>;
#[doc = "Low speed clock output selection Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL_A {
    #[doc = "0: LSI clock selected"]
    B_0X0 = 0,
    #[doc = "1: LSE clock selected"]
    B_0X1 = 1,
}
impl From<LSCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LSCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSCOSEL_A {
        match self.bits {
            false => LSCOSEL_A::B_0X0,
            true => LSCOSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSCOSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSCOSEL_A::B_0X1
    }
}
#[doc = "Field `LSCOSEL` writer - Low speed clock output selection Set and cleared by software."]
pub type LSCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, RCC_BDCR_SPEC, O, LSCOSEL_A>;
impl<'a, const O: u8> LSCOSEL_W<'a, O> {
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSCOSEL_A::B_0X0)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSCOSEL_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE)."]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software."]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed clock output enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    #[doc = "Bit 24 - Low speed clock output enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    #[doc = "Bit 25 - Low speed clock output selection Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_bdcr](index.html) module"]
pub struct RCC_BDCR_SPEC;
impl crate::RegisterSpec for RCC_BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_bdcr::R](R) reader structure"]
impl crate::Readable for RCC_BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_bdcr::W](W) writer structure"]
impl crate::Writable for RCC_BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_BDCR to value 0"]
impl crate::Resettable for RCC_BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
