#[doc = "Register `WEEK` reader"]
pub type R = crate::R<WeekSpec>;
#[doc = "Register `WEEK` writer"]
pub type W = crate::W<WeekSpec>;
#[doc = "Field `WEEK` reader - desc WEEK"]
pub type WeekR = crate::FieldReader;
#[doc = "Field `WEEK` writer - desc WEEK"]
pub type WeekW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc WEEK"]
    #[inline(always)]
    pub fn week(&self) -> WeekR {
        WeekR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc WEEK"]
    #[inline(always)]
    pub fn week(&mut self) -> WeekW<'_, WeekSpec> {
        WeekW::new(self, 0)
    }
}
#[doc = "desc WEEK\n\nYou can [`read`](crate::Reg::read) this register and get [`week::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`week::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeekSpec;
impl crate::RegisterSpec for WeekSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`week::R`](R) reader structure"]
impl crate::Readable for WeekSpec {}
#[doc = "`write(|w| ..)` method takes [`week::W`](W) writer structure"]
impl crate::Writable for WeekSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEEK to value 0"]
impl crate::Resettable for WeekSpec {}
