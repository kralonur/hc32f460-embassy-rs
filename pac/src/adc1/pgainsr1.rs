#[doc = "Register `PGAINSR1` reader"]
pub type R = crate::R<Pgainsr1Spec>;
#[doc = "Register `PGAINSR1` writer"]
pub type W = crate::W<Pgainsr1Spec>;
#[doc = "Field `PGAVSSEN` reader - desc PGAVSSEN"]
pub type PgavssenR = crate::BitReader;
#[doc = "Field `PGAVSSEN` writer - desc PGAVSSEN"]
pub type PgavssenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PGAVSSEN"]
    #[inline(always)]
    pub fn pgavssen(&self) -> PgavssenR {
        PgavssenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PGAVSSEN"]
    #[inline(always)]
    pub fn pgavssen(&mut self) -> PgavssenW<'_, Pgainsr1Spec> {
        PgavssenW::new(self, 0)
    }
}
#[doc = "desc PGAINSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pgainsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgainsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pgainsr1Spec;
impl crate::RegisterSpec for Pgainsr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgainsr1::R`](R) reader structure"]
impl crate::Readable for Pgainsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pgainsr1::W`](W) writer structure"]
impl crate::Writable for Pgainsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGAINSR1 to value 0"]
impl crate::Resettable for Pgainsr1Spec {}
