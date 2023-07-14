#[doc = "Register `FDCAN_TOCC` reader"]
pub struct R(crate::R<FDCAN_TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TOCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TOCC` writer"]
pub struct W(crate::W<FDCAN_TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TOCC_SPEC>;
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
impl From<crate::W<FDCAN_TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TOCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETOC` reader - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ETOC_R = crate::BitReader<ETOC_A>;
#[doc = "Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETOC_A {
    #[doc = "0: Timeout counter disabled"]
    B_0X0 = 0,
    #[doc = "1: Timeout counter enabled"]
    B_0X1 = 1,
}
impl From<ETOC_A> for bool {
    #[inline(always)]
    fn from(variant: ETOC_A) -> Self {
        variant as u8 != 0
    }
}
impl ETOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETOC_A {
        match self.bits {
            false => ETOC_A::B_0X0,
            true => ETOC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETOC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETOC_A::B_0X1
    }
}
#[doc = "Field `ETOC` writer - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ETOC_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_TOCC_SPEC, O, ETOC_A>;
impl<'a, const O: u8> ETOC_W<'a, O> {
    #[doc = "Timeout counter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETOC_A::B_0X0)
    }
    #[doc = "Timeout counter enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETOC_A::B_0X1)
    }
}
#[doc = "Field `TOS` reader - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TOS_R = crate::FieldReader<TOS_A>;
#[doc = "Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: Continuous operation"]
    B_0X0 = 0,
    #[doc = "1: Timeout controlled by Tx event FIFO"]
    B_0X1 = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    B_0X2 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    B_0X3 = 3,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOS_A {
    type Ux = u8;
}
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS_A {
        match self.bits {
            0 => TOS_A::B_0X0,
            1 => TOS_A::B_0X1,
            2 => TOS_A::B_0X2,
            3 => TOS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TOS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TOS_A::B_0X3
    }
}
#[doc = "Field `TOS` writer - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_TOCC_SPEC, 2, O, TOS_A>;
impl<'a, const O: u8> TOS_W<'a, O> {
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOS_A::B_0X0)
    }
    #[doc = "Timeout controlled by Tx event FIFO"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOS_A::B_0X1)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TOS_A::B_0X2)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TOS_A::B_0X3)
    }
}
#[doc = "Field `TOP` reader - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_TOCC_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<0> {
        ETOC_W::new(self)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<1> {
        TOS_W::new(self)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<16> {
        TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN timeout counter configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tocc](index.html) module"]
pub struct FDCAN_TOCC_SPEC;
impl crate::RegisterSpec for FDCAN_TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tocc::R](R) reader structure"]
impl crate::Readable for FDCAN_TOCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tocc::W](W) writer structure"]
impl crate::Writable for FDCAN_TOCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TOCC to value 0xffff_0000"]
impl crate::Resettable for FDCAN_TOCC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
