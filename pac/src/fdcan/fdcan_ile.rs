#[doc = "Register `FDCAN_ILE` reader"]
pub struct R(crate::R<FDCAN_ILE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ILE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ILE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ILE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_ILE` writer"]
pub struct W(crate::W<FDCAN_ILE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_ILE_SPEC>;
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
impl From<crate::W<FDCAN_ILE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_ILE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EINT0` reader - Enable interrupt line 0"]
pub type EINT0_R = crate::BitReader<EINT0_A>;
#[doc = "Enable interrupt line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT0_A {
    #[doc = "0: Interrupt line fdcan_intr1_it disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt line fdcan_intr1_it enabled"]
    B_0X1 = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT0_A {
        match self.bits {
            false => EINT0_A::B_0X0,
            true => EINT0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EINT0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EINT0_A::B_0X1
    }
}
#[doc = "Field `EINT0` writer - Enable interrupt line 0"]
pub type EINT0_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_ILE_SPEC, O, EINT0_A>;
impl<'a, const O: u8> EINT0_W<'a, O> {
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EINT0_A::B_0X0)
    }
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EINT0_A::B_0X1)
    }
}
#[doc = "Field `EINT1` reader - Enable interrupt line 1"]
pub type EINT1_R = crate::BitReader<EINT1_A>;
#[doc = "Enable interrupt line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT1_A {
    #[doc = "0: Interrupt line fdcan_intr0_it disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt line fdcan_intr0_it enabled"]
    B_0X1 = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT1_A {
        match self.bits {
            false => EINT1_A::B_0X0,
            true => EINT1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EINT1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EINT1_A::B_0X1
    }
}
#[doc = "Field `EINT1` writer - Enable interrupt line 1"]
pub type EINT1_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_ILE_SPEC, O, EINT1_A>;
impl<'a, const O: u8> EINT1_W<'a, O> {
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EINT1_A::B_0X0)
    }
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EINT1_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<0> {
        EINT0_W::new(self)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<1> {
        EINT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN interrupt line enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ile](index.html) module"]
pub struct FDCAN_ILE_SPEC;
impl crate::RegisterSpec for FDCAN_ILE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ile::R](R) reader structure"]
impl crate::Readable for FDCAN_ILE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ile::W](W) writer structure"]
impl crate::Writable for FDCAN_ILE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_ILE to value 0"]
impl crate::Resettable for FDCAN_ILE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
