#[doc = "Register `FDCAN_RXGFC` reader"]
pub struct R(crate::R<FDCAN_RXGFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXGFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXGFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXGFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXGFC` writer"]
pub struct W(crate::W<FDCAN_RXGFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXGFC_SPEC>;
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
impl From<crate::W<FDCAN_RXGFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXGFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFE` reader - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RRFE_R = crate::BitReader<RRFE_A>;
#[doc = "Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFE_A {
    #[doc = "0: Filter remote frames with 29-bit standard IDs"]
    B_0X0 = 0,
    #[doc = "1: Reject all remote frames with 29-bit standard IDs"]
    B_0X1 = 1,
}
impl From<RRFE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RRFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFE_A {
        match self.bits {
            false => RRFE_A::B_0X0,
            true => RRFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRFE_A::B_0X1
    }
}
#[doc = "Field `RRFE` writer - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RRFE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_RXGFC_SPEC, O, RRFE_A>;
impl<'a, const O: u8> RRFE_W<'a, O> {
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRFE_A::B_0X0)
    }
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRFE_A::B_0X1)
    }
}
#[doc = "Field `RRFS` reader - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RRFS_R = crate::BitReader<RRFS_A>;
#[doc = "Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFS_A {
    #[doc = "0: Filter remote frames with 11-bit standard IDs"]
    B_0X0 = 0,
    #[doc = "1: Reject all remote frames with 11-bit standard IDs"]
    B_0X1 = 1,
}
impl From<RRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RRFS_A) -> Self {
        variant as u8 != 0
    }
}
impl RRFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFS_A {
        match self.bits {
            false => RRFS_A::B_0X0,
            true => RRFS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRFS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRFS_A::B_0X1
    }
}
#[doc = "Field `RRFS` writer - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RRFS_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_RXGFC_SPEC, O, RRFS_A>;
impl<'a, const O: u8> RRFS_W<'a, O> {
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRFS_A::B_0X0)
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRFS_A::B_0X1)
    }
}
#[doc = "Field `ANFE` reader - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ANFE_R = crate::FieldReader<ANFE_A>;
#[doc = "Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFE_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    B_0X0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B_0X1 = 1,
    #[doc = "2: Reject"]
    B_0X2 = 2,
    #[doc = "3: Reject"]
    B_0X3 = 3,
}
impl From<ANFE_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFE_A {
    type Ux = u8;
}
impl ANFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANFE_A {
        match self.bits {
            0 => ANFE_A::B_0X0,
            1 => ANFE_A::B_0X1,
            2 => ANFE_A::B_0X2,
            3 => ANFE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ANFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ANFE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ANFE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ANFE_A::B_0X3
    }
}
#[doc = "Field `ANFE` writer - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ANFE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_RXGFC_SPEC, 2, O, ANFE_A>;
impl<'a, const O: u8> ANFE_W<'a, O> {
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X3)
    }
}
#[doc = "Field `ANFS` reader - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ANFS_R = crate::FieldReader<ANFS_A>;
#[doc = "Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFS_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    B_0X0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B_0X1 = 1,
    #[doc = "2: Reject"]
    B_0X2 = 2,
    #[doc = "3: Reject"]
    B_0X3 = 3,
}
impl From<ANFS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFS_A {
    type Ux = u8;
}
impl ANFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANFS_A {
        match self.bits {
            0 => ANFS_A::B_0X0,
            1 => ANFS_A::B_0X1,
            2 => ANFS_A::B_0X2,
            3 => ANFS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ANFS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ANFS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ANFS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ANFS_A::B_0X3
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ANFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_RXGFC_SPEC, 2, O, ANFS_A>;
impl<'a, const O: u8> ANFS_W<'a, O> {
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X3)
    }
}
#[doc = "Field `F1OM` reader - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F1OM_R = crate::BitReader;
#[doc = "Field `F1OM` writer - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F1OM_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_RXGFC_SPEC, O>;
#[doc = "Field `F0OM` reader - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F0OM_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_RXGFC_SPEC, O>;
#[doc = "Field `LSS` reader - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LSS_R = crate::FieldReader<LSS_A>;
#[doc = "List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSS_A {
    #[doc = "0: No standard message ID filter"]
    B_0X0 = 0,
}
impl From<LSS_A> for u8 {
    #[inline(always)]
    fn from(variant: LSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSS_A {
    type Ux = u8;
}
impl LSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LSS_A> {
        match self.bits {
            0 => Some(LSS_A::B_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSS_A::B_0X0
    }
}
#[doc = "Field `LSS` writer - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LSS_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_RXGFC_SPEC, 5, O, LSS_A>;
impl<'a, const O: u8> LSS_W<'a, O> {
    #[doc = "No standard message ID filter"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSS_A::B_0X0)
    }
}
#[doc = "Field `LSE` reader - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LSE_R = crate::FieldReader<LSE_A>;
#[doc = "List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSE_A {
    #[doc = "0: No extended message ID filter"]
    B_0X0 = 0,
}
impl From<LSE_A> for u8 {
    #[inline(always)]
    fn from(variant: LSE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSE_A {
    type Ux = u8;
}
impl LSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LSE_A> {
        match self.bits {
            0 => Some(LSE_A::B_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSE_A::B_0X0
    }
}
#[doc = "Field `LSE` writer - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LSE_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_RXGFC_SPEC, 4, O, LSE_A>;
impl<'a, const O: u8> LSE_W<'a, O> {
    #[doc = "No extended message ID filter"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSE_A::B_0X0)
    }
}
impl R {
    #[doc = "Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<0> {
        RRFE_W::new(self)
    }
    #[doc = "Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<1> {
        RRFS_W::new(self)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<2> {
        ANFE_W::new(self)
    }
    #[doc = "Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<4> {
        ANFS_W::new(self)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<8> {
        F1OM_W::new(self)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<9> {
        F0OM_W::new(self)
    }
    #[doc = "Bits 16:20 - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<16> {
        LSS_W::new(self)
    }
    #[doc = "Bits 24:27 - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<24> {
        LSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN global filter configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxgfc](index.html) module"]
pub struct FDCAN_RXGFC_SPEC;
impl crate::RegisterSpec for FDCAN_RXGFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxgfc::R](R) reader structure"]
impl crate::Readable for FDCAN_RXGFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxgfc::W](W) writer structure"]
impl crate::Writable for FDCAN_RXGFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXGFC to value 0"]
impl crate::Resettable for FDCAN_RXGFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
