#[doc = "Register `AIM` reader"]
pub struct R(crate::R<AIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIM` writer"]
pub struct W(crate::W<AIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIM_SPEC>;
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
impl From<crate::W<AIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRUDRIE` reader - Overrun/underrun interrupt enable"]
pub type OVRUDRIE_R = crate::BitReader;
#[doc = "Field `OVRUDRIE` writer - Overrun/underrun interrupt enable"]
pub type OVRUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
#[doc = "Field `MUTEDET` reader - Mute detection interrupt enable"]
pub type MUTEDET_R = crate::BitReader;
#[doc = "Field `MUTEDET` writer - Mute detection interrupt enable"]
pub type MUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
#[doc = "Field `WCKCFG` reader - Wrong clock configuration interrupt enable"]
pub type WCKCFG_R = crate::BitReader;
#[doc = "Field `WCKCFG` writer - Wrong clock configuration interrupt enable"]
pub type WCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
#[doc = "Field `FREQIE` reader - FIFO request interrupt enable"]
pub type FREQIE_R = crate::BitReader;
#[doc = "Field `FREQIE` writer - FIFO request interrupt enable"]
pub type FREQIE_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
#[doc = "Field `CNRDYIE` reader - Codec not ready interrupt enable"]
pub type CNRDYIE_R = crate::BitReader;
#[doc = "Field `CNRDYIE` writer - Codec not ready interrupt enable"]
pub type CNRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
#[doc = "Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable"]
pub type AFSDETIE_R = crate::BitReader;
#[doc = "Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable"]
pub type AFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
#[doc = "Field `LFSDET` reader - Late frame synchronization detection interrupt enable"]
pub type LFSDET_R = crate::BitReader;
#[doc = "Field `LFSDET` writer - Late frame synchronization detection interrupt enable"]
pub type LFSDET_W<'a, const O: u8> = crate::BitWriter<'a, AIM_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable"]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<0> {
        OVRUDRIE_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mutedet(&mut self) -> MUTEDET_W<1> {
        MUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wckcfg(&mut self) -> WCKCFG_W<2> {
        WCKCFG_W::new(self)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<3> {
        FREQIE_W::new(self)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<4> {
        CNRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<5> {
        AFSDETIE_W::new(self)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfsdet(&mut self) -> LFSDET_W<6> {
        LFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AInterrupt mask register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aim](index.html) module"]
pub struct AIM_SPEC;
impl crate::RegisterSpec for AIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aim::R](R) reader structure"]
impl crate::Readable for AIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aim::W](W) writer structure"]
impl crate::Writable for AIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIM to value 0"]
impl crate::Resettable for AIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
