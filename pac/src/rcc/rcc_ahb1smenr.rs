#[doc = "Register `RCC_AHB1SMENR` reader"]
pub struct R(crate::R<RCC_AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB1SMENR` writer"]
pub struct W(crate::W<RCC_AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB1SMENR_SPEC>;
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
impl From<crate::W<RCC_AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type DMA1SMEN_R = crate::BitReader<DMA1SMEN_A>;
#[doc = "DMA1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN_A {
    #[doc = "0: DMA1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: DMA1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<DMA1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1SMEN_A {
        match self.bits {
            false => DMA1SMEN_A::B_0X0,
            true => DMA1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1SMEN_A::B_0X1
    }
}
#[doc = "Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type DMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, DMA1SMEN_A>;
impl<'a, const O: u8> DMA1SMEN_W<'a, O> {
    #[doc = "DMA1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA1SMEN_A::B_0X0)
    }
    #[doc = "DMA1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA1SMEN_A::B_0X1)
    }
}
#[doc = "Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
pub type DMA2SMEN_R = crate::BitReader<DMA2SMEN_A>;
#[doc = "DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2SMEN_A {
    #[doc = "0: DMA2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: DMA2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<DMA2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2SMEN_A {
        match self.bits {
            false => DMA2SMEN_A::B_0X0,
            true => DMA2SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA2SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA2SMEN_A::B_0X1
    }
}
#[doc = "Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
pub type DMA2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, DMA2SMEN_A>;
impl<'a, const O: u8> DMA2SMEN_W<'a, O> {
    #[doc = "DMA2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA2SMEN_A::B_0X0)
    }
    #[doc = "DMA2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA2SMEN_A::B_0X1)
    }
}
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
pub type DMAMUX1SMEN_R = crate::BitReader<DMAMUX1SMEN_A>;
#[doc = "DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1SMEN_A {
    #[doc = "0: DMAMUX1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: DMAMUX1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<DMAMUX1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1SMEN_A {
        match self.bits {
            false => DMAMUX1SMEN_A::B_0X0,
            true => DMAMUX1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAMUX1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAMUX1SMEN_A::B_0X1
    }
}
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
pub type DMAMUX1SMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, DMAMUX1SMEN_A>;
impl<'a, const O: u8> DMAMUX1SMEN_W<'a, O> {
    #[doc = "DMAMUX1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAMUX1SMEN_A::B_0X0)
    }
    #[doc = "DMAMUX1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAMUX1SMEN_A::B_0X1)
    }
}
#[doc = "Field `CORDICSMEN` reader - CORDICSM clock enable. Set and cleared by software."]
pub type CORDICSMEN_R = crate::BitReader<CORDICSMEN_A>;
#[doc = "CORDICSM clock enable. Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORDICSMEN_A {
    #[doc = "0: CORDICSM clocks disabled."]
    B_0X0 = 0,
    #[doc = "1: CORDICSM clocks enabled."]
    B_0X1 = 1,
}
impl From<CORDICSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CORDICSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CORDICSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORDICSMEN_A {
        match self.bits {
            false => CORDICSMEN_A::B_0X0,
            true => CORDICSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CORDICSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CORDICSMEN_A::B_0X1
    }
}
#[doc = "Field `CORDICSMEN` writer - CORDICSM clock enable. Set and cleared by software."]
pub type CORDICSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, CORDICSMEN_A>;
impl<'a, const O: u8> CORDICSMEN_W<'a, O> {
    #[doc = "CORDICSM clocks disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CORDICSMEN_A::B_0X0)
    }
    #[doc = "CORDICSM clocks enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CORDICSMEN_A::B_0X1)
    }
}
#[doc = "Field `FMACSMEN` reader - FMACSM clock enable. Set and cleared by software."]
pub type FMACSMEN_R = crate::BitReader<FMACSMEN_A>;
#[doc = "FMACSM clock enable. Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMACSMEN_A {
    #[doc = "0: FMACSM clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: FMACSM clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<FMACSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMACSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMACSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMACSMEN_A {
        match self.bits {
            false => FMACSMEN_A::B_0X0,
            true => FMACSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMACSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMACSMEN_A::B_0X1
    }
}
#[doc = "Field `FMACSMEN` writer - FMACSM clock enable. Set and cleared by software."]
pub type FMACSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, FMACSMEN_A>;
impl<'a, const O: u8> FMACSMEN_W<'a, O> {
    #[doc = "FMACSM clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMACSMEN_A::B_0X0)
    }
    #[doc = "FMACSM clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMACSMEN_A::B_0X1)
    }
}
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FLASHSMEN_R = crate::BitReader<FLASHSMEN_A>;
#[doc = "Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHSMEN_A {
    #[doc = "0: Flash memory interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: Flash memory interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<FLASHSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHSMEN_A {
        match self.bits {
            false => FLASHSMEN_A::B_0X0,
            true => FLASHSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASHSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASHSMEN_A::B_0X1
    }
}
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, FLASHSMEN_A>;
impl<'a, const O: u8> FLASHSMEN_W<'a, O> {
    #[doc = "Flash memory interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FLASHSMEN_A::B_0X0)
    }
    #[doc = "Flash memory interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FLASHSMEN_A::B_0X1)
    }
}
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SRAM1SMEN_R = crate::BitReader<SRAM1SMEN_A>;
#[doc = "SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SMEN_A {
    #[doc = "0: SRAM1 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: SRAM1 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<SRAM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1SMEN_A {
        match self.bits {
            false => SRAM1SMEN_A::B_0X0,
            true => SRAM1SMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SRAM1SMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SRAM1SMEN_A::B_0X1
    }
}
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SRAM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, SRAM1SMEN_A>;
impl<'a, const O: u8> SRAM1SMEN_W<'a, O> {
    #[doc = "SRAM1 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SRAM1SMEN_A::B_0X0)
    }
    #[doc = "SRAM1 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SRAM1SMEN_A::B_0X1)
    }
}
#[doc = "Field `CRCSMEN` reader - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CRCSMEN_R = crate::BitReader<CRCSMEN_A>;
#[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN_A {
    #[doc = "0: CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0,
    #[doc = "1: CRC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B_0X1 = 1,
}
impl From<CRCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSMEN_A {
        match self.bits {
            false => CRCSMEN_A::B_0X0,
            true => CRCSMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCSMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCSMEN_A::B_0X1
    }
}
#[doc = "Field `CRCSMEN` writer - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1SMENR_SPEC, O, CRCSMEN_A>;
impl<'a, const O: u8> CRCSMEN_W<'a, O> {
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRCSMEN_A::B_0X0)
    }
    #[doc = "CRC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRCSMEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CORDICSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub fn cordicsmen(&self) -> CORDICSMEN_R {
        CORDICSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMACSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub fn fmacsmen(&self) -> FMACSMEN_R {
        FMACSMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<0> {
        DMA1SMEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<1> {
        DMA2SMEN_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<2> {
        DMAMUX1SMEN_W::new(self)
    }
    #[doc = "Bit 3 - CORDICSM clock enable. Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicsmen(&mut self) -> CORDICSMEN_W<3> {
        CORDICSMEN_W::new(self)
    }
    #[doc = "Bit 4 - FMACSM clock enable. Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacsmen(&mut self) -> FMACSMEN_W<4> {
        FMACSMEN_W::new(self)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<8> {
        FLASHSMEN_W::new(self)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<9> {
        SRAM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb1smenr](index.html) module"]
pub struct RCC_AHB1SMENR_SPEC;
impl crate::RegisterSpec for RCC_AHB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb1smenr::R](R) reader structure"]
impl crate::Readable for RCC_AHB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb1smenr::W](W) writer structure"]
impl crate::Writable for RCC_AHB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB1SMENR to value 0x130f"]
impl crate::Resettable for RCC_AHB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x130f;
}
