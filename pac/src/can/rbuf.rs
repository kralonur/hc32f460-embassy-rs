#[doc = "Register `RBUF` reader"]
pub type R = crate::R<RbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`rbuf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbufSpec;
impl crate::RegisterSpec for RbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf::R`](R) reader structure"]
impl crate::Readable for RbufSpec {}
#[doc = "`reset()` method sets RBUF to value 0"]
impl crate::Resettable for RbufSpec {}
