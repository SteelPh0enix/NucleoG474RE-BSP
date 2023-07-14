#[doc = "Register `PWR_CR5` reader"]
pub struct R(crate::R<PWR_CR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR5` writer"]
pub struct W(crate::W<PWR_CR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR5_SPEC>;
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
impl From<crate::W<PWR_CR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R1MODE` reader - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
pub type R1MODE_R = crate::BitReader<R1MODE_A>;
#[doc = "Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R1MODE_A {
    #[doc = "0: Main regulator in range 1 boost mode."]
    B_0X0 = 0,
    #[doc = "1: Main regulator in range 1 normal mode."]
    B_0X1 = 1,
}
impl From<R1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: R1MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl R1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R1MODE_A {
        match self.bits {
            false => R1MODE_A::B_0X0,
            true => R1MODE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == R1MODE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == R1MODE_A::B_0X1
    }
}
#[doc = "Field `R1MODE` writer - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
pub type R1MODE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CR5_SPEC, O, R1MODE_A>;
impl<'a, const O: u8> R1MODE_W<'a, O> {
    #[doc = "Main regulator in range 1 boost mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(R1MODE_A::B_0X0)
    }
    #[doc = "Main regulator in range 1 normal mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(R1MODE_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 8 - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
    #[inline(always)]
    #[must_use]
    pub fn r1mode(&mut self) -> R1MODE_W<8> {
        R1MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr5](index.html) module"]
pub struct PWR_CR5_SPEC;
impl crate::RegisterSpec for PWR_CR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr5::R](R) reader structure"]
impl crate::Readable for PWR_CR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr5::W](W) writer structure"]
impl crate::Writable for PWR_CR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR5 to value 0x0100"]
impl crate::Resettable for PWR_CR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
