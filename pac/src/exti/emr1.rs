#[doc = "Register `EMR1` reader"]
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR1` writer"]
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0` reader - Event Mask on line 0"]
pub type EM0_R = crate::BitReader;
#[doc = "Field `EM0` writer - Event Mask on line 0"]
pub type EM0_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM1` reader - Event Mask on line 1"]
pub type EM1_R = crate::BitReader;
#[doc = "Field `EM1` writer - Event Mask on line 1"]
pub type EM1_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM2` reader - Event Mask on line 2"]
pub type EM2_R = crate::BitReader;
#[doc = "Field `EM2` writer - Event Mask on line 2"]
pub type EM2_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM3` reader - Event Mask on line 3"]
pub type EM3_R = crate::BitReader;
#[doc = "Field `EM3` writer - Event Mask on line 3"]
pub type EM3_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM4` reader - Event Mask on line 4"]
pub type EM4_R = crate::BitReader;
#[doc = "Field `EM4` writer - Event Mask on line 4"]
pub type EM4_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM5` reader - Event Mask on line 5"]
pub type EM5_R = crate::BitReader;
#[doc = "Field `EM5` writer - Event Mask on line 5"]
pub type EM5_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM6` reader - Event Mask on line 6"]
pub type EM6_R = crate::BitReader;
#[doc = "Field `EM6` writer - Event Mask on line 6"]
pub type EM6_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM7` reader - Event Mask on line 7"]
pub type EM7_R = crate::BitReader;
#[doc = "Field `EM7` writer - Event Mask on line 7"]
pub type EM7_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM8` reader - Event Mask on line 8"]
pub type EM8_R = crate::BitReader;
#[doc = "Field `EM8` writer - Event Mask on line 8"]
pub type EM8_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM9` reader - Event Mask on line 9"]
pub type EM9_R = crate::BitReader;
#[doc = "Field `EM9` writer - Event Mask on line 9"]
pub type EM9_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM10` reader - Event Mask on line 10"]
pub type EM10_R = crate::BitReader;
#[doc = "Field `EM10` writer - Event Mask on line 10"]
pub type EM10_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM11` reader - Event Mask on line 11"]
pub type EM11_R = crate::BitReader;
#[doc = "Field `EM11` writer - Event Mask on line 11"]
pub type EM11_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM12` reader - Event Mask on line 12"]
pub type EM12_R = crate::BitReader;
#[doc = "Field `EM12` writer - Event Mask on line 12"]
pub type EM12_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM13` reader - Event Mask on line 13"]
pub type EM13_R = crate::BitReader;
#[doc = "Field `EM13` writer - Event Mask on line 13"]
pub type EM13_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM14` reader - Event Mask on line 14"]
pub type EM14_R = crate::BitReader;
#[doc = "Field `EM14` writer - Event Mask on line 14"]
pub type EM14_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM15` reader - Event Mask on line 15"]
pub type EM15_R = crate::BitReader;
#[doc = "Field `EM15` writer - Event Mask on line 15"]
pub type EM15_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM16` reader - Event Mask on line 16"]
pub type EM16_R = crate::BitReader;
#[doc = "Field `EM16` writer - Event Mask on line 16"]
pub type EM16_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM17` reader - Event Mask on line 17"]
pub type EM17_R = crate::BitReader;
#[doc = "Field `EM17` writer - Event Mask on line 17"]
pub type EM17_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM18` reader - Event Mask on line 18"]
pub type EM18_R = crate::BitReader;
#[doc = "Field `EM18` writer - Event Mask on line 18"]
pub type EM18_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM19` reader - Event Mask on line 19"]
pub type EM19_R = crate::BitReader;
#[doc = "Field `EM19` writer - Event Mask on line 19"]
pub type EM19_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM20` reader - Event Mask on line 20"]
pub type EM20_R = crate::BitReader;
#[doc = "Field `EM20` writer - Event Mask on line 20"]
pub type EM20_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM21` reader - Event Mask on line 21"]
pub type EM21_R = crate::BitReader;
#[doc = "Field `EM21` writer - Event Mask on line 21"]
pub type EM21_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM22` reader - Event Mask on line 22"]
pub type EM22_R = crate::BitReader;
#[doc = "Field `EM22` writer - Event Mask on line 22"]
pub type EM22_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM23` reader - Event Mask on line 23"]
pub type EM23_R = crate::BitReader;
#[doc = "Field `EM23` writer - Event Mask on line 23"]
pub type EM23_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM24` reader - Event Mask on line 24"]
pub type EM24_R = crate::BitReader;
#[doc = "Field `EM24` writer - Event Mask on line 24"]
pub type EM24_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM25` reader - Event Mask on line 25"]
pub type EM25_R = crate::BitReader;
#[doc = "Field `EM25` writer - Event Mask on line 25"]
pub type EM25_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM26` reader - Event Mask on line 26"]
pub type EM26_R = crate::BitReader;
#[doc = "Field `EM26` writer - Event Mask on line 26"]
pub type EM26_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM27` reader - Event Mask on line 27"]
pub type EM27_R = crate::BitReader;
#[doc = "Field `EM27` writer - Event Mask on line 27"]
pub type EM27_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM28` reader - Event Mask on line 28"]
pub type EM28_R = crate::BitReader;
#[doc = "Field `EM28` writer - Event Mask on line 28"]
pub type EM28_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM29` reader - Event Mask on line 29"]
pub type EM29_R = crate::BitReader;
#[doc = "Field `EM29` writer - Event Mask on line 29"]
pub type EM29_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM30` reader - Event Mask on line 30"]
pub type EM30_R = crate::BitReader;
#[doc = "Field `EM30` writer - Event Mask on line 30"]
pub type EM30_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
#[doc = "Field `EM31` reader - Event Mask on line 31"]
pub type EM31_R = crate::BitReader;
#[doc = "Field `EM31` writer - Event Mask on line 31"]
pub type EM31_W<'a, const O: u8> = crate::BitWriter<'a, EMR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    pub fn em22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event Mask on line 23"]
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event Mask on line 24"]
    #[inline(always)]
    pub fn em24(&self) -> EM24_R {
        EM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Event Mask on line 25"]
    #[inline(always)]
    pub fn em25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event Mask on line 26"]
    #[inline(always)]
    pub fn em26(&self) -> EM26_R {
        EM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Event Mask on line 27"]
    #[inline(always)]
    pub fn em27(&self) -> EM27_R {
        EM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Event Mask on line 28"]
    #[inline(always)]
    pub fn em28(&self) -> EM28_R {
        EM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Event Mask on line 29"]
    #[inline(always)]
    pub fn em29(&self) -> EM29_R {
        EM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Event Mask on line 30"]
    #[inline(always)]
    pub fn em30(&self) -> EM30_R {
        EM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Event Mask on line 31"]
    #[inline(always)]
    pub fn em31(&self) -> EM31_R {
        EM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> EM0_W<0> {
        EM0_W::new(self)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> EM1_W<1> {
        EM1_W::new(self)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> EM2_W<2> {
        EM2_W::new(self)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> EM3_W<3> {
        EM3_W::new(self)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn em4(&mut self) -> EM4_W<4> {
        EM4_W::new(self)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn em5(&mut self) -> EM5_W<5> {
        EM5_W::new(self)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn em6(&mut self) -> EM6_W<6> {
        EM6_W::new(self)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn em7(&mut self) -> EM7_W<7> {
        EM7_W::new(self)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn em8(&mut self) -> EM8_W<8> {
        EM8_W::new(self)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn em9(&mut self) -> EM9_W<9> {
        EM9_W::new(self)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn em10(&mut self) -> EM10_W<10> {
        EM10_W::new(self)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn em11(&mut self) -> EM11_W<11> {
        EM11_W::new(self)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn em12(&mut self) -> EM12_W<12> {
        EM12_W::new(self)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn em13(&mut self) -> EM13_W<13> {
        EM13_W::new(self)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn em14(&mut self) -> EM14_W<14> {
        EM14_W::new(self)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn em15(&mut self) -> EM15_W<15> {
        EM15_W::new(self)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn em16(&mut self) -> EM16_W<16> {
        EM16_W::new(self)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn em17(&mut self) -> EM17_W<17> {
        EM17_W::new(self)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn em18(&mut self) -> EM18_W<18> {
        EM18_W::new(self)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn em19(&mut self) -> EM19_W<19> {
        EM19_W::new(self)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn em20(&mut self) -> EM20_W<20> {
        EM20_W::new(self)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn em21(&mut self) -> EM21_W<21> {
        EM21_W::new(self)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn em22(&mut self) -> EM22_W<22> {
        EM22_W::new(self)
    }
    #[doc = "Bit 23 - Event Mask on line 23"]
    #[inline(always)]
    #[must_use]
    pub fn em23(&mut self) -> EM23_W<23> {
        EM23_W::new(self)
    }
    #[doc = "Bit 24 - Event Mask on line 24"]
    #[inline(always)]
    #[must_use]
    pub fn em24(&mut self) -> EM24_W<24> {
        EM24_W::new(self)
    }
    #[doc = "Bit 25 - Event Mask on line 25"]
    #[inline(always)]
    #[must_use]
    pub fn em25(&mut self) -> EM25_W<25> {
        EM25_W::new(self)
    }
    #[doc = "Bit 26 - Event Mask on line 26"]
    #[inline(always)]
    #[must_use]
    pub fn em26(&mut self) -> EM26_W<26> {
        EM26_W::new(self)
    }
    #[doc = "Bit 27 - Event Mask on line 27"]
    #[inline(always)]
    #[must_use]
    pub fn em27(&mut self) -> EM27_W<27> {
        EM27_W::new(self)
    }
    #[doc = "Bit 28 - Event Mask on line 28"]
    #[inline(always)]
    #[must_use]
    pub fn em28(&mut self) -> EM28_W<28> {
        EM28_W::new(self)
    }
    #[doc = "Bit 29 - Event Mask on line 29"]
    #[inline(always)]
    #[must_use]
    pub fn em29(&mut self) -> EM29_W<29> {
        EM29_W::new(self)
    }
    #[doc = "Bit 30 - Event Mask on line 30"]
    #[inline(always)]
    #[must_use]
    pub fn em30(&mut self) -> EM30_W<30> {
        EM30_W::new(self)
    }
    #[doc = "Bit 31 - Event Mask on line 31"]
    #[inline(always)]
    #[must_use]
    pub fn em31(&mut self) -> EM31_W<31> {
        EM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr1](index.html) module"]
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr1::R](R) reader structure"]
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr1::W](W) writer structure"]
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR1 to value 0"]
impl crate::Resettable for EMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
