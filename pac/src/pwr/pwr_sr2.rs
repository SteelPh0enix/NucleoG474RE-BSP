#[doc = "Register `PWR_SR2` reader"]
pub struct R(crate::R<PWR_SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REGLPS` reader - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased."]
pub type REGLPS_R = crate::BitReader<REGLPS_A>;
#[doc = "Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS_A {
    #[doc = "0: The low-power regulator is not ready"]
    B_0X0 = 0,
    #[doc = "1: The low-power regulator is ready"]
    B_0X1 = 1,
}
impl From<REGLPS_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPS_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPS_A {
        match self.bits {
            false => REGLPS_A::B_0X0,
            true => REGLPS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REGLPS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REGLPS_A::B_0X1
    }
}
#[doc = "Field `REGLPF` reader - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
pub type REGLPF_R = crate::BitReader<REGLPF_A>;
#[doc = "Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF_A {
    #[doc = "0: The regulator is ready in main mode (MR)"]
    B_0X0 = 0,
    #[doc = "1: The regulator is in low-power mode (LPR)"]
    B_0X1 = 1,
}
impl From<REGLPF_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPF_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPF_A {
        match self.bits {
            false => REGLPF_A::B_0X0,
            true => REGLPF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REGLPF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REGLPF_A::B_0X1
    }
}
#[doc = "Field `VOSF` reader - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
pub type VOSF_R = crate::BitReader<VOSF_A>;
#[doc = "Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF_A {
    #[doc = "0: The regulator is ready in the selected voltage range"]
    B_0X0 = 0,
    #[doc = "1: The regulator output voltage is changing to the required voltage level"]
    B_0X1 = 1,
}
impl From<VOSF_A> for bool {
    #[inline(always)]
    fn from(variant: VOSF_A) -> Self {
        variant as u8 != 0
    }
}
impl VOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOSF_A {
        match self.bits {
            false => VOSF_A::B_0X0,
            true => VOSF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VOSF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VOSF_A::B_0X1
    }
}
#[doc = "Field `PVDO` reader - Programmable voltage detector output"]
pub type PVDO_R = crate::BitReader<PVDO_A>;
#[doc = "Programmable voltage detector output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO_A {
    #[doc = "0: V&lt;sub>DD&lt;/sub> is above the selected PVD threshold"]
    B_0X0 = 0,
    #[doc = "1: V&lt;sub>DD&lt;/sub> is below the selected PVD threshold"]
    B_0X1 = 1,
}
impl From<PVDO_A> for bool {
    #[inline(always)]
    fn from(variant: PVDO_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDO_A {
        match self.bits {
            false => PVDO_A::B_0X0,
            true => PVDO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVDO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVDO_A::B_0X1
    }
}
#[doc = "Field `PVMO1` reader - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time."]
pub type PVMO1_R = crate::BitReader<PVMO1_A>;
#[doc = "Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO1_A {
    #[doc = "0: V&lt;sub>DDA&lt;/sub> voltage is above PVM1 threshold (around 1.62 V)."]
    B_0X0 = 0,
    #[doc = "1: V&lt;sub>DDA&lt;/sub> voltage is below PVM1 threshold (around 1.62 V)."]
    B_0X1 = 1,
}
impl From<PVMO1_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO1_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVMO1_A {
        match self.bits {
            false => PVMO1_A::B_0X0,
            true => PVMO1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVMO1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVMO1_A::B_0X1
    }
}
#[doc = "Field `PVMO2` reader - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time."]
pub type PVMO2_R = crate::BitReader<PVMO2_A>;
#[doc = "Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO2_A {
    #[doc = "0: V&lt;sub>DDA&lt;/sub> voltage is above PVM2 threshold (around 1.8 V)."]
    B_0X0 = 0,
    #[doc = "1: V&lt;sub>DDA&lt;/sub> voltage is below PVM2 threshold (around 1.8 V)."]
    B_0X1 = 1,
}
impl From<PVMO2_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO2_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVMO2_A {
        match self.bits {
            false => PVMO2_A::B_0X0,
            true => PVMO2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVMO2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVMO2_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 8 - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased."]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time."]
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time."]
    #[inline(always)]
    pub fn pvmo2(&self) -> PVMO2_R {
        PVMO2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_sr2](index.html) module"]
pub struct PWR_SR2_SPEC;
impl crate::RegisterSpec for PWR_SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_sr2::R](R) reader structure"]
impl crate::Readable for PWR_SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_SR2 to value 0"]
impl crate::Resettable for PWR_SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
