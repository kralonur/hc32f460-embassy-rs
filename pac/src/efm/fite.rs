#[doc = "Register `FITE` reader"]
pub type R = crate::R<FiteSpec>;
#[doc = "Register `FITE` writer"]
pub type W = crate::W<FiteSpec>;
#[doc = "Field `PEERRITE` reader - desc PEERRITE"]
pub type PeerriteR = crate::BitReader;
#[doc = "Field `PEERRITE` writer - desc PEERRITE"]
pub type PeerriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTENDITE` reader - desc OPTENDITE"]
pub type OptenditeR = crate::BitReader;
#[doc = "Field `OPTENDITE` writer - desc OPTENDITE"]
pub type OptenditeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLERRITE` reader - desc COLERRITE"]
pub type ColerriteR = crate::BitReader;
#[doc = "Field `COLERRITE` writer - desc COLERRITE"]
pub type ColerriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PEERRITE"]
    #[inline(always)]
    pub fn peerrite(&self) -> PeerriteR {
        PeerriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OPTENDITE"]
    #[inline(always)]
    pub fn optendite(&self) -> OptenditeR {
        OptenditeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc COLERRITE"]
    #[inline(always)]
    pub fn colerrite(&self) -> ColerriteR {
        ColerriteR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PEERRITE"]
    #[inline(always)]
    pub fn peerrite(&mut self) -> PeerriteW<'_, FiteSpec> {
        PeerriteW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OPTENDITE"]
    #[inline(always)]
    pub fn optendite(&mut self) -> OptenditeW<'_, FiteSpec> {
        OptenditeW::new(self, 1)
    }
    #[doc = "Bit 2 - desc COLERRITE"]
    #[inline(always)]
    pub fn colerrite(&mut self) -> ColerriteW<'_, FiteSpec> {
        ColerriteW::new(self, 2)
    }
}
#[doc = "desc FITE\n\nYou can [`read`](crate::Reg::read) this register and get [`fite::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fite::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiteSpec;
impl crate::RegisterSpec for FiteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fite::R`](R) reader structure"]
impl crate::Readable for FiteSpec {}
#[doc = "`write(|w| ..)` method takes [`fite::W`](W) writer structure"]
impl crate::Writable for FiteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FITE to value 0"]
impl crate::Resettable for FiteSpec {}
