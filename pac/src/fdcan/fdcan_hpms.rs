#[doc = "Register `FDCAN_HPMS` reader"]
pub struct R(crate::R<FDCAN_HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_HPMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIDX` reader - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= 1."]
pub type BIDX_R = crate::FieldReader;
#[doc = "Field `MSI` reader - Message storage indicator"]
pub type MSI_R = crate::FieldReader<MSI_A>;
#[doc = "Message storage indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSI_A {
    #[doc = "0: No FIFO selected"]
    B_0X0 = 0,
    #[doc = "1: FIFO overrun"]
    B_0X1 = 1,
    #[doc = "2: Message stored in FIFO 0"]
    B_0X2 = 2,
    #[doc = "3: Message stored in FIFO 1"]
    B_0X3 = 3,
}
impl From<MSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSI_A {
    type Ux = u8;
}
impl MSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSI_A {
        match self.bits {
            0 => MSI_A::B_0X0,
            1 => MSI_A::B_0X1,
            2 => MSI_A::B_0X2,
            3 => MSI_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSI_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MSI_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MSI_A::B_0X3
    }
}
#[doc = "Field `FIDX` reader - Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\]
- 1 or RXGFC\\[LSE\\]
- 1."]
pub type FIDX_R = crate::FieldReader;
#[doc = "Field `FLST` reader - Filter list Indicates the filter list of the matching filter element."]
pub type FLST_R = crate::BitReader<FLST_A>;
#[doc = "Filter list Indicates the filter list of the matching filter element.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLST_A {
    #[doc = "0: Standard filter list"]
    B_0X0 = 0,
    #[doc = "1: Extended filter list"]
    B_0X1 = 1,
}
impl From<FLST_A> for bool {
    #[inline(always)]
    fn from(variant: FLST_A) -> Self {
        variant as u8 != 0
    }
}
impl FLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLST_A {
        match self.bits {
            false => FLST_A::B_0X0,
            true => FLST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLST_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= 1."]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:7 - Message storage indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\]
- 1 or RXGFC\\[LSE\\]
- 1."]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Filter list Indicates the filter list of the matching filter element."]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FDCAN high-priority message status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_hpms](index.html) module"]
pub struct FDCAN_HPMS_SPEC;
impl crate::RegisterSpec for FDCAN_HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_hpms::R](R) reader structure"]
impl crate::Readable for FDCAN_HPMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_HPMS to value 0"]
impl crate::Resettable for FDCAN_HPMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
