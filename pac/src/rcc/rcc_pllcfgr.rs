#[doc = "Register `RCC_PLLCFGR` reader"]
pub struct R(crate::R<RCC_PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLLCFGR` writer"]
pub struct W(crate::W<RCC_PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLLCFGR_SPEC>;
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
impl From<crate::W<RCC_PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
pub type PLLSRC_R = crate::FieldReader<PLLSRC_A>;
#[doc = "Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: No clock sent to PLL"]
    B_0X0 = 0,
    #[doc = "1: No clock sent to PLL"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as PLL clock entry"]
    B_0X2 = 2,
    #[doc = "3: HSE clock selected as PLL clock entry"]
    B_0X3 = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSRC_A {
    type Ux = u8;
}
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::B_0X0,
            1 => PLLSRC_A::B_0X1,
            2 => PLLSRC_A::B_0X2,
            3 => PLLSRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLLSRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PLLSRC_A::B_0X3
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_PLLCFGR_SPEC, 2, O, PLLSRC_A>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLSRC_A::B_0X0)
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLSRC_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLLSRC_A::B_0X2)
    }
    #[doc = "HSE clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PLLSRC_A::B_0X3)
    }
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 &lt;= PLLM &lt;= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
pub type PLLM_R = crate::FieldReader<PLLM_A>;
#[doc = "Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 &lt;= PLLM &lt;= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM_A {
    #[doc = "0: PLLM = 1"]
    B_0X0 = 0,
    #[doc = "1: PLLM = 2"]
    B_0X1 = 1,
    #[doc = "2: PLLM = 3"]
    B_0X2 = 2,
    #[doc = "3: PLLM = 4"]
    B_0X3 = 3,
    #[doc = "4: PLLM = 5"]
    B_0X4 = 4,
    #[doc = "5: PLLM = 6"]
    B_0X5 = 5,
    #[doc = "6: PLLM = 7"]
    B_0X6 = 6,
    #[doc = "7: PLLM = 8"]
    B_0X7 = 7,
    #[doc = "8: PLLSYSM = 9"]
    B_0X8 = 8,
    #[doc = "15: PLLSYSM= 16"]
    B_0X_F = 15,
}
impl From<PLLM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLM_A {
    type Ux = u8;
}
impl PLLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLM_A> {
        match self.bits {
            0 => Some(PLLM_A::B_0X0),
            1 => Some(PLLM_A::B_0X1),
            2 => Some(PLLM_A::B_0X2),
            3 => Some(PLLM_A::B_0X3),
            4 => Some(PLLM_A::B_0X4),
            5 => Some(PLLM_A::B_0X5),
            6 => Some(PLLM_A::B_0X6),
            7 => Some(PLLM_A::B_0X7),
            8 => Some(PLLM_A::B_0X8),
            15 => Some(PLLM_A::B_0X_F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLM_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLLM_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PLLM_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PLLM_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PLLM_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PLLM_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PLLM_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PLLM_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X_F`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == PLLM_A::B_0X_F
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 &lt;= PLLM &lt;= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
pub type PLLM_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_PLLCFGR_SPEC, 4, O, PLLM_A>;
impl<'a, const O: u8> PLLM_W<'a, O> {
    #[doc = "PLLM = 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X0)
    }
    #[doc = "PLLM = 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X1)
    }
    #[doc = "PLLM = 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X2)
    }
    #[doc = "PLLM = 4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X3)
    }
    #[doc = "PLLM = 5"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X4)
    }
    #[doc = "PLLM = 6"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X5)
    }
    #[doc = "PLLM = 7"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X6)
    }
    #[doc = "PLLM = 8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X7)
    }
    #[doc = "PLLSYSM = 9"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X8)
    }
    #[doc = "PLLSYSM= 16"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(PLLM_A::B_0X_F)
    }
}
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =&lt; PLLN =&lt; 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
pub type PLLN_R = crate::FieldReader<PLLN_A>;
#[doc = "Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =&lt; PLLN =&lt; 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLN_A {
    #[doc = "0: PLLN = 0 wrong configuration"]
    B_0X0 = 0,
    #[doc = "1: PLLN = 1 wrong configuration"]
    B_0X1 = 1,
    #[doc = "7: PLLN = 7 wrong configuration"]
    B_0X7 = 7,
    #[doc = "8: PLLN = 8"]
    B_0X8 = 8,
    #[doc = "9: PLLN = 9"]
    B_0X9 = 9,
    #[doc = "127: PLLN = 127"]
    B_0X7F = 127,
}
impl From<PLLN_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLN_A {
    type Ux = u8;
}
impl PLLN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLN_A> {
        match self.bits {
            0 => Some(PLLN_A::B_0X0),
            1 => Some(PLLN_A::B_0X1),
            7 => Some(PLLN_A::B_0X7),
            8 => Some(PLLN_A::B_0X8),
            9 => Some(PLLN_A::B_0X9),
            127 => Some(PLLN_A::B_0X7F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLN_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PLLN_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PLLN_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PLLN_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == PLLN_A::B_0X7F
    }
}
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =&lt; PLLN =&lt; 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_PLLCFGR_SPEC, 7, O, PLLN_A>;
impl<'a, const O: u8> PLLN_W<'a, O> {
    #[doc = "PLLN = 0 wrong configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLN_A::B_0X0)
    }
    #[doc = "PLLN = 1 wrong configuration"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLN_A::B_0X1)
    }
    #[doc = "PLLN = 7 wrong configuration"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PLLN_A::B_0X7)
    }
    #[doc = "PLLN = 8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PLLN_A::B_0X8)
    }
    #[doc = "PLLN = 9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(PLLN_A::B_0X9)
    }
    #[doc = "PLLN = 127"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(PLLN_A::B_0X7F)
    }
}
#[doc = "Field `PLLPEN` reader - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
pub type PLLPEN_R = crate::BitReader<PLLPEN_A>;
#[doc = "Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPEN_A {
    #[doc = "0: PLL P clock output disable"]
    B_0X0 = 0,
    #[doc = "1: PLL P clock output enable"]
    B_0X1 = 1,
}
impl From<PLLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPEN_A {
        match self.bits {
            false => PLLPEN_A::B_0X0,
            true => PLLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLPEN_A::B_0X1
    }
}
#[doc = "Field `PLLPEN` writer - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_PLLCFGR_SPEC, O, PLLPEN_A>;
impl<'a, const O: u8> PLLPEN_W<'a, O> {
    #[doc = "PLL P clock output disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLPEN_A::B_0X0)
    }
    #[doc = "PLL P clock output enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLPEN_A::B_0X1)
    }
}
#[doc = "Field `PLLP` reader - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\]
is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PLLP_R = crate::BitReader<PLLP_A>;
#[doc = "Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\]
is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLP_A {
    #[doc = "0: PLLP = 7"]
    B_0X0 = 0,
    #[doc = "1: PLLP = 17"]
    B_0X1 = 1,
}
impl From<PLLP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            false => PLLP_A::B_0X0,
            true => PLLP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLP_A::B_0X1
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\]
is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PLLP_W<'a, const O: u8> = crate::BitWriter<'a, RCC_PLLCFGR_SPEC, O, PLLP_A>;
impl<'a, const O: u8> PLLP_W<'a, O> {
    #[doc = "PLLP = 7"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLP_A::B_0X0)
    }
    #[doc = "PLLP = 17"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLP_A::B_0X1)
    }
}
#[doc = "Field `PLLQEN` reader - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
pub type PLLQEN_R = crate::BitReader<PLLQEN_A>;
#[doc = "Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLQEN_A {
    #[doc = "0: PLL Q clock output disable"]
    B_0X0 = 0,
    #[doc = "1: PLL Q clock output enable"]
    B_0X1 = 1,
}
impl From<PLLQEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLQEN_A {
        match self.bits {
            false => PLLQEN_A::B_0X0,
            true => PLLQEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLQEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLQEN_A::B_0X1
    }
}
#[doc = "Field `PLLQEN` writer - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_PLLCFGR_SPEC, O, PLLQEN_A>;
impl<'a, const O: u8> PLLQEN_W<'a, O> {
    #[doc = "PLL Q clock output disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLQEN_A::B_0X0)
    }
    #[doc = "PLL Q clock output enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLQEN_A::B_0X1)
    }
}
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PLLQ_R = crate::FieldReader<PLLQ_A>;
#[doc = "Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ_A {
    #[doc = "0: PLLQ = 2"]
    B_0X0 = 0,
    #[doc = "1: PLLQ = 4"]
    B_0X1 = 1,
    #[doc = "2: PLLQ = 6"]
    B_0X2 = 2,
    #[doc = "3: PLLQ = 8"]
    B_0X3 = 3,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLQ_A {
    type Ux = u8;
}
impl PLLQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLQ_A {
        match self.bits {
            0 => PLLQ_A::B_0X0,
            1 => PLLQ_A::B_0X1,
            2 => PLLQ_A::B_0X2,
            3 => PLLQ_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLQ_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLQ_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLLQ_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PLLQ_A::B_0X3
    }
}
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_PLLCFGR_SPEC, 2, O, PLLQ_A>;
impl<'a, const O: u8> PLLQ_W<'a, O> {
    #[doc = "PLLQ = 2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLQ_A::B_0X0)
    }
    #[doc = "PLLQ = 4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLQ_A::B_0X1)
    }
    #[doc = "PLLQ = 6"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLLQ_A::B_0X2)
    }
    #[doc = "PLLQ = 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PLLQ_A::B_0X3)
    }
}
#[doc = "Field `PLLREN` reader - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
pub type PLLREN_R = crate::BitReader<PLLREN_A>;
#[doc = "PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLREN_A {
    #[doc = "0: PLL R clock output disable"]
    B_0X0 = 0,
    #[doc = "1: PLL R clock output enable"]
    B_0X1 = 1,
}
impl From<PLLREN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLREN_A {
        match self.bits {
            false => PLLREN_A::B_0X0,
            true => PLLREN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLREN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLREN_A::B_0X1
    }
}
#[doc = "Field `PLLREN` writer - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_PLLCFGR_SPEC, O, PLLREN_A>;
impl<'a, const O: u8> PLLREN_W<'a, O> {
    #[doc = "PLL R clock output disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLREN_A::B_0X0)
    }
    #[doc = "PLL R clock output enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLREN_A::B_0X1)
    }
}
#[doc = "Field `PLLR` reader - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PLLR_R = crate::FieldReader<PLLR_A>;
#[doc = "Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLR_A {
    #[doc = "0: PLLR = 2"]
    B_0X0 = 0,
    #[doc = "1: PLLR = 4"]
    B_0X1 = 1,
    #[doc = "2: PLLR = 6"]
    B_0X2 = 2,
    #[doc = "3: PLLR = 8"]
    B_0X3 = 3,
}
impl From<PLLR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLR_A {
    type Ux = u8;
}
impl PLLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLR_A {
        match self.bits {
            0 => PLLR_A::B_0X0,
            1 => PLLR_A::B_0X1,
            2 => PLLR_A::B_0X2,
            3 => PLLR_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLR_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLLR_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PLLR_A::B_0X3
    }
}
#[doc = "Field `PLLR` writer - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PLLR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_PLLCFGR_SPEC, 2, O, PLLR_A>;
impl<'a, const O: u8> PLLR_W<'a, O> {
    #[doc = "PLLR = 2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLR_A::B_0X0)
    }
    #[doc = "PLLR = 4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLR_A::B_0X1)
    }
    #[doc = "PLLR = 6"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLLR_A::B_0X2)
    }
    #[doc = "PLLR = 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PLLR_A::B_0X3)
    }
}
#[doc = "Field `PLLPDIV` reader - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
pub type PLLPDIV_R = crate::FieldReader<PLLPDIV_A>;
#[doc = "Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLPDIV_A {
    #[doc = "0: PLL P clock is controlled by the bit PLLP"]
    B_0X0 = 0,
    #[doc = "1: Reserved."]
    B_0X1 = 1,
    #[doc = "2: PLL P clock = VCO / 2"]
    B_0X2 = 2,
    #[doc = "31: PLL P clock = VCO / 31"]
    B_0X1F = 31,
}
impl From<PLLPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLPDIV_A {
    type Ux = u8;
}
impl PLLPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLPDIV_A> {
        match self.bits {
            0 => Some(PLLPDIV_A::B_0X0),
            1 => Some(PLLPDIV_A::B_0X1),
            2 => Some(PLLPDIV_A::B_0X2),
            31 => Some(PLLPDIV_A::B_0X1F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLPDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLPDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLLPDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X1F`"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == PLLPDIV_A::B_0X1F
    }
}
#[doc = "Field `PLLPDIV` writer - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
pub type PLLPDIV_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_PLLCFGR_SPEC, 5, O, PLLPDIV_A>;
impl<'a, const O: u8> PLLPDIV_W<'a, O> {
    #[doc = "PLL P clock is controlled by the bit PLLP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLPDIV_A::B_0X0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLPDIV_A::B_0X1)
    }
    #[doc = "PLL P clock = VCO / 2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLLPDIV_A::B_0X2)
    }
    #[doc = "PLL P clock = VCO / 31"]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut W {
        self.variant(PLLPDIV_A::B_0X1F)
    }
}
impl R {
    #[doc = "Bits 0:1 - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 &lt;= PLLM &lt;= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =&lt; PLLN =&lt; 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\]
is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 &lt;= PLLM &lt;= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<4> {
        PLLM_W::new(self)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =&lt; PLLN =&lt; 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    #[doc = "Bit 16 - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\]
is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    #[doc = "Bit 20 - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<20> {
        PLLQEN_W::new(self)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<21> {
        PLLQ_W::new(self)
    }
    #[doc = "Bit 24 - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<24> {
        PLLREN_W::new(self)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<25> {
        PLLR_W::new(self)
    }
    #[doc = "Bits 27:31 - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
    #[inline(always)]
    #[must_use]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<27> {
        PLLPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pllcfgr](index.html) module"]
pub struct RCC_PLLCFGR_SPEC;
impl crate::RegisterSpec for RCC_PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pllcfgr::R](R) reader structure"]
impl crate::Readable for RCC_PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pllcfgr::W](W) writer structure"]
impl crate::Writable for RCC_PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_PLLCFGR to value 0x1000"]
impl crate::Resettable for RCC_PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
