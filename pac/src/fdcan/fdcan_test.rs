#[doc = "Register `FDCAN_TEST` reader"]
pub struct R(crate::R<FDCAN_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TEST` writer"]
pub struct W(crate::W<FDCAN_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TEST_SPEC>;
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
impl From<crate::W<FDCAN_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBCK` reader - Loop back mode"]
pub type LBCK_R = crate::BitReader<LBCK_A>;
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCK_A {
    #[doc = "0: Reset value, Loop Back mode is disabled"]
    B_0X0 = 0,
    #[doc = "1: Loop Back mode is enabled (see Power down (Sleep mode))"]
    B_0X1 = 1,
}
impl From<LBCK_A> for bool {
    #[inline(always)]
    fn from(variant: LBCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LBCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBCK_A {
        match self.bits {
            false => LBCK_A::B_0X0,
            true => LBCK_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBCK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBCK_A::B_0X1
    }
}
#[doc = "Field `LBCK` writer - Loop back mode"]
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_TEST_SPEC, O, LBCK_A>;
impl<'a, const O: u8> LBCK_W<'a, O> {
    #[doc = "Reset value, Loop Back mode is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LBCK_A::B_0X0)
    }
    #[doc = "Loop Back mode is enabled (see Power down (Sleep mode))"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LBCK_A::B_0X1)
    }
}
#[doc = "Field `TX` reader - Control of transmit pin"]
pub type TX_R = crate::FieldReader<TX_A>;
#[doc = "Control of transmit pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    B_0X0 = 0,
    #[doc = "1: Sample point can be monitored at pin FDCANx_TX"]
    B_0X1 = 1,
    #[doc = "2: Dominant (0) level at pin FDCANx_TX"]
    B_0X2 = 2,
    #[doc = "3: Recessive (1) at pin FDCANx_TX"]
    B_0X3 = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_A {
    type Ux = u8;
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::B_0X0,
            1 => TX_A::B_0X1,
            2 => TX_A::B_0X2,
            3 => TX_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TX_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TX_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TX_A::B_0X3
    }
}
#[doc = "Field `TX` writer - Control of transmit pin"]
pub type TX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_TEST_SPEC, 2, O, TX_A>;
impl<'a, const O: u8> TX_W<'a, O> {
    #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TX_A::B_0X0)
    }
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TX_A::B_0X1)
    }
    #[doc = "Dominant (0) level at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TX_A::B_0X2)
    }
    #[doc = "Recessive (1) at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TX_A::B_0X3)
    }
}
#[doc = "Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX"]
pub type RX_R = crate::BitReader<RX_A>;
#[doc = "Receive pin Monitors the actual value of pin FDCANx_RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_A {
    #[doc = "0: The CAN bus is dominant (FDCANx_RX = 0)"]
    B_0X0 = 0,
    #[doc = "1: The CAN bus is recessive (FDCANx_RX = 1)"]
    B_0X1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::B_0X0,
            true => RX_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RX_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_test](index.html) module"]
pub struct FDCAN_TEST_SPEC;
impl crate::RegisterSpec for FDCAN_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_test::R](R) reader structure"]
impl crate::Readable for FDCAN_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_test::W](W) writer structure"]
impl crate::Writable for FDCAN_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TEST to value 0"]
impl crate::Resettable for FDCAN_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
