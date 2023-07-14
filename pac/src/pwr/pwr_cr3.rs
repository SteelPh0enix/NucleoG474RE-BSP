#[doc = "Register `PWR_CR3` reader"]
pub struct R(crate::R<PWR_CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR3` writer"]
pub struct W(crate::W<PWR_CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR3_SPEC>;
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
impl From<crate::W<PWR_CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
pub type EWUP1_R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O>;
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
pub type EWUP2_R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O>;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
pub type EWUP3_R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O>;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type EWUP4_R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type EWUP4_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O>;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
pub type EWUP5_R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
pub type EWUP5_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O>;
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub type RRS_R = crate::BitReader<RRS_A>;
#[doc = "SRAM2 retention in Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS_A {
    #[doc = "0: SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    B_0X0 = 0,
    #[doc = "1: SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    B_0X1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::B_0X0,
            true => RRS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRS_A::B_0X1
    }
}
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub type RRS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O, RRS_A>;
impl<'a, const O: u8> RRS_W<'a, O> {
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRS_A::B_0X0)
    }
    #[doc = "SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRS_A::B_0X1)
    }
}
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
pub type APC_R = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O>;
#[doc = "Field `UCPD1_STDBY` reader - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
pub type UCPD1_STDBY_R = crate::BitReader<UCPD1_STDBY_A>;
#[doc = "UCPD1_STDBY USB Type-C and Power Delivery standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD1_STDBY_A {
    #[doc = "0: Write 0 immediately after standby exit when using UCPD1, (and before writing any UCPD1 registers)."]
    B_0X0 = 0,
    #[doc = "1: Write 1 just before entering standby when using UCPD1."]
    B_0X1 = 1,
}
impl From<UCPD1_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: UCPD1_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPD1_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPD1_STDBY_A {
        match self.bits {
            false => UCPD1_STDBY_A::B_0X0,
            true => UCPD1_STDBY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UCPD1_STDBY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UCPD1_STDBY_A::B_0X1
    }
}
#[doc = "Field `UCPD1_STDBY` writer - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
pub type UCPD1_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O, UCPD1_STDBY_A>;
impl<'a, const O: u8> UCPD1_STDBY_W<'a, O> {
    #[doc = "Write 0 immediately after standby exit when using UCPD1, (and before writing any UCPD1 registers)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UCPD1_STDBY_A::B_0X0)
    }
    #[doc = "Write 1 just before entering standby when using UCPD1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UCPD1_STDBY_A::B_0X1)
    }
}
#[doc = "Field `UCPD1_DBDIS` reader - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
pub type UCPD1_DBDIS_R = crate::BitReader<UCPD1_DBDIS_A>;
#[doc = "USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD1_DBDIS_A {
    #[doc = "0: Enable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    B_0X0 = 0,
    #[doc = "1: Disable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    B_0X1 = 1,
}
impl From<UCPD1_DBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UCPD1_DBDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPD1_DBDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPD1_DBDIS_A {
        match self.bits {
            false => UCPD1_DBDIS_A::B_0X0,
            true => UCPD1_DBDIS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UCPD1_DBDIS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UCPD1_DBDIS_A::B_0X1
    }
}
#[doc = "Field `UCPD1_DBDIS` writer - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
pub type UCPD1_DBDIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O, UCPD1_DBDIS_A>;
impl<'a, const O: u8> UCPD1_DBDIS_W<'a, O> {
    #[doc = "Enable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UCPD1_DBDIS_A::B_0X0)
    }
    #[doc = "Disable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UCPD1_DBDIS_A::B_0X1)
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line"]
pub type EIWUL_R = crate::BitReader<EIWUL_A>;
#[doc = "Enable internal wakeup line\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL_A {
    #[doc = "0: Internal wakeup line disable."]
    B_0X0 = 0,
    #[doc = "1: Internal wakeup line enable."]
    B_0X1 = 1,
}
impl From<EIWUL_A> for bool {
    #[inline(always)]
    fn from(variant: EIWUL_A) -> Self {
        variant as u8 != 0
    }
}
impl EIWUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIWUL_A {
        match self.bits {
            false => EIWUL_A::B_0X0,
            true => EIWUL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EIWUL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EIWUL_A::B_0X1
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line"]
pub type EIWUL_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR3_SPEC, O, EIWUL_A>;
impl<'a, const O: u8> EIWUL_W<'a, O> {
    #[doc = "Internal wakeup line disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EIWUL_A::B_0X0)
    }
    #[doc = "Internal wakeup line enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EIWUL_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
    #[inline(always)]
    pub fn ucpd1_stdby(&self) -> UCPD1_STDBY_R {
        UCPD1_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    pub fn ucpd1_dbdis(&self) -> UCPD1_DBDIS_R {
        UCPD1_DBDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<0> {
        EWUP1_W::new(self)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<1> {
        EWUP2_W::new(self)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<2> {
        EWUP3_W::new(self)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<3> {
        EWUP4_W::new(self)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<4> {
        EWUP5_W::new(self)
    }
    #[doc = "Bit 8 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<8> {
        RRS_W::new(self)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<10> {
        APC_W::new(self)
    }
    #[doc = "Bit 13 - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_stdby(&mut self) -> UCPD1_STDBY_W<13> {
        UCPD1_STDBY_W::new(self)
    }
    #[doc = "Bit 14 - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_dbdis(&mut self) -> UCPD1_DBDIS_W<14> {
        UCPD1_DBDIS_W::new(self)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<15> {
        EIWUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr3](index.html) module"]
pub struct PWR_CR3_SPEC;
impl crate::RegisterSpec for PWR_CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr3::R](R) reader structure"]
impl crate::Readable for PWR_CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr3::W](W) writer structure"]
impl crate::Writable for PWR_CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0x8000"]
impl crate::Resettable for PWR_CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
