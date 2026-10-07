#[doc = "Register `SCCRUL` reader"]
pub type R = crate::R<SccrulSpec>;
#[doc = "Register `SCCRUL` writer"]
pub type W = crate::W<SccrulSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrulSpec;
impl crate::RegisterSpec for SccrulSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrul::R`](R) reader structure"]
impl crate::Readable for SccrulSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrul::W`](W) writer structure"]
impl crate::Writable for SccrulSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRUL to value 0"]
impl crate::Resettable for SccrulSpec {}
