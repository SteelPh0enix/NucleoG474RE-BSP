#[doc = "Register `RCC_CSR` reader"]
pub struct R(crate::R<RCC_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CSR` writer"]
pub struct W(crate::W<RCC_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CSR_SPEC>;
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
impl From<crate::W<RCC_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable Set and cleared by software."]
pub type LSION_R = crate::BitReader<LSION_A>;
#[doc = "LSI oscillator enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION_A {
    #[doc = "0: LSI oscillator OFF"]
    B_0X0 = 0,
    #[doc = "1: LSI oscillator ON"]
    B_0X1 = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::B_0X0,
            true => LSION_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSION_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSION_A::B_0X1
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable Set and cleared by software."]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CSR_SPEC, O, LSION_A>;
impl<'a, const O: u8> LSION_W<'a, O> {
    #[doc = "LSI oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSION_A::B_0X0)
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSION_A::B_0X1)
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
pub type LSIRDY_R = crate::BitReader<LSIRDY_A>;
#[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY_A {
    #[doc = "0: LSI oscillator not ready"]
    B_0X0 = 0,
    #[doc = "1: LSI oscillator ready"]
    B_0X1 = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::B_0X0,
            true => LSIRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDY_A::B_0X1
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag Set by software to clear the reset flags."]
pub type RMVF_R = crate::BitReader<RMVF_A>;
#[doc = "Remove reset flag Set by software to clear the reset flags.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear the reset flags"]
    B_0X1 = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMVF_A {
        match self.bits {
            false => RMVF_A::B_0X0,
            true => RMVF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RMVF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RMVF_A::B_0X1
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag Set by software to clear the reset flags."]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CSR_SPEC, O, RMVF_A>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RMVF_A::B_0X0)
    }
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RMVF_A::B_0X1)
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit."]
pub type OBLRSTF_R = crate::BitReader<OBLRSTF_A>;
#[doc = "Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF_A {
    #[doc = "0: No reset from Option Byte loading occurred"]
    B_0X0 = 0,
    #[doc = "1: Reset from Option Byte loading occurred"]
    B_0X1 = 1,
}
impl From<OBLRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl OBLRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBLRSTF_A {
        match self.bits {
            false => OBLRSTF_A::B_0X0,
            true => OBLRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OBLRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OBLRSTF_A::B_0X1
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
pub type PINRSTF_R = crate::BitReader<PINRSTF_A>;
#[doc = "Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF_A {
    #[doc = "0: No reset from NRST pin occurred"]
    B_0X0 = 0,
    #[doc = "1: Reset from NRST pin occurred"]
    B_0X1 = 1,
}
impl From<PINRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl PINRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINRSTF_A {
        match self.bits {
            false => PINRSTF_A::B_0X0,
            true => PINRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PINRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PINRSTF_A::B_0X1
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
pub type BORRSTF_R = crate::BitReader<BORRSTF_A>;
#[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF_A {
    #[doc = "0: No BOR occurred"]
    B_0X0 = 0,
    #[doc = "1: BOR occurred"]
    B_0X1 = 1,
}
impl From<BORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl BORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORRSTF_A {
        match self.bits {
            false => BORRSTF_A::B_0X0,
            true => BORRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BORRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BORRSTF_A::B_0X1
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
pub type SFTRSTF_R = crate::BitReader<SFTRSTF_A>;
#[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF_A {
    #[doc = "0: No software reset occurred"]
    B_0X0 = 0,
    #[doc = "1: Software reset occurred"]
    B_0X1 = 1,
}
impl From<SFTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl SFTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTRSTF_A {
        match self.bits {
            false => SFTRSTF_A::B_0X0,
            true => SFTRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SFTRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SFTRSTF_A::B_0X1
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF_A>;
#[doc = "Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF_A {
    #[doc = "0: No independent watchdog reset occurred"]
    B_0X0 = 0,
    #[doc = "1: Independent watchdog reset occurred"]
    B_0X1 = 1,
}
impl From<IWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDGRSTF_A {
        match self.bits {
            false => IWDGRSTF_A::B_0X0,
            true => IWDGRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDGRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDGRSTF_A::B_0X1
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF_A>;
#[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF_A {
    #[doc = "0: No window watchdog reset occurred"]
    B_0X0 = 0,
    #[doc = "1: Window watchdog reset occurred"]
    B_0X1 = 1,
}
impl From<WWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDGRSTF_A {
        match self.bits {
            false => WWDGRSTF_A::B_0X0,
            true => WWDGRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDGRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDGRSTF_A::B_0X1
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit."]
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF_A>;
#[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF_A {
    #[doc = "0: No illegal mode reset occurred"]
    B_0X0 = 0,
    #[doc = "1: Illegal mode reset occurred"]
    B_0X1 = 1,
}
impl From<LPWRRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl LPWRRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPWRRSTF_A {
        match self.bits {
            false => LPWRRSTF_A::B_0X0,
            true => LPWRRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPWRRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPWRRSTF_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag Set by software to clear the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bit 23 - Remove reset flag Set by software to clear the reset flags."]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_csr](index.html) module"]
pub struct RCC_CSR_SPEC;
impl crate::RegisterSpec for RCC_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_csr::R](R) reader structure"]
impl crate::Readable for RCC_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_csr::W](W) writer structure"]
impl crate::Writable for RCC_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CSR to value 0x0c00_0000"]
impl crate::Resettable for RCC_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
