#[doc = "Register `PWR_CR2` reader"]
pub struct R(crate::R<PWR_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR2` writer"]
pub struct W(crate::W<PWR_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR2_SPEC>;
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
impl From<crate::W<PWR_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDE` reader - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PVDE_R = crate::BitReader<PVDE_A>;
#[doc = "Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE_A {
    #[doc = "0: Programmable voltage detector disable."]
    B_0X0 = 0,
    #[doc = "1: Programmable voltage detector enable."]
    B_0X1 = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::B_0X0,
            true => PVDE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVDE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVDE_A::B_0X1
    }
}
#[doc = "Field `PVDE` writer - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR2_SPEC, O, PVDE_A>;
impl<'a, const O: u8> PVDE_W<'a, O> {
    #[doc = "Programmable voltage detector disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PVDE_A::B_0X0)
    }
    #[doc = "Programmable voltage detector enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PVDE_A::B_0X1)
    }
}
#[doc = "Field `PVDLS` reader - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PVDLS_R = crate::FieldReader<PVDLS_A>;
#[doc = "Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PVDLS_A {
    #[doc = "0: V&lt;sub>PVD0&lt;/sub> PVD threshold 0"]
    B_0X0 = 0,
    #[doc = "1: V&lt;sub>PVD1&lt;/sub> PVD threshold 1"]
    B_0X1 = 1,
    #[doc = "2: V&lt;sub>PVD2&lt;/sub> PVD threshold 2"]
    B_0X2 = 2,
    #[doc = "3: V&lt;sub>PVD3&lt;/sub> PVD threshold 3"]
    B_0X3 = 3,
    #[doc = "4: V&lt;sub>PVD4&lt;/sub> PVD threshold 4"]
    B_0X4 = 4,
    #[doc = "5: V&lt;sub>PVD5&lt;/sub> PVD threshold 5"]
    B_0X5 = 5,
    #[doc = "6: V&lt;sub>PVD6&lt;/sub> PVD threshold 6"]
    B_0X6 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to V&lt;sub>REFINT&lt;/sub>)"]
    B_0X7 = 7,
}
impl From<PVDLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PVDLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PVDLS_A {
    type Ux = u8;
}
impl PVDLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDLS_A {
        match self.bits {
            0 => PVDLS_A::B_0X0,
            1 => PVDLS_A::B_0X1,
            2 => PVDLS_A::B_0X2,
            3 => PVDLS_A::B_0X3,
            4 => PVDLS_A::B_0X4,
            5 => PVDLS_A::B_0X5,
            6 => PVDLS_A::B_0X6,
            7 => PVDLS_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVDLS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVDLS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PVDLS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PVDLS_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PVDLS_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PVDLS_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PVDLS_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PVDLS_A::B_0X7
    }
}
#[doc = "Field `PVDLS` writer - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PVDLS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, PWR_CR2_SPEC, 3, O, PVDLS_A>;
impl<'a, const O: u8> PVDLS_W<'a, O> {
    #[doc = "V&lt;sub>PVD0&lt;/sub> PVD threshold 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X0)
    }
    #[doc = "V&lt;sub>PVD1&lt;/sub> PVD threshold 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X1)
    }
    #[doc = "V&lt;sub>PVD2&lt;/sub> PVD threshold 2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X2)
    }
    #[doc = "V&lt;sub>PVD3&lt;/sub> PVD threshold 3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X3)
    }
    #[doc = "V&lt;sub>PVD4&lt;/sub> PVD threshold 4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X4)
    }
    #[doc = "V&lt;sub>PVD5&lt;/sub> PVD threshold 5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X5)
    }
    #[doc = "V&lt;sub>PVD6&lt;/sub> PVD threshold 6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X6)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to V&lt;sub>REFINT&lt;/sub>)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PVDLS_A::B_0X7)
    }
}
#[doc = "Field `PVMEN1` reader - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. ADC/COMP min voltage 1.62V"]
pub type PVMEN1_R = crate::BitReader<PVMEN1_A>;
#[doc = "Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. ADC/COMP min voltage 1.62V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMEN1_A {
    #[doc = "0: PVM1 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.62V threshold) disable."]
    B_0X0 = 0,
    #[doc = "1: PVM1 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.62V threshold) enable."]
    B_0X1 = 1,
}
impl From<PVMEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PVMEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVMEN1_A {
        match self.bits {
            false => PVMEN1_A::B_0X0,
            true => PVMEN1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVMEN1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVMEN1_A::B_0X1
    }
}
#[doc = "Field `PVMEN1` writer - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. ADC/COMP min voltage 1.62V"]
pub type PVMEN1_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR2_SPEC, O, PVMEN1_A>;
impl<'a, const O: u8> PVMEN1_W<'a, O> {
    #[doc = "PVM1 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.62V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PVMEN1_A::B_0X0)
    }
    #[doc = "PVM1 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.62V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PVMEN1_A::B_0X1)
    }
}
#[doc = "Field `PVMEN2` reader - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
pub type PVMEN2_R = crate::BitReader<PVMEN2_A>;
#[doc = "Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. DAC 1MSPS /DAC 15MSPS min voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMEN2_A {
    #[doc = "0: PVM2 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.8 V threshold) disable."]
    B_0X0 = 0,
    #[doc = "1: PVM2 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.8 V threshold) enable."]
    B_0X1 = 1,
}
impl From<PVMEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PVMEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVMEN2_A {
        match self.bits {
            false => PVMEN2_A::B_0X0,
            true => PVMEN2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVMEN2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVMEN2_A::B_0X1
    }
}
#[doc = "Field `PVMEN2` writer - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
pub type PVMEN2_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR2_SPEC, O, PVMEN2_A>;
impl<'a, const O: u8> PVMEN2_W<'a, O> {
    #[doc = "PVM2 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.8 V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PVMEN2_A::B_0X0)
    }
    #[doc = "PVM2 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.8 V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PVMEN2_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. ADC/COMP min voltage 1.62V"]
    #[inline(always)]
    pub fn pvmen1(&self) -> PVMEN1_R {
        PVMEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
    #[inline(always)]
    pub fn pvmen2(&self) -> PVMEN2_R {
        PVMEN2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 1:3 - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn pvdls(&mut self) -> PVDLS_W<1> {
        PVDLS_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. ADC/COMP min voltage 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen1(&mut self) -> PVMEN1_W<6> {
        PVMEN1_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
    #[inline(always)]
    #[must_use]
    pub fn pvmen2(&mut self) -> PVMEN2_W<7> {
        PVMEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr2](index.html) module"]
pub struct PWR_CR2_SPEC;
impl crate::RegisterSpec for PWR_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr2::R](R) reader structure"]
impl crate::Readable for PWR_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr2::W](W) writer structure"]
impl crate::Writable for PWR_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR2 to value 0"]
impl crate::Resettable for PWR_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
