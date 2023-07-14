#[doc = "Register `RCC_AHB1ENR` reader"]
pub struct R(crate::R<RCC_AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB1ENR` writer"]
pub struct W(crate::W<RCC_AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB1ENR_SPEC>;
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
impl From<crate::W<RCC_AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable Set and cleared by software."]
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
#[doc = "DMA1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN_A {
    #[doc = "0: DMA1 clock disable"]
    B_0X0 = 0,
    #[doc = "1: DMA1 clock enable"]
    B_0X1 = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::B_0X0,
            true => DMA1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1EN_A::B_0X1
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable Set and cleared by software."]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, DMA1EN_A>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    #[doc = "DMA1 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA1EN_A::B_0X0)
    }
    #[doc = "DMA1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA1EN_A::B_0X1)
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable Set and cleared by software."]
pub type DMA2EN_R = crate::BitReader<DMA2EN_A>;
#[doc = "DMA2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2EN_A {
    #[doc = "0: DMA2 clock disable"]
    B_0X0 = 0,
    #[doc = "1: DMA2 clock enable"]
    B_0X1 = 1,
}
impl From<DMA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2EN_A {
        match self.bits {
            false => DMA2EN_A::B_0X0,
            true => DMA2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA2EN_A::B_0X1
    }
}
#[doc = "Field `DMA2EN` writer - DMA2 clock enable Set and cleared by software."]
pub type DMA2EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, DMA2EN_A>;
impl<'a, const O: u8> DMA2EN_W<'a, O> {
    #[doc = "DMA2 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA2EN_A::B_0X0)
    }
    #[doc = "DMA2 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA2EN_A::B_0X1)
    }
}
#[doc = "Field `DMAMUX1EN` reader - DMAMUX1 clock enable Set and reset by software."]
pub type DMAMUX1EN_R = crate::BitReader<DMAMUX1EN_A>;
#[doc = "DMAMUX1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1EN_A {
    #[doc = "0: DMAMUX1 clock disabled"]
    B_0X0 = 0,
    #[doc = "1: DMAMUX1 clock enabled"]
    B_0X1 = 1,
}
impl From<DMAMUX1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1EN_A {
        match self.bits {
            false => DMAMUX1EN_A::B_0X0,
            true => DMAMUX1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAMUX1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAMUX1EN_A::B_0X1
    }
}
#[doc = "Field `DMAMUX1EN` writer - DMAMUX1 clock enable Set and reset by software."]
pub type DMAMUX1EN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, DMAMUX1EN_A>;
impl<'a, const O: u8> DMAMUX1EN_W<'a, O> {
    #[doc = "DMAMUX1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAMUX1EN_A::B_0X0)
    }
    #[doc = "DMAMUX1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAMUX1EN_A::B_0X1)
    }
}
#[doc = "Field `CORDICEN` reader - CORDIC clock enable Set and reset by software."]
pub type CORDICEN_R = crate::BitReader<CORDICEN_A>;
#[doc = "CORDIC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORDICEN_A {
    #[doc = "0: CORDIC clock disabled"]
    B_0X0 = 0,
    #[doc = "1: CORDIC clock enabled"]
    B_0X1 = 1,
}
impl From<CORDICEN_A> for bool {
    #[inline(always)]
    fn from(variant: CORDICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CORDICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORDICEN_A {
        match self.bits {
            false => CORDICEN_A::B_0X0,
            true => CORDICEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CORDICEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CORDICEN_A::B_0X1
    }
}
#[doc = "Field `CORDICEN` writer - CORDIC clock enable Set and reset by software."]
pub type CORDICEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, CORDICEN_A>;
impl<'a, const O: u8> CORDICEN_W<'a, O> {
    #[doc = "CORDIC clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CORDICEN_A::B_0X0)
    }
    #[doc = "CORDIC clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CORDICEN_A::B_0X1)
    }
}
#[doc = "Field `FMACEN` reader - FMAC enable Set and reset by software."]
pub type FMACEN_R = crate::BitReader<FMACEN_A>;
#[doc = "FMAC enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMACEN_A {
    #[doc = "0: FMAC clock disabled"]
    B_0X0 = 0,
    #[doc = "1: FMAC clock enabled"]
    B_0X1 = 1,
}
impl From<FMACEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMACEN_A {
        match self.bits {
            false => FMACEN_A::B_0X0,
            true => FMACEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMACEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMACEN_A::B_0X1
    }
}
#[doc = "Field `FMACEN` writer - FMAC enable Set and reset by software."]
pub type FMACEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, FMACEN_A>;
impl<'a, const O: u8> FMACEN_W<'a, O> {
    #[doc = "FMAC clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMACEN_A::B_0X0)
    }
    #[doc = "FMAC clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMACEN_A::B_0X1)
    }
}
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
pub type FLASHEN_R = crate::BitReader<FLASHEN_A>;
#[doc = "Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHEN_A {
    #[doc = "0: Flash memory interface clock disable"]
    B_0X0 = 0,
    #[doc = "1: Flash memory interface clock enable"]
    B_0X1 = 1,
}
impl From<FLASHEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHEN_A {
        match self.bits {
            false => FLASHEN_A::B_0X0,
            true => FLASHEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASHEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASHEN_A::B_0X1
    }
}
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
pub type FLASHEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, FLASHEN_A>;
impl<'a, const O: u8> FLASHEN_W<'a, O> {
    #[doc = "Flash memory interface clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FLASHEN_A::B_0X0)
    }
    #[doc = "Flash memory interface clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FLASHEN_A::B_0X1)
    }
}
#[doc = "Field `CRCEN` reader - CRC clock enable Set and cleared by software."]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
#[doc = "CRC clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    #[doc = "0: CRC clock disable"]
    B_0X0 = 0,
    #[doc = "1: CRC clock enable"]
    B_0X1 = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::B_0X0,
            true => CRCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCEN_A::B_0X1
    }
}
#[doc = "Field `CRCEN` writer - CRC clock enable Set and cleared by software."]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, RCC_AHB1ENR_SPEC, O, CRCEN_A>;
impl<'a, const O: u8> CRCEN_W<'a, O> {
    #[doc = "CRC clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRCEN_A::B_0X0)
    }
    #[doc = "CRC clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRCEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMAC enable Set and reset by software."]
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<2> {
        DMAMUX1EN_W::new(self)
    }
    #[doc = "Bit 3 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CORDICEN_W<3> {
        CORDICEN_W::new(self)
    }
    #[doc = "Bit 4 - FMAC enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FMACEN_W<4> {
        FMACEN_W::new(self)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<8> {
        FLASHEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb1enr](index.html) module"]
pub struct RCC_AHB1ENR_SPEC;
impl crate::RegisterSpec for RCC_AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb1enr::R](R) reader structure"]
impl crate::Readable for RCC_AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb1enr::W](W) writer structure"]
impl crate::Writable for RCC_AHB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB1ENR to value 0x0100"]
impl crate::Resettable for RCC_AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
