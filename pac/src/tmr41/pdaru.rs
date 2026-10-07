#[doc = "Register `PDARU` reader"]
pub type R = crate::R<PdaruSpec>;
#[doc = "Register `PDARU` writer"]
pub type W = crate::W<PdaruSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PDARU\n\nYou can [`read`](crate::Reg::read) this register and get [`pdaru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdaru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdaruSpec;
impl crate::RegisterSpec for PdaruSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdaru::R`](R) reader structure"]
impl crate::Readable for PdaruSpec {}
#[doc = "`write(|w| ..)` method takes [`pdaru::W`](W) writer structure"]
impl crate::Writable for PdaruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDARU to value 0"]
impl crate::Resettable for PdaruSpec {}
