#[doc = "Register `RCC_AHB1RSTR` reader"]
pub struct R(crate::R<RCC_AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB1RSTR` writer"]
pub struct W(crate::W<RCC_AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB1RSTR_SPEC>;
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
impl From<crate::W<RCC_AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 reset Set and cleared by software."]
pub type DMA1RST_R = crate::BitReader<DMA1RST_A>;
#[doc = "DMA1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DMA1"]
    B_0X1 = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1RST_A {
        match self.bits {
            false => DMA1RST_A::B_0X0,
            true => DMA1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1RST_A::B_0X1
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 reset Set and cleared by software."]
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, DMA1RST_A>;
impl<'a, const O: u8> DMA1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA1RST_A::B_0X0)
    }
    #[doc = "Reset DMA1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA1RST_A::B_0X1)
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 reset Set and cleared by software."]
pub type DMA2RST_R = crate::BitReader<DMA2RST_A>;
#[doc = "DMA2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DMA2"]
    B_0X1 = 1,
}
impl From<DMA2RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2RST_A {
        match self.bits {
            false => DMA2RST_A::B_0X0,
            true => DMA2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA2RST_A::B_0X1
    }
}
#[doc = "Field `DMA2RST` writer - DMA2 reset Set and cleared by software."]
pub type DMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, DMA2RST_A>;
impl<'a, const O: u8> DMA2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA2RST_A::B_0X0)
    }
    #[doc = "Reset DMA2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA2RST_A::B_0X1)
    }
}
#[doc = "Field `DMAMUX1RST` reader - Set and cleared by software."]
pub type DMAMUX1RST_R = crate::BitReader<DMAMUX1RST_A>;
#[doc = "Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1RST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset DMAMUX1"]
    B_0X1 = 1,
}
impl From<DMAMUX1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1RST_A {
        match self.bits {
            false => DMAMUX1RST_A::B_0X0,
            true => DMAMUX1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAMUX1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAMUX1RST_A::B_0X1
    }
}
#[doc = "Field `DMAMUX1RST` writer - Set and cleared by software."]
pub type DMAMUX1RST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, DMAMUX1RST_A>;
impl<'a, const O: u8> DMAMUX1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAMUX1RST_A::B_0X0)
    }
    #[doc = "Reset DMAMUX1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAMUX1RST_A::B_0X1)
    }
}
#[doc = "Field `CORDICRST` reader - Set and cleared by software"]
pub type CORDICRST_R = crate::BitReader<CORDICRST_A>;
#[doc = "Set and cleared by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORDICRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset CORDIC"]
    B_0X1 = 1,
}
impl From<CORDICRST_A> for bool {
    #[inline(always)]
    fn from(variant: CORDICRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CORDICRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORDICRST_A {
        match self.bits {
            false => CORDICRST_A::B_0X0,
            true => CORDICRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CORDICRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CORDICRST_A::B_0X1
    }
}
#[doc = "Field `CORDICRST` writer - Set and cleared by software"]
pub type CORDICRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, CORDICRST_A>;
impl<'a, const O: u8> CORDICRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CORDICRST_A::B_0X0)
    }
    #[doc = "Reset CORDIC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CORDICRST_A::B_0X1)
    }
}
#[doc = "Field `FMACRST` reader - Set and cleared by software"]
pub type FMACRST_R = crate::BitReader<FMACRST_A>;
#[doc = "Set and cleared by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMACRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset FMAC"]
    B_0X1 = 1,
}
impl From<FMACRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMACRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMACRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMACRST_A {
        match self.bits {
            false => FMACRST_A::B_0X0,
            true => FMACRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMACRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMACRST_A::B_0X1
    }
}
#[doc = "Field `FMACRST` writer - Set and cleared by software"]
pub type FMACRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, FMACRST_A>;
impl<'a, const O: u8> FMACRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMACRST_A::B_0X0)
    }
    #[doc = "Reset FMAC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMACRST_A::B_0X1)
    }
}
#[doc = "Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
pub type FLASHRST_R = crate::BitReader<FLASHRST_A>;
#[doc = "Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset Flash memory interface"]
    B_0X1 = 1,
}
impl From<FLASHRST_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHRST_A {
        match self.bits {
            false => FLASHRST_A::B_0X0,
            true => FLASHRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASHRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASHRST_A::B_0X1
    }
}
#[doc = "Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
pub type FLASHRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, FLASHRST_A>;
impl<'a, const O: u8> FLASHRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FLASHRST_A::B_0X0)
    }
    #[doc = "Reset Flash memory interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FLASHRST_A::B_0X1)
    }
}
#[doc = "Field `CRCRST` reader - CRC reset Set and cleared by software."]
pub type CRCRST_R = crate::BitReader<CRCRST_A>;
#[doc = "CRC reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRST_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Reset CRC"]
    B_0X1 = 1,
}
impl From<CRCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCRST_A {
        match self.bits {
            false => CRCRST_A::B_0X0,
            true => CRCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCRST_A::B_0X1
    }
}
#[doc = "Field `CRCRST` writer - CRC reset Set and cleared by software."]
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1RSTR_SPEC, O, CRCRST_A>;
impl<'a, const O: u8> CRCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRCRST_A::B_0X0)
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRCRST_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set and cleared by software."]
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set and cleared by software"]
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set and cleared by software"]
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 2 - Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W<2> {
        DMAMUX1RST_W::new(self)
    }
    #[doc = "Bit 3 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn cordicrst(&mut self) -> CORDICRST_W<3> {
        CORDICRST_W::new(self)
    }
    #[doc = "Bit 4 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn fmacrst(&mut self) -> FMACRST_W<4> {
        FMACRST_W::new(self)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<8> {
        FLASHRST_W::new(self)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb1rstr](index.html) module"]
pub struct RCC_AHB1RSTR_SPEC;
impl crate::RegisterSpec for RCC_AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb1rstr::R](R) reader structure"]
impl crate::Readable for RCC_AHB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb1rstr::W](W) writer structure"]
impl crate::Writable for RCC_AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB1RSTR to value 0"]
impl crate::Resettable for RCC_AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
