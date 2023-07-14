#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - FUNC"]
pub type FUNC_R = crate::FieldReader;
#[doc = "Field `FUNC` writer - FUNC"]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, CSR_SPEC, 4, O>;
#[doc = "Field `PRECISION` reader - PRECISION"]
pub type PRECISION_R = crate::FieldReader;
#[doc = "Field `PRECISION` writer - PRECISION"]
pub type PRECISION_W<'a, const O: u8> = crate::FieldWriter<'a, CSR_SPEC, 4, O>;
#[doc = "Field `SCALE` reader - SCALE"]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `SCALE` writer - SCALE"]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, CSR_SPEC, 3, O>;
#[doc = "Field `IEN` reader - IEN"]
pub type IEN_R = crate::BitReader;
#[doc = "Field `IEN` writer - IEN"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `DMAREN` reader - DMAREN"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMAREN"]
pub type DMAREN_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub type DMAWEN_R = crate::BitReader;
#[doc = "Field `DMAWEN` writer - DMAWEN"]
pub type DMAWEN_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `NRES` reader - NRES"]
pub type NRES_R = crate::BitReader;
#[doc = "Field `NRES` writer - NRES"]
pub type NRES_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `NARGS` reader - NARGS"]
pub type NARGS_R = crate::BitReader;
#[doc = "Field `NARGS` writer - NARGS"]
pub type NARGS_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `RESSIZE` reader - RESSIZE"]
pub type RESSIZE_R = crate::BitReader;
#[doc = "Field `RESSIZE` writer - RESSIZE"]
pub type RESSIZE_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `ARGSIZE` reader - ARGSIZE"]
pub type ARGSIZE_R = crate::BitReader;
#[doc = "Field `ARGSIZE` writer - ARGSIZE"]
pub type ARGSIZE_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
#[doc = "Field `RRDY` reader - RRDY"]
pub type RRDY_R = crate::BitReader;
#[doc = "Field `RRDY` writer - RRDY"]
pub type RRDY_W<'a, const O: u8> = crate::BitWriter<'a, CSR_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PRECISION"]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - SCALE"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 4:7 - PRECISION"]
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PRECISION_W<4> {
        PRECISION_W::new(self)
    }
    #[doc = "Bits 8:10 - SCALE"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<8> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<16> {
        IEN_W::new(self)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<17> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<18> {
        DMAWEN_W::new(self)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<19> {
        NRES_W::new(self)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<20> {
        NARGS_W::new(self)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<21> {
        RESSIZE_W::new(self)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<22> {
        ARGSIZE_W::new(self)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    #[must_use]
    pub fn rrdy(&mut self) -> RRDY_W<31> {
        RRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC Control Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
