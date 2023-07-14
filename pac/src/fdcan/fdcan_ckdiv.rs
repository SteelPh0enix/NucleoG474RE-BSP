#[doc = "Register `FDCAN_CKDIV` reader"]
pub struct R(crate::R<FDCAN_CKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_CKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_CKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_CKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_CKDIV` writer"]
pub struct W(crate::W<FDCAN_CKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_CKDIV_SPEC>;
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
impl From<crate::W<FDCAN_CKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_CKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PDIV_R = crate::FieldReader<PDIV_A>;
#[doc = "input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDIV_A {
    #[doc = "0: Divide by 1"]
    B_0X0 = 0,
    #[doc = "1: Divide by 2"]
    B_0X1 = 1,
    #[doc = "2: Divide by 4"]
    B_0X2 = 2,
    #[doc = "3: Divide by 6"]
    B_0X3 = 3,
    #[doc = "4: Divide by 8"]
    B_0X4 = 4,
    #[doc = "5: Divide by 10"]
    B_0X5 = 5,
    #[doc = "6: Divide by 12"]
    B_0X6 = 6,
    #[doc = "7: Divide by 14"]
    B_0X7 = 7,
    #[doc = "8: Divide by 16"]
    B_0X8 = 8,
    #[doc = "9: Divide by 18"]
    B_0X9 = 9,
    #[doc = "10: Divide by 20"]
    B_0X_A = 10,
    #[doc = "11: Divide by 22"]
    B_0X_B = 11,
    #[doc = "12: Divide by 24"]
    B_0X_C = 12,
    #[doc = "13: Divide by 26"]
    B_0X_D = 13,
    #[doc = "14: Divide by 28"]
    B_0X_E = 14,
    #[doc = "15: Divide by 30"]
    B_0X_F = 15,
}
impl From<PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PDIV_A {
    type Ux = u8;
}
impl PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIV_A {
        match self.bits {
            0 => PDIV_A::B_0X0,
            1 => PDIV_A::B_0X1,
            2 => PDIV_A::B_0X2,
            3 => PDIV_A::B_0X3,
            4 => PDIV_A::B_0X4,
            5 => PDIV_A::B_0X5,
            6 => PDIV_A::B_0X6,
            7 => PDIV_A::B_0X7,
            8 => PDIV_A::B_0X8,
            9 => PDIV_A::B_0X9,
            10 => PDIV_A::B_0X_A,
            11 => PDIV_A::B_0X_B,
            12 => PDIV_A::B_0X_C,
            13 => PDIV_A::B_0X_D,
            14 => PDIV_A::B_0X_E,
            15 => PDIV_A::B_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PDIV_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PDIV_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PDIV_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PDIV_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PDIV_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PDIV_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PDIV_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0X_A`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PDIV_A::B_0X_A
    }
    #[doc = "Checks if the value of the field is `B_0X_B`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PDIV_A::B_0X_B
    }
    #[doc = "Checks if the value of the field is `B_0X_C`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == PDIV_A::B_0X_C
    }
    #[doc = "Checks if the value of the field is `B_0X_D`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == PDIV_A::B_0X_D
    }
    #[doc = "Checks if the value of the field is `B_0X_E`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == PDIV_A::B_0X_E
    }
    #[doc = "Checks if the value of the field is `B_0X_F`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == PDIV_A::B_0X_F
    }
}
#[doc = "Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_CKDIV_SPEC, 4, O, PDIV_A>;
impl<'a, const O: u8> PDIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X1)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X2)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X4)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X5)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X6)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X7)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X8)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X9)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X_A)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X_B)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X_C)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X_D)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X_E)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X_F)
    }
}
impl R {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<0> {
        PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN CFG clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ckdiv](index.html) module"]
pub struct FDCAN_CKDIV_SPEC;
impl crate::RegisterSpec for FDCAN_CKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ckdiv::R](R) reader structure"]
impl crate::Readable for FDCAN_CKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ckdiv::W](W) writer structure"]
impl crate::Writable for FDCAN_CKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_CKDIV to value 0"]
impl crate::Resettable for FDCAN_CKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
