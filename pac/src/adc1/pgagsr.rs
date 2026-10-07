#[doc = "Register `PGAGSR` reader"]
pub type R = crate::R<PgagsrSpec>;
#[doc = "Register `PGAGSR` writer"]
pub type W = crate::W<PgagsrSpec>;
#[doc = "Field `GAIN` reader - desc GAIN"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - desc GAIN"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc GAIN"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc GAIN"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<'_, PgagsrSpec> {
        GainW::new(self, 0)
    }
}
#[doc = "desc PGAGSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pgagsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgagsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgagsrSpec;
impl crate::RegisterSpec for PgagsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgagsr::R`](R) reader structure"]
impl crate::Readable for PgagsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pgagsr::W`](W) writer structure"]
impl crate::Writable for PgagsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGAGSR to value 0"]
impl crate::Resettable for PgagsrSpec {}
