#[doc = "Register `PDBRW` reader"]
pub type R = crate::R<PdbrwSpec>;
#[doc = "Register `PDBRW` writer"]
pub type W = crate::W<PdbrwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PDBRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbrwSpec;
impl crate::RegisterSpec for PdbrwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdbrw::R`](R) reader structure"]
impl crate::Readable for PdbrwSpec {}
#[doc = "`write(|w| ..)` method takes [`pdbrw::W`](W) writer structure"]
impl crate::Writable for PdbrwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDBRW to value 0"]
impl crate::Resettable for PdbrwSpec {}
