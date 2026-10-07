#[doc = "Register `SCMDR` reader"]
pub type R = crate::R<ScmdrSpec>;
#[doc = "Register `SCMDR` writer"]
pub type W = crate::W<ScmdrSpec>;
#[doc = "Field `SCMD` reader - desc SCMD"]
pub type ScmdR = crate::FieldReader<u16>;
#[doc = "Field `SCMD` writer - desc SCMD"]
pub type ScmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc SCMD"]
    #[inline(always)]
    pub fn scmd(&self) -> ScmdR {
        ScmdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCMD"]
    #[inline(always)]
    pub fn scmd(&mut self) -> ScmdW<'_, ScmdrSpec> {
        ScmdW::new(self, 0)
    }
}
#[doc = "desc SCMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmdrSpec;
impl crate::RegisterSpec for ScmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmdr::R`](R) reader structure"]
impl crate::Readable for ScmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`scmdr::W`](W) writer structure"]
impl crate::Writable for ScmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMDR to value 0xffff"]
impl crate::Resettable for ScmdrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
