#[doc = "Register `RCC_CFGR` reader"]
pub struct R(crate::R<RCC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CFGR` writer"]
pub struct W(crate::W<RCC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFGR_SPEC>;
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
impl From<crate::W<RCC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
pub type SW_R = crate::FieldReader<SW_A>;
#[doc = "System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: Reserved, must be kept at reset value"]
    B_0X0 = 0,
    #[doc = "1: HSI16 selected as system clock"]
    B_0X1 = 1,
    #[doc = "2: HSE selected as system clock"]
    B_0X2 = 2,
    #[doc = "3: PLL selected as system clock"]
    B_0X3 = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW_A {
    type Ux = u8;
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            0 => SW_A::B_0X0,
            1 => SW_A::B_0X1,
            2 => SW_A::B_0X2,
            3 => SW_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SW_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SW_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SW_A::B_0X3
    }
}
#[doc = "Field `SW` writer - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
pub type SW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_CFGR_SPEC, 2, O, SW_A>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SW_A::B_0X0)
    }
    #[doc = "HSI16 selected as system clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SW_A::B_0X1)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SW_A::B_0X2)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SW_A::B_0X3)
    }
}
#[doc = "Field `SWS` reader - System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
pub type SWS_R = crate::FieldReader<SWS_A>;
#[doc = "System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "0: Reserved, must be kept at reset value"]
    B_0X0 = 0,
    #[doc = "1: HSI16 oscillator used as system clock"]
    B_0X1 = 1,
    #[doc = "2: HSE used as system clock"]
    B_0X2 = 2,
    #[doc = "3: PLL used as system clock"]
    B_0X3 = 3,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWS_A {
    type Ux = u8;
}
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWS_A {
        match self.bits {
            0 => SWS_A::B_0X0,
            1 => SWS_A::B_0X1,
            2 => SWS_A::B_0X2,
            3 => SWS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SWS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SWS_A::B_0X3
    }
}
#[doc = "Field `HPRE` reader - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
pub type HPRE_R = crate::FieldReader<HPRE_A>;
#[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "8: SYSCLK divided by 2"]
    B_0X8 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    B_0X9 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    B_0X_A = 10,
    #[doc = "11: SYSCLK divided by 16"]
    B_0X_B = 11,
    #[doc = "12: SYSCLK divided by 64"]
    B_0X_C = 12,
    #[doc = "13: SYSCLK divided by 128"]
    B_0X_D = 13,
    #[doc = "14: SYSCLK divided by 256"]
    B_0X_E = 14,
    #[doc = "15: SYSCLK divided by 512"]
    B_0X_F = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE_A {
    type Ux = u8;
}
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            8 => Some(HPRE_A::B_0X8),
            9 => Some(HPRE_A::B_0X9),
            10 => Some(HPRE_A::B_0X_A),
            11 => Some(HPRE_A::B_0X_B),
            12 => Some(HPRE_A::B_0X_C),
            13 => Some(HPRE_A::B_0X_D),
            14 => Some(HPRE_A::B_0X_E),
            15 => Some(HPRE_A::B_0X_F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == HPRE_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == HPRE_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0X_A`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == HPRE_A::B_0X_A
    }
    #[doc = "Checks if the value of the field is `B_0X_B`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == HPRE_A::B_0X_B
    }
    #[doc = "Checks if the value of the field is `B_0X_C`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == HPRE_A::B_0X_C
    }
    #[doc = "Checks if the value of the field is `B_0X_D`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == HPRE_A::B_0X_D
    }
    #[doc = "Checks if the value of the field is `B_0X_E`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == HPRE_A::B_0X_E
    }
    #[doc = "Checks if the value of the field is `B_0X_F`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == HPRE_A::B_0X_F
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CFGR_SPEC, 4, O, HPRE_A>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X8)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X9)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X_A)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X_B)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X_C)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X_D)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X_E)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(HPRE_A::B_0X_F)
    }
}
#[doc = "Field `PPRE1` reader - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
pub type PPRE1_R = crate::FieldReader<PPRE1_A>;
#[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1_A {
    #[doc = "4: HCLK divided by 2"]
    B_0X4 = 4,
    #[doc = "5: HCLK divided by 4"]
    B_0X5 = 5,
    #[doc = "6: HCLK divided by 8"]
    B_0X6 = 6,
    #[doc = "7: HCLK divided by 16"]
    B_0X7 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1_A {
    type Ux = u8;
}
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            4 => Some(PPRE1_A::B_0X4),
            5 => Some(PPRE1_A::B_0X5),
            6 => Some(PPRE1_A::B_0X6),
            7 => Some(PPRE1_A::B_0X7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PPRE1_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PPRE1_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PPRE1_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PPRE1_A::B_0X7
    }
}
#[doc = "Field `PPRE1` writer - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CFGR_SPEC, 3, O, PPRE1_A>;
impl<'a, const O: u8> PPRE1_W<'a, O> {
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PPRE1_A::B_0X4)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PPRE1_A::B_0X5)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PPRE1_A::B_0X6)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PPRE1_A::B_0X7)
    }
}
#[doc = "Field `PPRE2` reader - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
pub type PPRE2_R = crate::FieldReader<PPRE2_A>;
#[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE2_A {
    #[doc = "4: HCLK divided by 2"]
    B_0X4 = 4,
    #[doc = "5: HCLK divided by 4"]
    B_0X5 = 5,
    #[doc = "6: HCLK divided by 8"]
    B_0X6 = 6,
    #[doc = "7: HCLK divided by 16"]
    B_0X7 = 7,
}
impl From<PPRE2_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE2_A {
    type Ux = u8;
}
impl PPRE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE2_A> {
        match self.bits {
            4 => Some(PPRE2_A::B_0X4),
            5 => Some(PPRE2_A::B_0X5),
            6 => Some(PPRE2_A::B_0X6),
            7 => Some(PPRE2_A::B_0X7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PPRE2_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PPRE2_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PPRE2_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PPRE2_A::B_0X7
    }
}
#[doc = "Field `PPRE2` writer - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
pub type PPRE2_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CFGR_SPEC, 3, O, PPRE2_A>;
impl<'a, const O: u8> PPRE2_W<'a, O> {
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PPRE2_A::B_0X4)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PPRE2_A::B_0X5)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PPRE2_A::B_0X6)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PPRE2_A::B_0X7)
    }
}
#[doc = "Field `MCOSEL` reader - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
pub type MCOSEL_R = crate::FieldReader<MCOSEL_A>;
#[doc = "Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL_A {
    #[doc = "0: MCO output disabled, no clock on MCO"]
    B_0X0 = 0,
    #[doc = "1: SYSCLK system clock selected"]
    B_0X1 = 1,
    #[doc = "2: Reserved, must be kept at reset value"]
    B_0X2 = 2,
    #[doc = "3: HSI16 clock selected"]
    B_0X3 = 3,
    #[doc = "4: HSE clock selected"]
    B_0X4 = 4,
    #[doc = "5: Main PLL clock selected"]
    B_0X5 = 5,
    #[doc = "6: LSI clock selected"]
    B_0X6 = 6,
    #[doc = "7: LSE clock selected"]
    B_0X7 = 7,
    #[doc = "8: Internal HSI48 clock selected"]
    B_0X8 = 8,
}
impl From<MCOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOSEL_A {
    type Ux = u8;
}
impl MCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOSEL_A> {
        match self.bits {
            0 => Some(MCOSEL_A::B_0X0),
            1 => Some(MCOSEL_A::B_0X1),
            2 => Some(MCOSEL_A::B_0X2),
            3 => Some(MCOSEL_A::B_0X3),
            4 => Some(MCOSEL_A::B_0X4),
            5 => Some(MCOSEL_A::B_0X5),
            6 => Some(MCOSEL_A::B_0X6),
            7 => Some(MCOSEL_A::B_0X7),
            8 => Some(MCOSEL_A::B_0X8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCOSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCOSEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCOSEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCOSEL_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCOSEL_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MCOSEL_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MCOSEL_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MCOSEL_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MCOSEL_A::B_0X8
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CFGR_SPEC, 4, O, MCOSEL_A>;
impl<'a, const O: u8> MCOSEL_W<'a, O> {
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X0)
    }
    #[doc = "SYSCLK system clock selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X1)
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X2)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X3)
    }
    #[doc = "HSE clock selected"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X4)
    }
    #[doc = "Main PLL clock selected"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X5)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X6)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X7)
    }
    #[doc = "Internal HSI48 clock selected"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(MCOSEL_A::B_0X8)
    }
}
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
pub type MCOPRE_R = crate::FieldReader<MCOPRE_A>;
#[doc = "Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE_A {
    #[doc = "0: MCO is divided by 1"]
    B_0X0 = 0,
    #[doc = "1: MCO is divided by 2"]
    B_0X1 = 1,
    #[doc = "2: MCO is divided by 4"]
    B_0X2 = 2,
    #[doc = "3: MCO is divided by 8"]
    B_0X3 = 3,
    #[doc = "4: MCO is divided by 16"]
    B_0X4 = 4,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOPRE_A {
    type Ux = u8;
}
impl MCOPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOPRE_A> {
        match self.bits {
            0 => Some(MCOPRE_A::B_0X0),
            1 => Some(MCOPRE_A::B_0X1),
            2 => Some(MCOPRE_A::B_0X2),
            3 => Some(MCOPRE_A::B_0X3),
            4 => Some(MCOPRE_A::B_0X4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCOPRE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCOPRE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCOPRE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCOPRE_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCOPRE_A::B_0X4
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CFGR_SPEC, 3, O, MCOPRE_A>;
impl<'a, const O: u8> MCOPRE_W<'a, O> {
    #[doc = "MCO is divided by 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCOPRE_A::B_0X0)
    }
    #[doc = "MCO is divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCOPRE_A::B_0X1)
    }
    #[doc = "MCO is divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCOPRE_A::B_0X2)
    }
    #[doc = "MCO is divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MCOPRE_A::B_0X3)
    }
    #[doc = "MCO is divided by 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MCOPRE_A::B_0X4)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<24> {
        MCOSEL_W::new(self)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cfgr](index.html) module"]
pub struct RCC_CFGR_SPEC;
impl crate::RegisterSpec for RCC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cfgr::R](R) reader structure"]
impl crate::Readable for RCC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cfgr::W](W) writer structure"]
impl crate::Writable for RCC_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CFGR to value 0x05"]
impl crate::Resettable for RCC_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
