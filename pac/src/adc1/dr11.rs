#[doc = "Register `DR11` reader"]
pub type R = crate::R<Dr11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR11\n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr11Spec;
impl crate::RegisterSpec for Dr11Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr11::R`](R) reader structure"]
impl crate::Readable for Dr11Spec {}
#[doc = "`reset()` method sets DR11 to value 0"]
impl crate::Resettable for Dr11Spec {}
