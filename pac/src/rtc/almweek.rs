#[doc = "Register `ALMWEEK` reader"]
pub type R = crate::R<AlmweekSpec>;
#[doc = "Register `ALMWEEK` writer"]
pub type W = crate::W<AlmweekSpec>;
#[doc = "Field `ALMWEEK` reader - desc ALMWEEK"]
pub type AlmweekR = crate::FieldReader;
#[doc = "Field `ALMWEEK` writer - desc ALMWEEK"]
pub type AlmweekW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - desc ALMWEEK"]
    #[inline(always)]
    pub fn almweek(&self) -> AlmweekR {
        AlmweekR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc ALMWEEK"]
    #[inline(always)]
    pub fn almweek(&mut self) -> AlmweekW<'_, AlmweekSpec> {
        AlmweekW::new(self, 0)
    }
}
#[doc = "desc ALMWEEK\n\nYou can [`read`](crate::Reg::read) this register and get [`almweek::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almweek::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlmweekSpec;
impl crate::RegisterSpec for AlmweekSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`almweek::R`](R) reader structure"]
impl crate::Readable for AlmweekSpec {}
#[doc = "`write(|w| ..)` method takes [`almweek::W`](W) writer structure"]
impl crate::Writable for AlmweekSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALMWEEK to value 0"]
impl crate::Resettable for AlmweekSpec {}
