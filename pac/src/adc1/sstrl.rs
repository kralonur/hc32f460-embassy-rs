#[doc = "Register `SSTRL` reader"]
pub type R = crate::R<SstrlSpec>;
#[doc = "Register `SSTRL` writer"]
pub type W = crate::W<SstrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`sstrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstrlSpec;
impl crate::RegisterSpec for SstrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstrl::R`](R) reader structure"]
impl crate::Readable for SstrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sstrl::W`](W) writer structure"]
impl crate::Writable for SstrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTRL to value 0x0b"]
impl crate::Resettable for SstrlSpec {
    const RESET_VALUE: u8 = 0x0b;
}
