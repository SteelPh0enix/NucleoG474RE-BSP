#[doc = "Register `PWR_CR1` reader"]
pub struct R(crate::R<PWR_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR1` writer"]
pub struct W(crate::W<PWR_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR1_SPEC>;
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
impl From<crate::W<PWR_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
pub type LPMS_R = crate::FieldReader<LPMS_A>;
#[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS_A {
    #[doc = "0: Stop 0 mode"]
    B_0X0 = 0,
    #[doc = "1: Stop 1 mode"]
    B_0X1 = 1,
    #[doc = "3: Standby mode"]
    B_0X3 = 3,
}
impl From<LPMS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPMS_A {
    type Ux = u8;
}
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMS_A> {
        match self.bits {
            0 => Some(LPMS_A::B_0X0),
            1 => Some(LPMS_A::B_0X1),
            3 => Some(LPMS_A::B_0X3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPMS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPMS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LPMS_A::B_0X3
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, PWR_CR1_SPEC, 3, O, LPMS_A>;
impl<'a, const O: u8> LPMS_W<'a, O> {
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPMS_A::B_0X0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPMS_A::B_0X1)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LPMS_A::B_0X3)
    }
}
#[doc = "Field `FPD_STOP` reader - FPD_STOP"]
pub type FPD_STOP_R = crate::BitReader;
#[doc = "Field `FPD_STOP` writer - FPD_STOP"]
pub type FPD_STOP_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR1_SPEC, O>;
#[doc = "Field `DBP` reader - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_R = crate::BitReader<DBP_A>;
#[doc = "Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP_A {
    #[doc = "0: Access to RTC and Backup registers disabled"]
    B_0X0 = 0,
    #[doc = "1: Access to RTC and Backup registers enabled"]
    B_0X1 = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::B_0X0,
            true => DBP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBP_A::B_0X1
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR1_SPEC, O, DBP_A>;
impl<'a, const O: u8> DBP_W<'a, O> {
    #[doc = "Access to RTC and Backup registers disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBP_A::B_0X0)
    }
    #[doc = "Access to RTC and Backup registers enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBP_A::B_0X1)
    }
}
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<VOS_A>;
#[doc = "Voltage scaling range selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "0: Cannot be written (forbidden by hardware)"]
    B_0X0 = 0,
    #[doc = "1: Range 1"]
    B_0X1 = 1,
    #[doc = "2: Range 2"]
    B_0X2 = 2,
    #[doc = "3: Cannot be written (forbidden by hardware)"]
    B_0X3 = 3,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS_A {
    type Ux = u8;
}
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOS_A {
        match self.bits {
            0 => VOS_A::B_0X0,
            1 => VOS_A::B_0X1,
            2 => VOS_A::B_0X2,
            3 => VOS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VOS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VOS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == VOS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == VOS_A::B_0X3
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, PWR_CR1_SPEC, 2, O, VOS_A>;
impl<'a, const O: u8> VOS_W<'a, O> {
    #[doc = "Cannot be written (forbidden by hardware)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VOS_A::B_0X0)
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VOS_A::B_0X1)
    }
    #[doc = "Range 2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(VOS_A::B_0X2)
    }
    #[doc = "Cannot be written (forbidden by hardware)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(VOS_A::B_0X3)
    }
}
#[doc = "Field `LPR` reader - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
pub type LPR_R = crate::BitReader;
#[doc = "Field `LPR` writer - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
pub type LPR_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR1_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - FPD_STOP"]
    #[inline(always)]
    pub fn fpd_stop(&self) -> FPD_STOP_R {
        FPD_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    #[doc = "Bit 3 - FPD_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_stop(&mut self) -> FPD_STOP_W<3> {
        FPD_STOP_W::new(self)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<9> {
        VOS_W::new(self)
    }
    #[doc = "Bit 14 - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<14> {
        LPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr1](index.html) module"]
pub struct PWR_CR1_SPEC;
impl crate::RegisterSpec for PWR_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr1::R](R) reader structure"]
impl crate::Readable for PWR_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr1::W](W) writer structure"]
impl crate::Writable for PWR_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR1 to value 0x0200"]
impl crate::Resettable for PWR_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
