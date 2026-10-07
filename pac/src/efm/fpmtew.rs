#[doc = "Register `FPMTEW` reader"]
pub type R = crate::R<FpmtewSpec>;
#[doc = "Register `FPMTEW` writer"]
pub type W = crate::W<FpmtewSpec>;
#[doc = "Field `FPMTEW` reader - desc FPMTEW"]
pub type FpmtewR = crate::FieldReader<u32>;
#[doc = "Field `FPMTEW` writer - desc FPMTEW"]
pub type FpmtewW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - desc FPMTEW"]
    #[inline(always)]
    pub fn fpmtew(&self) -> FpmtewR {
        FpmtewR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - desc FPMTEW"]
    #[inline(always)]
    pub fn fpmtew(&mut self) -> FpmtewW<'_, FpmtewSpec> {
        FpmtewW::new(self, 0)
    }
}
#[doc = "desc FPMTEW\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmtew::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmtew::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpmtewSpec;
impl crate::RegisterSpec for FpmtewSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpmtew::R`](R) reader structure"]
impl crate::Readable for FpmtewSpec {}
#[doc = "`write(|w| ..)` method takes [`fpmtew::W`](W) writer structure"]
impl crate::Writable for FpmtewSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPMTEW to value 0"]
impl crate::Resettable for FpmtewSpec {}
