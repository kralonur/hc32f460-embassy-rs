#[doc = "Register `SCMBR` reader"]
pub type R = crate::R<ScmbrSpec>;
#[doc = "Register `SCMBR` writer"]
pub type W = crate::W<ScmbrSpec>;
#[doc = "Field `SCMB` reader - desc SCMB"]
pub type ScmbR = crate::FieldReader<u16>;
#[doc = "Field `SCMB` writer - desc SCMB"]
pub type ScmbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc SCMB"]
    #[inline(always)]
    pub fn scmb(&self) -> ScmbR {
        ScmbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCMB"]
    #[inline(always)]
    pub fn scmb(&mut self) -> ScmbW<'_, ScmbrSpec> {
        ScmbW::new(self, 0)
    }
}
#[doc = "desc SCMBR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmbrSpec;
impl crate::RegisterSpec for ScmbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmbr::R`](R) reader structure"]
impl crate::Readable for ScmbrSpec {}
#[doc = "`write(|w| ..)` method takes [`scmbr::W`](W) writer structure"]
impl crate::Writable for ScmbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMBR to value 0xffff"]
impl crate::Resettable for ScmbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
