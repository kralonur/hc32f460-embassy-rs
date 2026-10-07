#[doc = "Register `PWCMR` reader"]
pub type R = crate::R<PwcmrSpec>;
#[doc = "Register `PWCMR` writer"]
pub type W = crate::W<PwcmrSpec>;
#[doc = "Field `ADBUFE` reader - desc ADBUFE"]
pub type AdbufeR = crate::BitReader;
#[doc = "Field `ADBUFE` writer - desc ADBUFE"]
pub type AdbufeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - desc ADBUFE"]
    #[inline(always)]
    pub fn adbufe(&self) -> AdbufeR {
        AdbufeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - desc ADBUFE"]
    #[inline(always)]
    pub fn adbufe(&mut self) -> AdbufeW<'_, PwcmrSpec> {
        AdbufeW::new(self, 7)
    }
}
#[doc = "desc PWCMR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwcmrSpec;
impl crate::RegisterSpec for PwcmrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwcmr::R`](R) reader structure"]
impl crate::Readable for PwcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwcmr::W`](W) writer structure"]
impl crate::Writable for PwcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWCMR to value 0"]
impl crate::Resettable for PwcmrSpec {}
