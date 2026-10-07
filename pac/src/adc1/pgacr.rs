#[doc = "Register `PGACR` reader"]
pub type R = crate::R<PgacrSpec>;
#[doc = "Register `PGACR` writer"]
pub type W = crate::W<PgacrSpec>;
#[doc = "Field `PGACTL` reader - desc PGACTL"]
pub type PgactlR = crate::FieldReader;
#[doc = "Field `PGACTL` writer - desc PGACTL"]
pub type PgactlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc PGACTL"]
    #[inline(always)]
    pub fn pgactl(&self) -> PgactlR {
        PgactlR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc PGACTL"]
    #[inline(always)]
    pub fn pgactl(&mut self) -> PgactlW<'_, PgacrSpec> {
        PgactlW::new(self, 0)
    }
}
#[doc = "desc PGACR\n\nYou can [`read`](crate::Reg::read) this register and get [`pgacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgacrSpec;
impl crate::RegisterSpec for PgacrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgacr::R`](R) reader structure"]
impl crate::Readable for PgacrSpec {}
#[doc = "`write(|w| ..)` method takes [`pgacr::W`](W) writer structure"]
impl crate::Writable for PgacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGACR to value 0"]
impl crate::Resettable for PgacrSpec {}
