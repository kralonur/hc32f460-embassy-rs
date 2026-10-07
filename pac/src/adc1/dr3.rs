#[doc = "Register `DR3` reader"]
pub type R = crate::R<Dr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR3\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr3Spec;
impl crate::RegisterSpec for Dr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr3::R`](R) reader structure"]
impl crate::Readable for Dr3Spec {}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for Dr3Spec {}
