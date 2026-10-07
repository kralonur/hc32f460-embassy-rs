#[doc = "Register `PGAINSR0` reader"]
pub type R = crate::R<Pgainsr0Spec>;
#[doc = "Register `PGAINSR0` writer"]
pub type W = crate::W<Pgainsr0Spec>;
#[doc = "Field `PGAINSEL` reader - desc PGAINSEL"]
pub type PgainselR = crate::FieldReader<u16>;
#[doc = "Field `PGAINSEL` writer - desc PGAINSEL"]
pub type PgainselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc PGAINSEL"]
    #[inline(always)]
    pub fn pgainsel(&self) -> PgainselR {
        PgainselR::new(self.bits & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc PGAINSEL"]
    #[inline(always)]
    pub fn pgainsel(&mut self) -> PgainselW<'_, Pgainsr0Spec> {
        PgainselW::new(self, 0)
    }
}
#[doc = "desc PGAINSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`pgainsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgainsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pgainsr0Spec;
impl crate::RegisterSpec for Pgainsr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgainsr0::R`](R) reader structure"]
impl crate::Readable for Pgainsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pgainsr0::W`](W) writer structure"]
impl crate::Writable for Pgainsr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGAINSR0 to value 0"]
impl crate::Resettable for Pgainsr0Spec {}
