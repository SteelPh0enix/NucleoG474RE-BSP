#[doc = "Register `RCC_CICR` writer"]
pub struct W(crate::W<RCC_CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CICR_SPEC>;
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
impl From<crate::W<RCC_CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: LSIRDYF cleared"]
    B_0X1 = 1,
}
impl From<LSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, LSIRDYC_AW>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSIRDYC_AW::B_0X0)
    }
    #[doc = "LSIRDYF cleared"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSIRDYC_AW::B_0X1)
    }
}
#[doc = "LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: LSERDYF cleared"]
    B_0X1 = 1,
}
impl From<LSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, LSERDYC_AW>;
impl<'a, const O: u8> LSERDYC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSERDYC_AW::B_0X0)
    }
    #[doc = "LSERDYF cleared"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSERDYC_AW::B_0X1)
    }
}
#[doc = "HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear HSIRDYF flag"]
    B_0X1 = 1,
}
impl From<HSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYC` writer - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, HSIRDYC_AW>;
impl<'a, const O: u8> HSIRDYC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIRDYC_AW::B_0X0)
    }
    #[doc = "Clear HSIRDYF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIRDYC_AW::B_0X1)
    }
}
#[doc = "HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear HSERDYF flag"]
    B_0X1 = 1,
}
impl From<HSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, HSERDYC_AW>;
impl<'a, const O: u8> HSERDYC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSERDYC_AW::B_0X0)
    }
    #[doc = "Clear HSERDYF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSERDYC_AW::B_0X1)
    }
}
#[doc = "PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear PLLRDYF flag"]
    B_0X1 = 1,
}
impl From<PLLRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag."]
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, PLLRDYC_AW>;
impl<'a, const O: u8> PLLRDYC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLRDYC_AW::B_0X0)
    }
    #[doc = "Clear PLLRDYF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLRDYC_AW::B_0X1)
    }
}
#[doc = "Clock security system interrupt clear This bit is set by software to clear the CSSF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear CSSF flag"]
    B_0X1 = 1,
}
impl From<CSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear This bit is set by software to clear the CSSF flag."]
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, CSSC_AW>;
impl<'a, const O: u8> CSSC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSSC_AW::B_0X0)
    }
    #[doc = "Clear CSSF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSSC_AW::B_0X1)
    }
}
#[doc = "LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear LSECSSF flag"]
    B_0X1 = 1,
}
impl From<LSECSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
pub type LSECSSC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, LSECSSC_AW>;
impl<'a, const O: u8> LSECSSC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSECSSC_AW::B_0X0)
    }
    #[doc = "Clear LSECSSF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSECSSC_AW::B_0X1)
    }
}
#[doc = "HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYC_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear the HSI48RDYC flag"]
    B_0X1 = 1,
}
impl From<HSI48RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag."]
pub type HSI48RDYC_W<'a, const O: u8> = crate::BitWriter<'a, RCC_CICR_SPEC, O, HSI48RDYC_AW>;
impl<'a, const O: u8> HSI48RDYC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSI48RDYC_AW::B_0X0)
    }
    #[doc = "Clear the HSI48RDYC flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSI48RDYC_AW::B_0X1)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<0> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<3> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<4> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<5> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear This bit is set by software to clear the CSSF flag."]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<8> {
        CSSC_W::new(self)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<9> {
        LSECSSC_W::new(self)
    }
    #[doc = "Bit 10 - HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<10> {
        HSI48RDYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cicr](index.html) module"]
pub struct RCC_CICR_SPEC;
impl crate::RegisterSpec for RCC_CICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rcc_cicr::W](W) writer structure"]
impl crate::Writable for RCC_CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CICR to value 0"]
impl crate::Resettable for RCC_CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
