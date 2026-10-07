#[doc = "Register `UQID0` reader"]
pub type R = crate::R<Uqid0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc UQID0\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uqid0Spec;
impl crate::RegisterSpec for Uqid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uqid0::R`](R) reader structure"]
impl crate::Readable for Uqid0Spec {}
#[doc = "`reset()` method sets UQID0 to value 0"]
impl crate::Resettable for Uqid0Spec {}
