#[doc = "Register `MDSWCR` reader"]
pub type R = crate::R<MdswcrSpec>;
#[doc = "Register `MDSWCR` writer"]
pub type W = crate::W<MdswcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc MDSWCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mdswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdswcrSpec;
impl crate::RegisterSpec for MdswcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mdswcr::R`](R) reader structure"]
impl crate::Readable for MdswcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdswcr::W`](W) writer structure"]
impl crate::Writable for MdswcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDSWCR to value 0"]
impl crate::Resettable for MdswcrSpec {}
