#[doc = "Register `DACR` reader"]
pub type R = crate::R<DacrSpec>;
#[doc = "Register `DACR` writer"]
pub type W = crate::W<DacrSpec>;
#[doc = "Field `DA1EN` reader - desc DA1EN"]
pub type Da1enR = crate::BitReader;
#[doc = "Field `DA1EN` writer - desc DA1EN"]
pub type Da1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA2EN` reader - desc DA2EN"]
pub type Da2enR = crate::BitReader;
#[doc = "Field `DA2EN` writer - desc DA2EN"]
pub type Da2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DA1EN"]
    #[inline(always)]
    pub fn da1en(&self) -> Da1enR {
        Da1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DA2EN"]
    #[inline(always)]
    pub fn da2en(&self) -> Da2enR {
        Da2enR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DA1EN"]
    #[inline(always)]
    pub fn da1en(&mut self) -> Da1enW<'_, DacrSpec> {
        Da1enW::new(self, 0)
    }
    #[doc = "Bit 1 - desc DA2EN"]
    #[inline(always)]
    pub fn da2en(&mut self) -> Da2enW<'_, DacrSpec> {
        Da2enW::new(self, 1)
    }
}
#[doc = "desc DACR\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacrSpec;
impl crate::RegisterSpec for DacrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dacr::R`](R) reader structure"]
impl crate::Readable for DacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dacr::W`](W) writer structure"]
impl crate::Writable for DacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACR to value 0"]
impl crate::Resettable for DacrSpec {}
