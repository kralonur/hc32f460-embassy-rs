#[doc = "Register `DTDBR` reader"]
pub type R = crate::R<DtdbrSpec>;
#[doc = "Register `DTDBR` writer"]
pub type W = crate::W<DtdbrSpec>;
#[doc = "Field `DTDB` reader - desc DTDB"]
pub type DtdbR = crate::FieldReader<u16>;
#[doc = "Field `DTDB` writer - desc DTDB"]
pub type DtdbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc DTDB"]
    #[inline(always)]
    pub fn dtdb(&self) -> DtdbR {
        DtdbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DTDB"]
    #[inline(always)]
    pub fn dtdb(&mut self) -> DtdbW<'_, DtdbrSpec> {
        DtdbW::new(self, 0)
    }
}
#[doc = "desc DTDBR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtdbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtdbrSpec;
impl crate::RegisterSpec for DtdbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtdbr::R`](R) reader structure"]
impl crate::Readable for DtdbrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtdbr::W`](W) writer structure"]
impl crate::Writable for DtdbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTDBR to value 0xffff"]
impl crate::Resettable for DtdbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
