#[doc = "Register `UQID2` reader"]
pub type R = crate::R<Uqid2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc UQID2\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uqid2Spec;
impl crate::RegisterSpec for Uqid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uqid2::R`](R) reader structure"]
impl crate::Readable for Uqid2Spec {}
#[doc = "`reset()` method sets UQID2 to value 0"]
impl crate::Resettable for Uqid2Spec {}
