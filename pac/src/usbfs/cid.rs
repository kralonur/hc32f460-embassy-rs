#[doc = "Register `CID` reader"]
pub type R = crate::R<CidSpec>;
#[doc = "Register `CID` writer"]
pub type W = crate::W<CidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc CID\n\nYou can [`read`](crate::Reg::read) this register and get [`cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CidSpec;
impl crate::RegisterSpec for CidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid::R`](R) reader structure"]
impl crate::Readable for CidSpec {}
#[doc = "`write(|w| ..)` method takes [`cid::W`](W) writer structure"]
impl crate::Writable for CidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CID to value 0x1234_5678"]
impl crate::Resettable for CidSpec {
    const RESET_VALUE: u32 = 0x1234_5678;
}
