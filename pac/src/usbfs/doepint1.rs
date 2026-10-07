#[doc = "Register `DOEPINT1` reader"]
pub type R = crate::R<Doepint1Spec>;
#[doc = "Register `DOEPINT1` writer"]
pub type W = crate::W<Doepint1Spec>;
#[doc = "Field `XFRC` reader - desc XFRC"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - desc XFRC"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - desc EPDISD"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - desc EPDISD"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - desc STUP"]
pub type StupR = crate::BitReader;
#[doc = "Field `STUP` writer - desc STUP"]
pub type StupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - desc OTEPDIS"]
pub type OtepdisR = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - desc OTEPDIS"]
pub type OtepdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - desc B2BSTUP"]
pub type B2bstupR = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - desc B2BSTUP"]
pub type B2bstupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EpdisdR {
        EpdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc STUP"]
    #[inline(always)]
    pub fn stup(&self) -> StupR {
        StupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&self) -> OtepdisR {
        OtepdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2bstupR {
        B2bstupR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<'_, Doepint1Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EpdisdW<'_, Doepint1Spec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - desc STUP"]
    #[inline(always)]
    pub fn stup(&mut self) -> StupW<'_, Doepint1Spec> {
        StupW::new(self, 3)
    }
    #[doc = "Bit 4 - desc OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OtepdisW<'_, Doepint1Spec> {
        OtepdisW::new(self, 4)
    }
    #[doc = "Bit 6 - desc B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2bstupW<'_, Doepint1Spec> {
        B2bstupW::new(self, 6)
    }
}
#[doc = "desc DOEPINT1\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepint1Spec;
impl crate::RegisterSpec for Doepint1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint1::R`](R) reader structure"]
impl crate::Readable for Doepint1Spec {}
#[doc = "`write(|w| ..)` method takes [`doepint1::W`](W) writer structure"]
impl crate::Writable for Doepint1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPINT1 to value 0"]
impl crate::Resettable for Doepint1Spec {}
