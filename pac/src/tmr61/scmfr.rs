#[doc = "Register `SCMFR` reader"]
pub type R = crate::R<ScmfrSpec>;
#[doc = "Register `SCMFR` writer"]
pub type W = crate::W<ScmfrSpec>;
#[doc = "Field `SCMF` reader - desc SCMF"]
pub type ScmfR = crate::FieldReader<u16>;
#[doc = "Field `SCMF` writer - desc SCMF"]
pub type ScmfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc SCMF"]
    #[inline(always)]
    pub fn scmf(&self) -> ScmfR {
        ScmfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCMF"]
    #[inline(always)]
    pub fn scmf(&mut self) -> ScmfW<'_, ScmfrSpec> {
        ScmfW::new(self, 0)
    }
}
#[doc = "desc SCMFR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmfrSpec;
impl crate::RegisterSpec for ScmfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmfr::R`](R) reader structure"]
impl crate::Readable for ScmfrSpec {}
#[doc = "`write(|w| ..)` method takes [`scmfr::W`](W) writer structure"]
impl crate::Writable for ScmfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMFR to value 0xffff"]
impl crate::Resettable for ScmfrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
