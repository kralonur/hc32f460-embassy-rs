#[doc = "Register `CMPAR` reader"]
pub type R = crate::R<CmparSpec>;
#[doc = "Register `CMPAR` writer"]
pub type W = crate::W<CmparSpec>;
#[doc = "Field `CMPA` reader - desc CMPA"]
pub type CmpaR = crate::FieldReader<u16>;
#[doc = "Field `CMPA` writer - desc CMPA"]
pub type CmpaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CMPA"]
    #[inline(always)]
    pub fn cmpa(&self) -> CmpaR {
        CmpaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CMPA"]
    #[inline(always)]
    pub fn cmpa(&mut self) -> CmpaW<'_, CmparSpec> {
        CmpaW::new(self, 0)
    }
}
#[doc = "desc CMPAR\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmparSpec;
impl crate::RegisterSpec for CmparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpar::R`](R) reader structure"]
impl crate::Readable for CmparSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpar::W`](W) writer structure"]
impl crate::Writable for CmparSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPAR to value 0xffff"]
impl crate::Resettable for CmparSpec {
    const RESET_VALUE: u32 = 0xffff;
}
