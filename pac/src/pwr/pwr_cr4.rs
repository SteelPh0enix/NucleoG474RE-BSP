#[doc = "Register `PWR_CR4` reader"]
pub struct R(crate::R<PWR_CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR4` writer"]
pub struct W(crate::W<PWR_CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR4_SPEC>;
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
impl From<crate::W<PWR_CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
pub type WP1_R = crate::BitReader<WP1_A>;
#[doc = "Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP1_A {
    #[doc = "0: Detection on high level (rising edge)"]
    B_0X0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B_0X1 = 1,
}
impl From<WP1_A> for bool {
    #[inline(always)]
    fn from(variant: WP1_A) -> Self {
        variant as u8 != 0
    }
}
impl WP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP1_A {
        match self.bits {
            false => WP1_A::B_0X0,
            true => WP1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP1_A::B_0X1
    }
}
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
pub type WP1_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, WP1_A>;
impl<'a, const O: u8> WP1_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WP1_A::B_0X0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WP1_A::B_0X1)
    }
}
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
pub type WP2_R = crate::BitReader<WP2_A>;
#[doc = "Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP2_A {
    #[doc = "0: Detection on high level (rising edge)"]
    B_0X0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B_0X1 = 1,
}
impl From<WP2_A> for bool {
    #[inline(always)]
    fn from(variant: WP2_A) -> Self {
        variant as u8 != 0
    }
}
impl WP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP2_A {
        match self.bits {
            false => WP2_A::B_0X0,
            true => WP2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP2_A::B_0X1
    }
}
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
pub type WP2_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, WP2_A>;
impl<'a, const O: u8> WP2_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WP2_A::B_0X0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WP2_A::B_0X1)
    }
}
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
pub type WP3_R = crate::BitReader<WP3_A>;
#[doc = "Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP3_A {
    #[doc = "0: Detection on high level (rising edge)"]
    B_0X0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B_0X1 = 1,
}
impl From<WP3_A> for bool {
    #[inline(always)]
    fn from(variant: WP3_A) -> Self {
        variant as u8 != 0
    }
}
impl WP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP3_A {
        match self.bits {
            false => WP3_A::B_0X0,
            true => WP3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP3_A::B_0X1
    }
}
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
pub type WP3_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, WP3_A>;
impl<'a, const O: u8> WP3_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WP3_A::B_0X0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WP3_A::B_0X1)
    }
}
#[doc = "Field `WP4` reader - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
pub type WP4_R = crate::BitReader<WP4_A>;
#[doc = "Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP4_A {
    #[doc = "0: Detection on high level (rising edge)"]
    B_0X0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B_0X1 = 1,
}
impl From<WP4_A> for bool {
    #[inline(always)]
    fn from(variant: WP4_A) -> Self {
        variant as u8 != 0
    }
}
impl WP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP4_A {
        match self.bits {
            false => WP4_A::B_0X0,
            true => WP4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP4_A::B_0X1
    }
}
#[doc = "Field `WP4` writer - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
pub type WP4_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, WP4_A>;
impl<'a, const O: u8> WP4_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WP4_A::B_0X0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WP4_A::B_0X1)
    }
}
#[doc = "Field `WP5` reader - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
pub type WP5_R = crate::BitReader<WP5_A>;
#[doc = "Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP5_A {
    #[doc = "0: Detection on high level (rising edge)"]
    B_0X0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B_0X1 = 1,
}
impl From<WP5_A> for bool {
    #[inline(always)]
    fn from(variant: WP5_A) -> Self {
        variant as u8 != 0
    }
}
impl WP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP5_A {
        match self.bits {
            false => WP5_A::B_0X0,
            true => WP5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP5_A::B_0X1
    }
}
#[doc = "Field `WP5` writer - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
pub type WP5_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, WP5_A>;
impl<'a, const O: u8> WP5_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WP5_A::B_0X0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WP5_A::B_0X1)
    }
}
#[doc = "Field `VBE` reader - V&lt;sub>BAT&lt;/sub> battery charging enable"]
pub type VBE_R = crate::BitReader<VBE_A>;
#[doc = "V&lt;sub>BAT&lt;/sub> battery charging enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE_A {
    #[doc = "0: V&lt;sub>BAT&lt;/sub> battery charging disable"]
    B_0X0 = 0,
    #[doc = "1: V&lt;sub>BAT&lt;/sub> battery charging enable"]
    B_0X1 = 1,
}
impl From<VBE_A> for bool {
    #[inline(always)]
    fn from(variant: VBE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBE_A {
        match self.bits {
            false => VBE_A::B_0X0,
            true => VBE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VBE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VBE_A::B_0X1
    }
}
#[doc = "Field `VBE` writer - V&lt;sub>BAT&lt;/sub> battery charging enable"]
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, VBE_A>;
impl<'a, const O: u8> VBE_W<'a, O> {
    #[doc = "V&lt;sub>BAT&lt;/sub> battery charging disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VBE_A::B_0X0)
    }
    #[doc = "V&lt;sub>BAT&lt;/sub> battery charging enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VBE_A::B_0X1)
    }
}
#[doc = "Field `VBRS` reader - V&lt;sub>BAT&lt;/sub> battery charging resistor selection"]
pub type VBRS_R = crate::BitReader<VBRS_A>;
#[doc = "V&lt;sub>BAT&lt;/sub> battery charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS_A {
    #[doc = "0: Charge V&lt;sub>BAT&lt;/sub> through a 5 kOhms resistor"]
    B_0X0 = 0,
    #[doc = "1: Charge V&lt;sub>BAT&lt;/sub> through a 1.5 kOhms resistor"]
    B_0X1 = 1,
}
impl From<VBRS_A> for bool {
    #[inline(always)]
    fn from(variant: VBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl VBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBRS_A {
        match self.bits {
            false => VBRS_A::B_0X0,
            true => VBRS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VBRS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VBRS_A::B_0X1
    }
}
#[doc = "Field `VBRS` writer - V&lt;sub>BAT&lt;/sub> battery charging resistor selection"]
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR4_SPEC, O, VBRS_A>;
impl<'a, const O: u8> VBRS_W<'a, O> {
    #[doc = "Charge V&lt;sub>BAT&lt;/sub> through a 5 kOhms resistor"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VBRS_A::B_0X0)
    }
    #[doc = "Charge V&lt;sub>BAT&lt;/sub> through a 1.5 kOhms resistor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VBRS_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - V&lt;sub>BAT&lt;/sub> battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - V&lt;sub>BAT&lt;/sub> battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<0> {
        WP1_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<1> {
        WP2_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<2> {
        WP3_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<3> {
        WP4_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> WP5_W<4> {
        WP5_W::new(self)
    }
    #[doc = "Bit 8 - V&lt;sub>BAT&lt;/sub> battery charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    #[doc = "Bit 9 - V&lt;sub>BAT&lt;/sub> battery charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr4](index.html) module"]
pub struct PWR_CR4_SPEC;
impl crate::RegisterSpec for PWR_CR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr4::R](R) reader structure"]
impl crate::Readable for PWR_CR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr4::W](W) writer structure"]
impl crate::Writable for PWR_CR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR4 to value 0"]
impl crate::Resettable for PWR_CR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
