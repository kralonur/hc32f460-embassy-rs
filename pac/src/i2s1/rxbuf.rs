#[doc = "Register `RXBUF` reader"]
pub type R = crate::R<RxbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RXBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbuf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbufSpec;
impl crate::RegisterSpec for RxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbuf::R`](R) reader structure"]
impl crate::Readable for RxbufSpec {}
#[doc = "`reset()` method sets RXBUF to value 0"]
impl crate::Resettable for RxbufSpec {}
