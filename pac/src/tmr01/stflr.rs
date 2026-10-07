#[doc = "Register `STFLR` reader"]
pub type R = crate::R<StflrSpec>;
#[doc = "Register `STFLR` writer"]
pub type W = crate::W<StflrSpec>;
#[doc = "Field `CMFA` reader - desc CMFA"]
pub type CmfaR = crate::BitReader;
#[doc = "Field `CMFA` writer - desc CMFA"]
pub type CmfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMFB` reader - desc CMFB"]
pub type CmfbR = crate::BitReader;
#[doc = "Field `CMFB` writer - desc CMFB"]
pub type CmfbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMFA"]
    #[inline(always)]
    pub fn cmfa(&self) -> CmfaR {
        CmfaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - desc CMFB"]
    #[inline(always)]
    pub fn cmfb(&self) -> CmfbR {
        CmfbR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMFA"]
    #[inline(always)]
    pub fn cmfa(&mut self) -> CmfaW<'_, StflrSpec> {
        CmfaW::new(self, 0)
    }
    #[doc = "Bit 16 - desc CMFB"]
    #[inline(always)]
    pub fn cmfb(&mut self) -> CmfbW<'_, StflrSpec> {
        CmfbW::new(self, 16)
    }
}
#[doc = "desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StflrSpec;
impl crate::RegisterSpec for StflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stflr::R`](R) reader structure"]
impl crate::Readable for StflrSpec {}
#[doc = "`write(|w| ..)` method takes [`stflr::W`](W) writer structure"]
impl crate::Writable for StflrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STFLR to value 0"]
impl crate::Resettable for StflrSpec {}
