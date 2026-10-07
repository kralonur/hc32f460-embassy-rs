#[doc = "Register `SCMCR` reader"]
pub type R = crate::R<ScmcrSpec>;
#[doc = "Register `SCMCR` writer"]
pub type W = crate::W<ScmcrSpec>;
#[doc = "Field `SCMC` reader - desc SCMC"]
pub type ScmcR = crate::FieldReader<u16>;
#[doc = "Field `SCMC` writer - desc SCMC"]
pub type ScmcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc SCMC"]
    #[inline(always)]
    pub fn scmc(&self) -> ScmcR {
        ScmcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCMC"]
    #[inline(always)]
    pub fn scmc(&mut self) -> ScmcW<'_, ScmcrSpec> {
        ScmcW::new(self, 0)
    }
}
#[doc = "desc SCMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmcrSpec;
impl crate::RegisterSpec for ScmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmcr::R`](R) reader structure"]
impl crate::Readable for ScmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`scmcr::W`](W) writer structure"]
impl crate::Writable for ScmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMCR to value 0xffff"]
impl crate::Resettable for ScmcrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
