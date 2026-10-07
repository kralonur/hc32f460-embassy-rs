#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `RWREQ` reader - desc RWREQ"]
pub type RwreqR = crate::BitReader;
#[doc = "Field `RWREQ` writer - desc RWREQ"]
pub type RwreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWEN` reader - desc RWEN"]
pub type RwenR = crate::BitReader;
#[doc = "Field `RWEN` writer - desc RWEN"]
pub type RwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMF` reader - desc ALMF"]
pub type AlmfR = crate::BitReader;
#[doc = "Field `ALMF` writer - desc ALMF"]
pub type AlmfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDIE` reader - desc PRDIE"]
pub type PrdieR = crate::BitReader;
#[doc = "Field `PRDIE` writer - desc PRDIE"]
pub type PrdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIE` reader - desc ALMIE"]
pub type AlmieR = crate::BitReader;
#[doc = "Field `ALMIE` writer - desc ALMIE"]
pub type AlmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALME` reader - desc ALME"]
pub type AlmeR = crate::BitReader;
#[doc = "Field `ALME` writer - desc ALME"]
pub type AlmeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RWREQ"]
    #[inline(always)]
    pub fn rwreq(&self) -> RwreqR {
        RwreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RWEN"]
    #[inline(always)]
    pub fn rwen(&self) -> RwenR {
        RwenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ALMF"]
    #[inline(always)]
    pub fn almf(&self) -> AlmfR {
        AlmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PRDIE"]
    #[inline(always)]
    pub fn prdie(&self) -> PrdieR {
        PrdieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ALMIE"]
    #[inline(always)]
    pub fn almie(&self) -> AlmieR {
        AlmieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ALME"]
    #[inline(always)]
    pub fn alme(&self) -> AlmeR {
        AlmeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RWREQ"]
    #[inline(always)]
    pub fn rwreq(&mut self) -> RwreqW<'_, Cr2Spec> {
        RwreqW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RWEN"]
    #[inline(always)]
    pub fn rwen(&mut self) -> RwenW<'_, Cr2Spec> {
        RwenW::new(self, 1)
    }
    #[doc = "Bit 3 - desc ALMF"]
    #[inline(always)]
    pub fn almf(&mut self) -> AlmfW<'_, Cr2Spec> {
        AlmfW::new(self, 3)
    }
    #[doc = "Bit 5 - desc PRDIE"]
    #[inline(always)]
    pub fn prdie(&mut self) -> PrdieW<'_, Cr2Spec> {
        PrdieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ALMIE"]
    #[inline(always)]
    pub fn almie(&mut self) -> AlmieW<'_, Cr2Spec> {
        AlmieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc ALME"]
    #[inline(always)]
    pub fn alme(&mut self) -> AlmeW<'_, Cr2Spec> {
        AlmeW::new(self, 7)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
