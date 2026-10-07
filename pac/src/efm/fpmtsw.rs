#[doc = "Register `FPMTSW` reader"]
pub type R = crate::R<FpmtswSpec>;
#[doc = "Register `FPMTSW` writer"]
pub type W = crate::W<FpmtswSpec>;
#[doc = "Field `FPMTSW` reader - desc FPMTSW"]
pub type FpmtswR = crate::FieldReader<u32>;
#[doc = "Field `FPMTSW` writer - desc FPMTSW"]
pub type FpmtswW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - desc FPMTSW"]
    #[inline(always)]
    pub fn fpmtsw(&self) -> FpmtswR {
        FpmtswR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - desc FPMTSW"]
    #[inline(always)]
    pub fn fpmtsw(&mut self) -> FpmtswW<'_, FpmtswSpec> {
        FpmtswW::new(self, 0)
    }
}
#[doc = "desc FPMTSW\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmtsw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmtsw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpmtswSpec;
impl crate::RegisterSpec for FpmtswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpmtsw::R`](R) reader structure"]
impl crate::Readable for FpmtswSpec {}
#[doc = "`write(|w| ..)` method takes [`fpmtsw::W`](W) writer structure"]
impl crate::Writable for FpmtswSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPMTSW to value 0"]
impl crate::Resettable for FpmtswSpec {}
