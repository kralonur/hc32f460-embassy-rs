#[doc = "Register `PSPCR` reader"]
pub type R = crate::R<PspcrSpec>;
#[doc = "Register `PSPCR` writer"]
pub type W = crate::W<PspcrSpec>;
#[doc = "Field `SPFE` reader - desc SPFE"]
pub type SpfeR = crate::FieldReader;
#[doc = "Field `SPFE` writer - desc SPFE"]
pub type SpfeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - desc SPFE"]
    #[inline(always)]
    pub fn spfe(&self) -> SpfeR {
        SpfeR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc SPFE"]
    #[inline(always)]
    pub fn spfe(&mut self) -> SpfeW<'_, PspcrSpec> {
        SpfeW::new(self, 0)
    }
}
#[doc = "desc PSPCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pspcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PspcrSpec;
impl crate::RegisterSpec for PspcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pspcr::R`](R) reader structure"]
impl crate::Readable for PspcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pspcr::W`](W) writer structure"]
impl crate::Writable for PspcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSPCR to value 0"]
impl crate::Resettable for PspcrSpec {}
