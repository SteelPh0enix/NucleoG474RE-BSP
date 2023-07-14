#[doc = "Register `CCMR3_Output` reader"]
pub struct R(crate::R<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR3_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR3_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR3_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR3_Output` writer"]
pub struct W(crate::W<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR3_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR3_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR3_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable"]
pub type OC5FE_R = crate::BitReader;
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable"]
pub type OC5FE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable"]
pub type OC5PE_R = crate::BitReader;
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable"]
pub type OC5PE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
#[doc = "Field `OC5M` reader - Output compare 5 mode"]
pub type OC5M_R = crate::FieldReader;
#[doc = "Field `OC5M` writer - Output compare 5 mode"]
pub type OC5M_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR3_OUTPUT_SPEC, 3, O>;
#[doc = "Field `OC5CE` reader - Output compare 5 clear enable"]
pub type OC5CE_R = crate::BitReader;
#[doc = "Field `OC5CE` writer - Output compare 5 clear enable"]
pub type OC5CE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
#[doc = "Field `OC6FE` reader - Output compare 6 fast enable"]
pub type OC6FE_R = crate::BitReader;
#[doc = "Field `OC6FE` writer - Output compare 6 fast enable"]
pub type OC6FE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
#[doc = "Field `OC6PE` reader - Output compare 6 preload enable"]
pub type OC6PE_R = crate::BitReader;
#[doc = "Field `OC6PE` writer - Output compare 6 preload enable"]
pub type OC6PE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
#[doc = "Field `OC6M` reader - Output compare 6 mode"]
pub type OC6M_R = crate::FieldReader;
#[doc = "Field `OC6M` writer - Output compare 6 mode"]
pub type OC6M_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR3_OUTPUT_SPEC, 3, O>;
#[doc = "Field `OC6CE` reader - Output compare 6 clear enable"]
pub type OC6CE_R = crate::BitReader;
#[doc = "Field `OC6CE` writer - Output compare 6 clear enable"]
pub type OC6CE_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
#[doc = "Field `OC5M_bit3` reader - Output Compare 5 mode bit 3"]
pub type OC5M_BIT3_R = crate::FieldReader;
#[doc = "Field `OC5M_bit3` writer - Output Compare 5 mode bit 3"]
pub type OC5M_BIT3_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR3_OUTPUT_SPEC, 3, O>;
#[doc = "Field `OC6M_bit3` reader - Output Compare 6 mode bit 3"]
pub type OC6M_BIT3_R = crate::BitReader;
#[doc = "Field `OC6M_bit3` writer - Output Compare 6 mode bit 3"]
pub type OC6M_BIT3_W<'a, const O: u8> = crate::BitWriter<'a, CCMR3_OUTPUT_SPEC, O>;
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_bit3(&self) -> OC5M_BIT3_R {
        OC5M_BIT3_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_bit3(&self) -> OC6M_BIT3_R {
        OC6M_BIT3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OC5FE_W<2> {
        OC5FE_W::new(self)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OC5PE_W<3> {
        OC5PE_W::new(self)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m(&mut self) -> OC5M_W<4> {
        OC5M_W::new(self)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OC5CE_W<7> {
        OC5CE_W::new(self)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OC6FE_W<10> {
        OC6FE_W::new(self)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OC6PE_W<11> {
        OC6PE_W::new(self)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m(&mut self) -> OC6M_W<12> {
        OC6M_W::new(self)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OC6CE_W<15> {
        OC6CE_W::new(self)
    }
    #[doc = "Bits 16:18 - Output Compare 5 mode bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m_bit3(&mut self) -> OC5M_BIT3_W<16> {
        OC5M_BIT3_W::new(self)
    }
    #[doc = "Bit 24 - Output Compare 6 mode bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m_bit3(&mut self) -> OC6M_BIT3_W<24> {
        OC6M_BIT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr3_output](index.html) module"]
pub struct CCMR3_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR3_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr3_output::R](R) reader structure"]
impl crate::Readable for CCMR3_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr3_output::W](W) writer structure"]
impl crate::Writable for CCMR3_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR3_Output to value 0"]
impl crate::Resettable for CCMR3_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
