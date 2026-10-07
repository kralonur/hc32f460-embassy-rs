#[doc = "Register `OCCRUL` reader"]
pub type R = crate::R<OccrulSpec>;
#[doc = "Register `OCCRUL` writer"]
pub type W = crate::W<OccrulSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrulSpec;
impl crate::RegisterSpec for OccrulSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrul::R`](R) reader structure"]
impl crate::Readable for OccrulSpec {}
#[doc = "`write(|w| ..)` method takes [`occrul::W`](W) writer structure"]
impl crate::Writable for OccrulSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRUL to value 0"]
impl crate::Resettable for OccrulSpec {}
