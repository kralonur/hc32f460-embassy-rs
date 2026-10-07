#[doc = "Register `DR6` reader"]
pub type R = crate::R<Dr6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR6\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr6Spec;
impl crate::RegisterSpec for Dr6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr6::R`](R) reader structure"]
impl crate::Readable for Dr6Spec {}
#[doc = "`reset()` method sets DR6 to value 0"]
impl crate::Resettable for Dr6Spec {}
