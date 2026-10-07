#[doc = "Register `PWRCON` reader"]
pub type R = crate::R<PwrconSpec>;
#[doc = "Register `PWRCON` writer"]
pub type W = crate::W<PwrconSpec>;
#[doc = "Field `PWON` reader - desc PWON"]
pub type PwonR = crate::BitReader;
#[doc = "Field `PWON` writer - desc PWON"]
pub type PwonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PWON"]
    #[inline(always)]
    pub fn pwon(&self) -> PwonR {
        PwonR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PWON"]
    #[inline(always)]
    pub fn pwon(&mut self) -> PwonW<'_, PwrconSpec> {
        PwonW::new(self, 0)
    }
}
#[doc = "desc PWRCON\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrconSpec;
impl crate::RegisterSpec for PwrconSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrcon::R`](R) reader structure"]
impl crate::Readable for PwrconSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcon::W`](W) writer structure"]
impl crate::Writable for PwrconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCON to value 0"]
impl crate::Resettable for PwrconSpec {}
