#[doc = "Register `PDBRV` reader"]
pub type R = crate::R<PdbrvSpec>;
#[doc = "Register `PDBRV` writer"]
pub type W = crate::W<PdbrvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PDBRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbrvSpec;
impl crate::RegisterSpec for PdbrvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdbrv::R`](R) reader structure"]
impl crate::Readable for PdbrvSpec {}
#[doc = "`write(|w| ..)` method takes [`pdbrv::W`](W) writer structure"]
impl crate::Writable for PdbrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDBRV to value 0"]
impl crate::Resettable for PdbrvSpec {}
