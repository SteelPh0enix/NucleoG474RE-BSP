#[doc = "Register `RCC_CIFR` reader"]
pub struct R(crate::R<RCC_CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
pub type LSIRDYF_R = crate::BitReader<LSIRDYF_A>;
#[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF_A {
    #[doc = "0: No clock ready interrupt caused by the LSI oscillator"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the LSI oscillator"]
    B_0X1 = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::B_0X0,
            true => LSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDYF_A::B_0X1
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
pub type LSERDYF_R = crate::BitReader<LSERDYF_A>;
#[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF_A {
    #[doc = "0: No clock ready interrupt caused by the LSE oscillator"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the LSE oscillator"]
    B_0X1 = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::B_0X0,
            true => LSERDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDYF_A::B_0X1
    }
}
#[doc = "Field `HSIRDYF` reader - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
pub type HSIRDYF_R = crate::BitReader<HSIRDYF_A>;
#[doc = "HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF_A {
    #[doc = "0: No clock ready interrupt caused by the HSI16 oscillator"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the HSI16 oscillator"]
    B_0X1 = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::B_0X0,
            true => HSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDYF_A::B_0X1
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit."]
pub type HSERDYF_R = crate::BitReader<HSERDYF_A>;
#[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF_A {
    #[doc = "0: No clock ready interrupt caused by the HSE oscillator"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the HSE oscillator"]
    B_0X1 = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::B_0X0,
            true => HSERDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDYF_A::B_0X1
    }
}
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit."]
pub type PLLRDYF_R = crate::BitReader<PLLRDYF_A>;
#[doc = "PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF_A {
    #[doc = "0: No clock ready interrupt caused by PLL lock"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by PLL lock"]
    B_0X1 = 1,
}
impl From<PLLRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYF_A {
        match self.bits {
            false => PLLRDYF_A::B_0X0,
            true => PLLRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLRDYF_A::B_0X1
    }
}
#[doc = "Field `CSSF` reader - Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
pub type CSSF_R = crate::BitReader<CSSF_A>;
#[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF_A {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    B_0X0 = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    B_0X1 = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::B_0X0,
            true => CSSF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSSF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSSF_A::B_0X1
    }
}
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit."]
pub type LSECSSF_R = crate::BitReader<LSECSSF_A>;
#[doc = "LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF_A {
    #[doc = "0: No clock security interrupt caused by LSE clock failure"]
    B_0X0 = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure"]
    B_0X1 = 1,
}
impl From<LSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSF_A {
        match self.bits {
            false => LSECSSF_A::B_0X0,
            true => LSECSSF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSF_A::B_0X1
    }
}
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit."]
pub type HSI48RDYF_R = crate::BitReader<HSI48RDYF_A>;
#[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYF_A {
    #[doc = "0: No clock ready interrupt caused by the HSI48 oscillator"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the HSI48 oscillator"]
    B_0X1 = 1,
}
impl From<HSI48RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYF_A {
        match self.bits {
            false => HSI48RDYF_A::B_0X0,
            true => HSI48RDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSI48RDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSI48RDYF_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit."]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit."]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit."]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit."]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cifr](index.html) module"]
pub struct RCC_CIFR_SPEC;
impl crate::RegisterSpec for RCC_CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cifr::R](R) reader structure"]
impl crate::Readable for RCC_CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCC_CIFR to value 0"]
impl crate::Resettable for RCC_CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
