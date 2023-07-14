#[doc = "Register `FDCAN_TSCC` reader"]
pub struct R(crate::R<FDCAN_TSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TSCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TSCC` writer"]
pub struct W(crate::W<FDCAN_TSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TSCC_SPEC>;
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
impl From<crate::W<FDCAN_TSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TSCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TSS_R = crate::FieldReader<TSS_A>;
#[doc = "Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSS_A {
    #[doc = "0: Timestamp counter value always 0x0000"]
    B_0X0 = 0,
    #[doc = "1: Timestamp counter value incremented according to TCP"]
    B_0X1 = 1,
    #[doc = "2: External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    B_0X2 = 2,
    #[doc = "3: Same as 00."]
    B_0X3 = 3,
}
impl From<TSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSS_A {
    type Ux = u8;
}
impl TSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSS_A {
        match self.bits {
            0 => TSS_A::B_0X0,
            1 => TSS_A::B_0X1,
            2 => TSS_A::B_0X2,
            3 => TSS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TSS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TSS_A::B_0X3
    }
}
#[doc = "Field `TSS` writer - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_TSCC_SPEC, 2, O, TSS_A>;
impl<'a, const O: u8> TSS_W<'a, O> {
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSS_A::B_0X0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSS_A::B_0X1)
    }
    #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TSS_A::B_0X2)
    }
    #[doc = "Same as 00."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TSS_A::B_0X3)
    }
}
#[doc = "Field `TCP` reader - Timestamp counter prescaler"]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp counter prescaler"]
pub type TCP_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_TSCC_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp counter prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<0> {
        TSS_W::new(self)
    }
    #[doc = "Bits 16:19 - Timestamp counter prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TCP_W<16> {
        TCP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN timestamp counter configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tscc](index.html) module"]
pub struct FDCAN_TSCC_SPEC;
impl crate::RegisterSpec for FDCAN_TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tscc::R](R) reader structure"]
impl crate::Readable for FDCAN_TSCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tscc::W](W) writer structure"]
impl crate::Writable for FDCAN_TSCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TSCC to value 0"]
impl crate::Resettable for FDCAN_TSCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
