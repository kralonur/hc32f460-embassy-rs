#[doc = "Register `UQID1` reader"]
pub type R = crate::R<Uqid1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc UQID1\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uqid1Spec;
impl crate::RegisterSpec for Uqid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uqid1::R`](R) reader structure"]
impl crate::Readable for Uqid1Spec {}
#[doc = "`reset()` method sets UQID1 to value 0"]
impl crate::Resettable for Uqid1Spec {}
